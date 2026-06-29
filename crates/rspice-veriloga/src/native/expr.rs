#![allow(dead_code)]

use super::{JitError, JitResult};
use crate::canonical_ir::{EquationId, ExprId, HirAnalogOperator, HirExprKind, MirModel, NodeId};
use crate::codegen::{BytecodeProgram, CompiledModel, Instruction};
use crate::vm::{CURRENT_PAIR_GROUND, terminal_pair_current_index};
use smol_str::SmolStr;

const LOGICAL_EPSILON: f64 = 1.0e-15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    Assignment,
    ParameterDefault,
    StaticCondition,
    StampValue,
    Jacobian,
    ReactiveJacobian,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum VoltageNode {
    Terminal(usize),
    Internal(usize),
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NativeOp {
    Const(f64),
    LoadParam(usize),
    LoadParamGiven(usize),
    LoadPortConnected(usize),
    LoadVoltage { pos: VoltageNode, neg: VoltageNode },
    LoadCurrent(usize),
    LoadPriorCurrent(usize),
    LoadInternalVoltage(usize),
    LoadVariable(usize),
    LoadVariableDyn { base: usize, len: usize, lower: i64 },
    LoadBranchUnknown(usize),
    LoadTemperature,
    LoadThermalVoltage,
    LoadTime,
    Analysis(u8),
    LoadMfactor,
    Add,
    Sub,
    Mul,
    Div,
    AddConst(f64),
    SubConst(f64),
    MulConst(f64),
    DivConst(f64),
    SubFromConst(f64),
    DivFromConst(f64),
    Neg,
    Abs,
    Square,
    Sqrt,
    Compare(CompareOp),
    CompareConst(CompareOp, f64),
    Logical(LogicalOp),
    LogicalConst(LogicalOp, bool),
    IfElse,
    Extremum(ExtremumOp),
    ExtremumConst(ExtremumOp, f64),
    UnaryMath(UnaryMathOp),
    BinaryMath(BinaryMathOp),
    IntegerBinary(IntegerBinaryOp),
    TableLookup(usize),
    TableDerivative(usize),
    LimitState(usize),
    LaplaceState(usize),
    ZiState(usize),
    TimerState(usize),
    TransitionState(usize),
    SlewState(usize),
    AbsDelayState(usize),
    CrossState(usize),
    WhiteNoise,
    FlickerNoise,
    DdtState(usize),
    DdtJacobian,
    IdtState(usize),
    IdtJacobian,
    IdtModState(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompareOp {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogicalOp {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExtremumOp {
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnaryMathOp {
    Exp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Limexp,
    Asin,
    Acos,
    Atan,
    Floor,
    Ceil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryMathOp {
    Pow,
    Atan2,
    Hypot,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntegerBinaryOp {
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeProgram {
    ops: Vec<NativeOp>,
    max_stack_depth: usize,
    current_pair_dependencies: Vec<usize>,
    prior_current_dependencies: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeLoweringLimits<'a> {
    terminal_count: usize,
    internal_node_count: usize,
    parameter_count: usize,
    variable_count: usize,
    variable_names: &'a [SmolStr],
    branch_unknown_count: usize,
    lookup_table_count: usize,
    laplace_filter_count: usize,
    zi_filter_count: usize,
    available_current_pairs: &'a [usize],
    canonical_ddt_slots: &'a [(ExprId, usize)],
    canonical_idt_slots: &'a [(ExprId, usize)],
    canonical_idtmod_slots: &'a [(ExprId, usize)],
    canonical_transition_slots: &'a [(ExprId, usize)],
    canonical_slew_slots: &'a [(ExprId, usize)],
    canonical_absdelay_slots: &'a [(ExprId, usize)],
    canonical_laplace_slots: &'a [(ExprId, usize)],
    canonical_zi_slots: &'a [(ExprId, usize)],
    canonical_cross_slots: &'a [(ExprId, usize)],
    canonical_above_slots: &'a [(ExprId, usize)],
    canonical_timer_slots: &'a [(ExprId, usize)],
    canonical_limit_slots: &'a [(ExprId, usize)],
    canonical_table_lookup_slots: &'a [(ExprId, usize)],
}

impl<'a> NativeLoweringLimits<'a> {
    pub(crate) fn new(
        terminal_count: usize,
        internal_node_count: usize,
        parameter_count: usize,
        variable_count: usize,
        branch_unknown_count: usize,
    ) -> Self {
        Self {
            terminal_count,
            internal_node_count,
            parameter_count,
            variable_count,
            variable_names: &[],
            branch_unknown_count,
            lookup_table_count: 0,
            laplace_filter_count: 0,
            zi_filter_count: 0,
            available_current_pairs: &[],
            canonical_ddt_slots: &[],
            canonical_idt_slots: &[],
            canonical_idtmod_slots: &[],
            canonical_transition_slots: &[],
            canonical_slew_slots: &[],
            canonical_absdelay_slots: &[],
            canonical_laplace_slots: &[],
            canonical_zi_slots: &[],
            canonical_cross_slots: &[],
            canonical_above_slots: &[],
            canonical_timer_slots: &[],
            canonical_limit_slots: &[],
            canonical_table_lookup_slots: &[],
        }
    }

    pub(crate) fn for_model(model: &CompiledModel) -> NativeLoweringLimits<'_> {
        NativeLoweringLimits::new(
            model.num_terminals,
            model.internal_nodes,
            model.parameters.len(),
            model.num_variables,
            model.branch_sources.len(),
        )
        .with_variable_names(&model.variable_names)
        .with_lookup_table_count(model.lookup_tables.len())
        .with_laplace_filter_count(model.laplace_filters.len())
        .with_zi_filter_count(model.zi_filters.len())
    }

    pub(crate) fn with_available_current_pairs<'b>(
        self,
        available_current_pairs: &'b [usize],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_lookup_table_count(self, lookup_table_count: usize) -> Self {
        Self {
            lookup_table_count,
            ..self
        }
    }

    pub(crate) fn with_variable_names<'b>(
        self,
        variable_names: &'b [SmolStr],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: &[],
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_laplace_filter_count(self, laplace_filter_count: usize) -> Self {
        Self {
            laplace_filter_count,
            ..self
        }
    }

    pub(crate) fn with_zi_filter_count(self, zi_filter_count: usize) -> Self {
        Self {
            zi_filter_count,
            ..self
        }
    }

    pub(crate) fn with_canonical_ddt_slots<'b>(
        self,
        canonical_ddt_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_idt_slots<'b>(
        self,
        canonical_idt_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_idtmod_slots<'b>(
        self,
        canonical_idtmod_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_transition_slots<'b>(
        self,
        canonical_transition_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_slew_slots<'b>(
        self,
        canonical_slew_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_absdelay_slots<'b>(
        self,
        canonical_absdelay_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_laplace_slots<'b>(
        self,
        canonical_laplace_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_zi_slots<'b>(
        self,
        canonical_zi_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_cross_slots<'b>(
        self,
        canonical_cross_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_above_slots<'b>(
        self,
        canonical_above_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_timer_slots<'b>(
        self,
        canonical_timer_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_limit_slots<'b>(
        self,
        canonical_limit_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots,
            canonical_table_lookup_slots: self.canonical_table_lookup_slots,
        }
    }

    pub(crate) fn with_canonical_table_lookup_slots<'b>(
        self,
        canonical_table_lookup_slots: &'b [(ExprId, usize)],
    ) -> NativeLoweringLimits<'b>
    where
        'a: 'b,
    {
        NativeLoweringLimits {
            terminal_count: self.terminal_count,
            internal_node_count: self.internal_node_count,
            parameter_count: self.parameter_count,
            variable_count: self.variable_count,
            variable_names: self.variable_names,
            branch_unknown_count: self.branch_unknown_count,
            lookup_table_count: self.lookup_table_count,
            laplace_filter_count: self.laplace_filter_count,
            zi_filter_count: self.zi_filter_count,
            available_current_pairs: self.available_current_pairs,
            canonical_ddt_slots: self.canonical_ddt_slots,
            canonical_idt_slots: self.canonical_idt_slots,
            canonical_idtmod_slots: self.canonical_idtmod_slots,
            canonical_transition_slots: self.canonical_transition_slots,
            canonical_slew_slots: self.canonical_slew_slots,
            canonical_absdelay_slots: self.canonical_absdelay_slots,
            canonical_laplace_slots: self.canonical_laplace_slots,
            canonical_zi_slots: self.canonical_zi_slots,
            canonical_cross_slots: self.canonical_cross_slots,
            canonical_above_slots: self.canonical_above_slots,
            canonical_timer_slots: self.canonical_timer_slots,
            canonical_limit_slots: self.canonical_limit_slots,
            canonical_table_lookup_slots,
        }
    }

    fn canonical_ddt_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_ddt_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_idt_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_idt_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_idtmod_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_idtmod_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_transition_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_transition_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_slew_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_slew_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_absdelay_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_absdelay_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_laplace_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_laplace_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_zi_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_zi_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_cross_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_cross_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_above_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_above_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_timer_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_timer_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_limit_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_limit_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }

    fn canonical_table_lookup_slot(&self, expr_id: ExprId) -> Option<usize> {
        self.canonical_table_lookup_slots
            .iter()
            .find_map(|(id, slot)| (*id == expr_id).then_some(*slot))
    }
}

#[derive(Debug, Clone, Copy)]
enum CanonicalStateOperator {
    Ddt,
    Idt,
    IdtMod,
    Transition,
    Slew,
    Absdelay,
    Laplace,
    Zi,
    Cross,
    Above,
    Timer,
    Limit,
    TableLookup,
}

impl CanonicalStateOperator {
    fn name(self) -> &'static str {
        match self {
            Self::Ddt => "ddt",
            Self::Idt => "idt",
            Self::IdtMod => "idtmod",
            Self::Transition => "transition",
            Self::Slew => "slew",
            Self::Absdelay => "absdelay",
            Self::Laplace => "laplace",
            Self::Zi => "zi",
            Self::Cross => "cross",
            Self::Above => "above",
            Self::Timer => "timer",
            Self::Limit => "limit",
            Self::TableLookup => "table_model",
        }
    }

    fn bytecode_slot(self, instruction: &Instruction) -> Option<usize> {
        match (self, instruction) {
            (Self::Ddt, Instruction::DdtState(slot)) | (Self::Idt, Instruction::IdtState(slot)) => {
                Some(*slot)
            }
            (Self::IdtMod, Instruction::IdtModState(slot)) => Some(*slot),
            (Self::Transition, Instruction::TransitionState(slot)) => Some(*slot),
            (Self::Slew, Instruction::SlewState(slot)) => Some(*slot),
            (Self::Absdelay, Instruction::AbsDelayState(slot)) => Some(*slot),
            (Self::Laplace, Instruction::LaplaceState(slot)) => Some(*slot),
            (Self::Zi, Instruction::ZiState(slot)) => Some(*slot),
            (Self::Cross, Instruction::CrossState(slot)) => Some(*slot),
            (Self::Above, Instruction::AboveState(slot)) => Some(*slot),
            (Self::Timer, Instruction::TimerState(slot)) => Some(*slot),
            (Self::Limit, Instruction::LimitState(slot)) => Some(*slot),
            (Self::TableLookup, Instruction::TableLookup(slot)) => Some(*slot),
            _ => None,
        }
    }

    fn matches_call(self, name: &str, arg_count: usize) -> bool {
        let normalized = normalize_intrinsic_name(name);
        if normalized == "idtmod" {
            return match self {
                Self::Idt => arg_count <= 2,
                Self::IdtMod => arg_count >= 3,
                _ => false,
            };
        }
        match self {
            Self::Laplace => matches!(
                normalized.as_str(),
                "laplace_zp" | "laplace_zd" | "laplace_np" | "laplace_nd"
            ),
            Self::Zi => matches!(normalized.as_str(), "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"),
            _ => normalized == self.name(),
        }
    }

    fn matches_operator(self, op: &HirAnalogOperator) -> bool {
        match (self, op) {
            (Self::Ddt, HirAnalogOperator::Ddt { .. }) => true,
            (Self::Idt, HirAnalogOperator::Idt { .. }) => true,
            (Self::Idt, HirAnalogOperator::IdtMod { modulus: None, .. }) => true,
            (
                Self::IdtMod,
                HirAnalogOperator::IdtMod {
                    modulus: Some(_), ..
                },
            ) => true,
            (Self::Transition, HirAnalogOperator::Transition { .. }) => true,
            (Self::Slew, HirAnalogOperator::Slew { .. }) => true,
            (Self::Absdelay, HirAnalogOperator::Absdelay { .. }) => true,
            _ => false,
        }
    }
}

pub(crate) fn canonical_ddt_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Ddt,
    )
}

pub(crate) fn canonical_idt_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Idt,
    )
}

pub(crate) fn canonical_idtmod_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::IdtMod,
    )
}

pub(crate) fn canonical_transition_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Transition,
    )
}

pub(crate) fn canonical_slew_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Slew,
    )
}

pub(crate) fn canonical_absdelay_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Absdelay,
    )
}

pub(crate) fn canonical_laplace_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Laplace,
    )
}

pub(crate) fn canonical_zi_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Zi,
    )
}

pub(crate) fn canonical_cross_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Cross,
    )
}

pub(crate) fn canonical_above_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Above,
    )
}

pub(crate) fn canonical_timer_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Timer,
    )
}

pub(crate) fn canonical_limit_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::Limit,
    )
}

pub(crate) fn canonical_table_lookup_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
) -> JitResult<Vec<(ExprId, usize)>> {
    canonical_state_slots_for_equation(
        model,
        mir,
        equation_id,
        bytecode_program,
        CanonicalStateOperator::TableLookup,
    )
}

fn canonical_state_slots_for_equation(
    model: SmolStr,
    mir: &MirModel,
    equation_id: EquationId,
    bytecode_program: &BytecodeProgram,
    operator: CanonicalStateOperator,
) -> JitResult<Vec<(ExprId, usize)>> {
    let equation = mir.equations.get(usize::from(equation_id)).ok_or_else(|| {
        JitError::InvalidCanonicalIr {
            model: model.clone(),
            detail: format!("canonical equation {equation_id} is outside MIR equation arena")
                .into(),
        }
    })?;

    let mut canonical_exprs = Vec::new();
    collect_canonical_state_exprs(
        &model,
        mir,
        equation.expression.id,
        operator,
        &mut canonical_exprs,
    )?;

    let bytecode_slots = bytecode_program
        .instructions
        .iter()
        .filter_map(|instruction| operator.bytecode_slot(instruction))
        .collect::<Vec<_>>();

    if canonical_exprs.len() != bytecode_slots.len() {
        return Err(JitError::InvalidCanonicalIr {
            model,
            detail: format!(
                "canonical equation {equation_id} has {} {} operators but bytecode stamp has {} {}State slots",
                canonical_exprs.len(),
                operator.name(),
                bytecode_slots.len(),
                operator.name()
            )
            .into(),
        });
    }

    Ok(canonical_exprs.into_iter().zip(bytecode_slots).collect())
}

fn collect_canonical_state_exprs(
    model: &SmolStr,
    mir: &MirModel,
    expr_id: ExprId,
    operator: CanonicalStateOperator,
    slots: &mut Vec<ExprId>,
) -> JitResult<()> {
    let expression =
        mir.expressions
            .get(usize::from(expr_id))
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.clone(),
                detail: format!("canonical expression {expr_id} is outside MIR expression arena")
                    .into(),
            })?;

    match &expression.kind {
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => {}
        HirExprKind::SystemFunction { args, .. }
        | HirExprKind::Call { args, .. }
        | HirExprKind::ArrayLiteral { elements: args }
        | HirExprKind::NoiseSource { operands: args, .. } => {
            for arg in args {
                collect_canonical_state_exprs(model, mir, *arg, operator, slots)?;
            }
        }
        HirExprKind::Unary { operand, .. } | HirExprKind::ArrayAccess { index: operand, .. } => {
            collect_canonical_state_exprs(model, mir, *operand, operator, slots)?;
        }
        HirExprKind::Binary { left, right, .. } => {
            collect_canonical_state_exprs(model, mir, *left, operator, slots)?;
            collect_canonical_state_exprs(model, mir, *right, operator, slots)?;
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            collect_canonical_state_exprs(model, mir, *condition, operator, slots)?;
            collect_canonical_state_exprs(model, mir, *then_expr, operator, slots)?;
            collect_canonical_state_exprs(model, mir, *else_expr, operator, slots)?;
        }
        HirExprKind::AnalogOperator { op } => {
            collect_canonical_state_operator_children(model, mir, op, operator, slots)?;
        }
        HirExprKind::Laplace { expr, kind } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            match kind {
                crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles } => {
                    collect_canonical_state_expr_list(model, mir, zeros, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, poles, operator, slots)?;
                }
                crate::canonical_ir::HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
                    collect_canonical_state_expr_list(model, mir, zeros, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, denominator, operator, slots)?;
                }
                crate::canonical_ir::HirLaplaceKind::NumeratorPole { numerator, poles } => {
                    collect_canonical_state_expr_list(model, mir, numerator, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, poles, operator, slots)?;
                }
                crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                    numerator,
                    denominator,
                } => {
                    collect_canonical_state_expr_list(model, mir, numerator, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, denominator, operator, slots)?;
                }
            }
        }
        HirExprKind::Zi { expr, kind } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            match kind {
                crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles } => {
                    collect_canonical_state_expr_list(model, mir, zeros, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, poles, operator, slots)?;
                }
                crate::canonical_ir::HirZiKind::ZeroDenominator { zeros, denominator } => {
                    collect_canonical_state_expr_list(model, mir, zeros, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, denominator, operator, slots)?;
                }
                crate::canonical_ir::HirZiKind::NumeratorPole { numerator, poles } => {
                    collect_canonical_state_expr_list(model, mir, numerator, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, poles, operator, slots)?;
                }
                crate::canonical_ir::HirZiKind::NumeratorDenominator {
                    numerator,
                    denominator,
                } => {
                    collect_canonical_state_expr_list(model, mir, numerator, operator, slots)?;
                    collect_canonical_state_expr_list(model, mir, denominator, operator, slots)?;
                }
            }
        }
    }

    match &expression.kind {
        HirExprKind::SystemFunction { name, args } if operator.matches_call(name, args.len()) => {
            slots.push(expr_id);
        }
        HirExprKind::Call { name, args } if operator.matches_call(name, args.len()) => {
            slots.push(expr_id);
        }
        HirExprKind::AnalogOperator { op } if operator.matches_operator(op) => {
            slots.push(expr_id);
        }
        HirExprKind::Laplace { .. } if matches!(operator, CanonicalStateOperator::Laplace) => {
            slots.push(expr_id);
        }
        HirExprKind::Zi { .. } if matches!(operator, CanonicalStateOperator::Zi) => {
            slots.push(expr_id);
        }
        _ => {}
    }

    Ok(())
}

fn collect_canonical_state_operator_children(
    model: &SmolStr,
    mir: &MirModel,
    op: &HirAnalogOperator,
    operator: CanonicalStateOperator,
    slots: &mut Vec<ExprId>,
) -> JitResult<()> {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            if let Some(abstol) = abstol {
                collect_canonical_state_exprs(model, mir, *abstol, operator, slots)?;
            }
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            for child in [*ic, *assert, *abstol].into_iter().flatten() {
                collect_canonical_state_exprs(model, mir, child, operator, slots)?;
            }
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            for child in [*ic, *modulus, *offset, *abstol].into_iter().flatten() {
                collect_canonical_state_exprs(model, mir, child, operator, slots)?;
            }
        }
        HirAnalogOperator::Ddx { expr, probe } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            collect_canonical_state_exprs(model, mir, *probe, operator, slots)?;
        }
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            collect_canonical_state_exprs(model, mir, *delay, operator, slots)?;
            if let Some(max_delay) = max_delay {
                collect_canonical_state_exprs(model, mir, *max_delay, operator, slots)?;
            }
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            for child in [*delay, *rise, *fall, *tolerance].into_iter().flatten() {
                collect_canonical_state_exprs(model, mir, child, operator, slots)?;
            }
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
            for child in [*max_rise, *max_fall].into_iter().flatten() {
                collect_canonical_state_exprs(model, mir, child, operator, slots)?;
            }
        }
    }
    Ok(())
}

fn collect_canonical_state_expr_list(
    model: &SmolStr,
    mir: &MirModel,
    exprs: &[ExprId],
    operator: CanonicalStateOperator,
    slots: &mut Vec<ExprId>,
) -> JitResult<()> {
    for expr in exprs {
        collect_canonical_state_exprs(model, mir, *expr, operator, slots)?;
    }
    Ok(())
}

impl NativeProgram {
    pub(crate) fn from_bytecode(
        model: impl Into<SmolStr>,
        entry_kind: EntryKind,
        program: &BytecodeProgram,
        limits: NativeLoweringLimits<'_>,
    ) -> JitResult<Self> {
        let model = model.into();
        let mut ops = Vec::with_capacity(program.instructions.len());
        let mut current_pair_dependencies = Vec::new();
        let mut depth = 0usize;
        let mut max_stack_depth = 0usize;

        for instruction in &program.instructions {
            validate_entry_instruction(model.clone(), entry_kind, instruction)?;
            match instruction {
                Instruction::PushConst(value) => {
                    ops.push(NativeOp::Const(*value));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushParam(index) => {
                    validate_index(
                        model.clone(),
                        "PushParam parameter",
                        *index,
                        limits.parameter_count,
                    )?;
                    ops.push(NativeOp::LoadParam(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushParamGiven(index) => {
                    validate_index(
                        model.clone(),
                        "PushParamGiven parameter",
                        *index,
                        limits.parameter_count,
                    )?;
                    ops.push(NativeOp::LoadParamGiven(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushPortConnected(index) => {
                    validate_index(
                        model.clone(),
                        "PushPortConnected terminal",
                        *index,
                        limits.terminal_count,
                    )?;
                    ops.push(NativeOp::LoadPortConnected(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVoltage(pos, neg) => {
                    ops.push(NativeOp::LoadVoltage {
                        pos: lower_voltage_node(
                            model.clone(),
                            *pos,
                            limits.terminal_count,
                            limits.internal_node_count,
                        )?,
                        neg: lower_voltage_node(
                            model.clone(),
                            *neg,
                            limits.terminal_count,
                            limits.internal_node_count,
                        )?,
                    });
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushInternalVoltage(index) => {
                    validate_index(
                        model.clone(),
                        "PushInternalVoltage internal node",
                        *index,
                        limits.internal_node_count,
                    )?;
                    ops.push(NativeOp::LoadInternalVoltage(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVariable(index) => {
                    validate_index(
                        model.clone(),
                        "PushVariable variable",
                        *index,
                        limits.variable_count,
                    )?;
                    ops.push(NativeOp::LoadVariable(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVariableDyn { base, len, lower } => {
                    validate_range(
                        model.clone(),
                        "PushVariableDyn variable range",
                        *base,
                        *len,
                        limits.variable_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_dynamic_variable_read(&mut ops, *base, *len, *lower) {
                        continue;
                    }
                    ops.push(NativeOp::LoadVariableDyn {
                        base: *base,
                        len: *len,
                        lower: *lower,
                    });
                }
                Instruction::PushBranchCurrent(index) => {
                    validate_index(
                        model.clone(),
                        "PushBranchCurrent branch unknown",
                        *index,
                        limits.branch_unknown_count,
                    )?;
                    ops.push(NativeOp::LoadBranchUnknown(*index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushTemperature => {
                    ops.push(NativeOp::LoadTemperature);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushVt => {
                    ops.push(NativeOp::LoadThermalVoltage);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushTime => {
                    ops.push(NativeOp::LoadTime);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::Analysis(analysis_id) => {
                    ops.push(NativeOp::Analysis(*analysis_id));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::PushMfactor => {
                    ops.push(NativeOp::LoadMfactor);
                    push_stack(&mut depth, &mut max_stack_depth);
                }
                Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    if lower_constant_rhs_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    if lower_constant_lhs_identity_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    if lower_constant_lhs_noncommutative_arithmetic(&mut ops, instruction) {
                        continue;
                    }
                    ops.push(arithmetic_op(instruction));
                }
                Instruction::Pow | Instruction::FnPow => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_math(&mut ops, BinaryMathOp::Pow) {
                        continue;
                    }
                    if lower_constant_square_power(&mut ops) {
                        continue;
                    }
                    if lower_constant_identity_power(&mut ops) {
                        continue;
                    }
                    if lower_constant_reciprocal_power(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::BinaryMath(BinaryMathOp::Pow));
                }
                Instruction::Atan2 | Instruction::Mod => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = binary_math_op(instruction);
                    if lower_constant_binary_math(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::BinaryMath(op));
                }
                Instruction::Shl
                | Instruction::Shr
                | Instruction::BitAnd
                | Instruction::BitOr
                | Instruction::BitXor => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = integer_binary_op(instruction);
                    if lower_constant_integer_binary(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::IntegerBinary(op));
                }
                Instruction::TableLookup(table_id) => {
                    validate_index(
                        model.clone(),
                        "TableLookup table",
                        *table_id,
                        limits.lookup_table_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::TableLookup(*table_id));
                }
                Instruction::TableDerivative(table_id) => {
                    validate_index(
                        model.clone(),
                        "TableDerivative table",
                        *table_id,
                        limits.lookup_table_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::TableDerivative(*table_id));
                }
                Instruction::LimitState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::LimitState(*index));
                }
                Instruction::LaplaceState(filter_id) => {
                    validate_index(
                        model.clone(),
                        "LaplaceState filter",
                        *filter_id,
                        limits.laplace_filter_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::LaplaceState(*filter_id));
                }
                Instruction::ZiState(filter_id) => {
                    validate_index(
                        model.clone(),
                        "ZiState filter",
                        *filter_id,
                        limits.zi_filter_count,
                    )?;
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::ZiState(*filter_id));
                }
                Instruction::TimerState(timer_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::TimerState(*timer_id));
                }
                Instruction::TransitionState(filter_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        4,
                    )?;
                    depth -= 3;
                    ops.push(NativeOp::TransitionState(*filter_id));
                }
                Instruction::SlewState(filter_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        3,
                    )?;
                    depth -= 2;
                    ops.push(NativeOp::SlewState(*filter_id));
                }
                Instruction::AbsDelayState(buffer_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::AbsDelayState(*buffer_id));
                }
                Instruction::CrossState(detector_id) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::CrossState(*detector_id));
                }
                Instruction::WhiteNoise => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::WhiteNoise);
                }
                Instruction::FlickerNoise => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::FlickerNoise);
                }
                Instruction::Neg => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_neg(&mut ops) {
                        continue;
                    }
                    if lower_unary_composition(&mut ops, NativeOp::Neg) {
                        continue;
                    }
                    ops.push(NativeOp::Neg);
                }
                Instruction::Abs => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_abs(&mut ops) {
                        continue;
                    }
                    if lower_unary_composition(&mut ops, NativeOp::Abs) {
                        continue;
                    }
                    ops.push(NativeOp::Abs);
                }
                Instruction::Sqrt => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_sqrt(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::Sqrt);
                }
                Instruction::Exp
                | Instruction::Log
                | Instruction::Log10
                | Instruction::Sin
                | Instruction::Cos
                | Instruction::Tan
                | Instruction::Sinh
                | Instruction::Cosh
                | Instruction::Tanh
                | Instruction::Limexp
                | Instruction::Asin
                | Instruction::Acos
                | Instruction::Atan
                | Instruction::Floor
                | Instruction::Ceil => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    let op = unary_math_op(instruction);
                    if lower_constant_unary_math(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::UnaryMath(op));
                }
                Instruction::DdtState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::DdtState(*index));
                }
                Instruction::DdtJacobian => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::DdtJacobian);
                }
                Instruction::IdtState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        2,
                    )?;
                    depth -= 1;
                    ops.push(NativeOp::IdtState(*index));
                }
                Instruction::IdtJacobian => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    ops.push(NativeOp::IdtJacobian);
                }
                Instruction::IdtModState(index) => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        4,
                    )?;
                    depth -= 3;
                    ops.push(NativeOp::IdtModState(*index));
                }
                Instruction::Gt
                | Instruction::Lt
                | Instruction::Ge
                | Instruction::Le
                | Instruction::Eq
                | Instruction::Ne => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = compare_op(instruction);
                    if lower_constant_binary_compare(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_rhs_compare(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_lhs_compare(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::Compare(op));
                }
                Instruction::And | Instruction::Or => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    let op = logical_op(instruction);
                    if lower_constant_binary_logical(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_rhs_logical(&mut ops, op) {
                        continue;
                    }
                    if lower_constant_lhs_logical(&mut ops, op) {
                        continue;
                    }
                    ops.push(NativeOp::Logical(op));
                }
                Instruction::Not => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        1,
                    )?;
                    if lower_constant_logical_not(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::Logical(LogicalOp::Not));
                }
                Instruction::IfElse => {
                    require_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                        3,
                    )?;
                    depth -= 2;
                    if lower_constant_ifelse(&mut ops) {
                        continue;
                    }
                    ops.push(NativeOp::IfElse);
                }
                Instruction::Min | Instruction::Max => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_extremum(&mut ops, extremum_op(instruction)) {
                        continue;
                    }
                    if lower_constant_rhs_extremum(&mut ops, extremum_op(instruction)) {
                        continue;
                    }
                    ops.push(NativeOp::Extremum(extremum_op(instruction)));
                }
                Instruction::AboveState(_) => {
                    pop_binary_stack(
                        model.clone(),
                        entry_kind,
                        instruction_name(instruction),
                        depth,
                    )?;
                    depth -= 1;
                    if lower_constant_binary_compare(&mut ops, CompareOp::Gt) {
                        continue;
                    }
                    if lower_constant_rhs_compare(&mut ops, CompareOp::Gt) {
                        continue;
                    }
                    ops.push(NativeOp::Compare(CompareOp::Gt));
                }
                Instruction::PushCurrent(pos, neg) => {
                    let pair_index =
                        current_pair_index(model.clone(), *pos, *neg, limits.terminal_count)?;
                    if !limits.available_current_pairs.contains(&pair_index) {
                        return Err(JitError::unsupported_program_op(
                            model,
                            format!(
                                "PushCurrent terminal pair {} unavailable",
                                format_current_pair(*pos, *neg)
                            ),
                        ));
                    }
                    if !current_pair_dependencies.contains(&pair_index) {
                        current_pair_dependencies.push(pair_index);
                    }
                    ops.push(NativeOp::LoadCurrent(pair_index));
                    push_stack(&mut depth, &mut max_stack_depth);
                }
            }
        }

        if depth != 1 {
            return Err(stack_error(
                model,
                entry_kind,
                format!("final stack depth {depth}, expected 1"),
            ));
        }
        let optimized_max_stack_depth =
            compute_native_max_stack_depth(model.clone(), entry_kind, &ops)?;
        debug_assert!(optimized_max_stack_depth <= max_stack_depth);

        Ok(Self {
            ops,
            max_stack_depth: optimized_max_stack_depth,
            current_pair_dependencies,
            prior_current_dependencies: Vec::new(),
        })
    }

    pub(crate) fn from_mir_equation(
        model: impl Into<SmolStr>,
        entry_kind: EntryKind,
        mir: &MirModel,
        equation_id: EquationId,
        limits: NativeLoweringLimits<'_>,
    ) -> JitResult<Self> {
        let model = model.into();
        mir.validate()
            .map_err(|diagnostics| JitError::InvalidCanonicalIr {
                model: model.clone(),
                detail: diagnostics
                    .first()
                    .map(|diagnostic| diagnostic.message.clone())
                    .unwrap_or_else(|| "MIR validation failed".into())
                    .into(),
            })?;

        let equation = mir
            .equations
            .get(usize::from(equation_id))
            .filter(|equation| equation.id == equation_id)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.clone(),
                detail: format!("MIR equation {equation_id} is outside equation arena").into(),
            })?;

        let mut lowerer =
            MirEquationLowerer::new(model.clone(), entry_kind, mir, equation_id, limits);
        lowerer.lower(equation.expression.id)?;

        if lowerer.depth != 1 {
            return Err(stack_error(
                model.clone(),
                entry_kind,
                format!("final stack depth {}, expected 1", lowerer.depth),
            ));
        }
        let optimized_max_stack_depth =
            compute_native_max_stack_depth(model, entry_kind, &lowerer.ops)?;
        debug_assert!(optimized_max_stack_depth <= lowerer.max_stack_depth);

        Ok(Self {
            ops: lowerer.ops,
            max_stack_depth: optimized_max_stack_depth,
            current_pair_dependencies: lowerer.current_pair_dependencies,
            prior_current_dependencies: lowerer.prior_current_dependencies,
        })
    }

    pub(crate) fn ops(&self) -> &[NativeOp] {
        &self.ops
    }

    pub(crate) fn max_stack_depth(&self) -> usize {
        self.max_stack_depth
    }

    pub(crate) fn current_pair_dependencies(&self) -> &[usize] {
        &self.current_pair_dependencies
    }

    pub(crate) fn prior_current_dependencies(&self) -> &[usize] {
        &self.prior_current_dependencies
    }

    pub(crate) fn validate_dependency_metadata(&self) -> JitResult<()> {
        let current_pair_dependencies = collect_current_pair_dependencies(&self.ops);
        validate_dependency_list(
            "current-pair",
            &self.current_pair_dependencies,
            &current_pair_dependencies,
        )?;

        let prior_current_dependencies = collect_prior_current_dependencies(&self.ops);
        validate_dependency_list(
            "prior-current",
            &self.prior_current_dependencies,
            &prior_current_dependencies,
        )
    }
}

fn collect_current_pair_dependencies(ops: &[NativeOp]) -> Vec<usize> {
    let mut dependencies = Vec::new();
    for op in ops {
        if let NativeOp::LoadCurrent(index) = *op {
            push_unique_dependency(&mut dependencies, index);
        }
    }
    dependencies
}

fn collect_prior_current_dependencies(ops: &[NativeOp]) -> Vec<usize> {
    let mut dependencies = Vec::new();
    for op in ops {
        if let NativeOp::LoadPriorCurrent(index) = *op {
            push_unique_dependency(&mut dependencies, index);
        }
    }
    dependencies
}

fn push_unique_dependency(dependencies: &mut Vec<usize>, index: usize) {
    if !dependencies.contains(&index) {
        dependencies.push(index);
    }
}

fn validate_dependency_list(name: &str, recorded: &[usize], actual: &[usize]) -> JitResult<()> {
    if recorded == actual {
        return Ok(());
    }

    Err(JitError::InternalCompilerError {
        model: "native-program".into(),
        detail: format!(
            "{name} dependency metadata mismatch: recorded {recorded:?}, op stream requires {actual:?}"
        )
        .into(),
    })
}

struct MirEquationLowerer<'a, 'limits> {
    model: SmolStr,
    entry_kind: EntryKind,
    mir: &'a MirModel,
    equation_id: EquationId,
    limits: NativeLoweringLimits<'limits>,
    ops: Vec<NativeOp>,
    depth: usize,
    max_stack_depth: usize,
    current_pair_dependencies: Vec<usize>,
    prior_current_dependencies: Vec<usize>,
}

impl<'a, 'limits> MirEquationLowerer<'a, 'limits> {
    fn new(
        model: SmolStr,
        entry_kind: EntryKind,
        mir: &'a MirModel,
        equation_id: EquationId,
        limits: NativeLoweringLimits<'limits>,
    ) -> Self {
        Self {
            model,
            entry_kind,
            mir,
            equation_id,
            limits,
            ops: Vec::new(),
            depth: 0,
            max_stack_depth: 0,
            current_pair_dependencies: Vec::new(),
            prior_current_dependencies: Vec::new(),
        }
    }

    fn lower(&mut self, expr_id: ExprId) -> JitResult<()> {
        let expression = self.expression(expr_id)?;
        match &expression.kind {
            HirExprKind::Number { value, .. } => self.push(NativeOp::Const(*value)),
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str()),
            HirExprKind::ArrayAccess { array, index } => {
                self.lower_array_access(array.as_str(), *index)
            }
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.lower_branch_access(access.as_str(), pos.as_str(), neg.as_deref())
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access.as_str(), name.as_str())
            }
            HirExprKind::Unary { op, operand } => self.lower_unary(op.as_str(), *operand),
            HirExprKind::Binary { op, left, right } => {
                self.lower_binary(op.as_str(), *left, *right)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.lower(*condition)?;
                self.lower(*then_expr)?;
                self.lower(*else_expr)?;
                self.append_ifelse()
            }
            HirExprKind::SystemFunction { name, args } => {
                self.lower_system_function_call(expression.id, name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, args } => match normalize_intrinsic_name(name).as_str() {
                "ddt" => self.lower_ddt_operator(expression.id, args.as_slice(), None),
                "idt" => self.lower_idt_operator(expression.id, args.as_slice(), None, None, None),
                "idtmod" => self.lower_idtmod_call(expression.id, args.as_slice()),
                "transition" => self.lower_transition_call(expression.id, args.as_slice()),
                "slew" => self.lower_slew_call(expression.id, args.as_slice()),
                "absdelay" => self.lower_absdelay_call(expression.id, args.as_slice()),
                "laplace_zp" | "laplace_zd" | "laplace_np" | "laplace_nd" => {
                    self.lower_laplace_call(expression.id, args.as_slice())
                }
                "zi_zp" | "zi_zd" | "zi_np" | "zi_nd" => {
                    self.lower_zi_call(expression.id, args.as_slice())
                }
                "cross" => self.lower_cross_call(expression.id, args.as_slice()),
                "above" => self.lower_above_call(expression.id, args.as_slice()),
                "timer" => self.lower_timer_call(expression.id, args.as_slice()),
                "ddx" => self.lower_ddx_call(args.as_slice()),
                _ => self.lower_intrinsic_call(name.as_str(), args.as_slice()),
            },
            HirExprKind::AnalogOperator { op } => self.lower_analog_operator(expression.id, op),
            HirExprKind::Laplace { expr, .. } => self.lower_laplace_operator(expression.id, *expr),
            HirExprKind::Zi { expr, .. } => self.lower_zi_operator(expression.id, *expr),
            HirExprKind::NoiseSource {
                source, operands, ..
            } => self.lower_noise_source(source.as_str(), operands.as_slice()),
            HirExprKind::StringLiteral { .. } | HirExprKind::ArrayLiteral { .. } => Err(self
                .unsupported(format!(
                    "expression kind {}",
                    expression_kind_name(&expression.kind)
                ))),
        }
    }

    fn lower_noise_source(&mut self, source: &str, operands: &[ExprId]) -> JitResult<()> {
        match source.to_ascii_lowercase().as_str() {
            "white" => {
                if operands.len() != 1 {
                    return Err(self.unsupported(format!(
                        "noise source {source} expects one operand, found {}",
                        operands.len()
                    )));
                }
                self.lower(operands[0])?;
                self.ops.push(NativeOp::WhiteNoise);
                Ok(())
            }
            "flicker" => {
                if operands.len() != 2 {
                    return Err(self.unsupported(format!(
                        "noise source {source} expects two operands, found {}",
                        operands.len()
                    )));
                }
                self.lower(operands[0])?;
                self.lower(operands[1])?;
                self.pop_binary("canonical flicker noise source")?;
                self.ops.push(NativeOp::FlickerNoise);
                Ok(())
            }
            "table" => self.push(NativeOp::Const(0.0)),
            _ => Err(self.unsupported(format!("noise source {source}"))),
        }
    }

    fn lower_system_function_call(
        &mut self,
        expr_id: ExprId,
        name: &str,
        args: &[ExprId],
    ) -> JitResult<()> {
        match normalize_intrinsic_name(name).as_str() {
            "limit" => self.lower_limit_call(expr_id, args),
            "table_model" => self.lower_table_model_call(expr_id, args),
            _ => self.lower_intrinsic_call(name, args),
        }
    }

    fn lower_analog_operator(&mut self, expr_id: ExprId, op: &HirAnalogOperator) -> JitResult<()> {
        match op {
            HirAnalogOperator::Ddt { expr, abstol } => {
                self.lower_ddt_operator(expr_id, &[*expr], *abstol)
            }
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => self.lower_idt_operator(expr_id, &[*expr], *ic, *assert, *abstol),
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => self.lower_idtmod_operator(expr_id, *expr, *ic, *modulus, *offset, *abstol),
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
            } => self.lower_transition_operator(expr_id, *expr, *delay, *rise, *fall, *tolerance),
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => self.lower_slew_operator(expr_id, *expr, *max_rise, *max_fall),
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => self.lower_absdelay_operator(expr_id, *expr, Some(*delay), *max_delay),
            HirAnalogOperator::Ddx { expr, probe } => self.lower_ddx_projection(*expr, *probe),
            HirAnalogOperator::Limexp { expr } => {
                self.lower(*expr)?;
                if lower_constant_unary_math(&mut self.ops, UnaryMathOp::Limexp) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::UnaryMath(UnaryMathOp::Limexp))
                }
            }
            _ => Err(self.unsupported(format!("analog operator {}", analog_operator_name(op)))),
        }
    }

    fn lower_ddx_call(&mut self, args: &[ExprId]) -> JitResult<()> {
        let [expr, probe] = args else {
            return Err(self.unsupported(format!(
                "analog operator ddx expects two operands, found {}",
                args.len()
            )));
        };
        self.lower_ddx_projection(*expr, *probe)
    }

    fn lower_ddx_projection(&mut self, expr: ExprId, probe: ExprId) -> JitResult<()> {
        let (pos, neg) = self.ddx_probe_nodes(probe)?;
        if let Some(pos) = pos {
            self.lower_derivative(expr, pos)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }

        if let Some(neg) = neg {
            self.lower_derivative(expr, neg)?;
            self.append_arithmetic("Sub")?;
            self.push(NativeOp::Const(0.5))?;
            self.append_arithmetic("Mul")?;
        }

        Ok(())
    }

    fn ddx_probe_nodes(&self, probe: ExprId) -> JitResult<(Option<NodeId>, Option<NodeId>)> {
        let expression = self.expression(probe)?;
        match &expression.kind {
            HirExprKind::BranchAccess { access, pos, neg } if !is_flow_access(access) => Ok((
                self.branch_endpoint(pos.as_str())?,
                neg.as_deref()
                    .map(|node| self.branch_endpoint(node))
                    .transpose()?
                    .flatten(),
            )),
            HirExprKind::NamedBranchAccess { access, name } if !is_flow_access(access) => {
                let branch = self
                    .mir
                    .branches
                    .iter()
                    .find(|branch| branch.name.as_str() == name.as_str())
                    .ok_or_else(|| self.unsupported(format!("unknown ddx probe branch {name}")))?;
                Ok((branch.pos_node, branch.neg_node))
            }
            other => Err(self.unsupported(format!(
                "ddx probe must be a voltage access, found {}",
                expression_kind_name(other)
            ))),
        }
    }

    fn lower_derivative(&mut self, expr_id: ExprId, wrt: NodeId) -> JitResult<()> {
        let expression = self.expression(expr_id)?;
        match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::ArrayLiteral { .. }
            | HirExprKind::NoiseSource { .. } => self.push(NativeOp::Const(0.0)),
            HirExprKind::Identifier { name } => self.lower_identifier_derivative(name.as_str()),
            HirExprKind::BranchAccess { access, pos, neg } => self.lower_branch_access_derivative(
                access.as_str(),
                pos.as_str(),
                neg.as_deref(),
                wrt,
            ),
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access_derivative(access.as_str(), name.as_str(), wrt)
            }
            HirExprKind::Unary { op, operand } => {
                self.lower_unary_derivative(op.as_str(), *operand, wrt)
            }
            HirExprKind::Binary { op, left, right } => {
                self.lower_binary_derivative(op.as_str(), *left, *right, wrt)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.lower(*condition)?;
                self.lower_derivative(*then_expr, wrt)?;
                self.lower_derivative(*else_expr, wrt)?;
                self.append_ifelse()
            }
            HirExprKind::SystemFunction { name, .. } => {
                if matches!(
                    normalize_intrinsic_name(name).as_str(),
                    "temperature"
                        | "vt"
                        | "thermal_vt"
                        | "abstime"
                        | "realtime"
                        | "mfactor"
                        | "simparam"
                        | "param_given"
                        | "port_connected"
                        | "analysis"
                ) {
                    self.push(NativeOp::Const(0.0))
                } else {
                    Err(self.unsupported(format!("ddx derivative of system function '{name}'")))
                }
            }
            HirExprKind::Call { name, args } => {
                self.lower_call_derivative(name.as_str(), args.as_slice(), wrt)
            }
            HirExprKind::ArrayAccess { array, .. } => {
                Err(self.unsupported(format!("ddx derivative through array access {array}")))
            }
            HirExprKind::AnalogOperator { op } => self.lower_analog_operator_derivative(op, wrt),
            HirExprKind::Laplace { .. } | HirExprKind::Zi { .. } => Err(self.unsupported(format!(
                "ddx derivative of {}",
                expression_kind_name(&expression.kind)
            ))),
        }
    }

    fn lower_identifier_derivative(&mut self, name: &str) -> JitResult<()> {
        if self
            .mir
            .parameters
            .iter()
            .any(|parameter| parameter.name.as_str() == name)
        {
            return self.push(NativeOp::Const(0.0));
        }
        if self
            .limits
            .variable_names
            .iter()
            .any(|variable| variable.as_str() == name)
        {
            return Err(self.unsupported(format!("ddx derivative through variable {name}")));
        }
        Err(self.unsupported(format!("ddx derivative of identifier {name}")))
    }

    fn lower_branch_access_derivative(
        &mut self,
        access: &str,
        pos: &str,
        neg: Option<&str>,
        wrt: NodeId,
    ) -> JitResult<()> {
        if is_flow_access(access) {
            return self.push(NativeOp::Const(0.0));
        }
        let pos = self.branch_endpoint(pos)?;
        let neg = neg
            .map(|node| self.branch_endpoint(node))
            .transpose()?
            .flatten();
        self.push(NativeOp::Const(branch_voltage_derivative(pos, neg, wrt)))
    }

    fn lower_named_branch_access_derivative(
        &mut self,
        access: &str,
        name: &str,
        wrt: NodeId,
    ) -> JitResult<()> {
        if is_flow_access(access) {
            return self.push(NativeOp::Const(0.0));
        }
        let branch = self
            .mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown branch access {name}")))?;
        self.push(NativeOp::Const(branch_voltage_derivative(
            branch.pos_node,
            branch.neg_node,
            wrt,
        )))
    }

    fn lower_unary_derivative(&mut self, op: &str, operand: ExprId, wrt: NodeId) -> JitResult<()> {
        match op {
            "Pos" => self.lower_derivative(operand, wrt),
            "Neg" => {
                self.lower_derivative(operand, wrt)?;
                if lower_constant_neg(&mut self.ops) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::Neg)
                }
            }
            "Not" | "BitNot" => self.push(NativeOp::Const(0.0)),
            _ => Err(self.unsupported(format!("ddx derivative of unary operator {op}"))),
        }
    }

    fn lower_binary_derivative(
        &mut self,
        op: &str,
        left: ExprId,
        right: ExprId,
        wrt: NodeId,
    ) -> JitResult<()> {
        match op {
            "Add" | "Sub" => {
                self.lower_derivative(left, wrt)?;
                self.lower_derivative(right, wrt)?;
                self.append_arithmetic(op)
            }
            "Mul" => {
                self.lower_derivative(left, wrt)?;
                self.lower(right)?;
                self.append_arithmetic("Mul")?;
                self.lower(left)?;
                self.lower_derivative(right, wrt)?;
                self.append_arithmetic("Mul")?;
                self.append_arithmetic("Add")
            }
            "Div" => {
                self.lower_derivative(left, wrt)?;
                self.lower(right)?;
                self.append_arithmetic("Mul")?;
                self.lower(left)?;
                self.lower_derivative(right, wrt)?;
                self.append_arithmetic("Mul")?;
                self.append_arithmetic("Sub")?;
                self.lower(right)?;
                self.lower(right)?;
                self.append_arithmetic("Mul")?;
                self.append_arithmetic("Div")
            }
            "Pow" => self.lower_pow_derivative(left, right, wrt),
            "Mod" | "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" | "And" | "Or" | "BitAnd" | "BitOr"
            | "BitXor" | "Shl" | "Shr" => self.push(NativeOp::Const(0.0)),
            _ => Err(self.unsupported(format!("ddx derivative of binary operator {op}"))),
        }
    }

    fn lower_pow_derivative(&mut self, left: ExprId, right: ExprId, wrt: NodeId) -> JitResult<()> {
        let exponent = self
            .constant_number(right)
            .ok_or_else(|| self.unsupported("ddx derivative of non-constant power exponent"))?;
        self.push(NativeOp::Const(exponent))?;
        self.lower(left)?;
        self.push(NativeOp::Const(exponent - 1.0))?;
        self.append_binary_math_op(BinaryMathOp::Pow)?;
        self.append_arithmetic("Mul")?;
        self.lower_derivative(left, wrt)?;
        self.append_arithmetic("Mul")
    }

    fn lower_call_derivative(&mut self, name: &str, args: &[ExprId], wrt: NodeId) -> JitResult<()> {
        let normalized = normalize_intrinsic_name(name);
        match normalized.as_str() {
            "ddx" => Err(self.unsupported("nested ddx derivative")),
            "abs" | "fabs" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                self.push(NativeOp::Const(0.0))?;
                self.append_compare("Ge")?;
                self.push(NativeOp::Const(1.0))?;
                self.push(NativeOp::Const(-1.0))?;
                self.append_ifelse()?;
                self.lower_derivative(args[0], wrt)?;
                self.append_arithmetic("Mul")
            }
            "sqrt" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.push(NativeOp::Const(0.5))?;
                self.lower(args[0])?;
                self.append_unary(NativeOp::Sqrt)?;
                self.append_arithmetic("Div")?;
                self.lower_derivative(args[0], wrt)?;
                self.append_arithmetic("Mul")
            }
            "exp" => self.lower_unary_chain_derivative(name, args, wrt, UnaryMathOp::Exp),
            "ln" | "log" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower_derivative(args[0], wrt)?;
                self.lower(args[0])?;
                self.append_arithmetic("Div")
            }
            "sin" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                self.append_unary(NativeOp::UnaryMath(UnaryMathOp::Cos))?;
                self.lower_derivative(args[0], wrt)?;
                self.append_arithmetic("Mul")
            }
            "cos" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                self.append_unary(NativeOp::UnaryMath(UnaryMathOp::Sin))?;
                self.append_unary(NativeOp::Neg)?;
                self.lower_derivative(args[0], wrt)?;
                self.append_arithmetic("Mul")
            }
            "limexp" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                self.append_unary(NativeOp::UnaryMath(UnaryMathOp::Limexp))?;
                self.lower_derivative(args[0], wrt)?;
                self.append_arithmetic("Mul")
            }
            "pow" => {
                self.require_intrinsic_arity(name, args, 2)?;
                self.lower_pow_derivative(args[0], args[1], wrt)
            }
            "min" | "max" => {
                self.require_intrinsic_arity(name, args, 2)?;
                self.lower(args[0])?;
                self.lower(args[1])?;
                self.append_compare(if normalized == "min" { "Le" } else { "Ge" })?;
                self.lower_derivative(args[0], wrt)?;
                self.lower_derivative(args[1], wrt)?;
                self.append_ifelse()
            }
            "temperature" | "vt" | "thermal_vt" | "abstime" | "realtime" | "mfactor"
            | "simparam" | "param_given" | "port_connected" | "analysis" | "white_noise"
            | "flicker_noise" => self.push(NativeOp::Const(0.0)),
            _ => Err(self.unsupported(format!("ddx derivative of intrinsic function '{name}'"))),
        }
    }

    fn lower_unary_chain_derivative(
        &mut self,
        name: &str,
        args: &[ExprId],
        wrt: NodeId,
        op: UnaryMathOp,
    ) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 1)?;
        self.lower(args[0])?;
        self.append_unary(NativeOp::UnaryMath(op))?;
        self.lower_derivative(args[0], wrt)?;
        self.append_arithmetic("Mul")
    }

    fn lower_analog_operator_derivative(
        &mut self,
        op: &HirAnalogOperator,
        wrt: NodeId,
    ) -> JitResult<()> {
        match op {
            HirAnalogOperator::Limexp { expr } => {
                self.lower(*expr)?;
                self.append_unary(NativeOp::UnaryMath(UnaryMathOp::Limexp))?;
                self.lower_derivative(*expr, wrt)?;
                self.append_arithmetic("Mul")
            }
            HirAnalogOperator::Ddx { .. } => Err(self.unsupported("nested ddx derivative")),
            HirAnalogOperator::Ddt { .. }
            | HirAnalogOperator::Idt { .. }
            | HirAnalogOperator::IdtMod { .. }
            | HirAnalogOperator::Absdelay { .. }
            | HirAnalogOperator::Transition { .. }
            | HirAnalogOperator::Slew { .. }
            | HirAnalogOperator::LastCrossing { .. } => Err(self.unsupported(format!(
                "ddx derivative of analog operator {}",
                analog_operator_name(op)
            ))),
        }
    }

    fn constant_number(&self, expr_id: ExprId) -> Option<f64> {
        let expression = self.expression(expr_id).ok()?;
        match &expression.kind {
            HirExprKind::Number { value, .. } => Some(*value),
            _ => None,
        }
    }

    fn lower_limit_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (value, step) = match args {
            [value] => (*value, None),
            [value, step] => (*value, Some(*step)),
            _ => {
                return Err(self.unsupported(format!(
                    "system function $limit expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        let Some(slot) = self.limits.canonical_limit_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "system function $limit expression {expr_id} state slot"
            )));
        };
        self.lower(value)?;
        if let Some(step) = step {
            self.lower(step)?;
        } else {
            self.push(NativeOp::Const(0.7))?;
        }
        self.pop_binary("canonical $limit")?;
        self.ops.push(NativeOp::LimitState(slot));
        Ok(())
    }

    fn lower_table_model_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        if args.len() < 2 {
            return Err(self.unsupported(format!(
                "system function $table_model expects at least two operands, found {}",
                args.len()
            )));
        }
        let Some(table_id) = self.limits.canonical_table_lookup_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "system function $table_model expression {expr_id} table slot"
            )));
        };
        validate_index(
            self.model.clone(),
            "TableLookup table",
            table_id,
            self.limits.lookup_table_count,
        )?;
        self.lower(args[0])?;
        self.ops.push(NativeOp::TableLookup(table_id));
        Ok(())
    }

    fn lower_ddt_operator(
        &mut self,
        expr_id: ExprId,
        args: &[ExprId],
        abstol: Option<ExprId>,
    ) -> JitResult<()> {
        if abstol.is_some() {
            return Err(self.unsupported("analog operator ddt abstol argument"));
        }
        if args.len() != 1 {
            return Err(self.unsupported(format!(
                "analog operator ddt expects one operand, found {}",
                args.len()
            )));
        }
        let Some(slot) = self.limits.canonical_ddt_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator ddt expression {expr_id} state slot"
            )));
        };
        self.lower(args[0])?;
        self.ops.push(NativeOp::DdtState(slot));
        Ok(())
    }

    fn lower_idt_operator(
        &mut self,
        expr_id: ExprId,
        args: &[ExprId],
        ic: Option<ExprId>,
        assert: Option<ExprId>,
        abstol: Option<ExprId>,
    ) -> JitResult<()> {
        if assert.is_some() {
            return Err(self.unsupported("analog operator idt assert argument"));
        }
        if abstol.is_some() {
            return Err(self.unsupported("analog operator idt abstol argument"));
        }
        let (expr, ic) = match (args, ic) {
            ([expr], ic) => (*expr, ic),
            ([expr, ic], None) => (*expr, Some(*ic)),
            ([_, _], Some(_)) => {
                return Err(self.unsupported("analog operator idt duplicate initial condition"));
            }
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator idt expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        let Some(slot) = self.limits.canonical_idt_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator idt expression {expr_id} state slot"
            )));
        };
        self.lower(expr)?;
        if let Some(ic) = ic {
            self.lower(ic)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.pop_binary("canonical idt")?;
        self.ops.push(NativeOp::IdtState(slot));
        Ok(())
    }

    fn lower_idtmod_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, ic, modulus, offset) = match args {
            [_] | [_, _] => return self.lower_idt_operator(expr_id, args, None, None, None),
            [expr, ic, modulus] => (*expr, Some(*ic), Some(*modulus), None),
            [expr, ic, modulus, offset] => (*expr, Some(*ic), Some(*modulus), Some(*offset)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator idtmod expects three or four operands, found {}",
                    args.len()
                )));
            }
        };
        self.lower_idtmod_operator(expr_id, expr, ic, modulus, offset, None)
    }

    fn lower_idtmod_operator(
        &mut self,
        expr_id: ExprId,
        expr: ExprId,
        ic: Option<ExprId>,
        modulus: Option<ExprId>,
        offset: Option<ExprId>,
        abstol: Option<ExprId>,
    ) -> JitResult<()> {
        if abstol.is_some() {
            return Err(self.unsupported("analog operator idtmod abstol argument"));
        }
        let Some(modulus) = modulus else {
            if offset.is_some() {
                return Err(self.unsupported("analog operator idtmod offset without modulus"));
            }
            return self.lower_idt_operator(expr_id, &[expr], ic, None, None);
        };
        let Some(slot) = self.limits.canonical_idtmod_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator idtmod expression {expr_id} state slot"
            )));
        };
        self.lower(expr)?;
        if let Some(ic) = ic {
            self.lower(ic)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.lower(modulus)?;
        if let Some(offset) = offset {
            self.lower(offset)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical idtmod",
            self.depth,
            4,
        )?;
        self.depth -= 3;
        self.ops.push(NativeOp::IdtModState(slot));
        Ok(())
    }

    fn lower_transition_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, delay, rise, fall) = match args {
            [expr] => (*expr, None, None, None),
            [expr, delay] => (*expr, Some(*delay), None, None),
            [expr, delay, rise] => (*expr, Some(*delay), Some(*rise), None),
            [expr, delay, rise, fall] => (*expr, Some(*delay), Some(*rise), Some(*fall)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator transition expects one to four operands, found {}",
                    args.len()
                )));
            }
        };
        self.lower_transition_operator(expr_id, expr, delay, rise, fall, None)
    }

    fn lower_transition_operator(
        &mut self,
        expr_id: ExprId,
        expr: ExprId,
        delay: Option<ExprId>,
        rise: Option<ExprId>,
        fall: Option<ExprId>,
        tolerance: Option<ExprId>,
    ) -> JitResult<()> {
        if tolerance.is_some() {
            return Err(self.unsupported("analog operator transition tolerance argument"));
        }
        let Some(slot) = self.limits.canonical_transition_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator transition expression {expr_id} filter slot"
            )));
        };
        self.lower(expr)?;
        if let Some(delay) = delay {
            self.lower(delay)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        if let Some(rise) = rise {
            self.lower(rise)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        if let Some(fall) = fall {
            self.lower(fall)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical transition",
            self.depth,
            4,
        )?;
        self.depth -= 3;
        self.ops.push(NativeOp::TransitionState(slot));
        Ok(())
    }

    fn lower_slew_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, max_rise, max_fall) = match args {
            [expr] => (*expr, None, None),
            [expr, max_rise] => (*expr, Some(*max_rise), None),
            [expr, max_rise, max_fall] => (*expr, Some(*max_rise), Some(*max_fall)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator slew expects one to three operands, found {}",
                    args.len()
                )));
            }
        };
        self.lower_slew_operator(expr_id, expr, max_rise, max_fall)
    }

    fn lower_slew_operator(
        &mut self,
        expr_id: ExprId,
        expr: ExprId,
        max_rise: Option<ExprId>,
        max_fall: Option<ExprId>,
    ) -> JitResult<()> {
        let Some(slot) = self.limits.canonical_slew_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator slew expression {expr_id} filter slot"
            )));
        };
        self.lower(expr)?;
        if let Some(max_rise) = max_rise {
            self.lower(max_rise)?;
        } else {
            self.push(NativeOp::Const(f64::INFINITY))?;
        }
        if let Some(max_fall) = max_fall {
            self.lower(max_fall)?;
        } else {
            self.push(NativeOp::Const(f64::INFINITY))?;
        }
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical slew",
            self.depth,
            3,
        )?;
        self.depth -= 2;
        self.ops.push(NativeOp::SlewState(slot));
        Ok(())
    }

    fn lower_absdelay_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, delay) = match args {
            [expr] => (*expr, None),
            [expr, delay] => (*expr, Some(*delay)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator absdelay expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        self.lower_absdelay_operator(expr_id, expr, delay, None)
    }

    fn lower_absdelay_operator(
        &mut self,
        expr_id: ExprId,
        expr: ExprId,
        delay: Option<ExprId>,
        max_delay: Option<ExprId>,
    ) -> JitResult<()> {
        if max_delay.is_some() {
            return Err(self.unsupported("analog operator absdelay max_delay argument"));
        }
        let Some(slot) = self.limits.canonical_absdelay_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator absdelay expression {expr_id} buffer slot"
            )));
        };
        self.lower(expr)?;
        if let Some(delay) = delay {
            self.lower(delay)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.pop_binary("canonical absdelay")?;
        self.ops.push(NativeOp::AbsDelayState(slot));
        Ok(())
    }

    fn lower_laplace_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let [expr, _, _] = args else {
            return Err(self.unsupported(format!(
                "analog operator laplace expects three operands, found {}",
                args.len()
            )));
        };
        self.lower_laplace_operator(expr_id, *expr)
    }

    fn lower_laplace_operator(&mut self, expr_id: ExprId, expr: ExprId) -> JitResult<()> {
        let Some(slot) = self.limits.canonical_laplace_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator laplace expression {expr_id} filter slot"
            )));
        };
        validate_index(
            self.model.clone(),
            "LaplaceState filter",
            slot,
            self.limits.laplace_filter_count,
        )?;
        self.lower(expr)?;
        self.ops.push(NativeOp::LaplaceState(slot));
        Ok(())
    }

    fn lower_zi_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let [expr, _, _, _] = args else {
            return Err(self.unsupported(format!(
                "analog operator zi expects four operands, found {}",
                args.len()
            )));
        };
        self.lower_zi_operator(expr_id, *expr)
    }

    fn lower_zi_operator(&mut self, expr_id: ExprId, expr: ExprId) -> JitResult<()> {
        let Some(slot) = self.limits.canonical_zi_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator zi expression {expr_id} filter slot"
            )));
        };
        validate_index(
            self.model.clone(),
            "ZiState filter",
            slot,
            self.limits.zi_filter_count,
        )?;
        self.lower(expr)?;
        self.ops.push(NativeOp::ZiState(slot));
        Ok(())
    }

    fn lower_cross_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, direction) = match args {
            [expr] => (*expr, None),
            [expr, direction] => (*expr, Some(*direction)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator cross expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        let Some(slot) = self.limits.canonical_cross_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator cross expression {expr_id} detector slot"
            )));
        };
        self.lower(expr)?;
        if let Some(direction) = direction {
            self.lower(direction)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.pop_binary("canonical cross")?;
        self.ops.push(NativeOp::CrossState(slot));
        Ok(())
    }

    fn lower_above_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (expr, threshold) = match args {
            [expr] => (*expr, None),
            [expr, threshold] => (*expr, Some(*threshold)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator above expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        let Some(_slot) = self.limits.canonical_above_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator above expression {expr_id} detector slot"
            )));
        };
        self.lower(expr)?;
        if let Some(threshold) = threshold {
            self.lower(threshold)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.pop_binary("canonical above")?;
        if lower_constant_binary_compare(&mut self.ops, CompareOp::Gt)
            || lower_constant_rhs_compare(&mut self.ops, CompareOp::Gt)
        {
            Ok(())
        } else {
            self.ops.push(NativeOp::Compare(CompareOp::Gt));
            Ok(())
        }
    }

    fn lower_timer_call(&mut self, expr_id: ExprId, args: &[ExprId]) -> JitResult<()> {
        let (start, period) = match args {
            [start] => (*start, None),
            [start, period] => (*start, Some(*period)),
            _ => {
                return Err(self.unsupported(format!(
                    "analog operator timer expects one or two operands, found {}",
                    args.len()
                )));
            }
        };
        let Some(slot) = self.limits.canonical_timer_slot(expr_id) else {
            return Err(self.unsupported(format!(
                "analog operator timer expression {expr_id} state slot"
            )));
        };
        self.lower(start)?;
        if let Some(period) = period {
            self.lower(period)?;
        } else {
            self.push(NativeOp::Const(0.0))?;
        }
        self.pop_binary("canonical timer")?;
        self.ops.push(NativeOp::TimerState(slot));
        Ok(())
    }

    fn lower_named_branch_access(&mut self, access: &str, name: &str) -> JitResult<()> {
        if is_flow_access(access) {
            if let Some(branch_unknown) = self.named_branch_unknown(name) {
                let index = usize::from(branch_unknown.id);
                validate_index(
                    self.model.clone(),
                    "canonical branch unknown",
                    index,
                    self.limits.branch_unknown_count,
                )?;
                return self.push(NativeOp::LoadBranchUnknown(index));
            }
            return self.lower_prior_named_branch_current(name);
        }

        let branch = self.named_branch(name)?;
        let pos = self.lower_voltage_node_id(branch.pos_node)?;
        let neg = self.lower_voltage_node_id(branch.neg_node)?;
        self.push(NativeOp::LoadVoltage { pos, neg })
    }

    fn lower_prior_named_branch_current(&mut self, name: &str) -> JitResult<()> {
        let branch = self.named_branch(name)?;
        let prior_indices = self.prior_named_branch_current_indices(name, branch);
        let Some((first, rest)) = prior_indices.split_first() else {
            return Err(self.unsupported(format!("named branch current {name}")));
        };

        self.push_prior_current(*first)?;
        for index in rest {
            self.push_prior_current(*index)?;
            self.append_arithmetic("Add")?;
        }
        Ok(())
    }

    fn push_prior_current(&mut self, index: usize) -> JitResult<()> {
        if !self.prior_current_dependencies.contains(&index) {
            self.prior_current_dependencies.push(index);
        }
        self.push(NativeOp::LoadPriorCurrent(index))
    }

    fn prior_named_branch_current_indices(
        &self,
        name: &str,
        branch: &crate::canonical_ir::MirBranch,
    ) -> Vec<usize> {
        let equation_index = usize::from(self.equation_id);
        self.mir
            .equations
            .iter()
            .take(equation_index)
            .enumerate()
            .filter_map(|(index, equation)| {
                (equation.kind == crate::canonical_ir::MirEquationKind::Current
                    && self.equation_contributes_to_named_branch(equation, name, branch))
                .then_some(index)
            })
            .collect()
    }

    fn equation_contributes_to_named_branch(
        &self,
        equation: &crate::canonical_ir::MirEquation,
        name: &str,
        branch: &crate::canonical_ir::MirBranch,
    ) -> bool {
        if equation.branch.declared_name.as_deref() == Some(name) {
            return true;
        }

        equation.branch.pos_node == branch.pos_node
            && equation.branch.neg_node == branch.neg_node
            && self.unique_branch_name_for_endpoints(branch.pos_node, branch.neg_node) == Some(name)
    }

    fn unique_branch_name_for_endpoints(
        &self,
        pos_node: Option<crate::canonical_ir::NodeId>,
        neg_node: Option<crate::canonical_ir::NodeId>,
    ) -> Option<&str> {
        let mut matches = self
            .mir
            .branches
            .iter()
            .filter(|branch| branch.pos_node == pos_node && branch.neg_node == neg_node);
        let first = matches.next()?;
        matches.next().is_none().then_some(first.name.as_str())
    }

    fn lower_intrinsic_call(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        let normalized = normalize_intrinsic_name(name);
        match normalized.as_str() {
            "abs" | "fabs" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                if lower_constant_abs(&mut self.ops) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::Abs)
                }
            }
            "sqrt" => {
                self.require_intrinsic_arity(name, args, 1)?;
                self.lower(args[0])?;
                if lower_constant_sqrt(&mut self.ops) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::Sqrt)
                }
            }
            "exp" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Exp),
            "ln" | "log" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Log),
            "log10" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Log10),
            "sin" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Sin),
            "cos" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Cos),
            "tan" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Tan),
            "sinh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Sinh),
            "cosh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Cosh),
            "tanh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Tanh),
            "asinh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Asinh),
            "acosh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Acosh),
            "atanh" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Atanh),
            "limexp" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Limexp),
            "asin" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Asin),
            "acos" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Acos),
            "atan" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Atan),
            "floor" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Floor),
            "ceil" => self.lower_unary_math_intrinsic(name, args, UnaryMathOp::Ceil),
            "pow" => self.lower_binary_math_intrinsic(name, args, BinaryMathOp::Pow),
            "atan2" => self.lower_binary_math_intrinsic(name, args, BinaryMathOp::Atan2),
            "hypot" => self.lower_binary_math_intrinsic(name, args, BinaryMathOp::Hypot),
            "min" => self.lower_extremum_intrinsic(name, args, ExtremumOp::Min),
            "max" => self.lower_extremum_intrinsic(name, args, ExtremumOp::Max),
            "temperature" => {
                self.require_intrinsic_arity(name, args, 0)?;
                self.push(NativeOp::LoadTemperature)
            }
            "vt" | "thermal_vt" => self.lower_thermal_voltage_intrinsic(name, args),
            "abstime" | "realtime" => {
                self.require_intrinsic_arity(name, args, 0)?;
                self.push(NativeOp::LoadTime)
            }
            "mfactor" => {
                self.require_intrinsic_arity(name, args, 0)?;
                self.push(NativeOp::LoadMfactor)
            }
            "simparam" => self.lower_simparam_intrinsic(name, args),
            "param_given" => self.lower_param_given_intrinsic(name, args),
            "port_connected" => self.lower_port_connected_intrinsic(name, args),
            "analysis" => self.lower_analysis_intrinsic(name, args),
            "white_noise" => self.lower_white_noise_intrinsic(name, args),
            "flicker_noise" => self.lower_flicker_noise_intrinsic(name, args),
            _ => Err(self.unsupported(format!("intrinsic function '{name}'"))),
        }
    }

    fn lower_thermal_voltage_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity_range(name, args, 0, 1)?;
        if args.is_empty() {
            return self.push(NativeOp::LoadThermalVoltage);
        }

        self.lower(args[0])?;
        self.push(NativeOp::Const(8.617333262e-5))?;
        self.append_arithmetic("Mul")
    }

    fn lower_simparam_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity_range(name, args, 1, 2)?;
        let simparam_name = self.string_literal_argument(name, args[0])?;

        if let Some(default) = args.get(1).copied() {
            return self.lower(default);
        }

        let value = match simparam_name {
            "gmin" => 1.0e-12,
            "tnom" => 300.15,
            "simulatorVersion" => 1.0,
            _ => 0.0,
        };
        self.push(NativeOp::Const(value))
    }

    fn lower_param_given_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 1)?;
        let parameter_name = self.identifier_argument(name, args[0])?;
        let Some(parameter) = self
            .mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == parameter_name)
        else {
            return Err(self.unsupported(format!(
                "intrinsic function '{name}' parameter {parameter_name}"
            )));
        };
        let index = usize::from(parameter.id);
        validate_index(
            self.model.clone(),
            "canonical $param_given parameter",
            index,
            self.limits.parameter_count,
        )?;
        self.push(NativeOp::LoadParamGiven(index))
    }

    fn lower_port_connected_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 1)?;
        let port_name = self.identifier_argument(name, args[0])?;
        let index = self.lower_terminal_node(port_name)?;
        self.push(NativeOp::LoadPortConnected(index))
    }

    fn lower_analysis_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 1)?;
        let analysis_name = self.string_literal_argument(name, args[0])?;
        let analysis_id = match analysis_name.to_ascii_lowercase().as_str() {
            "dc" | "op" => 0,
            "ac" => 1,
            "tran" | "transient" => 2,
            "noise" => 3,
            "ic" => 4,
            "static" => 5,
            "smallsig" | "smallsignal" | "small_signal" => 6,
            _ => {
                return Err(self.unsupported(format!(
                    "intrinsic function '{name}' analysis name '{analysis_name}'"
                )));
            }
        };
        self.push(NativeOp::Analysis(analysis_id))
    }

    fn lower_white_noise_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity_range(name, args, 1, 2)?;
        self.lower(args[0])?;
        self.ops.push(NativeOp::WhiteNoise);
        Ok(())
    }

    fn lower_flicker_noise_intrinsic(&mut self, name: &str, args: &[ExprId]) -> JitResult<()> {
        self.require_intrinsic_arity_range(name, args, 2, 3)?;
        self.lower(args[0])?;
        self.lower(args[1])?;
        self.pop_binary("canonical flicker noise")?;
        self.ops.push(NativeOp::FlickerNoise);
        Ok(())
    }

    fn lower_unary_math_intrinsic(
        &mut self,
        name: &str,
        args: &[ExprId],
        op: UnaryMathOp,
    ) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 1)?;
        self.lower(args[0])?;
        if lower_constant_unary_math(&mut self.ops, op) {
            Ok(())
        } else {
            self.append_unary(NativeOp::UnaryMath(op))
        }
    }

    fn lower_binary_math_intrinsic(
        &mut self,
        name: &str,
        args: &[ExprId],
        op: BinaryMathOp,
    ) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 2)?;
        self.lower(args[0])?;
        self.lower(args[1])?;
        self.pop_binary("canonical binary intrinsic")?;
        self.append_binary_math_op(op)
    }

    fn lower_extremum_intrinsic(
        &mut self,
        name: &str,
        args: &[ExprId],
        op: ExtremumOp,
    ) -> JitResult<()> {
        self.require_intrinsic_arity(name, args, 2)?;
        self.lower(args[0])?;
        self.lower(args[1])?;
        self.append_extremum(op)
    }

    fn require_intrinsic_arity(
        &self,
        name: &str,
        args: &[ExprId],
        expected: usize,
    ) -> JitResult<()> {
        if args.len() != expected {
            return Err(self.unsupported(format!(
                "intrinsic function '{name}' expects {expected} argument(s), found {}",
                args.len()
            )));
        }
        Ok(())
    }

    fn require_intrinsic_arity_range(
        &self,
        name: &str,
        args: &[ExprId],
        min: usize,
        max: usize,
    ) -> JitResult<()> {
        if args.len() < min || args.len() > max {
            return Err(self.unsupported(format!(
                "intrinsic function '{name}' expects between {min} and {max} argument(s), found {}",
                args.len()
            )));
        }
        Ok(())
    }

    fn identifier_argument(&self, name: &str, expr_id: ExprId) -> JitResult<&'a str> {
        let expression = self.expression(expr_id)?;
        match &expression.kind {
            HirExprKind::Identifier { name } => Ok(name.as_str()),
            _ => Err(self.unsupported(format!(
                "intrinsic function '{name}' requires identifier argument"
            ))),
        }
    }

    fn string_literal_argument(&self, name: &str, expr_id: ExprId) -> JitResult<&'a str> {
        let expression = self.expression(expr_id)?;
        match &expression.kind {
            HirExprKind::StringLiteral { value } => Ok(value.as_str()),
            _ => Err(self.unsupported(format!(
                "intrinsic function '{name}' requires string literal argument"
            ))),
        }
    }

    fn lower_identifier(&mut self, name: &str) -> JitResult<()> {
        if let Some(parameter) = self
            .mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == name)
        {
            let index = usize::from(parameter.id);
            validate_index(
                self.model.clone(),
                "canonical parameter",
                index,
                self.limits.parameter_count,
            )?;
            return self.push(NativeOp::LoadParam(index));
        }

        if let Some(index) = self
            .limits
            .variable_names
            .iter()
            .position(|variable| variable.as_str() == name)
        {
            validate_index(
                self.model.clone(),
                "canonical variable",
                index,
                self.limits.variable_count,
            )?;
            return self.push(NativeOp::LoadVariable(index));
        }

        Err(self.unsupported(format!("identifier {name}")))
    }

    fn lower_array_access(&mut self, array: &str, index: ExprId) -> JitResult<()> {
        let Some((base, len, lower)) = self.resolve_array_variable_range(array)? else {
            return Err(self.unsupported(format!("array access {array}")));
        };
        validate_range(
            self.model.clone(),
            "canonical array variable range",
            base,
            len,
            self.limits.variable_count,
        )?;
        self.lower(index)?;
        if lower_constant_dynamic_variable_read(&mut self.ops, base, len, lower) {
            return Ok(());
        }
        self.ops
            .push(NativeOp::LoadVariableDyn { base, len, lower });
        Ok(())
    }

    fn resolve_array_variable_range(&self, array: &str) -> JitResult<Option<(usize, usize, i64)>> {
        let prefix = format!("{array}[");
        let mut slots = self
            .limits
            .variable_names
            .iter()
            .enumerate()
            .filter_map(|(slot, name)| {
                let text = name.as_str();
                let index = text
                    .strip_prefix(&prefix)?
                    .strip_suffix(']')?
                    .parse::<i64>()
                    .ok()?;
                Some((index, slot))
            })
            .collect::<Vec<_>>();
        if slots.is_empty() {
            return Ok(None);
        }

        slots.sort_by_key(|(index, _)| *index);
        let lower = slots[0].0;
        let base = slots[0].1;
        for (offset, (logical_index, slot)) in slots.iter().enumerate() {
            let expected_index = lower + offset as i64;
            let expected_slot = base + offset;
            if *logical_index != expected_index || *slot != expected_slot {
                return Err(self.unsupported(format!(
                    "array access {array} with non-contiguous runtime storage"
                )));
            }
        }

        Ok(Some((base, slots.len(), lower)))
    }

    fn lower_branch_access(&mut self, access: &str, pos: &str, neg: Option<&str>) -> JitResult<()> {
        if !is_flow_access(access) {
            let pos = self.lower_voltage_node(pos)?;
            let neg = neg
                .map(|node| self.lower_voltage_node(node))
                .transpose()?
                .unwrap_or(VoltageNode::Ground);
            return self.push(NativeOp::LoadVoltage { pos, neg });
        }

        if let Some((branch_unknown, reversed)) = self.resolve_current_branch_unknown(pos, neg)? {
            validate_index(
                self.model.clone(),
                "canonical branch unknown",
                branch_unknown,
                self.limits.branch_unknown_count,
            )?;
            self.push(NativeOp::LoadBranchUnknown(branch_unknown))?;
            if reversed {
                return self.append_unary(NativeOp::Neg);
            }
            return Ok(());
        }

        let pos = self.lower_current_endpoint(pos)?;
        let neg = neg
            .map(|node| self.lower_current_endpoint(node))
            .transpose()?
            .unwrap_or(CURRENT_PAIR_GROUND);
        let pair_index =
            current_pair_index(self.model.clone(), pos, neg, self.limits.terminal_count)?;
        if !self.limits.available_current_pairs.contains(&pair_index) {
            return Err(JitError::unsupported_program_op(
                self.model.clone(),
                format!(
                    "canonical branch current terminal pair {} unavailable",
                    format_current_pair(pos, neg)
                ),
            ));
        }
        if !self.current_pair_dependencies.contains(&pair_index) {
            self.current_pair_dependencies.push(pair_index);
        }
        self.push(NativeOp::LoadCurrent(pair_index))
    }

    fn resolve_current_branch_unknown(
        &self,
        pos: &str,
        neg: Option<&str>,
    ) -> JitResult<Option<(usize, bool)>> {
        let pos = self.branch_endpoint(pos)?;
        let neg = neg
            .map(|node| self.branch_endpoint(node))
            .transpose()?
            .flatten();

        let Some(branch_unknown) = self
            .mir
            .branch_unknowns
            .iter()
            .find(|branch_unknown| branch_unknown.equation == self.equation_id)
        else {
            return Ok(None);
        };

        let same_direction = branch_unknown.pos_node == pos && branch_unknown.neg_node == neg;
        let opposite_direction = branch_unknown.pos_node == neg && branch_unknown.neg_node == pos;
        if same_direction || opposite_direction {
            return Ok(Some((usize::from(branch_unknown.id), opposite_direction)));
        }

        Ok(None)
    }

    fn branch_endpoint(&self, name: &str) -> JitResult<Option<NodeId>> {
        if self.is_ground_node(name) {
            return Ok(None);
        }
        Ok(Some(self.node(name)?.id))
    }

    fn lower_unary(&mut self, op: &str, operand: ExprId) -> JitResult<()> {
        self.lower(operand)?;
        match op {
            "Pos" => Ok(()),
            "Neg" => {
                if lower_constant_neg(&mut self.ops) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::Neg)
                }
            }
            "Not" => {
                if lower_constant_logical_not(&mut self.ops) {
                    Ok(())
                } else {
                    self.append_unary(NativeOp::Logical(LogicalOp::Not))
                }
            }
            "BitNot" => {
                self.push(NativeOp::Const(-1.0))?;
                self.append_integer_binary("BitXor")
            }
            _ => Err(self.unsupported(format!("unary operator {op}"))),
        }
    }

    fn lower_binary(&mut self, op: &str, left: ExprId, right: ExprId) -> JitResult<()> {
        self.lower(left)?;
        self.lower(right)?;
        match op {
            "Add" | "Sub" | "Mul" | "Div" => self.append_arithmetic(op),
            "Pow" | "Mod" => self.append_binary_math(op),
            "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" => self.append_compare(op),
            "And" | "Or" => self.append_logical(op),
            "BitAnd" | "BitOr" | "BitXor" | "Shl" | "Shr" => self.append_integer_binary(op),
            _ => Err(self.unsupported(format!("binary operator {op}"))),
        }
    }

    fn append_arithmetic(&mut self, op: &str) -> JitResult<()> {
        self.pop_binary("canonical arithmetic")?;
        let instruction = match op {
            "Add" => Instruction::Add,
            "Sub" => Instruction::Sub,
            "Mul" => Instruction::Mul,
            "Div" => Instruction::Div,
            _ => unreachable!("append_arithmetic only accepts arithmetic operators"),
        };
        if lower_constant_binary_arithmetic(&mut self.ops, &instruction)
            || lower_constant_rhs_arithmetic(&mut self.ops, &instruction)
            || lower_constant_lhs_identity_arithmetic(&mut self.ops, &instruction)
            || lower_constant_lhs_noncommutative_arithmetic(&mut self.ops, &instruction)
        {
            return Ok(());
        }
        self.ops.push(arithmetic_op(&instruction));
        Ok(())
    }

    fn append_binary_math(&mut self, op: &str) -> JitResult<()> {
        self.pop_binary("canonical binary math")?;
        let op = match op {
            "Pow" => BinaryMathOp::Pow,
            "Mod" => BinaryMathOp::Mod,
            _ => unreachable!("append_binary_math only accepts binary math operators"),
        };
        self.append_binary_math_op(op)
    }

    fn append_binary_math_op(&mut self, op: BinaryMathOp) -> JitResult<()> {
        if lower_constant_binary_math(&mut self.ops, op)
            || (op == BinaryMathOp::Pow && lower_constant_square_power(&mut self.ops))
            || (op == BinaryMathOp::Pow && lower_constant_identity_power(&mut self.ops))
            || (op == BinaryMathOp::Pow && lower_constant_reciprocal_power(&mut self.ops))
        {
            return Ok(());
        }
        self.ops.push(NativeOp::BinaryMath(op));
        Ok(())
    }

    fn append_extremum(&mut self, op: ExtremumOp) -> JitResult<()> {
        self.pop_binary("canonical extremum")?;
        if lower_constant_binary_extremum(&mut self.ops, op)
            || lower_constant_rhs_extremum(&mut self.ops, op)
        {
            return Ok(());
        }
        self.ops.push(NativeOp::Extremum(op));
        Ok(())
    }

    fn append_compare(&mut self, op: &str) -> JitResult<()> {
        self.pop_binary("canonical comparison")?;
        let op = match op {
            "Gt" => CompareOp::Gt,
            "Lt" => CompareOp::Lt,
            "Ge" => CompareOp::Ge,
            "Le" => CompareOp::Le,
            "Eq" => CompareOp::Eq,
            "Ne" => CompareOp::Ne,
            _ => unreachable!("append_compare only accepts comparison operators"),
        };
        if lower_constant_binary_compare(&mut self.ops, op)
            || lower_constant_rhs_compare(&mut self.ops, op)
            || lower_constant_lhs_compare(&mut self.ops, op)
        {
            return Ok(());
        }
        self.ops.push(NativeOp::Compare(op));
        Ok(())
    }

    fn append_logical(&mut self, op: &str) -> JitResult<()> {
        self.pop_binary("canonical logical")?;
        let op = match op {
            "And" => LogicalOp::And,
            "Or" => LogicalOp::Or,
            _ => unreachable!("append_logical only accepts logical operators"),
        };
        if lower_constant_binary_logical(&mut self.ops, op)
            || lower_constant_rhs_logical(&mut self.ops, op)
            || lower_constant_lhs_logical(&mut self.ops, op)
        {
            return Ok(());
        }
        self.ops.push(NativeOp::Logical(op));
        Ok(())
    }

    fn append_integer_binary(&mut self, op: &str) -> JitResult<()> {
        self.pop_binary("canonical integer binary")?;
        let op = match op {
            "Shl" => IntegerBinaryOp::Shl,
            "Shr" => IntegerBinaryOp::Shr,
            "BitAnd" => IntegerBinaryOp::BitAnd,
            "BitOr" => IntegerBinaryOp::BitOr,
            "BitXor" => IntegerBinaryOp::BitXor,
            _ => unreachable!("append_integer_binary only accepts integer operators"),
        };
        if lower_constant_integer_binary(&mut self.ops, op) {
            return Ok(());
        }
        self.ops.push(NativeOp::IntegerBinary(op));
        Ok(())
    }

    fn append_unary(&mut self, op: NativeOp) -> JitResult<()> {
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical unary",
            self.depth,
            1,
        )?;
        if lower_unary_composition(&mut self.ops, op) {
            return Ok(());
        }
        self.ops.push(op);
        Ok(())
    }

    fn append_ternary(&mut self, op: NativeOp) -> JitResult<()> {
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical ternary",
            self.depth,
            3,
        )?;
        self.depth -= 2;
        self.ops.push(op);
        Ok(())
    }

    fn append_ifelse(&mut self) -> JitResult<()> {
        require_stack(
            self.model.clone(),
            self.entry_kind,
            "canonical ifelse",
            self.depth,
            3,
        )?;
        self.depth -= 2;
        if lower_constant_ifelse(&mut self.ops) {
            return Ok(());
        }
        self.ops.push(NativeOp::IfElse);
        Ok(())
    }

    fn pop_binary(&mut self, op_name: &'static str) -> JitResult<()> {
        pop_binary_stack(self.model.clone(), self.entry_kind, op_name, self.depth)?;
        self.depth -= 1;
        Ok(())
    }

    fn push(&mut self, op: NativeOp) -> JitResult<()> {
        self.ops.push(op);
        push_stack(&mut self.depth, &mut self.max_stack_depth);
        Ok(())
    }

    fn expression(&self, expr_id: ExprId) -> JitResult<&'a crate::canonical_ir::HirExpression> {
        self.mir
            .expressions
            .get(usize::from(expr_id))
            .filter(|expression| expression.id == expr_id)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: self.model.clone(),
                detail: format!("canonical expression {expr_id} is outside MIR arena").into(),
            })
    }

    fn lower_voltage_node(&self, name: &str) -> JitResult<VoltageNode> {
        if self.is_ground_node(name) {
            return Ok(VoltageNode::Ground);
        }
        let node = self.node(name)?;
        self.lower_voltage_node_id(Some(node.id))
    }

    fn lower_voltage_node_id(&self, node_id: Option<NodeId>) -> JitResult<VoltageNode> {
        let Some(node_id) = node_id else {
            return Ok(VoltageNode::Ground);
        };
        let node = self.node_by_id(node_id)?;
        let node_index = usize::from(node.id);
        if node.is_external {
            validate_index(
                self.model.clone(),
                "canonical voltage terminal",
                node_index,
                self.limits.terminal_count,
            )?;
            return Ok(VoltageNode::Terminal(node_index));
        }

        let external_count = self.external_node_count();
        let internal_index =
            node_index
                .checked_sub(external_count)
                .ok_or_else(|| JitError::InvalidCanonicalIr {
                    model: self.model.clone(),
                    detail: format!(
                        "canonical internal node {} appears before external nodes",
                        node.name
                    )
                    .into(),
                })?;
        validate_index(
            self.model.clone(),
            "canonical internal voltage",
            internal_index,
            self.limits.internal_node_count,
        )?;
        Ok(VoltageNode::Internal(internal_index))
    }

    fn named_branch(&self, name: &str) -> JitResult<&'a crate::canonical_ir::MirBranch> {
        self.mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: self.model.clone(),
                detail: format!("canonical branch {name} is outside MIR branch table").into(),
            })
    }

    fn named_branch_unknown(
        &self,
        name: &str,
    ) -> Option<&'a crate::canonical_ir::MirBranchUnknown> {
        self.mir
            .branch_unknowns
            .iter()
            .find(|branch_unknown| branch_unknown.declared_name.as_deref() == Some(name))
    }

    fn lower_terminal_node(&self, name: &str) -> JitResult<usize> {
        let node = self.node(name)?;
        let node_index = usize::from(node.id);
        if !node.is_external {
            return Err(self.unsupported(format!("branch current internal node {name}")));
        }
        validate_index(
            self.model.clone(),
            "canonical current terminal",
            node_index,
            self.limits.terminal_count,
        )?;
        Ok(node_index)
    }

    fn lower_current_endpoint(&self, name: &str) -> JitResult<usize> {
        if self.is_ground_node(name) {
            return Ok(CURRENT_PAIR_GROUND);
        }
        self.lower_terminal_node(name)
    }

    fn node(&self, name: &str) -> JitResult<&'a crate::canonical_ir::MirNode> {
        self.mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: self.model.clone(),
                detail: format!("canonical node {name} is outside MIR node arena").into(),
            })
    }

    fn node_by_id(&self, node_id: NodeId) -> JitResult<&'a crate::canonical_ir::MirNode> {
        self.mir
            .nodes
            .get(usize::from(node_id))
            .filter(|node| node.id == node_id)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: self.model.clone(),
                detail: format!("canonical node {node_id} is outside MIR node arena").into(),
            })
    }

    fn is_ground_node(&self, name: &str) -> bool {
        name == "0"
            || self
                .mir
                .ground_nodes
                .iter()
                .any(|node| node.as_str() == name)
    }

    fn external_node_count(&self) -> usize {
        self.mir
            .nodes
            .iter()
            .filter(|node| node.is_external)
            .count()
    }

    fn unsupported(&self, detail: impl Into<String>) -> JitError {
        JitError::unsupported_program_op(
            self.model.clone(),
            format!("expression {}", detail.into()),
        )
    }
}

fn normalize_intrinsic_name(name: &str) -> String {
    name.strip_prefix('$').unwrap_or(name).to_ascii_lowercase()
}

fn is_flow_access(access: &str) -> bool {
    matches!(access, "I" | "Pwr" | "F" | "Tau" | "MMF" | "Flow")
}

fn expression_kind_name(kind: &HirExprKind) -> &'static str {
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
        HirExprKind::Zi { .. } => "zi",
        HirExprKind::NoiseSource { .. } => "noise source",
    }
}

fn analog_operator_name(op: &HirAnalogOperator) -> &'static str {
    match op {
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

fn push_stack(depth: &mut usize, max_stack_depth: &mut usize) {
    *depth += 1;
    *max_stack_depth = (*max_stack_depth).max(*depth);
}

fn validate_entry_instruction(
    model: SmolStr,
    entry_kind: EntryKind,
    instruction: &Instruction,
) -> JitResult<()> {
    if matches!(entry_kind, EntryKind::StaticCondition)
        && !is_static_condition_instruction(instruction)
    {
        return Err(JitError::unsupported_program_op(
            model,
            format!("StaticCondition {}", instruction_name(instruction)),
        ));
    }

    if matches!(entry_kind, EntryKind::ParameterDefault)
        && !is_parameter_default_instruction(instruction)
    {
        return Err(JitError::unsupported_program_op(
            model,
            format!("ParameterDefault {}", instruction_name(instruction)),
        ));
    }

    Ok(())
}

fn is_parameter_default_instruction(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::PushConst(_)
            | Instruction::PushParam(_)
            | Instruction::PushParamGiven(_)
            | Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Pow
            | Instruction::FnPow
            | Instruction::Atan2
            | Instruction::Mod
            | Instruction::Shl
            | Instruction::Shr
            | Instruction::BitAnd
            | Instruction::BitOr
            | Instruction::BitXor
            | Instruction::Neg
            | Instruction::Abs
            | Instruction::Sqrt
            | Instruction::Gt
            | Instruction::Lt
            | Instruction::Ge
            | Instruction::Le
            | Instruction::Eq
            | Instruction::Ne
            | Instruction::And
            | Instruction::Or
            | Instruction::Not
            | Instruction::IfElse
            | Instruction::Min
            | Instruction::Max
            | Instruction::Exp
            | Instruction::Log
            | Instruction::Log10
            | Instruction::Sin
            | Instruction::Cos
            | Instruction::Tan
            | Instruction::Sinh
            | Instruction::Cosh
            | Instruction::Tanh
            | Instruction::Limexp
            | Instruction::Asin
            | Instruction::Acos
            | Instruction::Atan
            | Instruction::Floor
            | Instruction::Ceil
    )
}

fn is_static_condition_instruction(instruction: &Instruction) -> bool {
    matches!(
        instruction,
        Instruction::PushConst(_)
            | Instruction::PushParam(_)
            | Instruction::PushParamGiven(_)
            | Instruction::PushPortConnected(_)
            | Instruction::PushVariable(_)
            | Instruction::PushVariableDyn { .. }
            | Instruction::PushTemperature
            | Instruction::PushVt
            | Instruction::PushMfactor
            | Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Pow
            | Instruction::Mod
            | Instruction::Shl
            | Instruction::Shr
            | Instruction::BitAnd
            | Instruction::BitOr
            | Instruction::BitXor
            | Instruction::Neg
            | Instruction::Abs
            | Instruction::Sqrt
            | Instruction::Exp
            | Instruction::Log
            | Instruction::Log10
            | Instruction::Sin
            | Instruction::Cos
            | Instruction::Tan
            | Instruction::Sinh
            | Instruction::Cosh
            | Instruction::Tanh
            | Instruction::Min
            | Instruction::Max
            | Instruction::Limexp
            | Instruction::Asin
            | Instruction::Acos
            | Instruction::Atan
            | Instruction::Atan2
            | Instruction::Floor
            | Instruction::Ceil
            | Instruction::FnPow
            | Instruction::Gt
            | Instruction::Lt
            | Instruction::Ge
            | Instruction::Le
            | Instruction::Eq
            | Instruction::Ne
            | Instruction::And
            | Instruction::Or
            | Instruction::Not
            | Instruction::IfElse
    )
}

fn pop_binary_stack(
    model: SmolStr,
    entry_kind: EntryKind,
    op: &'static str,
    depth: usize,
) -> JitResult<()> {
    require_stack(model, entry_kind, op, depth, 2)
}

fn require_stack(
    model: SmolStr,
    entry_kind: EntryKind,
    op: &'static str,
    depth: usize,
    required: usize,
) -> JitResult<()> {
    if depth < required {
        return Err(stack_error(
            model,
            entry_kind,
            format!("{op} requires stack depth {required}, found {depth}"),
        ));
    }

    Ok(())
}

fn validate_index(model: SmolStr, op: &'static str, index: usize, len: usize) -> JitResult<()> {
    if index >= len {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} {index}"),
        ));
    }

    Ok(())
}

fn validate_range(
    model: SmolStr,
    op: &'static str,
    base: usize,
    len: usize,
    limit: usize,
) -> JitResult<()> {
    let Some(end) = base.checked_add(len) else {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} base {base} length {len} overflows"),
        ));
    };
    if len == 0 || end > limit {
        return Err(JitError::unsupported_program_op(
            model,
            format!("{op} {base}..{end} outside storage length {limit}"),
        ));
    }

    Ok(())
}

fn lower_constant_square_power(ops: &mut Vec<NativeOp>) -> bool {
    let Some(last) = ops.last_mut() else {
        return false;
    };
    if matches!(*last, NativeOp::Const(value) if value.to_bits() == 2.0_f64.to_bits()) {
        *last = NativeOp::Square;
        true
    } else {
        false
    }
}

fn lower_constant_identity_power(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last() else {
        return false;
    };
    if value.to_bits() != 1.0_f64.to_bits() {
        return false;
    }

    ops.pop();
    true
}

fn lower_constant_reciprocal_power(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last() else {
        return false;
    };
    if value.to_bits() != (-1.0_f64).to_bits() {
        return false;
    }

    ops.pop();
    ops.push(NativeOp::DivFromConst(1.0));
    true
}

fn lower_constant_binary_math(ops: &mut Vec<NativeOp>, op: BinaryMathOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(constant_binary_math(op, left, right)));
    true
}

fn lower_constant_integer_binary(ops: &mut Vec<NativeOp>, op: IntegerBinaryOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let Some(value) = constant_integer_binary(op, left, right) else {
        return false;
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_unary_math(ops: &mut Vec<NativeOp>, op: UnaryMathOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = constant_unary_math(op, *value);
    true
}

fn lower_constant_dynamic_variable_read(
    ops: &mut Vec<NativeOp>,
    base: usize,
    len: usize,
    lower: i64,
) -> bool {
    let Some(NativeOp::Const(raw_index)) = ops.last_mut() else {
        return false;
    };
    let Some(slot) = constant_dynamic_variable_slot(*raw_index, base, len, lower) else {
        return false;
    };
    *ops.last_mut().expect("constant index op still present") = NativeOp::LoadVariable(slot);
    true
}

fn lower_constant_neg(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = f64::from_bits(value.to_bits() ^ 0x8000_0000_0000_0000);
    true
}

fn lower_constant_abs(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = f64::from_bits(value.to_bits() & 0x7fff_ffff_ffff_ffff);
    true
}

fn lower_constant_sqrt(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    if value.is_nan() || *value < 0.0 {
        return false;
    }
    *value = value.sqrt();
    true
}

fn lower_unary_composition(ops: &mut Vec<NativeOp>, op: NativeOp) -> bool {
    match op {
        NativeOp::Neg => {
            if matches!(ops.last(), Some(NativeOp::Neg)) {
                ops.pop();
                true
            } else {
                false
            }
        }
        NativeOp::Abs => match ops.last_mut() {
            Some(last @ NativeOp::Neg) => {
                *last = NativeOp::Abs;
                true
            }
            Some(NativeOp::Abs) => true,
            _ => false,
        },
        _ => false,
    }
}

fn lower_constant_rhs_arithmetic(ops: &mut Vec<NativeOp>, instruction: &Instruction) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    let is_identity_rhs = match instruction {
        Instruction::Sub => value.to_bits() == 0.0_f64.to_bits(),
        Instruction::Mul | Instruction::Div => value.to_bits() == 1.0_f64.to_bits(),
        _ => false,
    };
    if is_identity_rhs {
        ops.pop();
        return true;
    }
    let op = match instruction {
        Instruction::Add => NativeOp::AddConst(value),
        Instruction::Sub => NativeOp::SubConst(value),
        Instruction::Mul => NativeOp::MulConst(value),
        Instruction::Div => NativeOp::DivConst(value),
        _ => return false,
    };
    ops.pop();
    ops.push(op);
    true
}

fn lower_constant_binary_arithmetic(ops: &mut Vec<NativeOp>, instruction: &Instruction) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = match instruction {
        Instruction::Add => left + right,
        Instruction::Sub => left - right,
        Instruction::Mul => left * right,
        Instruction::Div => left / right,
        _ => return false,
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_lhs_noncommutative_arithmetic(
    ops: &mut Vec<NativeOp>,
    instruction: &Instruction,
) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    let op = match instruction {
        Instruction::Sub => NativeOp::SubFromConst(value),
        Instruction::Div => NativeOp::DivFromConst(value),
        _ => return false,
    };
    ops.remove(lhs_index);
    ops.push(op);
    true
}

fn lower_constant_lhs_identity_arithmetic(
    ops: &mut Vec<NativeOp>,
    instruction: &Instruction,
) -> bool {
    if !matches!(instruction, Instruction::Mul) || ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    if value.to_bits() != 1.0_f64.to_bits() {
        return false;
    }
    ops.remove(lhs_index);
    true
}

fn lower_constant_binary_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = if constant_compare(op, left, right) {
        1.0
    } else {
        0.0
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_rhs_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::CompareConst(op, value));
    true
}

fn lower_constant_lhs_compare(ops: &mut Vec<NativeOp>, op: CompareOp) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    ops.remove(lhs_index);
    ops.push(NativeOp::CompareConst(flip_compare_op(op), value));
    true
}

fn lower_constant_binary_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    if ops.len() < 2 {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let rhs_index = ops.len() - 1;
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    let value = if constant_logical(op, left, right) {
        1.0
    } else {
        0.0
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(value));
    true
}

fn lower_constant_rhs_extremum(ops: &mut Vec<NativeOp>, op: ExtremumOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::ExtremumConst(op, value));
    true
}

fn lower_constant_binary_extremum(ops: &mut Vec<NativeOp>, op: ExtremumOp) -> bool {
    let (lhs_index, rhs_index) = match ops.len().checked_sub(2) {
        Some(lhs_index) => (lhs_index, lhs_index + 1),
        None => return false,
    };
    let (NativeOp::Const(left), NativeOp::Const(right)) = (ops[lhs_index], ops[rhs_index]) else {
        return false;
    };
    ops.truncate(lhs_index);
    ops.push(NativeOp::Const(constant_extremum(op, left, right)));
    true
}

fn lower_constant_rhs_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    let Some(NativeOp::Const(value)) = ops.last().copied() else {
        return false;
    };
    ops.pop();
    ops.push(NativeOp::LogicalConst(op, constant_truthy(value)));
    true
}

fn lower_constant_lhs_logical(ops: &mut Vec<NativeOp>, op: LogicalOp) -> bool {
    if ops.len() < 2 {
        return false;
    }
    let rhs_index = ops.len() - 1;
    if !is_independent_value_push(&ops[rhs_index]) {
        return false;
    }

    let lhs_index = ops.len() - 2;
    let NativeOp::Const(value) = ops[lhs_index] else {
        return false;
    };
    ops.remove(lhs_index);
    ops.push(NativeOp::LogicalConst(op, constant_truthy(value)));
    true
}

fn lower_constant_logical_not(ops: &mut Vec<NativeOp>) -> bool {
    let Some(NativeOp::Const(value)) = ops.last_mut() else {
        return false;
    };
    *value = if value.abs() < LOGICAL_EPSILON {
        1.0
    } else {
        0.0
    };
    true
}

fn lower_constant_ifelse(ops: &mut Vec<NativeOp>) -> bool {
    let (condition, then_value, else_value) = match ops.as_slice() {
        [
            ..,
            NativeOp::Const(condition),
            NativeOp::Const(then_value),
            NativeOp::Const(else_value),
        ] => (*condition, *then_value, *else_value),
        _ => return false,
    };
    let selected = if constant_truthy(condition) {
        then_value
    } else {
        else_value
    };
    ops.truncate(ops.len() - 3);
    ops.push(NativeOp::Const(selected));
    true
}

fn constant_truthy(value: f64) -> bool {
    value.abs() > LOGICAL_EPSILON
}

fn constant_compare(op: CompareOp, left: f64, right: f64) -> bool {
    match op {
        CompareOp::Gt => left > right,
        CompareOp::Lt => left < right,
        CompareOp::Ge => left >= right,
        CompareOp::Le => left <= right,
        CompareOp::Eq => (left - right).abs() < LOGICAL_EPSILON,
        CompareOp::Ne => (left - right).abs() >= LOGICAL_EPSILON,
    }
}

fn constant_logical(op: LogicalOp, left: f64, right: f64) -> bool {
    match op {
        LogicalOp::And => constant_truthy(left) && constant_truthy(right),
        LogicalOp::Or => constant_truthy(left) || constant_truthy(right),
        LogicalOp::Not => unreachable!("binary logical lowering only accepts and/or"),
    }
}

fn constant_extremum(op: ExtremumOp, left: f64, right: f64) -> f64 {
    match op {
        ExtremumOp::Min => left.min(right),
        ExtremumOp::Max => left.max(right),
    }
}

fn constant_binary_math(op: BinaryMathOp, left: f64, right: f64) -> f64 {
    match op {
        BinaryMathOp::Pow => left.powf(right),
        BinaryMathOp::Atan2 => left.atan2(right),
        BinaryMathOp::Hypot => left.hypot(right),
        BinaryMathOp::Mod => left % right,
    }
}

fn constant_integer_binary(op: IntegerBinaryOp, left: f64, right: f64) -> Option<f64> {
    let left = left as i64;
    let right = right as i64;
    let value = match op {
        IntegerBinaryOp::Shl => left.checked_shl(u32::try_from(right).ok()?)?,
        IntegerBinaryOp::Shr => left.checked_shr(u32::try_from(right).ok()?)?,
        IntegerBinaryOp::BitAnd => left & right,
        IntegerBinaryOp::BitOr => left | right,
        IntegerBinaryOp::BitXor => left ^ right,
    };
    Some(value as f64)
}

fn constant_unary_math(op: UnaryMathOp, value: f64) -> f64 {
    match op {
        UnaryMathOp::Exp => value.exp(),
        UnaryMathOp::Log => value.ln(),
        UnaryMathOp::Log10 => value.log10(),
        UnaryMathOp::Sin => value.sin(),
        UnaryMathOp::Cos => value.cos(),
        UnaryMathOp::Tan => value.tan(),
        UnaryMathOp::Sinh => value.sinh(),
        UnaryMathOp::Cosh => value.cosh(),
        UnaryMathOp::Tanh => value.tanh(),
        UnaryMathOp::Asinh => value.asinh(),
        UnaryMathOp::Acosh => value.acosh(),
        UnaryMathOp::Atanh => value.atanh(),
        UnaryMathOp::Limexp => constant_limexp(value),
        UnaryMathOp::Asin => value.asin(),
        UnaryMathOp::Acos => value.acos(),
        UnaryMathOp::Atan => value.atan(),
        UnaryMathOp::Floor => value.floor(),
        UnaryMathOp::Ceil => value.ceil(),
    }
}

fn constant_limexp(value: f64) -> f64 {
    const LIMIT: f64 = 40.0;
    if value > LIMIT {
        let exp_limit = LIMIT.exp();
        exp_limit * (1.0 + value - LIMIT)
    } else if value < -LIMIT {
        (-LIMIT).exp()
    } else {
        value.exp()
    }
}

pub(super) fn constant_dynamic_variable_slot(
    raw_index: f64,
    base: usize,
    len: usize,
    lower: i64,
) -> Option<usize> {
    if !raw_index.is_finite() {
        return None;
    }

    let index = rounded_i64_without_saturation(raw_index)?;
    let offset = index.checked_sub(lower)?;
    let offset = usize::try_from(offset).ok()?;
    if offset >= len {
        return None;
    }

    base.checked_add(offset)
}

fn rounded_i64_without_saturation(value: f64) -> Option<i64> {
    const I64_MAX_EXCLUSIVE_AS_F64: f64 = 9_223_372_036_854_775_808.0;

    let rounded = value.round();
    if rounded < i64::MIN as f64 || rounded >= I64_MAX_EXCLUSIVE_AS_F64 {
        return None;
    }

    Some(rounded as i64)
}

fn is_independent_value_push(op: &NativeOp) -> bool {
    !matches!(op, NativeOp::Const(_)) && native_op_stack_effect(op) == (0, 1)
}

fn compute_native_max_stack_depth(
    model: SmolStr,
    entry_kind: EntryKind,
    ops: &[NativeOp],
) -> JitResult<usize> {
    let mut depth = 0usize;
    let mut max_stack_depth = 0usize;

    for op in ops {
        let (pops, pushes) = native_op_stack_effect(op);
        if depth < pops {
            return Err(stack_error(
                model.clone(),
                entry_kind,
                format!("optimized native op {op:?} requires stack depth {pops}, found {depth}"),
            ));
        }
        depth = depth - pops + pushes;
        max_stack_depth = max_stack_depth.max(depth);
    }

    if depth != 1 {
        return Err(stack_error(
            model,
            entry_kind,
            format!("optimized native stack depth {depth}, expected 1"),
        ));
    }

    Ok(max_stack_depth)
}

fn native_op_stack_effect(op: &NativeOp) -> (usize, usize) {
    match op {
        NativeOp::Const(_)
        | NativeOp::LoadParam(_)
        | NativeOp::LoadParamGiven(_)
        | NativeOp::LoadPortConnected(_)
        | NativeOp::LoadVoltage { .. }
        | NativeOp::LoadCurrent(_)
        | NativeOp::LoadPriorCurrent(_)
        | NativeOp::LoadInternalVoltage(_)
        | NativeOp::LoadVariable(_)
        | NativeOp::LoadBranchUnknown(_)
        | NativeOp::LoadTemperature
        | NativeOp::LoadThermalVoltage
        | NativeOp::LoadTime
        | NativeOp::Analysis(_)
        | NativeOp::LoadMfactor => (0, 1),

        NativeOp::LoadVariableDyn { .. }
        | NativeOp::AddConst(_)
        | NativeOp::SubConst(_)
        | NativeOp::MulConst(_)
        | NativeOp::DivConst(_)
        | NativeOp::SubFromConst(_)
        | NativeOp::DivFromConst(_)
        | NativeOp::CompareConst(_, _)
        | NativeOp::ExtremumConst(_, _)
        | NativeOp::LogicalConst(_, _)
        | NativeOp::Neg
        | NativeOp::Abs
        | NativeOp::Square
        | NativeOp::Sqrt
        | NativeOp::Logical(LogicalOp::Not)
        | NativeOp::UnaryMath(_)
        | NativeOp::TableLookup(_)
        | NativeOp::TableDerivative(_)
        | NativeOp::LaplaceState(_)
        | NativeOp::ZiState(_)
        | NativeOp::WhiteNoise
        | NativeOp::DdtState(_)
        | NativeOp::DdtJacobian
        | NativeOp::IdtJacobian => (1, 1),

        NativeOp::Add
        | NativeOp::Sub
        | NativeOp::Mul
        | NativeOp::Div
        | NativeOp::Compare(_)
        | NativeOp::Logical(LogicalOp::And | LogicalOp::Or)
        | NativeOp::Extremum(_)
        | NativeOp::BinaryMath(_)
        | NativeOp::IntegerBinary(_)
        | NativeOp::LimitState(_)
        | NativeOp::TimerState(_)
        | NativeOp::AbsDelayState(_)
        | NativeOp::CrossState(_)
        | NativeOp::FlickerNoise
        | NativeOp::IdtState(_) => (2, 1),

        NativeOp::SlewState(_) | NativeOp::IfElse => (3, 1),
        NativeOp::TransitionState(_) | NativeOp::IdtModState(_) => (4, 1),
    }
}

fn stack_error(model: SmolStr, entry_kind: EntryKind, detail: String) -> JitError {
    JitError::InvalidCanonicalIr {
        model,
        detail: format!("{entry_kind:?} expression stack invalid: {detail}").into(),
    }
}

fn lower_voltage_node(
    model: SmolStr,
    node: usize,
    terminal_count: usize,
    internal_node_count: usize,
) -> JitResult<VoltageNode> {
    if node == usize::MAX {
        return Ok(VoltageNode::Ground);
    }

    if node < terminal_count {
        return Ok(VoltageNode::Terminal(node));
    }

    let internal_index = node - terminal_count;
    if internal_index < internal_node_count {
        return Ok(VoltageNode::Internal(internal_index));
    }

    Err(JitError::unsupported_program_op(
        model,
        format!("PushVoltage unified node {node}"),
    ))
}

fn current_pair_index(
    model: SmolStr,
    pos: usize,
    neg: usize,
    terminal_count: usize,
) -> JitResult<usize> {
    terminal_pair_current_index(pos, neg, terminal_count).ok_or_else(|| {
        JitError::unsupported_program_op(
            model,
            format!(
                "PushCurrent terminal pair {}",
                format_current_pair(pos, neg)
            ),
        )
    })
}

fn branch_voltage_derivative(pos: Option<NodeId>, neg: Option<NodeId>, wrt: NodeId) -> f64 {
    let mut value = 0.0;
    if pos == Some(wrt) {
        value += 1.0;
    }
    if neg == Some(wrt) {
        value -= 1.0;
    }
    value
}

fn format_current_pair(pos: usize, neg: usize) -> String {
    format!(
        "{},{}",
        format_current_endpoint(pos),
        format_current_endpoint(neg)
    )
}

fn format_current_endpoint(endpoint: usize) -> String {
    if endpoint == CURRENT_PAIR_GROUND {
        "ground".to_string()
    } else {
        endpoint.to_string()
    }
}

fn instruction_name(instruction: &Instruction) -> &'static str {
    match instruction {
        Instruction::PushConst(_) => "PushConst",
        Instruction::PushParam(_) => "PushParam",
        Instruction::PushParamGiven(_) => "PushParamGiven",
        Instruction::PushBranchCurrent(_) => "PushBranchCurrent",
        Instruction::PushVoltage(_, _) => "PushVoltage",
        Instruction::PushCurrent(_, _) => "PushCurrent",
        Instruction::PushInternalVoltage(_) => "PushInternalVoltage",
        Instruction::PushVariable(_) => "PushVariable",
        Instruction::PushVariableDyn { .. } => "PushVariableDyn",
        Instruction::PushTemperature => "PushTemperature",
        Instruction::PushVt => "PushVt",
        Instruction::PushTime => "PushTime",
        Instruction::PushMfactor => "PushMfactor",
        Instruction::PushPortConnected(_) => "PushPortConnected",
        Instruction::ZiState(_) => "ZiState",
        Instruction::Add => "Add",
        Instruction::Sub => "Sub",
        Instruction::Mul => "Mul",
        Instruction::Div => "Div",
        Instruction::Pow => "Pow",
        Instruction::Mod => "Mod",
        Instruction::Shl => "Shl",
        Instruction::Shr => "Shr",
        Instruction::BitAnd => "BitAnd",
        Instruction::BitOr => "BitOr",
        Instruction::BitXor => "BitXor",
        Instruction::Neg => "Neg",
        Instruction::Abs => "Abs",
        Instruction::Sqrt => "Sqrt",
        Instruction::Exp => "Exp",
        Instruction::Log => "Log",
        Instruction::Log10 => "Log10",
        Instruction::Sin => "Sin",
        Instruction::Cos => "Cos",
        Instruction::Tan => "Tan",
        Instruction::Sinh => "Sinh",
        Instruction::Cosh => "Cosh",
        Instruction::Tanh => "Tanh",
        Instruction::Min => "Min",
        Instruction::Max => "Max",
        Instruction::Limexp => "Limexp",
        Instruction::Asin => "Asin",
        Instruction::Acos => "Acos",
        Instruction::Atan => "Atan",
        Instruction::Atan2 => "Atan2",
        Instruction::Floor => "Floor",
        Instruction::Ceil => "Ceil",
        Instruction::FnPow => "FnPow",
        Instruction::Gt => "Gt",
        Instruction::Lt => "Lt",
        Instruction::Ge => "Ge",
        Instruction::Le => "Le",
        Instruction::Eq => "Eq",
        Instruction::Ne => "Ne",
        Instruction::And => "And",
        Instruction::Or => "Or",
        Instruction::Not => "Not",
        Instruction::DdtState(_) => "DdtState",
        Instruction::IdtState(_) => "IdtState",
        Instruction::IdtModState(_) => "IdtModState",
        Instruction::DdtJacobian => "DdtJacobian",
        Instruction::IdtJacobian => "IdtJacobian",
        Instruction::TableDerivative(_) => "TableDerivative",
        Instruction::LimitState(_) => "LimitState",
        Instruction::TableLookup(_) => "TableLookup",
        Instruction::AbsDelayState(_) => "AbsDelayState",
        Instruction::TransitionState(_) => "TransitionState",
        Instruction::SlewState(_) => "SlewState",
        Instruction::CrossState(_) => "CrossState",
        Instruction::WhiteNoise => "WhiteNoise",
        Instruction::FlickerNoise => "FlickerNoise",
        Instruction::Analysis(_) => "Analysis",
        Instruction::AboveState(_) => "AboveState",
        Instruction::TimerState(_) => "TimerState",
        Instruction::LaplaceState(_) => "LaplaceState",
        Instruction::IfElse => "IfElse",
    }
}

fn compare_op(instruction: &Instruction) -> CompareOp {
    match instruction {
        Instruction::Gt => CompareOp::Gt,
        Instruction::Lt => CompareOp::Lt,
        Instruction::Ge => CompareOp::Ge,
        Instruction::Le => CompareOp::Le,
        Instruction::Eq => CompareOp::Eq,
        Instruction::Ne => CompareOp::Ne,
        _ => unreachable!("comparison lowering only accepts ordered comparison instructions"),
    }
}

fn flip_compare_op(op: CompareOp) -> CompareOp {
    match op {
        CompareOp::Gt => CompareOp::Lt,
        CompareOp::Lt => CompareOp::Gt,
        CompareOp::Ge => CompareOp::Le,
        CompareOp::Le => CompareOp::Ge,
        CompareOp::Eq => CompareOp::Eq,
        CompareOp::Ne => CompareOp::Ne,
    }
}

fn logical_op(instruction: &Instruction) -> LogicalOp {
    match instruction {
        Instruction::And => LogicalOp::And,
        Instruction::Or => LogicalOp::Or,
        _ => unreachable!("logical lowering only accepts binary logical instructions"),
    }
}

fn extremum_op(instruction: &Instruction) -> ExtremumOp {
    match instruction {
        Instruction::Min => ExtremumOp::Min,
        Instruction::Max => ExtremumOp::Max,
        _ => unreachable!("extremum lowering only accepts min/max instructions"),
    }
}

fn unary_math_op(instruction: &Instruction) -> UnaryMathOp {
    match instruction {
        Instruction::Exp => UnaryMathOp::Exp,
        Instruction::Log => UnaryMathOp::Log,
        Instruction::Log10 => UnaryMathOp::Log10,
        Instruction::Sin => UnaryMathOp::Sin,
        Instruction::Cos => UnaryMathOp::Cos,
        Instruction::Tan => UnaryMathOp::Tan,
        Instruction::Sinh => UnaryMathOp::Sinh,
        Instruction::Cosh => UnaryMathOp::Cosh,
        Instruction::Tanh => UnaryMathOp::Tanh,
        Instruction::Limexp => UnaryMathOp::Limexp,
        Instruction::Asin => UnaryMathOp::Asin,
        Instruction::Acos => UnaryMathOp::Acos,
        Instruction::Atan => UnaryMathOp::Atan,
        Instruction::Floor => UnaryMathOp::Floor,
        Instruction::Ceil => UnaryMathOp::Ceil,
        _ => unreachable!("unary math lowering only accepts supported unary math instructions"),
    }
}

fn binary_math_op(instruction: &Instruction) -> BinaryMathOp {
    match instruction {
        Instruction::Pow | Instruction::FnPow => BinaryMathOp::Pow,
        Instruction::Atan2 => BinaryMathOp::Atan2,
        Instruction::Mod => BinaryMathOp::Mod,
        _ => unreachable!("binary math lowering only accepts supported binary math instructions"),
    }
}

fn arithmetic_op(instruction: &Instruction) -> NativeOp {
    match instruction {
        Instruction::Add => NativeOp::Add,
        Instruction::Sub => NativeOp::Sub,
        Instruction::Mul => NativeOp::Mul,
        Instruction::Div => NativeOp::Div,
        _ => unreachable!("arithmetic lowering only accepts add/sub/mul/div instructions"),
    }
}

fn integer_binary_op(instruction: &Instruction) -> IntegerBinaryOp {
    match instruction {
        Instruction::Shl => IntegerBinaryOp::Shl,
        Instruction::Shr => IntegerBinaryOp::Shr,
        Instruction::BitAnd => IntegerBinaryOp::BitAnd,
        Instruction::BitOr => IntegerBinaryOp::BitOr,
        Instruction::BitXor => IntegerBinaryOp::BitXor,
        _ => unreachable!("integer binary lowering only accepts supported integer instructions"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        AnalogOperator, BinaryExpr, BinaryOp, BranchAccess, Expression, NoiseSource, NumberLit,
        PortDirection,
    };
    use crate::canonical_ir::{CanonicalMetadata, HirExprKind, HirModel, MirModel};
    use crate::semantic::{AnalyzedContribution, AnalyzedModule, AnalyzedPort, SymbolTable};
    use crate::source::Span;
    use crate::types::ValueType;
    use crate::{CompilerOptions, VerilogACompiler};
    use std::collections::HashMap;

    fn limits(terminal_count: usize, internal_node_count: usize) -> NativeLoweringLimits<'static> {
        NativeLoweringLimits::new(terminal_count, internal_node_count, 8, 8, 8)
            .with_lookup_table_count(8)
    }

    #[test]
    fn native_program_dependency_metadata_accepts_matching_loads() {
        let program = NativeProgram {
            ops: vec![
                NativeOp::LoadCurrent(4),
                NativeOp::LoadCurrent(4),
                NativeOp::LoadPriorCurrent(1),
                NativeOp::LoadPriorCurrent(2),
                NativeOp::LoadPriorCurrent(1),
            ],
            max_stack_depth: 5,
            current_pair_dependencies: vec![4],
            prior_current_dependencies: vec![1, 2],
        };

        program
            .validate_dependency_metadata()
            .expect("matching metadata must pass");
    }

    #[test]
    fn native_program_dependency_metadata_rejects_missing_current_load() {
        let program = NativeProgram {
            ops: vec![NativeOp::LoadCurrent(4)],
            max_stack_depth: 1,
            current_pair_dependencies: Vec::new(),
            prior_current_dependencies: Vec::new(),
        };

        let err = program
            .validate_dependency_metadata()
            .expect_err("missing current dependency must fail native compile");
        let msg = err.to_string();
        assert!(msg.contains("current-pair dependency metadata mismatch"));
        assert!(msg.contains("recorded []"));
        assert!(msg.contains("op stream requires [4]"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn native_program_dependency_metadata_rejects_stale_prior_current_load() {
        let program = NativeProgram {
            ops: vec![NativeOp::Const(1.0)],
            max_stack_depth: 1,
            current_pair_dependencies: Vec::new(),
            prior_current_dependencies: vec![0],
        };

        let err = program
            .validate_dependency_metadata()
            .expect_err("stale prior-current dependency must fail native compile");
        let msg = err.to_string();
        assert!(msg.contains("prior-current dependency metadata mismatch"));
        assert!(msg.contains("recorded [0]"));
        assert!(msg.contains("op stream requires []"));
        assert!(msg.contains("no interpreter fallback"));
    }

    fn analyzed_two_terminal_hir(module_name: &str, expression: Expression) -> HirModel {
        let span = Span::dummy();
        let analyzed = AnalyzedModule {
            name: module_name.into(),
            ports: vec![
                AnalyzedPort {
                    name: "p".into(),
                    direction: PortDirection::Inout,
                    discipline: "electrical".into(),
                    nature_potential: Some("voltage".into()),
                    nature_flow: Some("current".into()),
                },
                AnalyzedPort {
                    name: "n".into(),
                    direction: PortDirection::Inout,
                    discipline: "electrical".into(),
                    nature_potential: Some("voltage".into()),
                    nature_flow: Some("current".into()),
                },
            ],
            parameters: Vec::new(),
            param_aliases: Vec::new(),
            variables: Vec::new(),
            branches: Vec::new(),
            contributions: vec![AnalyzedContribution {
                branch: "p,n".into(),
                declared_branch: None,
                is_current: true,
                indirect: false,
                expression,
                expr_type: ValueType::Real,
                span,
            }],
            statements: Vec::new(),
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
            arrays: HashMap::new(),
            symbol_table: SymbolTable::new(),
        };
        let metadata = CanonicalMetadata::for_source("fixture", module_name);
        HirModel::from_analyzed_module(&metadata, &analyzed)
    }

    fn analyzed_two_terminal_mir(module_name: &str, expression: Expression) -> MirModel {
        let hir = analyzed_two_terminal_hir(module_name, expression);
        MirModel::from_hir(&hir).expect("lower explicit canonical HIR to MIR")
    }

    fn number(value: f64, raw: &str) -> Expression {
        Expression::Number(NumberLit {
            value,
            raw: raw.into(),
            span: Span::dummy(),
        })
    }

    fn explicit_analog_limexp_mir() -> MirModel {
        let span = Span::dummy();
        let expression = Expression::AnalogOperator(AnalogOperator::Limexp {
            expr: Box::new(Expression::BranchAccess(BranchAccess::Nodes {
                access: "V".into(),
                pos: "p".into(),
                neg: Some("n".into()),
                span,
            })),
            span,
        });
        let hir = analyzed_two_terminal_hir("mir_analog_limexp", expression);
        let contribution_expr = &hir.contributions[0].expression;
        assert!(
            matches!(
                hir.expressions[usize::from(contribution_expr.id)].kind,
                HirExprKind::AnalogOperator { .. }
            ),
            "test fixture must preserve explicit canonical analog operator"
        );

        MirModel::from_hir(&hir).expect("lower explicit analog limexp HIR to MIR")
    }

    fn explicit_noise_sources_mir() -> MirModel {
        let span = Span::dummy();
        analyzed_two_terminal_mir(
            "mir_explicit_noise_sources",
            Expression::Binary(BinaryExpr {
                op: BinaryOp::Add,
                left: Box::new(Expression::Binary(BinaryExpr {
                    op: BinaryOp::Add,
                    left: Box::new(Expression::NoiseSource(NoiseSource::White {
                        power: Box::new(number(1.0, "1.0")),
                        name: Some("white".into()),
                        span,
                    })),
                    right: Box::new(Expression::NoiseSource(NoiseSource::Flicker {
                        power: Box::new(number(2.0, "2.0")),
                        exponent: Box::new(number(1.0, "1.0")),
                        name: Some("flicker".into()),
                        span,
                    })),
                    span,
                })),
                right: Box::new(Expression::NoiseSource(NoiseSource::Table {
                    data: vec![number(1.0, "1.0"), number(1.0e-18, "1.0e-18")],
                    name: Some("table".into()),
                    span,
                })),
                span,
            }),
        )
    }

    #[test]
    fn lowers_supported_stack_program_to_native_expr_ops() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushVoltage(0, 1),
                Instruction::PushParam(0),
                Instruction::Div,
                Instruction::PushConst(2.0),
                Instruction::Mul,
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(2, 0))
                .expect("lower supported program");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::LoadParam(0),
                NativeOp::Div,
                NativeOp::MulConst(2.0),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_simple_mir_equation_expression_to_native_program() {
        let source = r#"
module mir_res(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    I(p, n) <+ V(p, n) / r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_res",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 0),
        )
        .expect("lower MIR equation to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::LoadParam(0),
                NativeOp::Div,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_unary_compositions_without_redundant_ops() {
        let source = r#"
module mir_unary_composition(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ abs(-V(p, n));
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_unary_composition",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical unary composition to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Abs,
            ]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_constant_sqrt_to_literal() {
        let source = r#"
module mir_constant_sqrt(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ sqrt(49.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_constant_sqrt",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical constant sqrt to native program");

        assert_eq!(program.ops(), &[NativeOp::Const(7.0)]);
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_constant_ifelse_to_selected_literal() {
        let source = r#"
module mir_constant_ifelse(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ (1.0e-15 ? 7.0 : -0.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_constant_ifelse",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical constant ifelse to native program");

        match program.ops() {
            [NativeOp::Const(value)] => assert_eq!(value.to_bits(), (-0.0_f64).to_bits()),
            ops => panic!("expected folded ifelse literal, got {ops:?}"),
        }
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_constant_above_to_ordered_literal() {
        let source = r#"
module mir_constant_above(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ above(2.0, 1.0);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let equation_id = crate::canonical_ir::EquationId::new(0);
        let bytecode_program = &model.stamp_programs[0].value_program;
        let above_slots = canonical_above_slots_for_equation(
            "mir_constant_above".into(),
            &artifact.mir,
            equation_id,
            bytecode_program,
        )
        .expect("map canonical above to bytecode detector slot");

        assert_eq!(above_slots.len(), 1);

        let program = NativeProgram::from_mir_equation(
            "mir_constant_above",
            EntryKind::StampValue,
            &artifact.mir,
            equation_id,
            NativeLoweringLimits::new(2, 0, 0, 0, 0).with_canonical_above_slots(&above_slots),
        )
        .expect("lower canonical constant above to native program");

        assert_eq!(program.ops(), &[NativeOp::Const(1.0)]);
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_constant_min_to_literal() {
        let source = r#"
module mir_constant_min(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ min(5.0, -2.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_constant_min",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical constant min to native program");

        assert_eq!(program.ops(), &[NativeOp::Const(-2.0)]);
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_potential_branch_current_unknown_to_native_program() {
        let source = r#"
module mir_potential_branch_current(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    V(p, n) <+ I(p, n) * r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_potential_branch_current",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 1),
        )
        .expect("lower canonical potential branch current to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadBranchUnknown(0),
                NativeOp::LoadParam(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_reversed_canonical_potential_branch_current_unknown_with_sign_flip() {
        let source = r#"
module mir_reversed_potential_branch_current(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    V(p, n) <+ I(n, p) * r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_reversed_potential_branch_current",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 1),
        )
        .expect("lower reversed canonical potential branch current to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadBranchUnknown(0),
                NativeOp::Neg,
                NativeOp::LoadParam(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_idtmod_without_modulus_as_idt_state() {
        let source = r#"
module mir_idtmod_nomod(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ idtmod(1.0, 0.5);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let equation_id = crate::canonical_ir::EquationId::new(0);
        let bytecode_program = &model.stamp_programs[0].value_program;
        let idt_slots = canonical_idt_slots_for_equation(
            "mir_idtmod_nomod".into(),
            &artifact.mir,
            equation_id,
            bytecode_program,
        )
        .expect("map modulus-free idtmod to bytecode idt slot");
        let idtmod_slots = canonical_idtmod_slots_for_equation(
            "mir_idtmod_nomod".into(),
            &artifact.mir,
            equation_id,
            bytecode_program,
        )
        .expect("modulus-free idtmod is not an idtmod state slot");

        assert_eq!(idt_slots.len(), 1);
        assert!(idtmod_slots.is_empty());

        let program = NativeProgram::from_mir_equation(
            "mir_idtmod_nomod",
            EntryKind::StampValue,
            &artifact.mir,
            equation_id,
            NativeLoweringLimits::new(2, 0, 0, 0, 0)
                .with_canonical_idt_slots(&idt_slots)
                .with_canonical_idtmod_slots(&idtmod_slots),
        )
        .expect("lower canonical idtmod without modulus as idt state");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::Const(0.5),
                NativeOp::IdtState(0),
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_named_branch_voltage_to_native_program() {
        let source = r#"
module mir_named_branch_voltage(p, n);
  inout p, n;
  electrical p, n;
  branch (p, n) probe;
  parameter real r = 2.0;
  analog begin
    I(p, n) <+ V(probe) / r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_named_branch_voltage",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 0),
        )
        .expect("lower canonical named branch voltage to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::LoadParam(0),
                NativeOp::Div,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_custom_potential_node_access_to_native_voltage() {
        let source = r#"
`include "disciplines.vams"
module mir_custom_potential_node(p, n);
  inout p, n;
  electrical p, n;
  thermal t;
  analog begin
    I(p, n) <+ Temp(t);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_custom_potential_node",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 1, 0, 0, 0),
        )
        .expect("lower canonical custom potential node access to native program");

        assert_eq!(
            program.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Internal(0),
                neg: VoltageNode::Ground,
            }]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_custom_potential_named_branch_to_native_voltage() {
        let source = r#"
`include "disciplines.vams"
module mir_custom_potential_named_branch(p, n);
  inout p, n;
  electrical p, n;
  thermal t;
  branch (t) th;
  analog begin
    I(p, n) <+ Temp(th);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_custom_potential_named_branch",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 1, 0, 0, 0),
        )
        .expect("lower canonical custom potential named branch to native program");

        assert_eq!(
            program.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Internal(0),
                neg: VoltageNode::Ground,
            }]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_named_potential_branch_current_to_native_program() {
        let source = r#"
module mir_named_potential_branch_current(p, n);
  inout p, n;
  electrical p, n;
  branch (p, n) probe;
  parameter real r = 2.0;
  analog begin
    V(probe) <+ I(probe) * r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_named_potential_branch_current",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 1),
        )
        .expect("lower canonical named potential branch current to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadBranchUnknown(0),
                NativeOp::LoadParam(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_single_ended_potential_current_probe_to_branch_unknown() {
        let source = r#"
module mir_single_ended_potential_current(p);
  inout p;
  electrical p;
  parameter real r = 2.0;
  analog begin
    V(p) <+ I(p) * r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_single_ended_potential_current",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(1, 0, 1, 0, 1),
        )
        .expect("lower canonical single-ended potential branch current to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadBranchUnknown(0),
                NativeOp::LoadParam(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_explicit_ground_potential_current_probe_to_branch_unknown() {
        let source = r#"
module mir_explicit_ground_potential_current(p);
  inout p;
  electrical p;
  parameter real r = 2.0;
  analog begin
    V(p) <+ I(p, 0) * r;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_explicit_ground_potential_current",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(1, 0, 1, 0, 1),
        )
        .expect("lower canonical explicit-ground potential branch current to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadBranchUnknown(0),
                NativeOp::LoadParam(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_named_branch_current_from_prior_contribution() {
        let source = r#"
module mir_named_branch_current_probe(p, n);
  inout p, n;
  electrical p, n, sense_node;
  branch (sense_node) sense;
  analog begin
    I(sense) <+ V(p, n);
    I(p, n) <+ 2.0 * I(sense);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_named_branch_current_probe",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(1),
            NativeLoweringLimits::new(2, 1, 0, 0, 0),
        )
        .expect("lower canonical named branch current from prior contribution");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::Const(2.0),
                NativeOp::LoadPriorCurrent(0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
        assert_eq!(program.current_pair_dependencies(), &[]);
        assert_eq!(program.prior_current_dependencies(), &[0]);
    }

    #[test]
    fn lowers_canonical_named_branch_current_as_sum_of_prior_contributions() {
        let source = r#"
module mir_named_branch_current_sum(p, n);
  inout p, n;
  electrical p, n, sense_node;
  branch (sense_node) sense;
  analog begin
    I(sense) <+ 1.0;
    I(sense) <+ V(p, n);
    I(p, n) <+ I(sense);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_named_branch_current_sum",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(2),
            NativeLoweringLimits::new(2, 1, 0, 0, 0),
        )
        .expect("lower canonical named branch current sum from prior contributions");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadPriorCurrent(0),
                NativeOp::LoadPriorCurrent(1),
                NativeOp::Add,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
        assert_eq!(program.prior_current_dependencies(), &[0, 1]);
    }

    #[test]
    fn lowers_canonical_large_signal_noise_sources_to_native_zero_ops() {
        let source = r#"
module mir_noise_sources(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ white_noise($temperature, "thermal") + flicker_noise(2.0, 1.0, "flicker");
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_noise_sources",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical noise sources to native large-signal zero ops");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::WhiteNoise,
                NativeOp::Const(2.0),
                NativeOp::Const(1.0),
                NativeOp::FlickerNoise,
                NativeOp::Add,
            ]
        );
        assert_eq!(program.max_stack_depth(), 3);
    }

    #[test]
    fn lowers_explicit_canonical_noise_source_nodes_to_native_zero_ops() {
        let mir = explicit_noise_sources_mir();

        let program = NativeProgram::from_mir_equation(
            "mir_explicit_noise_sources",
            EntryKind::StampValue,
            &mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower explicit canonical noise source nodes to native large-signal zero ops");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::WhiteNoise,
                NativeOp::Const(2.0),
                NativeOp::Const(1.0),
                NativeOp::FlickerNoise,
                NativeOp::Add,
                NativeOp::AddConst(0.0),
            ]
        );
        assert_eq!(program.max_stack_depth(), 3);
    }

    #[test]
    fn mir_equation_lowering_rejects_unsupported_canonical_expression_without_fallback() {
        let source = r#"
module mir_unsupported(p, n);
  inout p, n;
  electrical p, n;
  real x;
  analog begin
    x = 1.0;
    I(p, n) <+ x;
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let error = NativeProgram::from_mir_equation(
            "mir_unsupported",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect_err("unsupported MIR variable identifier must hard-fail");

        assert!(
            error
                .to_string()
                .contains("native JIT does not support canonical op expression identifier x"),
            "{error}"
        );
    }

    #[test]
    fn lowers_canonical_intrinsic_calls_to_native_program() {
        let source = r#"
module mir_intrinsics(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    I(p, n) <+ sqrt(abs(V(p, n))) + max(r, 1.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_intrinsics",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 1, 0, 0),
        )
        .expect("lower canonical intrinsics to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Abs,
                NativeOp::Sqrt,
                NativeOp::LoadParam(0),
                NativeOp::ExtremumConst(ExtremumOp::Max, 1.0),
                NativeOp::Add,
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_canonical_limexp_call_to_native_program() {
        let source = r#"
module mir_limexp(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ limexp(V(p, n));
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_limexp",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical limexp to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::UnaryMath(UnaryMathOp::Limexp),
            ]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_analog_limexp_operator_to_native_program() {
        let mir = explicit_analog_limexp_mir();

        let program = NativeProgram::from_mir_equation(
            "mir_analog_limexp",
            EntryKind::StampValue,
            &mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower explicit canonical analog limexp operator to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::UnaryMath(UnaryMathOp::Limexp),
            ]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_binary_intrinsic_calls_to_native_program() {
        let source = r#"
module mir_pow(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ pow(V(p, n), 2.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_pow",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical binary intrinsic to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Square,
            ]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_identity_power_to_native_base_expression() {
        let source = r#"
module mir_identity_pow(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ pow(V(p, n), 1.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_identity_pow",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical identity power to native program");

        assert_eq!(
            program.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(0),
                neg: VoltageNode::Terminal(1),
            }]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_reciprocal_power_to_native_div_from_const() {
        let source = r#"
module mir_reciprocal_pow(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ pow(V(p, n), -1.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_reciprocal_pow",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical reciprocal power to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::DivFromConst(1.0),
            ]
        );
        assert_eq!(program.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_canonical_hypot_intrinsic_to_native_program() {
        let source = r#"
module mir_hypot(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ hypot(V(p, n), 2.0);
  end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let program = NativeProgram::from_mir_equation(
            "mir_hypot",
            EntryKind::StampValue,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower canonical hypot to native program");

        assert_eq!(
            program.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Const(2.0),
                NativeOp::BinaryMath(BinaryMathOp::Hypot),
            ]
        );
        assert_eq!(program.max_stack_depth(), 2);
    }

    #[test]
    fn lowers_constant_rhs_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Add, NativeOp::AddConst(4.0)),
            (Instruction::Sub, NativeOp::SubConst(4.0)),
            (Instruction::Mul, NativeOp::MulConst(4.0)),
            (Instruction::Div, NativeOp::DivConst(4.0)),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-rhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS arithmetic has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
        }
    }

    #[test]
    fn keeps_positive_zero_rhs_add_to_preserve_signed_zero() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.0),
                Instruction::Add,
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "literal-rhs-add-zero",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect("positive RHS zero addition should remain explicit");

        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::AddConst(0.0)]
        );
    }

    #[test]
    fn drops_rhs_arithmetic_identities_without_runtime_ops() {
        let cases = [
            ("sub-positive-zero", Instruction::Sub, 0.0),
            ("mul-one", Instruction::Mul, 1.0),
            ("div-one", Instruction::Div, 1.0),
        ];

        for (name, instruction, literal) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(literal),
                    instruction.clone(),
                ],
            };

            let lowered =
                NativeProgram::from_bytecode(name, EntryKind::Assignment, &program, limits(0, 0))
                    .expect("RHS arithmetic identity should lower away");

            assert_eq!(lowered.max_stack_depth(), 1, "{name}");
            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature], "{name}");
        }
    }

    #[test]
    fn keeps_rhs_arithmetic_non_identities_as_runtime_ops() {
        let cases = [
            ("sub-negative-zero", Instruction::Sub, -0.0),
            ("mul-negative-one", Instruction::Mul, -1.0),
            ("div-negative-one", Instruction::Div, -1.0),
        ];

        for (name, instruction, literal) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(literal),
                    instruction.clone(),
                ],
            };

            let lowered =
                NativeProgram::from_bytecode(name, EntryKind::Assignment, &program, limits(0, 0))
                    .expect("non-identity RHS arithmetic literal should remain explicit");

            assert_eq!(lowered.max_stack_depth(), 1, "{name}");
            let [NativeOp::LoadTemperature, op] = lowered.ops() else {
                panic!("{name}: expected load plus explicit constant op, got {lowered:?}");
            };
            let value = match (instruction, *op) {
                (Instruction::Sub, NativeOp::SubConst(value))
                | (Instruction::Mul, NativeOp::MulConst(value))
                | (Instruction::Div, NativeOp::DivConst(value)) => value,
                _ => panic!("{name}: unexpected native op {op:?}"),
            };
            assert_eq!(value.to_bits(), literal.to_bits(), "{name}");
        }
    }

    #[test]
    fn drops_lhs_multiplicative_identity_without_runtime_op() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(1.0),
                Instruction::PushTemperature,
                Instruction::Mul,
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "literal-lhs-mul-one",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect("LHS multiplicative identity should lower away");

        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature]);
    }

    #[test]
    fn folds_constant_binary_arithmetic_to_exact_literal() {
        let cases = [
            (
                "add finite",
                Instruction::Add,
                3.25_f64,
                4.5_f64,
                (3.25_f64 + 4.5_f64).to_bits(),
            ),
            (
                "sub finite",
                Instruction::Sub,
                3.25_f64,
                4.5_f64,
                (3.25_f64 - 4.5_f64).to_bits(),
            ),
            (
                "mul signed zero",
                Instruction::Mul,
                -0.0_f64,
                4.5_f64,
                (-0.0_f64 * 4.5_f64).to_bits(),
            ),
            (
                "div negative zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                (10.0_f64 / -0.0_f64).to_bits(),
            ),
            (
                "mul unordered",
                Instruction::Mul,
                f64::from_bits(0x7ff8_0000_0000_0003),
                4.5_f64,
                (f64::from_bits(0x7ff8_0000_0000_0003) * 4.5_f64).to_bits(),
            ),
        ];

        for (case, instruction, left, right, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-binary-arithmetic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant binary arithmetic folds to exact literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should not allocate an arithmetic stack slot"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_constant_lhs_noncommutative_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Sub, NativeOp::SubFromConst(10.0)),
            (Instruction::Div, NativeOp::DivFromConst(10.0)),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(10.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-lhs-arithmetic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS arithmetic has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == 10.0_f64.to_bits())
                ),
                "{instruction_name} should consume the LHS literal in the arithmetic op"
            );
            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
        }
    }

    #[test]
    fn lowers_thermal_voltage_context_read() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVt],
        };

        let lowered =
            NativeProgram::from_bytecode("vt", EntryKind::StampValue, &program, limits(0, 0))
                .expect("thermal voltage is a native context read");

        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.ops(), &[NativeOp::LoadThermalVoltage]);
        assert!(lowered.current_pair_dependencies().is_empty());
    }

    #[test]
    fn lowers_sqrt_as_native_unary_op() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(0, 1), Instruction::Sqrt],
        };

        let lowered =
            NativeProgram::from_bytecode("sqrt", EntryKind::StampValue, &program, limits(2, 0))
                .expect("sqrt has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                },
                NativeOp::Sqrt,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn folds_safe_constant_sqrt_to_exact_literal() {
        let cases = [
            ("finite-square", 49.0_f64.to_bits()),
            ("finite-nonsquare", 2.0_f64.to_bits()),
            ("positive-zero", 0.0_f64.to_bits()),
            ("negative-zero", (-0.0_f64).to_bits()),
            ("positive-infinity", f64::INFINITY.to_bits()),
        ];

        for (case, input_bits) in cases {
            let input = f64::from_bits(input_bits);
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushConst(input), Instruction::Sqrt],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-sqrt",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("safe constant sqrt should fold to exact literal");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), input.sqrt().to_bits(), "{case}");
                }
                ops => panic!("{case}: expected folded sqrt literal, got {ops:?}"),
            }
        }
    }

    #[test]
    fn keeps_unsafe_constant_sqrt_as_runtime_op() {
        let cases = [
            ("negative-finite", (-1.0_f64).to_bits()),
            ("quiet-nan", 0x7ff8_0000_0000_0007),
            ("signaling-nan", 0x7ff0_0000_0000_0007),
        ];

        for (case, input_bits) in cases {
            let input = f64::from_bits(input_bits);
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushConst(input), Instruction::Sqrt],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-sqrt-runtime",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("unsafe constant sqrt should keep runtime op");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            match lowered.ops() {
                [NativeOp::Const(value), NativeOp::Sqrt] => {
                    assert_eq!(value.to_bits(), input_bits, "{case}");
                }
                ops => panic!("{case}: expected literal plus runtime sqrt, got {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_abs_as_native_unary_op() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::Abs],
        };

        let lowered =
            NativeProgram::from_bytecode("abs", EntryKind::Assignment, &program, limits(0, 0))
                .expect("abs has a direct native x64 lowering");

        assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, NativeOp::Abs]);
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn drops_exact_unary_composition_runtime_ops() {
        let cases = [
            (
                "double-neg",
                vec![
                    Instruction::PushTemperature,
                    Instruction::Neg,
                    Instruction::Neg,
                ],
                vec![NativeOp::LoadTemperature],
            ),
            (
                "double-abs",
                vec![
                    Instruction::PushTemperature,
                    Instruction::Abs,
                    Instruction::Abs,
                ],
                vec![NativeOp::LoadTemperature, NativeOp::Abs],
            ),
            (
                "abs-after-neg",
                vec![
                    Instruction::PushTemperature,
                    Instruction::Neg,
                    Instruction::Abs,
                ],
                vec![NativeOp::LoadTemperature, NativeOp::Abs],
            ),
        ];

        for (name, instructions, expected_ops) in cases {
            let program = BytecodeProgram { instructions };

            let lowered =
                NativeProgram::from_bytecode(name, EntryKind::Assignment, &program, limits(0, 0))
                    .expect("exact unary compositions should lower without redundant runtime ops");

            assert_eq!(lowered.max_stack_depth(), 1, "{name}");
            assert_eq!(lowered.ops(), expected_ops.as_slice(), "{name}");
        }
    }

    #[test]
    fn folds_constant_unary_ops_to_exact_literals() {
        let cases = [
            (
                "neg negative literal",
                Instruction::Neg,
                (-3.5_f64).to_bits(),
                3.5_f64.to_bits(),
            ),
            (
                "neg negative zero",
                Instruction::Neg,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "neg positive nan",
                Instruction::Neg,
                0x7ff8_0000_0000_0001,
                0xfff8_0000_0000_0001,
            ),
            (
                "abs negative literal",
                Instruction::Abs,
                (-3.5_f64).to_bits(),
                3.5_f64.to_bits(),
            ),
            (
                "abs negative zero",
                Instruction::Abs,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "abs negative nan",
                Instruction::Abs,
                0xfff8_0000_0000_0001,
                0x7ff8_0000_0000_0001,
            ),
        ];

        for (case, instruction, input_bits, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(f64::from_bits(input_bits)),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-unary",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant unary op folds to exact literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should not allocate a scratch stack slot"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_ordered_comparisons_as_native_binary_ops() {
        let cases = [
            (Instruction::Gt, CompareOp::Gt),
            (Instruction::Lt, CompareOp::Lt),
            (Instruction::Ge, CompareOp::Ge),
            (Instruction::Le, CompareOp::Le),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("cmp", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("ordered comparison has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Compare(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_constant_rhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Gt),
            (Instruction::Lt, CompareOp::Lt),
            (Instruction::Ge, CompareOp::Ge),
            (Instruction::Le, CompareOp::Le),
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(300.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "cmp-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS comparison has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered
                    .ops()
                    .iter()
                    .any(|op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())),
                "{expected:?} should consume the RHS literal in the compare op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn lowers_constant_lhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Lt),
            (Instruction::Lt, CompareOp::Gt),
            (Instruction::Ge, CompareOp::Le),
            (Instruction::Le, CompareOp::Ge),
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(300.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "cmp-literal-lhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS comparison has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())
                ),
                "{expected:?} should consume the LHS literal in the compare op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn folds_constant_comparisons_to_exact_literals() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 4.0, 1.0),
            ("gt-unordered", Instruction::Gt, f64::NAN, 4.0, 0.0),
            ("ge-equal", Instruction::Ge, 4.0, 4.0, 1.0),
            ("lt-true", Instruction::Lt, 3.0, 4.0, 1.0),
            ("le-equal", Instruction::Le, 4.0, 4.0, 1.0),
            ("le-unordered", Instruction::Le, 4.0, f64::NAN, 0.0),
            ("eq-within-epsilon", Instruction::Eq, 0.0, 0.5e-15, 1.0),
            ("eq-at-epsilon", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-unordered", Instruction::Eq, f64::NAN, 0.0, 0.0),
            ("ne-within-epsilon", Instruction::Ne, 0.0, 0.5e-15, 0.0),
            ("ne-at-epsilon", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-unordered", Instruction::Ne, 0.0, f64::NAN, 0.0),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-compare",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant comparison folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact boolean literal"
            );
        }
    }

    #[test]
    fn lowers_equality_comparisons_as_native_binary_ops() {
        let cases = [
            (Instruction::Eq, CompareOp::Eq),
            (Instruction::Ne, CompareOp::Ne),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("eq", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("equality comparison has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Compare(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn lowers_logical_ops_as_native_ops() {
        let cases = [
            (Instruction::And, LogicalOp::And),
            (Instruction::Or, LogicalOp::Or),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("logical binary op has a direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Logical(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }

        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::Not],
        };

        let lowered =
            NativeProgram::from_bytecode("not", EntryKind::Assignment, &program, limits(0, 0))
                .expect("logical not has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::Logical(LogicalOp::Not)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_constant_rhs_logical_ops_without_extra_stack_slot() {
        let cases = [
            (Instruction::And, 2.0e-15, LogicalOp::And, true),
            (Instruction::And, 1.0e-15, LogicalOp::And, false),
            (Instruction::And, f64::NAN, LogicalOp::And, false),
            (Instruction::Or, -2.0e-15, LogicalOp::Or, true),
            (Instruction::Or, 0.5e-15, LogicalOp::Or, false),
            (Instruction::Or, f64::NAN, LogicalOp::Or, false),
        ];

        for (instruction, rhs, expected_op, expected_truthy) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS logical op has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == rhs.to_bits())
                ),
                "{expected_op:?} should consume the RHS literal in the logical op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LogicalConst(expected_op, expected_truthy)
                ]
            );
        }
    }

    #[test]
    fn lowers_constant_lhs_logical_ops_without_extra_stack_slot() {
        let cases = [
            (Instruction::And, 2.0e-15, LogicalOp::And, true),
            (Instruction::And, 1.0e-15, LogicalOp::And, false),
            (Instruction::And, f64::NAN, LogicalOp::And, false),
            (Instruction::Or, -2.0e-15, LogicalOp::Or, true),
            (Instruction::Or, 0.5e-15, LogicalOp::Or, false),
            (Instruction::Or, f64::NAN, LogicalOp::Or, false),
        ];

        for (instruction, lhs, expected_op, expected_truthy) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "logic-literal-lhs",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant LHS logical op has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for LHS constants"
            );
            assert!(
                !lowered.ops().iter().any(
                    |op| matches!(op, NativeOp::Const(value) if value.to_bits() == lhs.to_bits())
                ),
                "{expected_op:?} should consume the LHS literal in the logical op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LogicalConst(expected_op, expected_truthy)
                ]
            );
        }
    }

    #[test]
    fn folds_constant_logical_ops_to_exact_literals() {
        let cases = [
            ("and-both-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                0.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                0.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-false", Instruction::Or, 1.0e-15, 0.5e-15, 0.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 0.0),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-logic",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant logical op folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact boolean literal"
            );
        }
    }

    #[test]
    fn folds_constant_logical_not_to_exact_literal() {
        let cases = [
            ("positive-true", 2.0e-15, 0.0),
            ("negative-true", -2.0e-15, 0.0),
            ("positive-at-epsilon", 1.0e-15, 0.0),
            ("negative-at-epsilon", -1.0e-15, 0.0),
            ("zero", 0.0, 1.0),
            ("unordered", f64::NAN, 0.0),
        ];

        for (case, value, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushConst(value), Instruction::Not],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-not",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant logical not folds to a literal");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact boolean literal"
            );
        }
    }

    #[test]
    fn lowers_ifelse_as_native_stack_select_op() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(7.0),
                Instruction::PushConst(3.0),
                Instruction::IfElse,
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("ifelse", EntryKind::Assignment, &program, limits(0, 0))
                .expect("ifelse has a direct native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(7.0),
                NativeOp::Const(3.0),
                NativeOp::IfElse,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 3);
    }

    #[test]
    fn folds_constant_ifelse_to_selected_literal() {
        let then_nan_bits = 0x7ff8_0000_0000_0001_u64;
        let else_neg_zero_bits = (-0.0_f64).to_bits();
        let condition_nan_bits = 0x7ff8_0000_0000_0002_u64;
        let cases = [
            (
                "true",
                2.0e-15_f64.to_bits(),
                7.0_f64.to_bits(),
                3.0_f64.to_bits(),
                7.0_f64.to_bits(),
            ),
            (
                "within-epsilon",
                0.5e-15_f64.to_bits(),
                7.0_f64.to_bits(),
                3.0_f64.to_bits(),
                3.0_f64.to_bits(),
            ),
            (
                "at-epsilon",
                1.0e-15_f64.to_bits(),
                7.0_f64.to_bits(),
                3.0_f64.to_bits(),
                3.0_f64.to_bits(),
            ),
            (
                "unordered-condition",
                condition_nan_bits,
                7.0_f64.to_bits(),
                3.0_f64.to_bits(),
                3.0_f64.to_bits(),
            ),
            (
                "selected-then-bits",
                2.0e-15_f64.to_bits(),
                then_nan_bits,
                3.0_f64.to_bits(),
                then_nan_bits,
            ),
            (
                "selected-else-bits",
                0.0_f64.to_bits(),
                7.0_f64.to_bits(),
                else_neg_zero_bits,
                else_neg_zero_bits,
            ),
        ];

        for (case, condition_bits, then_bits, else_bits, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(f64::from_bits(condition_bits)),
                    Instruction::PushConst(f64::from_bits(then_bits)),
                    Instruction::PushConst(f64::from_bits(else_bits)),
                    Instruction::IfElse,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "ifelse",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant ifelse should fold to selected literal");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            match lowered.ops() {
                [NativeOp::Const(value)] => assert_eq!(value.to_bits(), expected_bits, "{case}"),
                ops => panic!("{case}: expected folded ifelse literal, got {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_min_max_as_native_binary_ops() {
        let cases = [
            (Instruction::Min, ExtremumOp::Min),
            (Instruction::Max, ExtremumOp::Max),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "minmax",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("min/max have direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::LoadVariable(0),
                    NativeOp::Extremum(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn folds_constant_min_max_to_exact_literals() {
        let left_nan_bits = 0x7ff8_0000_0000_0001_u64;
        let right_nan_bits = 0x7ff8_0000_0000_0002_u64;
        let cases = [
            (
                "min-left-smaller",
                Instruction::Min,
                ExtremumOp::Min,
                (-2.0_f64).to_bits(),
                5.0_f64.to_bits(),
            ),
            (
                "min-right-smaller",
                Instruction::Min,
                ExtremumOp::Min,
                5.0_f64.to_bits(),
                (-2.0_f64).to_bits(),
            ),
            (
                "min-left-nan",
                Instruction::Min,
                ExtremumOp::Min,
                left_nan_bits,
                5.0_f64.to_bits(),
            ),
            (
                "min-right-nan",
                Instruction::Min,
                ExtremumOp::Min,
                5.0_f64.to_bits(),
                right_nan_bits,
            ),
            (
                "min-both-nan",
                Instruction::Min,
                ExtremumOp::Min,
                left_nan_bits,
                right_nan_bits,
            ),
            (
                "min-left-neg-zero",
                Instruction::Min,
                ExtremumOp::Min,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "min-right-neg-zero",
                Instruction::Min,
                ExtremumOp::Min,
                0.0_f64.to_bits(),
                (-0.0_f64).to_bits(),
            ),
            (
                "max-left-larger",
                Instruction::Max,
                ExtremumOp::Max,
                5.0_f64.to_bits(),
                (-2.0_f64).to_bits(),
            ),
            (
                "max-right-larger",
                Instruction::Max,
                ExtremumOp::Max,
                (-2.0_f64).to_bits(),
                5.0_f64.to_bits(),
            ),
            (
                "max-left-nan",
                Instruction::Max,
                ExtremumOp::Max,
                left_nan_bits,
                5.0_f64.to_bits(),
            ),
            (
                "max-right-nan",
                Instruction::Max,
                ExtremumOp::Max,
                5.0_f64.to_bits(),
                right_nan_bits,
            ),
            (
                "max-both-nan",
                Instruction::Max,
                ExtremumOp::Max,
                left_nan_bits,
                right_nan_bits,
            ),
            (
                "max-left-pos-zero",
                Instruction::Max,
                ExtremumOp::Max,
                0.0_f64.to_bits(),
                (-0.0_f64).to_bits(),
            ),
            (
                "max-right-pos-zero",
                Instruction::Max,
                ExtremumOp::Max,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
        ];

        for (case, instruction, op, left_bits, right_bits) in cases {
            let left = f64::from_bits(left_bits);
            let right = f64::from_bits(right_bits);
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "minmax-constant",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant min/max should fold to a literal");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            match lowered.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(
                        value.to_bits(),
                        constant_extremum(op, left, right).to_bits(),
                        "{case}"
                    );
                }
                ops => panic!("{case}: expected folded min/max literal, got {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_constant_rhs_min_max_without_extra_stack_slot() {
        let cases = [
            (Instruction::Min, ExtremumOp::Min),
            (Instruction::Max, ExtremumOp::Max),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(300.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "minmax-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant RHS min/max has a direct native lowering");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{instruction_name} should not allocate a second XMM stack slot for RHS constants"
            );
            assert!(
                !lowered
                    .ops()
                    .iter()
                    .any(|op| matches!(op, NativeOp::Const(value) if value.to_bits() == 300.0_f64.to_bits())),
                "{expected:?} should consume the RHS literal in the min/max op"
            );
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::ExtremumConst(expected, 300.0)
                ]
            );
        }
    }

    #[test]
    fn lowers_transcendental_functions_as_native_unary_math_ops() {
        let cases = [
            (Instruction::Exp, UnaryMathOp::Exp),
            (Instruction::Log, UnaryMathOp::Log),
            (Instruction::Log10, UnaryMathOp::Log10),
            (Instruction::Sin, UnaryMathOp::Sin),
            (Instruction::Cos, UnaryMathOp::Cos),
            (Instruction::Tan, UnaryMathOp::Tan),
            (Instruction::Sinh, UnaryMathOp::Sinh),
            (Instruction::Cosh, UnaryMathOp::Cosh),
            (Instruction::Tanh, UnaryMathOp::Tanh),
            (Instruction::Limexp, UnaryMathOp::Limexp),
            (Instruction::Asin, UnaryMathOp::Asin),
            (Instruction::Acos, UnaryMathOp::Acos),
            (Instruction::Atan, UnaryMathOp::Atan),
            (Instruction::Floor, UnaryMathOp::Floor),
            (Instruction::Ceil, UnaryMathOp::Ceil),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };

            let lowered =
                NativeProgram::from_bytecode("math", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("transcendental functions have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[NativeOp::LoadTemperature, NativeOp::UnaryMath(expected)]
            );
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn folds_constant_unary_math_to_exact_literals() {
        let cases = [
            ("exp", Instruction::Exp, 0.5, 0.5_f64.exp()),
            ("log", Instruction::Log, 2.5, 2.5_f64.ln()),
            ("log10", Instruction::Log10, 100.0, 100.0_f64.log10()),
            ("sin", Instruction::Sin, 0.5, 0.5_f64.sin()),
            ("cos", Instruction::Cos, 0.5, 0.5_f64.cos()),
            ("tan", Instruction::Tan, 0.25, 0.25_f64.tan()),
            ("sinh", Instruction::Sinh, 0.25, 0.25_f64.sinh()),
            ("cosh", Instruction::Cosh, 0.25, 0.25_f64.cosh()),
            ("tanh", Instruction::Tanh, 0.25, 0.25_f64.tanh()),
            (
                "limexp-linear",
                Instruction::Limexp,
                45.0,
                constant_limexp(45.0),
            ),
            (
                "limexp-negative",
                Instruction::Limexp,
                -50.0,
                constant_limexp(-50.0),
            ),
            ("asin", Instruction::Asin, 0.25, 0.25_f64.asin()),
            ("asin-domain-nan", Instruction::Asin, 2.0, 2.0_f64.asin()),
            ("acos", Instruction::Acos, 0.25, 0.25_f64.acos()),
            ("atan", Instruction::Atan, 0.25, 0.25_f64.atan()),
            ("floor", Instruction::Floor, 3.75, 3.75_f64.floor()),
            (
                "floor-negative",
                Instruction::Floor,
                -3.25,
                (-3.25_f64).floor(),
            ),
            ("ceil", Instruction::Ceil, 3.25, 3.25_f64.ceil()),
            (
                "ceil-negative",
                Instruction::Ceil,
                -3.75,
                (-3.75_f64).ceil(),
            ),
        ];

        for (case, instruction, input, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushConst(input), instruction],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-unary-math",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant unary math folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{case} should fold to the exact helper-equivalent bit pattern"
                ),
                other => panic!("{case} should fold to one literal op, got {other:?}"),
            }
        }
    }

    #[test]
    fn lowers_binary_math_functions_as_native_binary_math_ops() {
        let cases = [
            (Instruction::Pow, 2.25, BinaryMathOp::Pow),
            (Instruction::FnPow, 2.25, BinaryMathOp::Pow),
            (Instruction::Atan2, 2.0, BinaryMathOp::Atan2),
            (Instruction::Mod, 2.0, BinaryMathOp::Mod),
        ];

        for (instruction, exponent, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(exponent),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("math", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("binary math functions have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(exponent),
                    NativeOp::BinaryMath(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn folds_constant_binary_math_to_exact_literals() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                2.0_f64.powf(3.0),
            ),
            ("fn-pow", Instruction::FnPow, 4.0, 0.5, 4.0_f64.powf(0.5)),
            (
                "pow-domain-nan",
                Instruction::Pow,
                -4.0,
                0.5,
                (-4.0_f64).powf(0.5),
            ),
            ("atan2", Instruction::Atan2, 0.5, 0.25, 0.5_f64.atan2(0.25)),
            (
                "atan2-signed-zero",
                Instruction::Atan2,
                -0.0,
                -0.0,
                (-0.0_f64).atan2(-0.0),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, 5.25_f64 % 2.0),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                -5.25_f64 % 2.0,
            ),
            (
                "mod-zero-divisor",
                Instruction::Mod,
                5.25,
                0.0,
                5.25_f64 % 0.0,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-binary-math",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant binary math folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            match lowered.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{case} should fold to the exact helper-equivalent bit pattern"
                ),
                other => panic!("{case} should fold to one literal op, got {other:?}"),
            }
        }
    }

    #[test]
    fn lowers_constant_square_power_as_native_square_op() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "square",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant-square power has direct native x64 lowering");

            assert_eq!(
                lowered.ops(),
                &[NativeOp::LoadTemperature, NativeOp::Square]
            );
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_constant_identity_power_as_native_base_expression() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "identity-power",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant-one power keeps the base expression without helper call");

            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature]);
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_constant_reciprocal_power_as_native_div_from_const() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(-1.0),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "reciprocal-power",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant-minus-one power becomes native reciprocal");

            assert_eq!(
                lowered.ops(),
                &[NativeOp::LoadTemperature, NativeOp::DivFromConst(1.0)]
            );
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_integer_binary_functions_as_native_integer_ops() {
        let cases = [
            (Instruction::Shl, IntegerBinaryOp::Shl),
            (Instruction::Shr, IntegerBinaryOp::Shr),
            (Instruction::BitAnd, IntegerBinaryOp::BitAnd),
            (Instruction::BitOr, IntegerBinaryOp::BitOr),
            (Instruction::BitXor, IntegerBinaryOp::BitXor),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
            };

            let lowered =
                NativeProgram::from_bytecode("bits", EntryKind::Assignment, &program, limits(0, 0))
                    .expect("integer binary ops have native x64 helper-call lowering");

            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(2.0),
                    NativeOp::IntegerBinary(expected),
                ]
            );
            assert_eq!(lowered.max_stack_depth(), 2);
        }
    }

    #[test]
    fn folds_safe_constant_integer_binary_ops_to_exact_literals() {
        let cases = [
            ("shl", Instruction::Shl, 3.0, 2.0, ((3_i64) << 2) as f64),
            ("shr", Instruction::Shr, -16.0, 2.0, ((-16_i64) >> 2) as f64),
            (
                "bitand",
                Instruction::BitAnd,
                13.0,
                6.0,
                (13_i64 & 6) as f64,
            ),
            ("bitor", Instruction::BitOr, 8.0, 3.0, (8_i64 | 3) as f64),
            (
                "bitxor",
                Instruction::BitXor,
                15.0,
                6.0,
                (15_i64 ^ 6) as f64,
            ),
            (
                "truncates-operands",
                Instruction::BitAnd,
                13.75,
                6.25,
                (13_i64 & 6) as f64,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-integer-binary",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("safe constant integer binary op folds to a literal");

            assert_eq!(
                lowered.max_stack_depth(),
                1,
                "{case} should only need the folded literal"
            );
            assert_eq!(
                lowered.ops(),
                &[NativeOp::Const(expected)],
                "{case} should fold to an exact helper-equivalent literal"
            );
        }
    }

    #[test]
    fn leaves_unsafe_constant_shift_counts_as_runtime_integer_ops() {
        let cases = [
            (
                "left-shift-negative",
                Instruction::Shl,
                3.0,
                -1.0,
                IntegerBinaryOp::Shl,
            ),
            (
                "left-shift-too-wide",
                Instruction::Shl,
                3.0,
                64.0,
                IntegerBinaryOp::Shl,
            ),
            (
                "right-shift-too-wide",
                Instruction::Shr,
                3.0,
                64.0,
                IntegerBinaryOp::Shr,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "literal-unsafe-shift",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("unsafe constant shift remains a runtime integer helper op");

            assert_eq!(lowered.max_stack_depth(), 2, "{case}");
            assert_eq!(
                lowered.ops(),
                &[
                    NativeOp::Const(left),
                    NativeOp::Const(right),
                    NativeOp::IntegerBinary(expected),
                ],
                "{case} should preserve current helper-call behavior"
            );
        }
    }

    #[test]
    fn lowers_table_model_ops_as_native_table_ops() {
        let cases = [
            (Instruction::TableLookup(2), NativeOp::TableLookup(2)),
            (
                Instruction::TableDerivative(3),
                NativeOp::TableDerivative(3),
            ),
        ];

        for (instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };

            let lowered = NativeProgram::from_bytecode(
                "table",
                EntryKind::Assignment,
                &program,
                limits(0, 0).with_lookup_table_count(4),
            )
            .expect("table model ops have native x64 helper-call lowering");

            assert_eq!(lowered.ops(), &[NativeOp::LoadTemperature, expected]);
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowers_limit_state_as_native_stateful_op() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.5),
                Instruction::LimitState(2),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("limit", EntryKind::Assignment, &program, limits(0, 0))
                .expect("limit state has native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.5),
                NativeOp::LimitState(2),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_limit_state_without_value_and_step() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LimitState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-limit",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("limit state requires value and step operands");
        let msg = error.to_string();
        assert!(
            msg.contains("LimitState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_above_state_as_native_ordered_compare() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(300.0),
                Instruction::AboveState(7),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("above", EntryKind::Assignment, &program, limits(0, 0))
                .expect("above state has native x64 lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::CompareConst(CompareOp::Gt, 300.0),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn folds_constant_above_state_to_ordered_literal() {
        let left_nan_bits = 0x7ff8_0000_0000_0001_u64;
        let right_nan_bits = 0x7ff8_0000_0000_0002_u64;
        let cases = [
            (
                "true",
                2.0_f64.to_bits(),
                1.0_f64.to_bits(),
                1.0_f64.to_bits(),
            ),
            (
                "false",
                1.0_f64.to_bits(),
                2.0_f64.to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "left-nan",
                left_nan_bits,
                1.0_f64.to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "right-nan",
                1.0_f64.to_bits(),
                right_nan_bits,
                0.0_f64.to_bits(),
            ),
        ];

        for (case, left_bits, right_bits, expected_bits) in cases {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(f64::from_bits(left_bits)),
                    Instruction::PushConst(f64::from_bits(right_bits)),
                    Instruction::AboveState(7),
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                "above-literal",
                EntryKind::Assignment,
                &program,
                limits(0, 0),
            )
            .expect("constant above state should fold to ordered literal");

            assert_eq!(lowered.max_stack_depth(), 1, "{case}");
            match lowered.ops() {
                [NativeOp::Const(value)] => assert_eq!(value.to_bits(), expected_bits, "{case}"),
                ops => panic!("{case}: expected folded above literal, got {ops:?}"),
            }
        }
    }

    #[test]
    fn lowers_timer_state_as_native_context_event() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.5),
                Instruction::TimerState(3),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("timer", EntryKind::Assignment, &program, limits(0, 0))
                .expect("timer state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::Const(0.5),
                NativeOp::TimerState(3),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_timer_state_without_start_and_period() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0), Instruction::TimerState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-timer",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("timer state requires start and period operands");
        let msg = error.to_string();
        assert!(
            msg.contains("TimerState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_timer_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition TimerState"),
            (EntryKind::ParameterDefault, "ParameterDefault TimerState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.5),
                    Instruction::TimerState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-timer-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("timer state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_transition_state_as_native_context_filter() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.2),
                Instruction::PushConst(0.4),
                Instruction::PushConst(0.4),
                Instruction::TransitionState(3),
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "transition",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect("transition state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.2),
                NativeOp::Const(0.4),
                NativeOp::Const(0.4),
                NativeOp::TransitionState(3),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 4);
    }

    #[test]
    fn lowering_rejects_transition_state_without_all_operands() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.2),
                Instruction::PushConst(0.4),
                Instruction::TransitionState(0),
            ],
        };

        let error = NativeProgram::from_bytecode(
            "bad-transition",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("transition state requires input, delay, rise, and fall operands");
        let msg = error.to_string();
        assert!(
            msg.contains("TransitionState requires stack depth 4"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_transition_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (
                EntryKind::StaticCondition,
                "StaticCondition TransitionState",
            ),
            (
                EntryKind::ParameterDefault,
                "ParameterDefault TransitionState",
            ),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.2),
                    Instruction::PushConst(0.4),
                    Instruction::PushConst(0.4),
                    Instruction::TransitionState(0),
                ],
            };

            let error = NativeProgram::from_bytecode(
                "bad-transition-entry",
                entry_kind,
                &program,
                limits(0, 0),
            )
            .expect_err("transition state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_slew_state_as_native_context_filter() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::SlewState(4),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("slew", EntryKind::Assignment, &program, limits(0, 0))
                .expect("slew state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::SlewState(4),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 3);
    }

    #[test]
    fn lowering_rejects_slew_state_without_all_operands() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(2.0),
                Instruction::SlewState(0),
            ],
        };

        let error =
            NativeProgram::from_bytecode("bad-slew", EntryKind::Assignment, &program, limits(0, 0))
                .expect_err("slew state requires input, positive slew, and negative slew operands");
        let msg = error.to_string();
        assert!(
            msg.contains("SlewState requires stack depth 3"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_slew_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition SlewState"),
            (EntryKind::ParameterDefault, "ParameterDefault SlewState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(3.0),
                    Instruction::SlewState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-slew-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("slew state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_absdelay_state_as_native_context_buffer() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(0.5),
                Instruction::AbsDelayState(5),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("absdelay", EntryKind::Assignment, &program, limits(0, 0))
                .expect("absdelay state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(0.5),
                NativeOp::AbsDelayState(5),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_absdelay_state_without_value_and_delay() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::AbsDelayState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-absdelay",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("absdelay state requires input and delay operands");
        let msg = error.to_string();
        assert!(
            msg.contains("AbsDelayState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_absdelay_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition AbsDelayState"),
            (
                EntryKind::ParameterDefault,
                "ParameterDefault AbsDelayState",
            ),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.5),
                    Instruction::AbsDelayState(0),
                ],
            };

            let error = NativeProgram::from_bytecode(
                "bad-absdelay-entry",
                entry_kind,
                &program,
                limits(0, 0),
            )
            .expect_err("absdelay state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_cross_state_as_native_context_detector() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushTemperature,
                Instruction::PushConst(1.0),
                Instruction::CrossState(4),
            ],
        };

        let lowered =
            NativeProgram::from_bytecode("cross", EntryKind::Assignment, &program, limits(0, 0))
                .expect("cross state has native x64 helper-call lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadTemperature,
                NativeOp::Const(1.0),
                NativeOp::CrossState(4),
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_cross_state_without_value_and_direction() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::CrossState(0)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-cross",
            EntryKind::Assignment,
            &program,
            limits(0, 0),
        )
        .expect_err("cross state requires value and direction operands");
        let msg = error.to_string();
        assert!(
            msg.contains("CrossState requires stack depth 2"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_cross_state_in_restricted_entry_kinds() {
        for (entry_kind, expected) in [
            (EntryKind::StaticCondition, "StaticCondition CrossState"),
            (EntryKind::ParameterDefault, "ParameterDefault CrossState"),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(1.0),
                    Instruction::CrossState(0),
                ],
            };

            let error =
                NativeProgram::from_bytecode("bad-cross-entry", entry_kind, &program, limits(0, 0))
                    .expect_err("cross state must stay out of restricted entries");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{entry_kind:?}: got {msg}");
            assert!(
                msg.contains("no interpreter fallback"),
                "{entry_kind:?}: got {msg}"
            );
        }
    }

    #[test]
    fn lowers_laplace_state_when_filter_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LaplaceState(0)],
        };

        let lowered = NativeProgram::from_bytecode(
            "laplace",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_laplace_filter_count(1),
        )
        .expect("Laplace filter state has native helper lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::LaplaceState(0)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowering_rejects_laplace_state_outside_known_filters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::LaplaceState(1)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-laplace",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_laplace_filter_count(1),
        )
        .expect_err("Laplace filter id outside compiled table must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("LaplaceState filter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_zi_state_when_filter_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::ZiState(0)],
        };

        let lowered = NativeProgram::from_bytecode(
            "zi",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_zi_filter_count(1),
        )
        .expect("zi filter state has native helper lowering");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadTemperature, NativeOp::ZiState(0)]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowering_rejects_zi_state_outside_known_filters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushTemperature, Instruction::ZiState(1)],
        };

        let error = NativeProgram::from_bytecode(
            "bad-zi",
            EntryKind::StampValue,
            &program,
            limits(0, 0).with_zi_filter_count(1),
        )
        .expect_err("zi filter id outside compiled table must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("ZiState filter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_large_signal_noise_sources_as_native_zero_ops() {
        let cases = [
            (
                "white",
                vec![Instruction::PushTemperature, Instruction::WhiteNoise],
                &[NativeOp::LoadTemperature, NativeOp::WhiteNoise][..],
                1,
            ),
            (
                "flicker",
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                ],
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::Const(1.0),
                    NativeOp::FlickerNoise,
                ][..],
                2,
            ),
        ];

        for (name, instructions, expected, expected_depth) in cases {
            let program = BytecodeProgram { instructions };

            let lowered = NativeProgram::from_bytecode(
                format!("noise-{name}"),
                EntryKind::StampValue,
                &program,
                limits(0, 0),
            )
            .expect("large-signal noise source has native x64 zero lowering");

            assert_eq!(lowered.ops(), expected, "{name}");
            assert_eq!(lowered.max_stack_depth(), expected_depth, "{name}");
        }
    }

    #[test]
    fn lowering_rejects_noise_sources_without_required_operands() {
        let cases = [
            (
                "WhiteNoise",
                vec![Instruction::WhiteNoise],
                "WhiteNoise requires stack depth 1",
            ),
            (
                "FlickerNoise",
                vec![Instruction::PushTemperature, Instruction::FlickerNoise],
                "FlickerNoise requires stack depth 2",
            ),
        ];

        for (name, instructions, expected) in cases {
            let program = BytecodeProgram { instructions };

            let error = NativeProgram::from_bytecode(
                "bad-noise",
                EntryKind::StampValue,
                &program,
                limits(0, 0),
            )
            .expect_err("malformed noise bytecode must fail closed");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: got {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: got {msg}");
        }
    }

    #[test]
    fn lowering_rejects_table_model_ops_outside_known_tables() {
        let cases = [
            (
                "TableLookup",
                Instruction::TableLookup(1),
                "TableLookup table 1",
            ),
            (
                "TableDerivative",
                Instruction::TableDerivative(1),
                "TableDerivative table 1",
            ),
        ];

        for (name, instruction, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushTemperature, instruction],
            };
            let limits = NativeLoweringLimits::new(0, 0, 0, 0, 0).with_lookup_table_count(1);

            let error =
                NativeProgram::from_bytecode("bad", EntryKind::Assignment, &program, limits)
                    .expect_err("table id outside registered lookup tables must fail closed");
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: got {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: got {msg}");
        }
    }

    #[test]
    fn lowers_terminal_to_ground_voltage_without_usize_sentinel() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(0, usize::MAX)],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(1, 0))
                .expect("lower terminal-to-ground voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(0),
                neg: VoltageNode::Ground,
            }]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_ground_to_terminal_voltage_without_usize_sentinel() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(usize::MAX, 0)],
        };

        let lowered =
            NativeProgram::from_bytecode("res", EntryKind::StampValue, &program, limits(1, 0))
                .expect("lower ground-to-terminal voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Ground,
                neg: VoltageNode::Terminal(0),
            }]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn lowers_unified_internal_voltage_index_when_internal_count_is_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(1, 2)],
        };

        let lowered =
            NativeProgram::from_bytecode("int", EntryKind::StampValue, &program, limits(2, 1))
                .expect("lower terminal-to-internal voltage");

        assert_eq!(
            lowered.ops(),
            &[NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(1),
                neg: VoltageNode::Internal(0),
            }]
        );
    }

    #[test]
    fn lowering_rejects_unified_voltage_index_outside_known_nodes() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushVoltage(3, usize::MAX)],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(2, 1))
                .expect_err("node outside terminals plus internals must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushVoltage unified node 3"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_direct_internal_voltage_outside_known_nodes() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushInternalVoltage(1)],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(0, 1))
                .expect_err("direct internal voltage outside known internals must fail closed");
        let msg = error.to_string();
        assert!(
            msg.contains("PushInternalVoltage internal node 1"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowers_terminal_pair_current_probe_when_terminals_are_known() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };
        let available = [1];

        let lowered = NativeProgram::from_bytecode(
            "probe",
            EntryKind::StampValue,
            &program,
            limits(2, 0).with_available_current_pairs(&available),
        )
        .expect("terminal-pair current probes are native-loadable");

        assert_eq!(lowered.ops(), &[NativeOp::LoadCurrent(1)]);
        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.current_pair_dependencies(), &[1]);
    }

    #[test]
    fn lowers_terminal_to_ground_current_probe_when_pair_is_available() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, usize::MAX)],
        };
        let available = [current_pair_index("probe".into(), 0, usize::MAX, 2).unwrap()];

        let lowered = NativeProgram::from_bytecode(
            "probe",
            EntryKind::StampValue,
            &program,
            limits(2, 0).with_available_current_pairs(&available),
        )
        .expect("terminal-to-ground current probes are native-loadable");

        assert_eq!(lowered.ops(), &[NativeOp::LoadCurrent(2)]);
        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.current_pair_dependencies(), &[2]);
    }

    #[test]
    fn lowers_ground_to_terminal_current_probe_when_pair_is_available() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(usize::MAX, 0)],
        };
        let available = [current_pair_index("probe".into(), usize::MAX, 0, 2).unwrap()];

        let lowered = NativeProgram::from_bytecode(
            "probe",
            EntryKind::StampValue,
            &program,
            limits(2, 0).with_available_current_pairs(&available),
        )
        .expect("ground-to-terminal current probes are native-loadable");

        assert_eq!(lowered.ops(), &[NativeOp::LoadCurrent(6)]);
        assert_eq!(lowered.max_stack_depth(), 1);
        assert_eq!(lowered.current_pair_dependencies(), &[6]);
    }

    #[test]
    fn lowering_rejects_current_probe_before_pair_is_available() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };

        let error =
            NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program, limits(2, 0))
                .expect_err("current probes must not read unavailable terminal-pair slots");
        let msg = error.to_string();
        assert!(
            msg.contains("PushCurrent terminal pair 0,1 unavailable"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_current_probe_outside_terminal_pairs() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 2)],
        };

        let error =
            NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program, limits(2, 1))
                .expect_err("current probes outside terminal pair matrix must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushCurrent terminal pair 0,2"), "got: {msg}");
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn lowering_rejects_param_given_outside_known_parameters() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushParamGiven(1)],
        };
        let limits = NativeLoweringLimits::new(0, 0, 1, 0, 0);

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
            .expect_err("parameter-given index outside known parameters must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushParamGiven parameter 1"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_port_connected_outside_known_terminals() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushPortConnected(2)],
        };
        let limits = NativeLoweringLimits::new(2, 0, 0, 0, 0);

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
            .expect_err("port-connected index outside known terminals must fail closed");
        let msg = error.to_string();
        assert!(msg.contains("PushPortConnected terminal 2"), "got: {msg}");
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_direct_indexed_loads_outside_known_storage() {
        let cases = [
            (
                "PushParam",
                Instruction::PushParam(1),
                NativeLoweringLimits::new(0, 0, 1, 0, 0),
                "PushParam parameter 1",
            ),
            (
                "PushVariable",
                Instruction::PushVariable(1),
                NativeLoweringLimits::new(0, 0, 0, 1, 0),
                "PushVariable variable 1",
            ),
            (
                "PushBranchCurrent",
                Instruction::PushBranchCurrent(1),
                NativeLoweringLimits::new(0, 0, 0, 0, 1),
                "PushBranchCurrent branch unknown 1",
            ),
        ];

        for (name, instruction, limits, expected) in cases {
            let program = BytecodeProgram {
                instructions: vec![instruction],
            };

            let error =
                NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits)
                    .unwrap_err();
            let msg = error.to_string();
            assert!(msg.contains(expected), "{name}: {msg}");
            assert!(msg.contains("no interpreter fallback"), "{name}: {msg}");
        }
    }

    #[test]
    fn lowers_dynamic_variable_read_when_range_is_known() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 2,
                    len: 3,
                    lower: 1,
                },
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "dyn-read",
            EntryKind::StampValue,
            &program,
            NativeLoweringLimits::new(0, 0, 1, 5, 0),
        )
        .expect("dynamic variable reads inside known storage have native lowering");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadParam(0),
                NativeOp::LoadVariableDyn {
                    base: 2,
                    len: 3,
                    lower: 1
                }
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn folds_constant_dynamic_variable_read_to_direct_load() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(2.49),
                Instruction::PushVariableDyn {
                    base: 2,
                    len: 3,
                    lower: 1,
                },
            ],
        };

        let lowered = NativeProgram::from_bytecode(
            "const-dyn-read",
            EntryKind::StampValue,
            &program,
            NativeLoweringLimits::new(0, 0, 0, 5, 0),
        )
        .expect("in-range constant dynamic variable read lowers directly");

        assert_eq!(lowered.ops(), &[NativeOp::LoadVariable(3)]);
        assert_eq!(lowered.max_stack_depth(), 1);
    }

    #[test]
    fn preserves_unsafe_constant_dynamic_variable_reads_on_helper_path() {
        for (name, raw_index, lower) in [
            ("nan", f64::NAN, 0),
            ("infinity", f64::INFINITY, 0),
            ("huge finite", 1.0e300, i64::MAX),
            ("out of range", 2.0, 0),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(raw_index),
                    Instruction::PushVariableDyn {
                        base: 0,
                        len: 1,
                        lower,
                    },
                ],
            };

            let lowered = NativeProgram::from_bytecode(
                name,
                EntryKind::StampValue,
                &program,
                NativeLoweringLimits::new(0, 0, 0, 1, 0),
            )
            .expect("known dynamic variable range lowers natively");

            match lowered.ops() {
                [
                    NativeOp::Const(actual),
                    NativeOp::LoadVariableDyn {
                        base: 0,
                        len: 1,
                        lower: actual_lower,
                    },
                ] if raw_index.is_nan() == actual.is_nan()
                    && (raw_index.is_nan() || actual.to_bits() == raw_index.to_bits())
                    && *actual_lower == lower => {}
                other => panic!("{name}: expected helper-path dynamic read, got {other:?}"),
            }
            assert_eq!(lowered.max_stack_depth(), 1);
        }
    }

    #[test]
    fn lowering_rejects_dynamic_variable_read_outside_known_storage() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 3,
                    len: 3,
                    lower: 1,
                },
            ],
        };

        let error = NativeProgram::from_bytecode(
            "bad-dyn-read",
            EntryKind::StampValue,
            &program,
            NativeLoweringLimits::new(0, 0, 1, 5, 0),
        )
        .expect_err("dynamic variable range outside storage must fail closed");
        let msg = error.to_string();
        assert!(
            msg.contains("PushVariableDyn variable range 3..6 outside storage length 5"),
            "got: {msg}"
        );
        assert!(msg.contains("no interpreter fallback"), "got: {msg}");
    }

    #[test]
    fn lowering_rejects_unbalanced_stack() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::Add],
        };

        let error =
            NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, limits(0, 0))
                .expect_err("binary op without operands must fail");
        assert!(error.to_string().contains("stack"));
    }
}
