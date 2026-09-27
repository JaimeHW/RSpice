//! Signed PDK runtime validation, executable source sealing and callback execution.

mod callback;
mod package;

pub use package::{
    PdkModelSourceParts, SealedPdkModelProcessBinding, SealedPdkModelSources,
    SealedPdkVerilogAArtifact, SealedPdkVerilogABinding, ValidatedPdkTechnologyPackage,
    validate_archive, validate_archive_bytes, validate_runtime_compatibility,
};
