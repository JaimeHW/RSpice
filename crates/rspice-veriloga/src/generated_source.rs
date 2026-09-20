//! Data-only generated-source artifacts carried by compilation reports.
//!
//! These types remain available without `rust-codegen`, so runtime-only builds
//! can read and validate the same serialized reports as compiler tooling. The
//! optional `rust_backend` owns emission, not the artifact format.

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedRustFile {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedRustDevice {
    pub module_name: String,
    pub public_model_name: String,
    pub folder_name: String,
    pub files: Vec<GeneratedRustFile>,
    pub source_digest: String,
    pub source_identity: String,
    pub accepted_state_shape_identity: [u8; 32],
}
