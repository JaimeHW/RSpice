//! Tests for the generated netlist document's rules.
//!
//! A refresh is one atomic transition, validation is bound to exact content,
//! and a document from a future schema is rejected rather than partially
//! understood.

use serde_json::Value;

use super::*;

const GENERATED: &str = "RSpice generated deck\r\n.include \"models/core.lib\"\r\nV1 in 0 1\r\nR1 in 0 1k\r\n.op\r\n.end\r\n";

fn content_digest(bytes: &[u8]) -> ContentDigest {
    digest(bytes)
}

fn locator(value: &str) -> SourceLocator {
    SourceLocator::try_new(value, value).expect("valid locator")
}

fn generated(source: &str, input_marker: &[u8]) -> GeneratedArtifact {
    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-generator-v1",
        GenerationInput::new(ObjectRevision::INITIAL, content_digest(input_marker)),
    )
    .expect("valid provenance");
    GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        Vec::new(),
        vec![
            GeneratedSourceMapEntry::try_new(1, "user/top", "schematic", None, None)
                .expect("source map"),
        ],
    )
    .expect("valid generated artifact")
}

fn document() -> NetlistDocument {
    let id = NetlistDocumentId::try_from_uuid(
        Uuid::parse_str("c6ec5125-95dc-4a0c-93cc-24f5fef1d820").expect("UUID"),
    )
    .expect("non-nil");
    NetlistDocument::from_generated(id, generated(GENERATED, b"input-a")).expect("valid document")
}

#[test]
fn generated_document_preserves_exact_utf8_and_crlf_bytes() {
    let document = document();
    assert_eq!(document.source_bytes(), GENERATED.as_bytes());
    assert_eq!(
        document.content_digest(),
        content_digest(GENERATED.as_bytes())
    );
    assert_eq!(document.include_directives().len(), 1);
    assert_eq!(document.dependencies().len(), 1);
    assert!(matches!(
        document.dependencies()[0].resolution(),
        DependencyResolution::Unresolved
    ));
}

#[test]
fn stale_generated_refresh_is_transactional() {
    let mut document = document();
    let before = document.clone();
    let error = document
        .update_generated_artifact(
            content_digest(b"stale"),
            generated("new\n.end\n", b"input-b"),
        )
        .expect_err("conflict");
    assert!(matches!(
        error,
        DocumentError::GeneratedArtifactConflict { .. }
    ));
    assert_eq!(document, before);
}

#[test]
fn a_refresh_revokes_validation_and_resolves_its_own_includes() {
    let mut document = document();
    document
        .acknowledge_validation(document.content_digest(), Vec::new())
        .expect("validation receipt");
    assert!(document.validation().is_some());

    let current = document
        .generated_artifact()
        .expect("generated")
        .content_digest();
    document
        .update_generated_artifact(
            current,
            generated("changed\n.include other.lib\nR1 1 0 1k\n.end\n", b"input-b"),
        )
        .expect("refresh");
    assert!(document.validation().is_none());
    assert!(matches!(
        document.dependencies()[0].resolution(),
        DependencyResolution::Unresolved
    ));
    assert_eq!(document.dependencies()[0].locator().locator(), "other.lib");
}

#[test]
fn validation_is_bound_to_exact_content() {
    let mut document = NetlistDocument::from_generated(
        NetlistDocumentId::new(),
        generated("broken\nR1 only-one-node\n.end\n", b"input-a"),
    )
    .expect("document");
    let diagnostic =
        ValidationDiagnostic::try_new(DiagnosticSeverity::Error, "invalid resistor", 2, 1)
            .expect("diagnostic");
    document
        .acknowledge_validation(document.content_digest(), vec![diagnostic])
        .expect("validation transaction");
    let report = document.validation().expect("report");
    assert_eq!(report.content_digest(), document.content_digest());
    assert!(!report.is_valid());
    assert_eq!(report.error_count(), 1);
    assert!(report.diagnostics()[0].position().line() >= 1);
    let before = document.clone();
    assert!(matches!(
        document.acknowledge_validation(content_digest(b"previous source"), Vec::new()),
        Err(DocumentError::ContentConflict { .. })
    ));
    assert_eq!(document, before);
}

#[test]
fn diagnostic_positions_are_unicode_scalar_columns_and_checked() {
    let mut document = NetlistDocument::from_generated(
        NetlistDocumentId::new(),
        generated("unicode\nRμ μ 0 1k\n.end\n", b"input-a"),
    )
    .expect("document");
    let diagnostic = ValidationDiagnostic::try_new(DiagnosticSeverity::Warning, "review μ", 2, 4)
        .expect("diagnostic");
    document
        .acknowledge_validation(document.content_digest(), vec![diagnostic])
        .expect("valid position");
    let before = document.clone();
    assert!(matches!(
        document.acknowledge_validation(
            document.content_digest(),
            vec![
                ValidationDiagnostic::try_new(DiagnosticSeverity::Error, "outside", 2, 99)
                    .expect("shape")
            ]
        ),
        Err(DocumentError::InvalidDiagnostic(_))
    ));
    assert_eq!(document, before);
}

#[test]
fn sealed_dependency_graph_retains_exact_member_bytes_and_edges() {
    let root_source = "sealed root\n.include a.lib\n.end\n";
    let a_source = b"a member\r\n.include b.lib\r\n.model a r r=1k\r\n";
    let b_source = "b member μ\n.model b r r=2k\n".as_bytes();
    let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
        .resolve_utf8(a_source.to_vec())
        .expect("resolved a");
    let b = DependencyMetadata::unresolved_transitive(locator("a.lib"), 0, locator("b.lib"))
        .resolve_utf8(b_source.to_vec())
        .expect("resolved b");
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance"),
        root_source.as_bytes().to_vec(),
        vec![a, b],
        Vec::new(),
    )
    .expect("sealed artifact");

    assert!(artifact.dependency_graph_is_sealed());
    assert_eq!(
        artifact.dependencies()[0].source_bytes(),
        Some(a_source.as_slice())
    );
    assert_eq!(artifact.dependencies()[1].source_bytes(), Some(b_source));
    assert_eq!(artifact.dependencies()[1].parent_include_index(), Some(0));
    assert_eq!(
        artifact.dependencies()[0].resolution().byte_length(),
        Some(a_source.len())
    );
}

#[test]
fn resolution_edges_separate_requested_paths_from_portable_member_identity() {
    let root_source = "root\n.include ../vendor/a.lib\n.end\n";
    let a_locator = SourceLocator::try_new("vendor/a.lib", "a.lib")
        .expect("logical member")
        .with_native_origin(r"C:\cache\vendor\a.lib")
        .expect("native origin");
    let b_locator = SourceLocator::try_new("shared/b.lib", "b.lib").expect("logical member");
    let a_source = b"a\n.include ../shared/b.lib\n";
    let a = DependencyMetadata::unresolved_direct_to(0, "../vendor/a.lib", a_locator.clone())
        .expect("direct edge")
        .resolve_utf8(a_source.to_vec())
        .expect("resolved a");
    let b =
        DependencyMetadata::unresolved_transitive_to(a_locator, 0, "../shared/b.lib", b_locator)
            .expect("transitive edge")
            .resolve_utf8(b"b\n.model b r r=1k\n".to_vec())
            .expect("resolved b");
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance"),
        root_source.as_bytes().to_vec(),
        vec![a, b],
        Vec::new(),
    )
    .expect("resolved graph");

    assert_eq!(
        artifact.dependencies()[0].requested_locator(),
        "../vendor/a.lib"
    );
    assert_eq!(
        artifact.dependencies()[0].locator().logical_identity(),
        "vendor/a.lib"
    );
    assert_eq!(
        artifact.dependencies()[0].locator().native_origin(),
        Some(r"C:\cache\vendor\a.lib")
    );
    assert_eq!(
        artifact.dependencies()[1].requested_locator(),
        "../shared/b.lib"
    );
}

#[test]
fn generated_source_map_is_canonical_and_rejects_duplicate_or_out_of_range_lines() {
    let provenance = || {
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance")
    };
    let line_two = GeneratedSourceMapEntry::try_new(
        2,
        "user/top",
        "schematic",
        Some("XAMP".to_owned()),
        Some("RLOAD".to_owned()),
    )
    .expect("mapping");
    let line_one =
        GeneratedSourceMapEntry::try_new(1, "user/top", "schematic", None, None).expect("mapping");
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance(),
        b"mapped\nR1 in 0 1k\n.end\n".to_vec(),
        Vec::new(),
        vec![line_two.clone(), line_one],
    )
    .expect("canonical source map");
    assert_eq!(
        artifact
            .source_map()
            .iter()
            .map(GeneratedSourceMapEntry::generated_line)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    let mapped = artifact.source_map_entry(2).expect("line mapping");
    assert_eq!(mapped.cell_identity(), "user/top");
    assert_eq!(mapped.view_identity(), "schematic");
    assert_eq!(mapped.instance_identity(), Some("XAMP"));
    assert_eq!(mapped.component_identity(), Some("RLOAD"));

    let duplicate = GeneratedArtifact::try_from_utf8(
        provenance(),
        b"mapped\nR1 in 0 1k\n.end\n".to_vec(),
        Vec::new(),
        vec![line_two.clone(), line_two],
    );
    assert!(matches!(duplicate, Err(DocumentError::InvalidSourceMap(_))));
    let outside = GeneratedArtifact::try_from_utf8(
        provenance(),
        b"one line\n".to_vec(),
        Vec::new(),
        vec![
            GeneratedSourceMapEntry::try_new(2, "user/top", "schematic", None, None)
                .expect("entry shape"),
        ],
    );
    assert!(matches!(outside, Err(DocumentError::InvalidSourceMap(_))));
    assert!(
        GeneratedSourceMapEntry::try_new(1, "user/top", "schematic", Some(String::new()), None,)
            .is_err()
    );
}

#[test]
fn generated_source_map_inverts_to_every_line_one_component_emitted() {
    let view = "user/top/schematic";
    let amplifier = GeneratedSourceMapEntry::component_identity_for(view, 41);
    let load = GeneratedSourceMapEntry::component_identity_for(view, 42);
    assert_eq!(amplifier, "user/top/schematic/component/41");
    let entry = |line: usize, instance: &str, identity: &str| {
        GeneratedSourceMapEntry::try_new(
            line,
            "user/top",
            view,
            Some(instance.to_owned()),
            Some(identity.to_owned()),
        )
        .expect("mapping")
    };
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance"),
        b"XAMP in out amp\nRLOAD out 0 1k\n+ tc1=0\n.end\n".to_vec(),
        Vec::new(),
        // Deliberately out of source order: the artifact canonicalizes the
        // map, and the inversion has to follow the canonical order, not the
        // order a generator happened to emit.
        vec![
            entry(3, "RLOAD", &load),
            entry(1, "XAMP", &amplifier),
            entry(2, "RLOAD", &load),
        ],
    )
    .expect("canonical source map");

    assert_eq!(artifact.generated_lines_for_component(&amplifier), [1]);
    // Two cards, one component: the first is the instance's own card.
    assert_eq!(artifact.generated_lines_for_component(&load), [2, 3]);
    // Identities are matched the way every other source-map comparison
    // matches them, so a case-shifted view key still resolves.
    assert_eq!(
        artifact.generated_lines_for_component("USER/TOP/SCHEMATIC/component/41"),
        [1]
    );
    // A component the deck states nothing about has no lines, and says so
    // rather than resolving to a neighbour's card.
    assert!(
        artifact
            .generated_lines_for_component(&GeneratedSourceMapEntry::component_identity_for(
                view, 43
            ))
            .is_empty()
    );
    assert!(
        artifact
            .generated_lines_for_component("other/cell/schematic/component/41")
            .is_empty()
    );

    assert_eq!(
        artifact
            .source_map_entry(2)
            .and_then(|entry| entry.component_id()),
        Some(42)
    );
    assert_eq!(
        artifact
            .source_map_entry(4)
            .and_then(|entry| entry.component_id()),
        None
    );

    // The inversion is rebuilt from the persisted map, so a reopened project
    // answers the same question as the one that generated it.
    let round_tripped: GeneratedArtifact =
        serde_json::from_value(serde_json::to_value(&artifact).expect("serializes"))
            .expect("deserializes");
    assert_eq!(round_tripped.generated_lines_for_component(&load), [2, 3]);
}

#[test]
fn dependency_graph_rejects_missing_ambiguous_and_cyclic_edges() {
    let root_source = "root\n.include a.lib\n.end\n";
    let provenance = || {
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance")
    };
    let a_source = b"a\n.include b.lib\n";
    let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
        .resolve_utf8(a_source.to_vec())
        .expect("resolved a");

    let missing = GeneratedArtifact::try_from_utf8(
        provenance(),
        root_source.as_bytes().to_vec(),
        vec![a.clone()],
        Vec::new(),
    );
    assert!(matches!(missing, Err(DocumentError::InvalidDependency(_))));

    let ambiguous = GeneratedArtifact::try_from_utf8(
        provenance(),
        root_source.as_bytes().to_vec(),
        vec![a.clone(), a.clone()],
        Vec::new(),
    );
    assert!(matches!(
        ambiguous,
        Err(DocumentError::InvalidDependency(_))
    ));

    let b_source = b"b\n.include a.lib\n";
    let b = DependencyMetadata::unresolved_transitive(locator("a.lib"), 0, locator("b.lib"))
        .resolve_utf8(b_source.to_vec())
        .expect("resolved b");
    let back_to_a =
        DependencyMetadata::unresolved_transitive(locator("b.lib"), 0, locator("a.lib"))
            .resolve_utf8(a_source.to_vec())
            .expect("resolved a again");
    let cyclic = GeneratedArtifact::try_from_utf8(
        provenance(),
        root_source.as_bytes().to_vec(),
        vec![a, b, back_to_a],
        Vec::new(),
    );
    assert!(matches!(cyclic, Err(DocumentError::InvalidDependency(_))));
}

#[test]
fn revision_and_identity_are_stable_and_noops_do_not_create_revisions() {
    let mut document = document();
    let id = document.id();
    let current = document
        .generated_artifact()
        .expect("generated")
        .content_digest();
    let receipt = document
        .update_generated_artifact(current, generated("next\n.end\n", b"input-b"))
        .expect("transition");
    assert_eq!(receipt.document_id(), id);
    assert_eq!(document.id(), id);
    assert!(receipt.revision() > receipt.previous_revision());
    let current = document
        .generated_artifact()
        .expect("generated")
        .content_digest();
    let noop = document
        .update_generated_artifact(current, generated("next\n.end\n", b"input-b"))
        .expect("noop");
    assert_eq!(noop.revision(), receipt.revision());
}

#[test]
fn serde_round_trip_rebuilds_derived_navigation_without_losing_state() {
    let document = NetlistDocument::from_generated(
        NetlistDocumentId::new(),
        generated("Generated μ\n.include μ.lib\nRμ μ 0 1k\n.end\n", b"input-a"),
    )
    .expect("document");
    let json = serde_json::to_string(&document).expect("serialize");
    let restored: NetlistDocument = serde_json::from_str(&json).expect("restore");

    assert_eq!(restored, document);
    assert_eq!(restored.include_directives(), document.include_directives());
    assert_eq!(restored.source_bytes(), document.source_bytes());
}

#[test]
fn deserialization_rejects_tampered_active_and_generated_bytes() {
    let document = document();
    let mut value = serde_json::to_value(&document).expect("serialize");
    value["source"] = Value::String("tampered\n.end\n".to_owned());
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());

    let mut value = serde_json::to_value(&document).expect("serialize");
    value["generated_artifact"]["source"] = Value::String("tampered generated\n.end\n".to_owned());
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
}

#[test]
fn deserialization_rejects_tampered_dependency_member_bytes() {
    let root_source = "root\n.include a.lib\n.end\n";
    let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
        .resolve_utf8(b"member\n.model a r r=1k\n".to_vec())
        .expect("resolved member");
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "generator",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
        )
        .expect("provenance"),
        root_source.as_bytes().to_vec(),
        vec![a],
        Vec::new(),
    )
    .expect("artifact");
    let document =
        NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).expect("document");
    let mut value = serde_json::to_value(document).expect("serialize");
    value["dependencies"][0]["resolution"]["source"] =
        Value::String("tampered member\n".to_owned());
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
}

#[test]
fn deserialization_rejects_tampered_generated_source_map() {
    let document = document();
    let mut value = serde_json::to_value(document).expect("serialize");
    value["generated_artifact"]["source_map"][0]["generated_line"] = Value::from(999);
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
}

#[test]
fn deserialization_rejects_future_schema_and_stale_validation() {
    let mut document = document();
    document
        .acknowledge_validation(document.content_digest(), Vec::new())
        .expect("validate");
    let mut value = serde_json::to_value(&document).expect("serialize");
    value["schema_version"] = Value::from(99);
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());

    let mut value = serde_json::to_value(&document).expect("serialize");
    value["validation"]["content_digest"] = Value::String(content_digest(b"other").to_string());
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
}

#[test]
fn nil_identity_and_invalid_locator_are_rejected() {
    assert_eq!(
        NetlistDocumentId::try_from_uuid(Uuid::nil()),
        Err(DocumentError::NilDocumentIdentity)
    );
    assert!(SourceLocator::try_new("\n", "bad").is_err());
    assert!(SourceLocator::try_new("valid", "\0").is_err());
}

#[test]
fn portable_logical_identity_is_separate_from_optional_native_origin() {
    let locator = SourceLocator::try_new("models/vendor/core.lib", "core.lib")
        .expect("portable identity")
        .with_native_origin(r"C:\foundry\models\core.lib")
        .expect("native origin");
    assert_eq!(locator.logical_identity(), "models/vendor/core.lib");
    assert_eq!(locator.display_name(), "core.lib");
    assert_eq!(locator.native_origin(), Some(r"C:\foundry\models\core.lib"));
    let json = serde_json::to_string(&locator).expect("serialize");
    assert_eq!(
        serde_json::from_str::<SourceLocator>(&json).expect("restore"),
        locator
    );
}
