//! Portable active connection-source transport, independent of a device ABI.

use serde::{Deserialize, Serialize};

/// Exact active connection declarations and authored bodies from one prepared
/// source. This is source-level transport: it contains no dummy device, JIT
/// address, or assumed executable connect-body implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionLibraryArtifact {
    schema_version: u32,
    source_package: String,
    preprocessed_source: String,
    identity: [u8; 32],
}

impl ConnectionLibraryArtifact {
    const SCHEMA_VERSION: u32 = 1;

    pub(crate) fn from_prepared(source_package: &str, source: &str) -> Self {
        let mut artifact = Self {
            schema_version: Self::SCHEMA_VERSION,
            source_package: source_package.to_owned(),
            preprocessed_source: source.to_owned(),
            identity: [0; 32],
        };
        artifact.identity = artifact.computed_identity();
        artifact
    }

    pub fn source_package(&self) -> &str {
        &self.source_package
    }

    pub fn preprocessed_source(&self) -> &str {
        &self.preprocessed_source
    }

    pub fn identity(&self) -> &[u8; 32] {
        &self.identity
    }

    fn computed_identity(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice.connection-library-artifact\0");
        hasher.update(&self.schema_version.to_le_bytes());
        for field in [&self.source_package, &self.preprocessed_source] {
            hasher.update(&(field.len() as u64).to_le_bytes());
            hasher.update(field.as_bytes());
        }
        *hasher.finalize().as_bytes()
    }

    /// Check source transport before parsing or registering it. The containing
    /// project/package remains responsible for authenticating artifact identity.
    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(format!(
                "unsupported connection library schema {}; expected {}",
                self.schema_version,
                Self::SCHEMA_VERSION
            ));
        }
        if self.source_package.trim().is_empty() || self.source_package.contains('\0') {
            return Err(
                "connection library source package must be non-empty and contain no NUL".to_owned(),
            );
        }
        if self.identity != self.computed_identity() {
            return Err(
                "connection library artifact identity does not match its source and provenance"
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// Resolve the transported source without preprocessing or filesystem I/O.
    /// Definitions and rules are analyzed together by the normal front end.
    pub fn connect_specification(&self) -> Result<crate::ConnectSpecification, String> {
        self.validate_integrity()?;
        let specification = crate::VerilogACompiler::default()
            .connect_specification_from_preprocessed(&self.preprocessed_source)
            .map_err(|error| format!("connection library '{}': {error}", self.source_package))?;
        if !specification.rules.has_declarations() {
            return Err(format!(
                "connection library '{}' contains no active connectmodule or connectrules declarations",
                self.source_package
            ));
        }
        Ok(specification)
    }
}
