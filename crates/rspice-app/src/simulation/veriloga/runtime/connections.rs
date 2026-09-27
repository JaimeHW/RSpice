//! Sealed connection source transported alongside executable device models.

use super::{
    PreparedVerilogASourceBinding, valid_sealed_source_key, valid_veriloga_netlist_identifier,
};
use crate::product::ContentDigest;
use sha2::{Digest as _, Sha256};

/// A connection library has source provenance and an import alias, but no
/// device terminals, canonical device IR or JIT executable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreparedVerilogAConnectionLibrary {
    source_key: String,
    source_digest: ContentDigest,
    netlist_alias: String,
    artifact: rspice_veriloga::ConnectionLibraryArtifact,
    artifact_digest: ContentDigest,
}

impl PreparedVerilogAConnectionLibrary {
    pub(super) fn try_new(
        source_key: String,
        source_digest: ContentDigest,
        netlist_alias: String,
        artifact: rspice_veriloga::ConnectionLibraryArtifact,
    ) -> Result<Self, String> {
        let mut library = Self {
            source_key,
            source_digest,
            netlist_alias,
            artifact,
            artifact_digest: ContentDigest::from_bytes([0; 32]),
        };
        library.artifact_digest = library.expected_digest();
        library.validate()?;
        Ok(library)
    }

    fn expected_digest(&self) -> ContentDigest {
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.prepared-veriloga-connections/v1\0");
        for bytes in [
            self.source_key.as_bytes(),
            self.source_digest.as_bytes(),
            self.netlist_alias.as_bytes(),
            self.artifact.identity().as_slice(),
        ] {
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        if !valid_sealed_source_key(&self.source_key) {
            return Err(
                "Verilog-AMS connection library has an invalid sealed virtual source key"
                    .to_owned(),
            );
        }
        if !valid_veriloga_netlist_identifier(&self.netlist_alias) {
            return Err("Verilog-AMS connection library has an invalid netlist alias".to_owned());
        }
        self.artifact.validate_integrity()?;
        if self.expected_digest() != self.artifact_digest {
            return Err(
                "Verilog-AMS connection library digest does not match its payload".to_owned(),
            );
        }
        Ok(())
    }

    pub(super) fn binding(&self) -> PreparedVerilogASourceBinding<'_> {
        PreparedVerilogASourceBinding {
            source_key: &self.source_key,
            netlist_alias: &self.netlist_alias,
            artifact_digest: self.artifact_digest,
            is_connection_library: true,
        }
    }

    pub(super) fn registration(
        &self,
    ) -> Result<rspice_core::ProjectVerilogASourceRegistration, String> {
        self.validate()?;
        Ok(rspice_core::ProjectVerilogASourceRegistration::Connections(
            rspice_core::ProjectVerilogAConnectionLibraryRegistration {
                source_key: self.source_key.clone().into(),
                aliases: vec![self.netlist_alias.clone()],
                artifact: self.artifact.clone(),
            },
        ))
    }
}
