//! Signed PDK runtime validation, executable source sealing and callback execution.

mod callback;
mod package;
mod project_binding;

pub use project_binding::{
    project_signed_technology_package, validate_project_binding, validate_project_pin,
    validate_project_technology_inputs,
};

pub use package::{
    PdkModelSourceParts, SealedPdkModelProcessBinding, SealedPdkModelSources,
    SealedPdkVerilogAArtifact, SealedPdkVerilogABinding, ValidatedPdkTechnologyPackage,
    validate_archive, validate_archive_bytes, validate_runtime_compatibility,
};
