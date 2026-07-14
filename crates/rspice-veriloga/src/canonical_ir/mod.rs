//! Canonical multi-level Verilog-A IR.
//!
//! This module is the new semantic compiler target for Verilog-A models.
//! The bytecode VM remains the non-native runtime path while native mode is
//! owned by the RSpice native JIT contract/backend as this IR is introduced
//! and verified.

pub mod artifact;
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;
pub mod noise;
pub mod opt;
pub mod opt_eval;

pub use artifact::CanonicalIrArtifact;
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
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId, ModuleId,
    NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId, SymbolId,
    ValueId, VariableId,
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
pub use opt_eval::{
    OptEvalError, OptEvalInputs, OptEvalSnapshot, OptEvalValue, OptEvaluatedDerivative,
    evaluate_opt_model,
};
