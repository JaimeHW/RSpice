//! Canonical multi-level Verilog-A IR.
//!
//! The semantic compiler target for Verilog-A models, and the input every
//! backend other than the bytecode VM consumes:
//!
//! | Level | Module | Shape |
//! | :--- | :--- | :--- |
//! | HIR | [`hir`] | Source-shaped: nested control flow, named entities |
//! | MIR | [`mir`] | Solver-shaped: flat equations, branch unknowns, state slots |
//! [`cfg`] is the production scalar level: basic blocks and SSA values, built
//! from HIR's structured body by [`cfg_lower`].
//!
//! [`noise`] lifts noise sources out into a separate plan along the way, since
//! they are contribution expressions in the time domain but independent
//! injected sources in `.noise`. [`state`] names the runtime record every
//! dynamic operator owns, so a backend addresses state by canonical site rather
//! than by position in some lowered program. [`artifact`] seals the result,
//! while [`ids`], [`metadata`], and [`diagnostic`] supply the typed indices,
//! provenance digests, and phase-tagged diagnostics used across all levels.

pub mod ad;
pub mod artifact;
pub mod cfg;
pub mod cfg_complex;
pub mod cfg_eval;
pub mod cfg_lower;
pub mod cfg_opt;
pub mod diagnostic;
pub mod digital;
pub mod digital_eval;
pub mod digital_lower;
pub mod digital_value;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;
pub mod noise;
mod parameter_array;
pub mod schedule;
pub mod state;

pub use ad::{AdFunction, AdSeed, differentiate};
pub use artifact::CanonicalIrArtifact;
pub use cfg::{
    CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction, CfgIntegerBitwiseOp, CfgLaplaceTransfer,
    CfgStateSite, CfgTerminator, CfgUnaryOp, CfgValidationError, CfgValue, CfgValueKind,
    CfgValueType, CfgVariable, CfgZiPolynomial, DigitalWait, SsaBuilder,
};
pub use cfg_complex::{COMPLEX_STEP, ComplexStep};
pub use cfg_eval::{
    CfgEvalError, CfgEvalInputs, CfgEvalSnapshot, CfgScalar, evaluate as evaluate_cfg,
};
pub use cfg_lower::CfgModel;
pub use cfg_opt::{optimize as optimize_cfg, prune_to_outputs as prune_cfg_to_outputs};
pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalDriver, DigitalDriverId, DigitalEdge,
    DigitalProcessKind, DigitalSchedulingRegion, DigitalSensitivityOrigin, DigitalSensitivityTerm,
    DigitalSignal, DigitalStaticSensitivity, DigitalWriteSelect, DigitalWriteTarget,
};
pub use digital_eval::{
    DigitalDeferredUpdate, DigitalDrive, DigitalEnvironment, DigitalEvalError,
    DigitalProcessOutcome, DigitalResumeState, DigitalScalar, DigitalSuspension,
    DigitalWaitRequest, apply_deferred as apply_deferred_digital_update, classify_edge,
    resume as resume_digital_process, start as start_digital_process,
};
pub use digital_value::FourStateValue;
pub use hir::{
    CanonicalValueType, HirAnalogOperator, HirArray, HirAssignment, HirBranch, HirContribution,
    HirContributionKind, HirExprKind, HirExprRef, HirExpression, HirInternalNode,
    HirLimiterArgument, HirLoop, HirModel, HirParamRange, HirParameter, HirParameterDimension,
    HirPort, HirStatement, HirVariable,
};
pub use ids::{
    ArrayId, BlockId, BranchId, BranchUnknownId, ContributionId, DigitalLocalId, DigitalProcessId,
    DigitalSignalId, DisciplineId, EquationId, ExprId, ModuleId, NodeId, NoiseSourceId, ParamId,
    PortId, RegionId, ScheduleId, ShapeId, SourceId, StateId, SymbolId, ValueId, VariableId,
};
pub use metadata::{CANONICAL_IR_SCHEMA_VERSION, CanonicalMetadata, StableDigest, source_identity};
pub use mir::{
    MirAnalysisDomain, MirBranch, MirBranchRef, MirBranchUnknown, MirEquation, MirEquationKind,
    MirModel, MirNode, MirParameterSlot, MirStateSlot,
};
pub use noise::{
    CanonicalNoiseEndpoint, CanonicalNoiseSource, CanonicalNoiseSourceKind,
    CanonicalNoiseSourcePlan, CanonicalNoiseTable,
};
pub use schedule::{
    InvalidationClass as CfgInvalidationClass, ParameterDependency, Schedule as CfgSchedule,
    StaticDependencies, StructuralGuard, schedule as schedule_cfg,
    schedule_with_parameter_scopes as schedule_cfg_with_parameter_scopes, structural_guards,
};
pub use state::{
    CanonicalStateFamily, CanonicalStateLayout, CanonicalStateOperator, CanonicalStateSite,
    CfgStateAllocation, CfgStateAllocationError, EmissionCensus,
};
