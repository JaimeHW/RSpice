//! Canonical multi-level Verilog-A IR.
//!
//! The semantic compiler target for Verilog-A models, and the input every
//! backend other than the bytecode VM consumes. Three levels are built in
//! sequence, each preserved in the final artifact rather than discarded:
//!
//! | Level | Module | Shape |
//! | :--- | :--- | :--- |
//! | HIR | [`hir`] | Source-shaped: nested control flow, named entities |
//! | MIR | [`mir`] | Solver-shaped: flat equations, branch unknowns, state slots |
//! | OptIR | [`opt`] | Scalarized value graph with derivative lanes and a schedule |
//!
//! [`cfg`] is the level that replaces OptIR: basic blocks and SSA values, built
//! from HIR's structured body by [`cfg_lower`]. See
//! `design/VERILOGA_BACKEND_PLAN.md` for what it is replacing and why.
//!
//! [`noise`] lifts noise sources out into a separate plan along the way, since
//! they are contribution expressions in the time domain but independent
//! injected sources in `.noise`. [`artifact`] seals the result, [`opt_eval`]
//! provides the reference interpreter backends are checked against, and
//! [`ids`], [`metadata`], and [`diagnostic`] supply the typed indices,
//! provenance digests, and phase-tagged diagnostics used across all levels.

pub mod ad;
pub mod artifact;
pub mod cfg;
pub mod cfg_complex;
pub mod cfg_eval;
pub mod cfg_lower;
pub mod cfg_opt;
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;
pub mod noise;
pub mod opt;
pub mod opt_eval;
pub mod schedule;

pub use ad::{AdFunction, AdSeed, differentiate};
pub use artifact::CanonicalIrArtifact;
pub use cfg::{
    CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp,
    CfgValidationError, CfgValue, CfgValueKind, CfgValueType, CfgVariable, SsaBuilder,
};
pub use cfg_complex::{COMPLEX_STEP, ComplexStep};
pub use cfg_eval::{
    CfgEvalError, CfgEvalInputs, CfgEvalSnapshot, CfgScalar, evaluate as evaluate_cfg,
};
pub use cfg_lower::CfgModel;
pub use cfg_opt::optimize as optimize_cfg;
pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use hir::{
    CanonicalValueType, HirAnalogOperator, HirArray, HirAssignment, HirBranch, HirContribution,
    HirContributionKind, HirCrossDirection, HirExprKind, HirExprRef, HirExpression,
    HirInternalNode, HirLaplaceKind, HirLimiterArgument, HirLoop, HirModel, HirParamRange,
    HirParameter, HirPort, HirStatement, HirVariable, HirZiKind,
};
pub use ids::{
    ArrayId, BlockId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, ShapeId, SourceId,
    StateId, SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
pub use mir::{
    MirAnalysisDomain, MirBranch, MirBranchRef, MirBranchUnknown, MirEquation, MirEquationKind,
    MirModel, MirNode, MirParameterSlot, MirStateSlot,
};
pub use noise::{
    CanonicalNoiseEndpoint, CanonicalNoiseSource, CanonicalNoiseSourceKind,
    CanonicalNoiseSourcePlan, CanonicalNoiseTable,
};
pub use opt::{
    DerivativeLane, DerivativeLaneKind, InvalidationClass, OptBinaryOp, OptDerivative, OptModel,
    OptOp, OptRuntimeLoop, OptRuntimeLoopAssignment, OptRuntimeLoopVariable, OptSchedule,
    OptUnaryOp, OptValue, OptValueKind, OptValueType,
};
pub use schedule::{
    InvalidationClass as CfgInvalidationClass, Schedule as CfgSchedule, schedule as schedule_cfg,
};
pub use opt_eval::{
    OptEvalError, OptEvalInputs, OptEvalSnapshot, OptEvalValue, OptEvaluatedDerivative,
    evaluate_opt_model,
};
