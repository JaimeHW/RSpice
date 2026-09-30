//! Signed registry, source-sealing and tamper regressions using the private runtime owner.

use super::*;
use crate::pdk::test_fixtures::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signer as _, SigningKey};
use rspice_model_library::pdk::PdkTrustAuditAction;
use rspice_model_library::pdk::manifest::package_path_to_host_path;
use rspice_model_library::pdk::manifest::{
    signed_model_virtual_root, validate_manifest, validate_package_path,
};
use rspice_model_library::pdk::package::authenticate_archive;
use std::path::PathBuf;

#[test]
fn schema_five_layer_aliases_and_via_definitions_are_typed_and_cross_validated() {
    let (bytes, _, _) = fixture_archive();
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    manifest.layer_aliases.push(PdkLayerAlias {
        alias: "m1_drawing".to_owned(),
        layer: "metal1".to_owned(),
        purpose: "drawing".to_owned(),
    });
    manifest.vias.push(PdkViaDefinition {
        via_id: "cont_active_m1".to_owned(),
        lower_layer: "active".to_owned(),
        cut_layer: "cont".to_owned(),
        upper_layer: "metal1".to_owned(),
        cut_width_meters: 1.6e-7,
        cut_height_meters: 1.6e-7,
        lower_enclosure_meters: 5.0e-8,
        upper_enclosure_meters: 5.0e-8,
        maximum_rows: 8,
        maximum_columns: 8,
        maximum_rms_current_per_cut_amperes: Some(8.0e-3),
    });
    validate_manifest(&manifest).expect("schema-five physical contracts validate");

    let mut invalid_alias = manifest.clone();
    invalid_alias.layer_aliases[0].alias = "metal1".to_owned();
    assert!(matches!(
        validate_manifest(&invalid_alias),
        Err(PdkTechnologyError::Duplicate(_))
    ));

    let mut invalid_via = manifest;
    invalid_via.vias[0].cut_layer = "active".to_owned();
    assert!(matches!(
        validate_manifest(&invalid_via),
        Err(PdkTechnologyError::InvalidField(_)) | Err(PdkTechnologyError::InvalidReference(_))
    ));
}

#[test]
fn via_connectivity_rejects_cut_or_marker_endpoints() {
    let (bytes, _, _) = fixture_archive();
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    let metal = manifest
        .layers
        .iter_mut()
        .find(|layer| layer.name == "metal1")
        .expect("fixture metal layer");
    metal.kind = PdkLayerKind::Marker;
    let error = validate_manifest(&manifest).expect_err("marker is not a via endpoint");
    assert!(
        error.to_string().contains("not a conductor layer"),
        "{error}"
    );
}

#[test]
fn signed_symbols_materialize_exact_archive_bound_sources() {
    let (bytes, trust, _) = fixture_archive_with_symbols();
    let (_, package) = validate_archive_bytes(&bytes, &trust).expect("signed symbol validates");
    assert_eq!(package.manifest().symbol_definitions.len(), 1);
    assert_eq!(
        package.manifest().symbol_definitions[0]
            .netlist
            .model
            .as_ref()
            .and_then(|model| model.source_path.as_deref()),
        Some("models/demo.lib")
    );

    let definition = package
        .symbol_definitions()
        .first()
        .expect("runtime symbol");
    definition
        .validate()
        .expect("materialized symbol validates");
    let source = definition
        .netlist
        .model
        .as_ref()
        .and_then(|model| model.source_path.as_deref())
        .expect("materialized source");
    assert_eq!(
        PathBuf::from(source),
        signed_model_virtual_root(&package.archive_digest().to_string())
            .join(package_path_to_host_path("models/demo.lib"))
    );
}

#[test]
fn signed_symbols_reject_provider_or_artifact_authority_mismatch() {
    let (bytes, trust, _) = fixture_archive_with_symbols();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    let definition = manifest.symbol_definitions.first_mut().unwrap();
    let rspice_model_library::symbol::SymbolSourceContract::Model { model, .. } =
        &mut definition.source
    else {
        unreachable!()
    };
    model.library = "signed-pdk:unrelated-provider".to_owned();
    definition.netlist.model = Some(model.clone());
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());

    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
        Err(PdkTechnologyError::InvalidReference(_))
    ));
}

#[test]
fn signed_model_sources_materialize_exact_process_sections_and_archive_identity() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(
            &bytes,
            &trust,
            &authority,
            "Install executable model-source fixture",
        )
        .expect("signed package installs");
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .expect("exact project-bound model sources seal");
    assert_eq!(sealed.as_parts().sources.len(), 1);
    assert_eq!(sealed.as_parts().process_bindings.len(), 5);
    assert_eq!(sealed.as_parts().binding, package.binding());
    assert_eq!(sealed.as_parts().archive_digest, package.archive_digest());

    let combined = crate::model_sources::seal_catalog_execution_sources(
        &rspice_model_library::ModelCatalog::default(),
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .expect("empty ordinary model catalog seals")
    .with_pdk_model_sources(sealed)
    .expect("signed PDK closure merges");
    let tt = combined
        .reference_process_model_cards(rspice_app_types::product::ProcessCorner::TT)
        .expect("TT materializes");
    assert_eq!(tt.len(), 1);
    assert!(tt[0].contains("vto=0.55"));
    assert!(!tt[0].contains("vto=0.60"));
    assert!(!tt[0].to_ascii_lowercase().contains(".lib "));

    let corner_bindings = combined
        .corner_model_bindings(&[
            rspice_app_types::product::ProcessCorner::SS,
            rspice_app_types::product::ProcessCorner::FF,
        ])
        .expect("explicit signed corner sections materialize");
    assert_eq!(corner_bindings.len(), 2);
    assert!(
        corner_bindings[0]
            .materialized_model_cards
            .contains("vto=0.60")
    );
    assert!(
        corner_bindings[1]
            .materialized_model_cards
            .contains("vto=0.50")
    );
    let (identity, digest) = combined
        .pdk_model_identity()
        .expect("prepared model snapshot binds signed package");
    assert!(identity.contains("demo180@2.3.1"));
    assert_eq!(digest, package.archive_digest());
}

#[test]
fn signed_veriloga_closure_compiles_and_retains_exact_archive_authority() {
    let (bytes, trust, authority) = fixture_archive_with_veriloga();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(
            &bytes,
            &trust,
            &authority,
            "Install signed Verilog-A fixture",
        )
        .expect("signed Verilog-A package installs");
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .expect("exact signed Verilog-A closure seals");
    assert_eq!(sealed.as_parts().veriloga_artifacts.len(), 2);
    assert_eq!(sealed.as_parts().veriloga_bindings.len(), 1);
    let combined = crate::model_sources::seal_catalog_execution_sources(
        &rspice_model_library::ModelCatalog::default(),
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .unwrap()
    .with_pdk_model_sources(sealed)
    .unwrap();
    let (binding, archive_digest, artifacts, bindings) = combined
        .pdk_veriloga_authority()
        .expect("signed runtime authority retained");
    assert_eq!(binding, &package.binding());
    assert_eq!(archive_digest, package.archive_digest());
    let runtime = crate::veriloga::compile_signed_pdk_source_runtime(
        binding,
        archive_digest,
        artifacts,
        &bindings[0],
    )
    .expect("retained signed source recompiles");
    assert!(runtime.source_key().starts_with("__rspice_pdk__/"));
    assert_eq!(runtime.source_digest(), package.archive_digest());
    assert_eq!(runtime.module_name(), "pdk_resistor");
    assert_eq!(runtime.netlist_alias(), "pdk_resistor_model");
    assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
    let encoded = serde_json::to_vec(&runtime).unwrap();
    let restored: crate::veriloga::PreparedVerilogARuntime =
        serde_json::from_slice(&encoded).unwrap();
    restored.validate().expect("worker payload revalidates");
    assert_eq!(restored, runtime);
}

#[test]
fn signed_veriloga_rejects_tampered_or_untyped_dependency_bytes() {
    let (bytes, trust, _) = fixture_archive_with_veriloga();
    let mut tampered: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    tampered
        .files
        .iter_mut()
        .find(|file| file.path.ends_with("resistance.vams"))
        .unwrap()
        .content_base64 = STANDARD.encode(b"`define PDK_RESISTANCE 251.0\n");
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&tampered).unwrap(), &trust),
        Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
    ));

    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut untyped: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&untyped.manifest_base64).unwrap()).unwrap();
    manifest
        .artifacts
        .iter_mut()
        .find(|artifact| artifact.path.ends_with("resistance.vams"))
        .unwrap()
        .kind = PdkTechnologyArtifactKind::Documentation;
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    untyped.manifest_base64 = STANDARD.encode(&manifest_bytes);
    untyped.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&untyped).unwrap(), &trust),
        Err(PdkTechnologyError::ModelMaterialization(_))
    ));
}

#[test]
fn signed_veriloga_manifest_rejects_alias_collisions_unreachable_sources_and_schema_downgrade() {
    let (bytes, trust, _) = fixture_archive_with_veriloga();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);

    let mut collision: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&collision.manifest_base64).unwrap()).unwrap();
    let mut duplicate = manifest.veriloga_sources[0].clone();
    duplicate.source_id = "second-runtime".to_owned();
    duplicate.module_name = "second_module".to_owned();
    duplicate.netlist_alias = "PDK_RESISTOR_MODEL".to_owned();
    manifest.veriloga_sources.push(duplicate);
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    collision.manifest_base64 = STANDARD.encode(&manifest_bytes);
    collision.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&collision).unwrap(), &trust),
        Err(PdkTechnologyError::Duplicate(_))
    ));

    let mut unreachable: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&unreachable.manifest_base64).unwrap()).unwrap();
    let orphan = b"module orphan(p); inout p; electrical p; analog I(p) <+ 0.0; endmodule\n";
    manifest.artifacts.push(PdkTechnologyArtifact {
        path: "veriloga/orphan.va".to_owned(),
        kind: PdkTechnologyArtifactKind::VerilogASource,
        size_bytes: u64::try_from(orphan.len()).unwrap(),
        sha256: content_digest(orphan),
    });
    unreachable.files.push(PdkTechnologyArchiveFile {
        path: "veriloga/orphan.va".to_owned(),
        content_base64: STANDARD.encode(orphan),
    });
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    unreachable.manifest_base64 = STANDARD.encode(&manifest_bytes);
    unreachable.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    authenticate_archive(&unreachable, &trust).expect("archive bytes are authentic");
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&unreachable).unwrap(), &trust),
        Err(PdkTechnologyError::ModelMaterialization(_))
    ));

    let mut downgraded: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&downgraded.manifest_base64).unwrap()).unwrap();
    manifest.schema_version = 1;
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    downgraded.manifest_base64 = STANDARD.encode(&manifest_bytes);
    downgraded.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&downgraded).unwrap(), &trust),
        Err(PdkTechnologyError::InvalidField(_))
    ));
}

#[test]
fn signed_model_sections_close_package_relative_dependencies_in_memory() {
    let (bytes, trust, authority) = fixture_archive();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    let root = STANDARD.decode(&archive.files[0].content_base64).unwrap();
    let root = String::from_utf8(root)
        .unwrap()
        .replacen(".lib TT\n", ".lib TT\n.include \"parts/common.inc\"\n", 1)
        .into_bytes();
    archive.files[0].content_base64 = STANDARD.encode(&root);
    manifest.artifacts[0].size_bytes = u64::try_from(root.len()).unwrap();
    manifest.artifacts[0].sha256 = content_digest(&root);

    let dependency = b".model pdk_helper d is=1e-14\n".to_vec();
    manifest.artifacts.push(PdkTechnologyArtifact {
        path: "models/parts/common.inc".to_owned(),
        kind: PdkTechnologyArtifactKind::Model,
        size_bytes: u64::try_from(dependency.len()).unwrap(),
        sha256: content_digest(&dependency),
    });
    archive.files.push(PdkTechnologyArchiveFile {
        path: "models/parts/common.inc".to_owned(),
        content_base64: STANDARD.encode(&dependency),
    });
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    let signed = serde_json::to_vec(&archive).unwrap();

    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(
            &signed,
            &trust,
            &authority,
            "Install dependency-closed package",
        )
        .expect("package-relative dependency validates");
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .expect("dependency closure seals");
    assert_eq!(sealed.as_parts().sources.len(), 2);
    assert_eq!(sealed.as_parts().edges.len(), 1);
    let combined = crate::model_sources::seal_catalog_execution_sources(
        &rspice_model_library::ModelCatalog::default(),
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .unwrap()
    .with_pdk_model_sources(sealed)
    .unwrap();
    let cards = combined
        .reference_process_model_cards(rspice_app_types::product::ProcessCorner::TT)
        .unwrap();
    assert!(cards[0].contains(".model pdk_helper d is=1e-14"));
    assert!(!cards[0].contains(".include"));
    assert!(!cards[0].contains("/rspice-pdk/"));
}

#[test]
fn signed_model_contract_rejects_external_dependencies_and_missing_reference_process() {
    let (bytes, trust, _) = fixture_archive();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();

    let external = b".include \"../../outside.lib\"\n.model nmos_demo nmos level=1".to_vec();
    archive.files[0].content_base64 = STANDARD.encode(&external);
    manifest.artifacts[0].size_bytes = u64::try_from(external.len()).unwrap();
    manifest.artifacts[0].sha256 = content_digest(&external);
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
        Err(PdkTechnologyError::ModelMaterialization(_))
    ));

    let (bytes, trust, _) = fixture_archive();
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    manifest
        .model_sources
        .retain(|contract| contract.process != PdkModelProcess::Tt);
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
        Err(PdkTechnologyError::InvalidReference(_))
    ));
}

#[test]
fn project_model_sealing_rejects_post_validation_archive_mutation() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install exact package")
        .expect("install");
    let package = registry.validated_packages()[0].clone();
    registry.archives[0].files[0].content_base64 = STANDARD.encode(b".model attacker nmos level=1");
    assert!(matches!(
        registry.seal_model_sources_for_binding(&package.binding(), package.archive_digest()),
        Err(PdkTechnologyError::NotRuntimeValidated(_))
    ));
}

#[test]
fn signed_archive_verifies_every_exact_artifact_and_contract() {
    let (bytes, trust, _) = fixture_archive();
    let (_, package) = validate_archive_bytes(&bytes, &trust).expect("archive validates");

    assert_eq!(package.manifest().package_id, "demo180");
    assert_eq!(package.manifest().layers.len(), 3);
    assert_eq!(package.artifact_digests().len(), 2);
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        package.archive_digest(),
        content_digest(&serde_json::to_vec(&archive).unwrap())
    );
}

#[test]
fn recognition_and_extraction_contracts_are_typed_complete_and_source_bound() {
    let (bytes, trust, _) = fixture_archive();
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
    let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();

    let specialized = [
        (
            "recognition/nmos.json",
            PdkTechnologyArtifactKind::RecognitionMap,
            br#"{"device":"nmos"}"#.as_slice(),
        ),
        (
            "extraction/rc.json",
            PdkTechnologyArtifactKind::ExtractionRule,
            br#"{"quantities":["r","c"]}"#.as_slice(),
        ),
        (
            "qualification/nmos-layout.json",
            PdkTechnologyArtifactKind::QualificationVector,
            br#"{"shapes":[]}"#.as_slice(),
        ),
        (
            "qualification/rc-layout.json",
            PdkTechnologyArtifactKind::QualificationVector,
            br#"{"wires":[]}"#.as_slice(),
        ),
        (
            "qualification/rc-reference.json",
            PdkTechnologyArtifactKind::QualificationReference,
            br#"{"r":10.0,"c":1e-15}"#.as_slice(),
        ),
    ];
    for (path, kind, content) in specialized {
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: path.to_owned(),
            kind,
            size_bytes: u64::try_from(content.len()).unwrap(),
            sha256: content_digest(content),
        });
        archive.files.push(PdkTechnologyArchiveFile {
            path: path.to_owned(),
            content_base64: STANDARD.encode(content),
        });
    }
    manifest.recognition = vec![PdkRecognitionContract {
        contract_id: "recognize-nmos".to_owned(),
        device_class: "nmos".to_owned(),
        rule_artifact_path: "recognition/nmos.json".to_owned(),
        terminals: vec![
            PdkRecognitionTerminal {
                terminal_name: "source".to_owned(),
                layer: "active".to_owned(),
                purpose: "drawing".to_owned(),
            },
            PdkRecognitionTerminal {
                terminal_name: "drain".to_owned(),
                layer: "active".to_owned(),
                purpose: "drawing".to_owned(),
            },
        ],
        qualification_vectors: vec![PdkRecognitionQualificationVector {
            vector_id: "recognize-nmos-positive".to_owned(),
            layout_artifact_path: "qualification/nmos-layout.json".to_owned(),
            expected_instance_count: 1,
        }],
    }];
    manifest.extraction = vec![PdkExtractionContract {
        contract_id: "extract-metal-rc".to_owned(),
        rule_artifact_path: "extraction/rc.json".to_owned(),
        quantities: vec![
            PdkExtractionQuantity::Resistance,
            PdkExtractionQuantity::Capacitance,
        ],
        layer_purposes: vec![PdkLayerPurposeRef {
            layer: "metal1".to_owned(),
            purpose: "drawing".to_owned(),
        }],
        qualification_vectors: vec![PdkExtractionQualificationVector {
            vector_id: "extract-metal-rc-reference".to_owned(),
            layout_artifact_path: "qualification/rc-layout.json".to_owned(),
            reference_artifact_path: "qualification/rc-reference.json".to_owned(),
        }],
    }];

    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    let signed = serde_json::to_vec(&archive).unwrap();
    let (_, package) = validate_archive_bytes(&signed, &trust).expect("typed contracts validate");
    assert_eq!(package.manifest().recognition.len(), 1);
    assert_eq!(package.manifest().extraction.len(), 1);

    let mut invalid = manifest.clone();
    invalid.recognition[0].terminals[0].purpose = "undeclared".to_owned();
    let invalid_bytes = serde_json::to_vec(&invalid).unwrap();
    archive.manifest_base64 = STANDARD.encode(&invalid_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&invalid_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
        Err(PdkTechnologyError::InvalidReference(_))
    ));

    let mut duplicate = manifest;
    duplicate.extraction[0].qualification_vectors[0].layout_artifact_path =
        "qualification/nmos-layout.json".to_owned();
    let duplicate_bytes = serde_json::to_vec(&duplicate).unwrap();
    archive.manifest_base64 = STANDARD.encode(&duplicate_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&duplicate_bytes).to_bytes());
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
        Err(PdkTechnologyError::Duplicate(_))
    ));
}

#[test]
fn project_pin_resolves_only_the_exact_currently_trusted_archive() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install for project pin")
        .expect("install");
    let package = registry
        .validated_packages()
        .first()
        .expect("installed package validates");
    let pin =
        rspice_model_library::ProjectSignedTechnologyPin::from_package_metadata(package.metadata())
            .expect("project pin");
    crate::pdk::validate_project_pin(registry.validated_packages(), &pin)
        .expect("exact trusted archive resolves");

    let json = serde_json::to_string(&pin).expect("pin serializes");
    let restored: rspice_model_library::ProjectSignedTechnologyPin =
        serde_json::from_str(&json).expect("pin deserializes");
    assert_eq!(restored, pin);

    let mut revoked = trust;
    revoked.keys[0].revoked = true;
    registry
        .revalidate_installed(&revoked)
        .expect_err("revocation invalidates runtime packages");
    assert!(matches!(
        crate::pdk::validate_project_pin(registry.validated_packages(), &pin),
        Err(rspice_model_library::TechnologyBindingError::SignedPackageUnavailable { .. })
    ));
}

#[test]
fn publisher_key_provision_and_revocation_are_immutable_and_hash_chained() {
    let (bytes, fixture_trust, authority) = fixture_archive();
    let key = fixture_trust.keys[0].clone();
    let mut trust = PdkPublisherTrustStore::default();
    let provision = trust
        .provision_key(key.clone(), &authority, "Approve foundry ceremony key")
        .expect("provision");
    assert_eq!(provision.action, PdkTrustAuditAction::Provision);
    assert!(validate_archive_bytes(&bytes, &trust).is_ok());

    let before_duplicate = trust.clone();
    assert!(matches!(
        trust.provision_key(key.clone(), &authority, "Duplicate"),
        Err(PdkTechnologyError::ImmutableTrustKey(_))
    ));
    assert_eq!(trust, before_duplicate);

    let revoke = trust
        .revoke_key(
            &key.publisher_id,
            &key.key_id,
            &authority,
            "Publisher key retired",
        )
        .expect("revoke");
    assert_eq!(revoke.action, PdkTrustAuditAction::Revoke);
    assert_eq!(
        revoke.previous_receipt_digest,
        Some(provision.receipt_digest)
    );
    assert!(matches!(
        validate_archive_bytes(&bytes, &trust),
        Err(PdkTechnologyError::RevokedPublisherKey { .. })
    ));
    let before_second_revoke = trust.clone();
    assert!(matches!(
        trust.revoke_key(&key.publisher_id, &key.key_id, &authority, "Revoke again"),
        Err(PdkTechnologyError::ImmutableTrustKey(_))
    ));
    assert_eq!(trust, before_second_revoke);

    let json = serde_json::to_string(&trust).expect("serialize governed trust");
    let restored: PdkPublisherTrustStore =
        serde_json::from_str(&json).expect("deserialize governed trust");
    restored.validate().expect("restored audit validates");

    let mut tampered = serde_json::to_value(&restored).unwrap();
    tampered["audit"][0]["reason"] = serde_json::json!("altered");
    let tampered: PdkPublisherTrustStore = serde_json::from_value(tampered).unwrap();
    assert!(matches!(
        tampered.validate(),
        Err(PdkTechnologyError::TrustAuditCorrupted(_))
    ));
}

#[test]
fn tampered_artifact_signature_and_unknown_key_fail_closed() {
    let (bytes, trust, _) = fixture_archive();
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    archive.files[0].content_base64 = STANDARD.encode(b"tampered");
    let tampered = serde_json::to_vec(&archive).unwrap();
    assert!(matches!(
        validate_archive_bytes(&tampered, &trust),
        Err(PdkTechnologyError::ArtifactSizeMismatch { .. })
            | Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
    ));

    let mut signature_tampered: SignedPdkTechnologyArchive =
        serde_json::from_slice(&bytes).unwrap();
    signature_tampered.signature_base64 = STANDARD.encode([0_u8; 64]);
    assert!(matches!(
        validate_archive_bytes(&serde_json::to_vec(&signature_tampered).unwrap(), &trust),
        Err(PdkTechnologyError::InvalidSignature { .. })
    ));

    assert!(matches!(
        validate_archive_bytes(&bytes, &PdkPublisherTrustStore::default()),
        Err(PdkTechnologyError::UntrustedPublisher { .. })
    ));
}

#[test]
fn network_callbacks_and_incomplete_stream_maps_are_rejected() {
    let (bytes, trust, _) = fixture_archive();
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
    let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
    manifest.callbacks[0]
        .capabilities
        .push(PdkCallbackCapability::Network);
    assert!(matches!(
        validate_manifest(&manifest),
        Err(PdkTechnologyError::ForbiddenCapability(_))
    ));

    manifest.callbacks[0].capabilities.pop();
    manifest.stream_map.pop();
    assert!(matches!(
        validate_manifest(&manifest),
        Err(PdkTechnologyError::MissingMapping(_))
    ));

    let mut revoked = trust;
    revoked.keys[0].revoked = true;
    assert!(matches!(
        validate_archive_bytes(&bytes, &revoked),
        Err(PdkTechnologyError::RevokedPublisherKey { .. })
    ));
}

#[test]
fn target_declarations_are_nonempty_and_activation_is_platform_scoped() {
    let (bytes, trust, authority) = fixture_archive();
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
    let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
    manifest.compatibility.targets.clear();
    assert!(matches!(
        validate_manifest(&manifest),
        Err(PdkTechnologyError::InvalidField(_))
    ));

    manifest.compatibility.targets = vec![match current_execution_target() {
        PdkExecutionTarget::Desktop => PdkExecutionTarget::WebAssembly,
        PdkExecutionTarget::WebAssembly | PdkExecutionTarget::Mobile => PdkExecutionTarget::Desktop,
    }];
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    let restricted_bytes = serde_json::to_vec(&archive).unwrap();

    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(
            &restricted_bytes,
            &trust,
            &authority,
            "Install platform-restricted package",
        )
        .expect("a platform-restricted package can be inspected");
    assert!(matches!(
        registry.activate(
            "demo180",
            "2.3.1",
            &authority,
            "Attempt incompatible activation"
        ),
        Err(PdkTechnologyError::IncompatibleRuntime(_))
    ));
    assert!(registry.active_binding().is_none());
}

#[test]
fn install_activation_and_rollback_are_hash_chained_and_revalidated() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    let install = registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install reviewed package")
        .expect("install");
    let activate = registry
        .activate(
            "demo180",
            "2.3.1",
            &authority,
            "Activate for new project bindings",
        )
        .expect("activate");
    assert_eq!(
        activate.previous_receipt_digest,
        Some(install.receipt_digest)
    );
    registry.validate_audit_chain().expect("audit chain");
    assert!(registry.active_package().is_some());

    let json = serde_json::to_string(&registry).unwrap();
    let mut restored: PdkTechnologyRegistry = serde_json::from_str(&json).unwrap();
    assert!(restored.active_package().is_none());
    assert!(!restored.runtime_ready());
    restored
        .revalidate_installed(&trust)
        .expect("revalidate persisted archive");
    assert!(restored.active_package().is_some());

    // The same active target is not a rollback. A future revision must be
    // activated before returning to this retained prior binding.
    assert!(matches!(
        restored.rollback_to("demo180", "2.3.1", &authority, "No intervening revision"),
        Err(PdkTechnologyError::InvalidTransition(_))
    ));
}

#[test]
fn immutable_revision_and_tampered_audit_fail_before_mutation() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install")
        .unwrap();
    let before = registry.clone();
    assert!(matches!(
        registry.install_archive_bytes(&bytes, &trust, &authority, "Install again"),
        Err(PdkTechnologyError::ImmutableRevision(_))
    ));
    assert_eq!(registry, before);

    registry.audit[0].reason = "altered".to_owned();
    let tampered = registry.clone();
    assert!(matches!(
        registry.activate("demo180", "2.3.1", &authority, "Activate"),
        Err(PdkTechnologyError::AuditCorrupted(_))
    ));
    assert_eq!(registry, tampered);
}

#[test]
fn recomputed_hashes_cannot_disguise_impossible_transitions_or_wrong_archives() {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install")
        .unwrap();

    let target = registry.audit[0].target.clone();
    registry.audit[0].after_active = Some(target.clone());
    registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
    registry.active = Some(target);
    assert!(matches!(
        registry.validate_audit_chain(),
        Err(PdkTechnologyError::AuditCorrupted(_))
    ));

    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "Install")
        .unwrap();
    registry.audit[0].archive_digest = content_digest(b"wrong archive");
    registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
    let errors = registry
        .revalidate_installed(&trust)
        .expect_err("receipt must bind the installed archive");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("archive digest does not match"))
    );
}

#[test]
fn unknown_manifest_fields_and_unsafe_paths_are_rejected() {
    let (bytes, trust, _) = fixture_archive();
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
    value["unknown"] = serde_json::json!(true);
    assert!(
        serde_json::from_value::<PdkTechnologyManifest>(value)
            .unwrap_err()
            .to_string()
            .contains("unknown field")
    );
    assert!(matches!(
        validate_package_path("artifact", "../secret"),
        Err(PdkTechnologyError::InvalidField(_))
    ));
    assert!(trust.validate().is_ok());
}
