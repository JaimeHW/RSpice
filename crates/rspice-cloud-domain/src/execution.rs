//! Versioned, customer-visible simulation execution provenance.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    CURRENT_REVISION_CONTENT_DIGEST_VERSION, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
    LEGACY_REVISION_CONTENT_DIGEST_VERSION,
};

pub const LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION: u8 = 1;
pub const VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION: u8 = 2;
pub const CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION: u8 = 3;
pub const MAX_SIMULATION_EXECUTION_MANIFEST_BYTES: usize = 128 * 1024;
pub const MAX_SIMULATION_EXECUTION_ARTIFACTS: usize = 100;
pub const MAX_SIMULATION_EXECUTION_ARTIFACT_BYTES: u64 = 53_687_091_200;
pub const MAX_SIMULATION_ATTEMPTS: i32 = 20;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SimulationExecutionManifest {
    pub protocol_version: u8,
    pub engine_protocol_version: u8,
    pub attempt: i32,
    pub worker_class: String,
    pub engine: SimulationExecutionEngine,
    pub revision: SimulationExecutionRevision,
    pub request: SimulationExecutionRequest,
    pub artifacts: Vec<SimulationExecutionArtifact>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SimulationExecutionEngine {
    pub name: String,
    pub build: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_mode: Option<SimulationExecutionRuntimeMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solver_sha256: Option<String>,
    /// Byte-verified only in protocol version 3. Older retained manifests used
    /// this field as declarative operator metadata.
    pub model_library_sha256: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulationExecutionRuntimeMode {
    SelfContained,
    Delegating,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SimulationExecutionRevision {
    pub content_digest_version: i16,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SimulationExecutionRequest {
    pub digest_version: i16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SimulationExecutionArtifact {
    pub id: Uuid,
    pub kind: String,
    pub sha256: String,
    pub size_bytes: u64,
}

/// Validates the complete bounded execution-provenance projection shared by
/// workers, the API, public snapshots, and native/browser clients. Version 1
/// and version 2 remain readable for retained attempts; version 3 binds the
/// adapter's runtime mode and any separately executed solver binary.
#[must_use]
pub fn is_valid_simulation_execution_manifest(value: &Value) -> bool {
    let Ok(encoded) = serde_json::to_vec(value) else {
        return false;
    };
    if encoded.len() > MAX_SIMULATION_EXECUTION_MANIFEST_BYTES {
        return false;
    }
    let Ok(manifest) = serde_json::from_slice::<SimulationExecutionManifest>(&encoded) else {
        return false;
    };
    is_valid_manifest(&manifest)
}

fn is_valid_manifest(manifest: &SimulationExecutionManifest) -> bool {
    if !(1..=MAX_SIMULATION_ATTEMPTS).contains(&manifest.attempt)
        || !valid_identifier(&manifest.worker_class, 80)
        || !valid_clean_text(&manifest.engine.name, 120)
        || !valid_clean_text(&manifest.engine.build, 256)
        || !manifest
            .engine
            .model_library_sha256
            .as_deref()
            .is_none_or(valid_sha256)
        || !manifest
            .engine
            .solver_sha256
            .as_deref()
            .is_none_or(valid_sha256)
        || manifest.request.digest_version != CURRENT_SIMULATION_REQUEST_DIGEST_VERSION
        || manifest.artifacts.len() > MAX_SIMULATION_EXECUTION_ARTIFACTS
    {
        return false;
    }

    match manifest.protocol_version {
        LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION => {
            if manifest.engine.runtime_mode.is_some()
                || manifest.engine.solver_sha256.is_some()
                || !manifest
                    .engine
                    .adapter_sha256
                    .as_deref()
                    .is_none_or(valid_sha256)
            {
                return false;
            }
        }
        VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION => {
            if manifest.engine.runtime_mode.is_some()
                || manifest.engine.solver_sha256.is_some()
                || !manifest
                    .engine
                    .adapter_sha256
                    .as_deref()
                    .is_some_and(valid_sha256)
            {
                return false;
            }
        }
        CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION => {
            if !manifest
                .engine
                .adapter_sha256
                .as_deref()
                .is_some_and(valid_sha256)
                || !match manifest.engine.runtime_mode {
                    Some(SimulationExecutionRuntimeMode::SelfContained) => {
                        manifest.engine.solver_sha256.is_none()
                    }
                    Some(SimulationExecutionRuntimeMode::Delegating) => manifest
                        .engine
                        .solver_sha256
                        .as_deref()
                        .is_some_and(valid_sha256),
                    _ => false,
                }
            {
                return false;
            }
        }
        _ => return false,
    }

    // Engine protocol 4 replaced protocol 3 when the adapter moved every
    // analysis family onto the shared typed result document. The request
    // envelope did not change, so both pair with revision content digest 2 and
    // retained protocol-3 execution evidence stays readable.
    let expected_engine_protocols: &[u8] = match manifest.revision.content_digest_version {
        LEGACY_REVISION_CONTENT_DIGEST_VERSION if manifest.artifacts.is_empty() => &[1],
        LEGACY_REVISION_CONTENT_DIGEST_VERSION => &[2],
        CURRENT_REVISION_CONTENT_DIGEST_VERSION => &[3, 4],
        _ => return false,
    };
    if !expected_engine_protocols.contains(&manifest.engine_protocol_version) {
        return false;
    }

    let mut artifact_ids = HashSet::with_capacity(manifest.artifacts.len());
    manifest.artifacts.iter().all(|artifact| {
        !artifact.id.is_nil()
            && artifact_ids.insert(artifact.id)
            && matches!(
                artifact.kind.as_str(),
                "circuit_attachment" | "model_library"
            )
            && valid_sha256(&artifact.sha256)
            && (1..=MAX_SIMULATION_EXECUTION_ARTIFACT_BYTES).contains(&artifact.size_bytes)
    })
}

fn valid_identifier(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn valid_clean_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn current_manifest() -> Value {
        json!({
            "protocol_version": CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION,
            "engine_protocol_version": 3,
            "attempt": 2,
            "worker_class": "shared",
            "engine": {
                "name": "rspice-engine",
                "build": "2026.07.20",
                "runtime_mode": "self_contained",
                "adapter_sha256": "ab".repeat(32),
                "solver_sha256": null,
                "model_library_sha256": null,
            },
            "revision": {"content_digest_version": CURRENT_REVISION_CONTENT_DIGEST_VERSION},
            "request": {"digest_version": CURRENT_SIMULATION_REQUEST_DIGEST_VERSION},
            "artifacts": [{
                "id": Uuid::from_u128(1),
                "kind": "model_library",
                "sha256": "cd".repeat(32),
                "size_bytes": 1024,
            }],
        })
    }

    #[test]
    fn current_manifest_requires_verified_adapter_identity() {
        let mut manifest = current_manifest();
        manifest["future_scheduler"] = json!({"isolation": "confidential-vm"});
        assert!(is_valid_simulation_execution_manifest(&manifest));

        let mut missing_adapter = manifest.clone();
        missing_adapter["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("adapter_sha256");
        assert!(!is_valid_simulation_execution_manifest(&missing_adapter));

        let mut uppercase_adapter = manifest;
        uppercase_adapter["engine"]["adapter_sha256"] = json!("AB".repeat(32));
        assert!(!is_valid_simulation_execution_manifest(&uppercase_adapter));
    }

    #[test]
    fn retained_version_one_manifest_remains_readable() {
        let mut manifest = current_manifest();
        manifest["protocol_version"] = json!(LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION);
        manifest["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("runtime_mode");
        manifest["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("solver_sha256");

        // One briefly deployed version-1 producer included the verified adapter
        // digest before the manifest version was advanced. Keep that evidence
        // readable as well as the original version-1 shape that omitted it.
        assert!(is_valid_simulation_execution_manifest(&manifest));

        manifest["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("adapter_sha256");

        assert!(is_valid_simulation_execution_manifest(&manifest));
    }

    #[test]
    fn retained_version_two_manifest_remains_readable() {
        let mut manifest = current_manifest();
        manifest["protocol_version"] =
            json!(VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION);
        manifest["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("runtime_mode");
        manifest["engine"]
            .as_object_mut()
            .expect("engine object")
            .remove("solver_sha256");

        assert!(is_valid_simulation_execution_manifest(&manifest));
    }

    #[test]
    fn delegating_runtime_requires_verified_solver_identity() {
        let mut manifest = current_manifest();
        manifest["engine"]["runtime_mode"] = json!("delegating");
        assert!(!is_valid_simulation_execution_manifest(&manifest));

        manifest["engine"]["solver_sha256"] = json!("ef".repeat(32));
        assert!(is_valid_simulation_execution_manifest(&manifest));

        manifest["engine"]["runtime_mode"] = json!("self_contained");
        assert!(!is_valid_simulation_execution_manifest(&manifest));

        manifest["engine"]["runtime_mode"] = json!("external_path");
        manifest["engine"]["solver_sha256"] = Value::Null;
        assert!(!is_valid_simulation_execution_manifest(&manifest));
    }

    #[test]
    fn provenance_shape_and_cross_field_semantics_fail_closed() {
        let baseline = current_manifest();
        for mutation in [
            ("attempt", json!(0)),
            ("engine_protocol_version", json!(2)),
            ("protocol_version", json!(4)),
        ] {
            let mut malformed = baseline.clone();
            malformed[mutation.0] = mutation.1;
            assert!(!is_valid_simulation_execution_manifest(&malformed));
        }

        // The adapter's protocol-4 result contract shares the version-2
        // revision digest, so both engine protocols read back; neither an
        // older nor a newer one does.
        let mut protocol_four = baseline.clone();
        protocol_four["engine_protocol_version"] = json!(4);
        assert!(is_valid_simulation_execution_manifest(&protocol_four));
        let mut protocol_five = baseline.clone();
        protocol_five["engine_protocol_version"] = json!(5);
        assert!(!is_valid_simulation_execution_manifest(&protocol_five));

        let mut duplicate = baseline.clone();
        duplicate["artifacts"] = json!([
            baseline["artifacts"][0].clone(),
            baseline["artifacts"][0].clone(),
        ]);
        assert!(!is_valid_simulation_execution_manifest(&duplicate));

        let mut invalid_size = baseline;
        invalid_size["artifacts"][0]["size_bytes"] = json!(0);
        assert!(!is_valid_simulation_execution_manifest(&invalid_size));
    }
}
