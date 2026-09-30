//! Existing signed-package fixtures shared by runtime and application tests.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signer as _, SigningKey};
use rspice_model_library::pdk::contracts::*;
use rspice_model_library::pdk::manifest::PdkTechnologyManifest;
use rspice_model_library::pdk::{
    PdkAdministrativeAuthority, PdkPublisherTrustStore, TrustedPdkPublisherKey, content_digest,
};

pub fn fixture_archive() -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let model_bytes = br#".lib TT
.model nmos_demo nmos level=1 vto=0.55
.endl TT
.lib SS
.model nmos_demo nmos level=1 vto=0.60
.endl SS
.lib FF
.model nmos_demo nmos level=1 vto=0.50
.endl FF
.lib SF
.model nmos_demo nmos level=1 vto=0.58
.endl SF
.lib FS
.model nmos_demo nmos level=1 vto=0.52
.endl FS
"#
    .to_vec();
    let callback_bytes = wat::parse_str(
        r#"(module
                (memory (export "memory") 1 2)
                (func (export "derive") (result i32)
                    i32.const 0))"#,
    )
    .unwrap();
    let manifest = PdkTechnologyManifest {
        schema_version: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
        package_id: "demo180".to_owned(),
        technology_name: "Demo 180 nm".to_owned(),
        revision: "2.3.1".to_owned(),
        publisher_id: "rspice-foundry-demo".to_owned(),
        signing_key_id: "ceremony-01".to_owned(),
        license_spdx: "LicenseRef-RSpice-Demo-PDK".to_owned(),
        process_node_nm: 180,
        database_unit_meters: 1.0e-9,
        stack_name: "1P2M".to_owned(),
        compatibility: PdkTechnologyCompatibility {
            minimum_engine_version: "0.1.0".to_owned(),
            minimum_viewer_version: "0.1.0".to_owned(),
            targets: vec![
                PdkExecutionTarget::Desktop,
                PdkExecutionTarget::WebAssembly,
                PdkExecutionTarget::Mobile,
            ],
        },
        model_sources: PdkModelProcess::ALL
            .into_iter()
            .map(|process| PdkModelProcessContract {
                process,
                sources: vec![PdkModelSectionSource {
                    source_id: format!("demo-models-{}", process.keyword().to_ascii_lowercase()),
                    domain: PdkModelDomain::Composite,
                    artifact_path: "models/demo.lib".to_owned(),
                    section: Some(process.keyword().to_owned()),
                }],
                required_domains: vec![PdkModelDomain::Composite],
            })
            .collect(),
        veriloga_sources: Vec::new(),
        symbol_definitions: Vec::new(),
        layers: vec![
            PdkTechnologyLayer {
                name: "active".to_owned(),
                order: 0,
                kind: PdkLayerKind::Active,
                purposes: vec!["drawing".to_owned()],
                role: "diffusion".to_owned(),
                display_rgba: [64, 160, 96, 255],
            },
            PdkTechnologyLayer {
                name: "cont".to_owned(),
                order: 1,
                kind: PdkLayerKind::Cut,
                purposes: vec!["drawing".to_owned()],
                role: "active to metal1".to_owned(),
                display_rgba: [192, 192, 192, 255],
            },
            PdkTechnologyLayer {
                name: "metal1".to_owned(),
                order: 2,
                kind: PdkLayerKind::Metal,
                purposes: vec!["drawing".to_owned(), "pin".to_owned()],
                role: "routing".to_owned(),
                display_rgba: [64, 144, 208, 255],
            },
        ],
        layer_aliases: Vec::new(),
        stream_map: vec![
            PdkStreamMapEntry {
                layer: "active".to_owned(),
                purpose: "drawing".to_owned(),
                stream_layer: 1,
                stream_datatype: 0,
            },
            PdkStreamMapEntry {
                layer: "cont".to_owned(),
                purpose: "drawing".to_owned(),
                stream_layer: 2,
                stream_datatype: 0,
            },
            PdkStreamMapEntry {
                layer: "metal1".to_owned(),
                purpose: "drawing".to_owned(),
                stream_layer: 3,
                stream_datatype: 0,
            },
            PdkStreamMapEntry {
                layer: "metal1".to_owned(),
                purpose: "pin".to_owned(),
                stream_layer: 3,
                stream_datatype: 1,
            },
        ],
        connectivity: vec![PdkConnectivityEdge {
            from_layer: "active".to_owned(),
            through_layer: "cont".to_owned(),
            to_layer: "metal1".to_owned(),
        }],
        vias: Vec::new(),
        recognition: Vec::new(),
        extraction: Vec::new(),
        callbacks: vec![PdkCallbackContract {
            callback_id: "derive-device".to_owned(),
            artifact_path: "callbacks/derive.wasm".to_owned(),
            abi_version: PDK_CALLBACK_ABI_VERSION,
            entrypoint: "derive".to_owned(),
            capabilities: vec![
                PdkCallbackCapability::ReadPackage,
                PdkCallbackCapability::WriteDerivedMetadata,
            ],
        }],
        artifacts: vec![
            PdkTechnologyArtifact {
                path: "models/demo.lib".to_owned(),
                kind: PdkTechnologyArtifactKind::Model,
                size_bytes: u64::try_from(model_bytes.len()).unwrap(),
                sha256: content_digest(&model_bytes),
            },
            PdkTechnologyArtifact {
                path: "callbacks/derive.wasm".to_owned(),
                kind: PdkTechnologyArtifactKind::Callback,
                size_bytes: u64::try_from(callback_bytes.len()).unwrap(),
                sha256: content_digest(&callback_bytes),
            },
        ],
    };
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    let archive = SignedPdkTechnologyArchive {
        schema_version: PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION,
        manifest_base64: STANDARD.encode(&manifest_bytes),
        signature_base64: STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes()),
        files: vec![
            PdkTechnologyArchiveFile {
                path: "models/demo.lib".to_owned(),
                content_base64: STANDARD.encode(model_bytes),
            },
            PdkTechnologyArchiveFile {
                path: "callbacks/derive.wasm".to_owned(),
                content_base64: STANDARD.encode(callback_bytes),
            },
        ],
    };
    (
        serde_json::to_vec(&archive).unwrap(),
        {
            let mut trust = PdkPublisherTrustStore::default();
            trust.keys = vec![TrustedPdkPublisherKey {
                publisher_id: "rspice-foundry-demo".to_owned(),
                key_id: "ceremony-01".to_owned(),
                verifying_key: signing_key.verifying_key().to_bytes(),
                revoked: false,
            }];
            trust
        },
        PdkAdministrativeAuthority {
            actor_id: "cad-admin@example.com".to_owned(),
            authority_id: "role:pdk-administrator".to_owned(),
        },
    )
}

pub fn fixture_archive_with_veriloga()
-> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
    fixture_archive_with_veriloga_source(
        br#"`include "parts/resistance.vams"
module pdk_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = `PDK_RESISTANCE;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    )
}

pub fn fixture_archive_with_veriloga_source(
    root: &[u8],
) -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
    let (bytes, trust, authority) = fixture_archive();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    let dependency = b"`define PDK_RESISTANCE 250.0\n".to_vec();
    for (path, content) in [
        ("veriloga/pdk_resistor.va", root),
        ("veriloga/parts/resistance.vams", dependency.as_slice()),
    ] {
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: path.to_owned(),
            kind: PdkTechnologyArtifactKind::VerilogASource,
            size_bytes: u64::try_from(content.len()).unwrap(),
            sha256: content_digest(content),
        });
        archive.files.push(PdkTechnologyArchiveFile {
            path: path.to_owned(),
            content_base64: STANDARD.encode(content),
        });
    }
    manifest.veriloga_sources = vec![PdkVerilogASourceContract {
        source_id: "pdk-resistor-runtime".to_owned(),
        root_artifact_path: "veriloga/pdk_resistor.va".to_owned(),
        module_name: "pdk_resistor".to_owned(),
        netlist_alias: "pdk_resistor_model".to_owned(),
    }];
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    (serde_json::to_vec(&archive).unwrap(), trust, authority)
}

pub fn fixture_signed_symbol(
    manifest: &PdkTechnologyManifest,
) -> rspice_model_library::symbol::ModelBoundSymbolDefinition {
    let mut model = rspice_model_library::symbol::SymbolModelReference::new(
        "signed-pdk:demo-models-tt",
        "nmos_demo",
    )
    .with_source_path("models/demo.lib");
    model.section = Some("TT".to_owned());
    model.revision = Some(manifest.revision.clone());
    let pins = [
        ("D", rspice_model_library::symbol::SymbolPinSide::Right),
        ("G", rspice_model_library::symbol::SymbolPinSide::Left),
        ("S", rspice_model_library::symbol::SymbolPinSide::Right),
        ("B", rspice_model_library::symbol::SymbolPinSide::Bottom),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, side))| {
        rspice_model_library::symbol::SymbolPinDefinition::new(
            name,
            rspice_model_library::symbol::SymbolElectricalType::Analog,
            rspice_design_model::port::PortDirection::InOut,
            side,
            index + 1,
        )
    })
    .collect::<Vec<_>>();
    let ports = pins
        .iter()
        .map(rspice_model_library::symbol::SymbolPinDefinition::port_spec)
        .collect();
    rspice_model_library::symbol::ModelBoundSymbolDefinition::new(
        rspice_model_library::symbol::SymbolIdentity::new(
            &manifest.package_id,
            "nmos_demo",
            1,
            "signed-pdk:demo180/nmos_demo",
        ),
        rspice_model_library::symbol::SymbolSourceContract::model(model.clone(), ports),
        pins,
        rspice_model_library::symbol::SymbolGraphicTemplate::RectangularIc,
        rspice_model_library::symbol::SymbolParameterForm {
            revision: 1,
            sections: Vec::new(),
        },
        rspice_model_library::symbol::SymbolNetlistBinding {
            device_prefix: "M".to_owned(),
            model: Some(model),
            template: "M{name} {nodes} {model} {params}".to_owned(),
            parameter_order: Vec::new(),
        },
        rspice_model_library::symbol::GeneratedSymbolViews::default(),
    )
}

pub fn fixture_archive_with_symbols()
-> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
    let (bytes, trust, authority) = fixture_archive();
    let signing_key = SigningKey::from_bytes(&[0x42; 32]);
    let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
    let mut manifest: PdkTechnologyManifest =
        serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
    manifest.symbol_definitions = vec![fixture_signed_symbol(&manifest)];
    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
    archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
    (serde_json::to_vec(&archive).unwrap(), trust, authority)
}
