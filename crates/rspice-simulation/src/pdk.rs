//! Signed PDK runtime validation, executable source sealing and callback execution.

mod callback;
mod config;
pub mod import;
mod registry;
pub use config::PdkConfig;
pub use registry::{PdkTechnologyAuditAction, PdkTechnologyAuditReceipt, PdkTechnologyRegistry};
mod package;
mod project_binding;
#[cfg(any(test, feature = "pdk-test-fixtures"))]
pub mod test_fixtures;

pub use project_binding::{
    project_signed_technology_package, validate_project_binding, validate_project_pin,
    validate_project_technology_inputs,
};

pub use package::{
    PdkModelSourceParts, SealedPdkModelProcessBinding, SealedPdkModelSources,
    SealedPdkVerilogAArtifact, SealedPdkVerilogABinding, ValidatedPdkTechnologyPackage,
    validate_archive, validate_archive_bytes, validate_runtime_compatibility,
};
