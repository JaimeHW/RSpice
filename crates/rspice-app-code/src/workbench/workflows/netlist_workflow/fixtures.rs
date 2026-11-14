//! What the netlist workflow's cases seed and read: a generated bundle with
//! a two-level include closure, a model holding an owned source under a
//! given edit strategy, and the reviewed vendor corpus. The workflow's own
//! cases and the code feature's whole-application tests share them.

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod vendor_profiles;

pub fn generated_bundle_fixture() -> Vec<u8> {
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, SourceLocator,
    };

    let root = "bundle fixture\n.include \"models/base.lib\"\nV1 out 0 1\n.op\n.end\n";
    let base = SourceLocator::try_new("models/base.lib", "base.lib").unwrap();
    let child = SourceLocator::try_new("models/devices/core.lib", "core.lib").unwrap();
    let dependencies = vec![
        DependencyMetadata::unresolved_direct_to(0, "models/base.lib", base.clone())
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::Vendor)
            .resolve_utf8(b".include \"devices/core.lib\"\n.model base nmos level=1\n".to_vec())
            .unwrap(),
        DependencyMetadata::unresolved_transitive_to(base, 0, "devices/core.lib", child)
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::TechnologyPackage)
            .resolve_utf8(b".model core nmos level=1 vto=0.45\n".to_vec())
            .unwrap(),
    ];
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "netlist-bundle-import-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
            ),
        )
        .unwrap(),
        root.as_bytes().to_vec(),
        dependencies,
        Vec::new(),
    )
    .unwrap();
    // SPICE is already the canonical dialect, so the fixture renders each
    // document unchanged and the bundle under test is exactly what the
    // generator produced.
    crate::io::build_generated_bundle(
        &artifact,
        crate::io::NetlistFormat::Spice,
        true,
        str::to_owned,
    )
    .unwrap()
}

/// The owned-strategy fixture seeded into a session's model: a generated
/// deck copied into an owned source the author rewrote, held under
/// `strategy`.
pub fn seed_owned_strategy(
    model: &mut rspice_app_state::workbench::app_state::model::Model,
    generated_source: &str,
    authored_source: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) {
    use crate::product::ObjectRevision;
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, content_digest,
    };

    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-workflow-test",
        GenerationInput::new(ObjectRevision::INITIAL, content_digest("test-inputs")),
    )
    .expect("valid generated provenance");
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        generated_source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .expect("valid generated artifact");
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact)
        .expect("valid generated document");
    let mut owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .expect("editable copy");
    owned
        .replace_editable_source(owned.content_digest(), authored_source.as_bytes().to_vec())
        .expect("authored source");

    model.workspace.content.netlist_source = Some(authored_source.to_owned());
    model.workspace.content.netlist_document = Some(owned);
    model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "owned.cir".to_owned(),
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(authored_source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
}
