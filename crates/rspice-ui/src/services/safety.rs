//! Safety Services Module
//!
//! Provides Safe Operating Area (SOA) and design rule checking.

pub mod soa_manager;

pub use soa_manager::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoARuleVerdict, SoAViolation,
    ViolationSeverity,
};
