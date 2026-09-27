//! Persisted callback contracts and application integration coverage.
//!
//! The simulation service owns execution of signed callbacks; project and
//! registry transactions consume the canonical portable evidence records.

pub use rspice_model_library::pdk::callback::{
    MAX_PROJECT_PDK_CALLBACK_RECEIPTS, PdkCallbackError, PdkCallbackExecutionInput,
    PdkCallbackExecutionReceipt, ProjectPdkCallbackReceipt,
};

#[cfg(test)]
mod tests {
    use super::super::technology_package::PdkTechnologyBinding;
    use super::*;
    use crate::product::ContentDigest;
    use crate::state::pdk_config::{
        PdkAdministrativeAuthority, PdkPublisherTrustStore, PdkTechnologyRegistry,
    };
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use ed25519_dalek::{Signer as _, SigningKey};
    use rspice_model_library::pdk::content_digest;
    use rspice_model_library::pdk::contracts::{
        PDK_CALLBACK_ABI_VERSION, PdkCallbackCapability, SignedPdkTechnologyArchive,
    };
    use std::collections::BTreeMap;

    fn callback_archive(
        wat_source: &str,
        capabilities: Vec<PdkCallbackCapability>,
    ) -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = super::super::technology_package::tests::fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: super::super::technology_package::PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let callback_bytes = wat::parse_str(wat_source).unwrap();
        let callback = &mut manifest.callbacks[0];
        callback.capabilities = capabilities;
        callback.abi_version = PDK_CALLBACK_ABI_VERSION;
        callback.entrypoint = "derive".to_owned();
        let artifact = manifest
            .artifacts
            .iter_mut()
            .find(|artifact| artifact.path == callback.artifact_path)
            .unwrap();
        artifact.size_bytes = u64::try_from(callback_bytes.len()).unwrap();
        artifact.sha256 = content_digest(&callback_bytes);
        let file = archive
            .files
            .iter_mut()
            .find(|file| file.path == callback.artifact_path)
            .unwrap();
        file.content_base64 = STANDARD.encode(callback_bytes);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(
            SigningKey::from_bytes(&[0x42; 32])
                .sign(&manifest_bytes)
                .to_bytes(),
        );
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    fn install(
        bytes: &[u8],
        trust: &PdkPublisherTrustStore,
        authority: &PdkAdministrativeAuthority,
    ) -> (PdkTechnologyRegistry, PdkTechnologyBinding, ContentDigest) {
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(bytes, trust, authority, "Install callback fixture")
            .unwrap();
        let package = registry.validated_packages()[0].clone();
        (registry, package.binding(), package.archive_digest())
    }

    #[test]
    fn signed_callback_executes_with_capabilities_and_verifiable_provenance() {
        let wat = r#"(module
            (import "rspice" "project_parameter_len" (func $len (param i32 i32) (result i32)))
            (import "rspice" "project_parameter_read" (func $read (param i32 i32 i32 i32) (result i32)))
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 2)
            (data (i32.const 0) "width")
            (data (i32.const 16) "derived.width")
            (func (export "derive") (result i32)
                (local $length i32)
                i32.const 0
                i32.const 5
                call $len
                local.tee $length
                i32.const 0
                i32.lt_s
                if
                    i32.const 10
                    return
                end
                i32.const 0
                i32.const 5
                i32.const 64
                i32.const 64
                call $read
                i32.const 0
                i32.lt_s
                if
                    i32.const 11
                    return
                end
                i32.const 16
                i32.const 13
                i32.const 64
                local.get $length
                call $emit))"#;
        let (bytes, trust, authority) = callback_archive(
            wat,
            vec![
                PdkCallbackCapability::ReadProjectParameters,
                PdkCallbackCapability::WriteDerivedMetadata,
            ],
        );
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let input = PdkCallbackExecutionInput {
            project_parameters: BTreeMap::from([("width".to_owned(), "2.5u".to_owned())]),
        };
        let receipt = registry
            .execute_callback_for_binding(&binding, archive_digest, "DERIVE-DEVICE", &input)
            .unwrap();
        receipt.validate().unwrap();
        assert_eq!(
            receipt.derived_metadata.get("derived.width"),
            Some(&"2.5u".to_owned())
        );
        assert_eq!(receipt.package_binding, binding);
        assert_eq!(receipt.archive_digest, archive_digest);
        assert!(receipt.fuel_consumed > 0);

        let repeated = registry
            .execute_callback_for_binding(&binding, archive_digest, "derive-device", &input)
            .unwrap();
        assert_eq!(receipt, repeated);
    }

    #[test]
    fn callback_import_without_signed_capability_is_rejected_at_install() {
        let wat = r#"(module
            (import "rspice" "project_parameter_len" (func (param i32 i32) (result i32)))
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32) i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(wat, Vec::new());
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        rspice_model_library::pdk::package::authenticate_archive(&archive, &trust)
            .expect("archive bytes are authentic");
        let mut registry = PdkTechnologyRegistry::default();
        let before = registry.clone();
        let error = registry
            .install_archive_bytes(&bytes, &trust, &authority, "Reject unauthorized import")
            .unwrap_err();
        assert!(error.to_string().contains("requires signed capability"));
        assert!(registry.archives().is_empty());
        assert_eq!(
            registry, before,
            "failed callback validation must not publish"
        );
    }

    #[test]
    fn callback_reads_only_exact_signed_package_bytes() {
        let wat = r#"(module
            (import "rspice" "package_file_len" (func $len (param i32 i32) (result i32)))
            (import "rspice" "package_file_read" (func $read (param i32 i32 i32 i32) (result i32)))
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 2)
            (data (i32.const 0) "models/demo.lib")
            (data (i32.const 32) "model.prefix")
            (func (export "derive") (result i32)
                i32.const 0
                i32.const 15
                call $len
                i32.const 0
                i32.lt_s
                if
                    i32.const 20
                    return
                end
                i32.const 0
                i32.const 15
                i32.const 128
                i32.const 1024
                call $read
                i32.const 0
                i32.lt_s
                if
                    i32.const 21
                    return
                end
                i32.const 32
                i32.const 12
                i32.const 128
                i32.const 4
                call $emit))"#;
        let (bytes, trust, authority) = callback_archive(
            wat,
            vec![
                PdkCallbackCapability::ReadPackage,
                PdkCallbackCapability::WriteDerivedMetadata,
            ],
        );
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let receipt = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap();
        assert_eq!(
            receipt.derived_metadata.get("model.prefix"),
            Some(&".lib".to_owned())
        );
        receipt.validate().unwrap();
    }

    #[test]
    fn executable_callback_contract_rejects_schema_downgrade() {
        let (bytes, trust, authority) = callback_archive(
            r#"(module
                (memory (export "memory") 1 1)
                (func (export "derive") (result i32) i32.const 0))"#,
            Vec::new(),
        );
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: super::super::technology_package::PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.schema_version = 2;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let bytes = serde_json::to_vec(&archive).unwrap();
        let mut registry = PdkTechnologyRegistry::default();
        let error = registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Reject callback schema downgrade",
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("executable callbacks require manifest schema 3")
        );
    }

    #[test]
    fn callback_fuel_and_memory_limits_fail_closed() {
        let infinite = r#"(module
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32)
                (loop $again br $again)
                i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(infinite, Vec::new());
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::Execution(_)));

        let oversized = r#"(module
            (memory (export "memory") 200 200)
            (func (export "derive") (result i32) i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(oversized, Vec::new());
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::Instantiation(_)));
    }

    #[test]
    fn callback_host_rejects_invalid_memory_even_when_guest_ignores_status() {
        let wat = r#"(module
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32)
                i32.const 65530
                i32.const 32
                i32.const 0
                i32.const 1
                call $emit
                drop
                i32.const 0))"#;
        let (bytes, trust, authority) =
            callback_archive(wat, vec![PdkCallbackCapability::WriteDerivedMetadata]);
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::HostViolation(_)));
    }
}
