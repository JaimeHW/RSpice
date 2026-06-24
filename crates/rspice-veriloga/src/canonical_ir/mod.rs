//! Canonical multi-level Verilog-A IR.
//!
//! This module is the new semantic compiler target for Verilog-A models.
//! The existing bytecode and Cranelift paths remain legacy runtime paths
//! while this IR is introduced and verified.

pub mod diagnostic;
pub mod ids;
pub mod metadata;

pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId, ModuleId,
    NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId, SymbolId,
    ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
