//! Tests for the netlist document's edit and schema rules.
//!
//! Generated source cannot be edited or saved directly, an editable
//! replacement is one atomic transition, and a document from a future schema
//! is rejected rather than partially understood.

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
    assert_eq!(document.ownership(), DocumentOwnership::Generated);
    assert!(!document.is_dirty());
    assert_eq!(document.include_directives().len(), 1);
    assert_eq!(document.dependencies().len(), 1);
    assert!(matches!(
        document.dependencies()[0].resolution(),
        DependencyResolution::Unresolved
    ));
}

#[test]
fn invalid_utf8_never_partially_mutates_a_document() {
    let mut document = document();
    let before = document.clone();
    let error = document
        .import_source(
            document.content_digest(),
            locator("bad.cir"),
            vec![b'R', 0xff],
        )
        .expect_err("invalid UTF-8");
    assert!(matches!(error, DocumentError::InvalidUtf8 { .. }));
    assert_eq!(document, before);
}

#[test]
fn imported_origin_survives_make_editable_edit_and_save_as() {
    let mut document = document();
    let imported = b"Imported deck\nR9 x 0 9k\n.end\n".to_vec();
    document
        .import_source(
            document.content_digest(),
            locator("imports/a.cir"),
            imported,
        )
        .expect("import");
    assert_eq!(document.ownership(), DocumentOwnership::Imported);
    assert!(!document.is_dirty());
    let import_origin = document
        .provenance()
        .imported()
        .expect("import provenance")
        .clone();

    document
        .make_editable(document.content_digest())
        .expect("make editable");
    document
        .replace_editable_source(
            document.content_digest(),
            b"Imported deck\nR9 x 0 10k\n.end\n".to_vec(),
        )
        .expect("edit");
    assert!(document.is_dirty());
    document
        .acknowledge_save(document.content_digest(), locator("saved/b.cir"))
        .expect("save as");

    assert_eq!(document.provenance().imported(), Some(&import_origin));
    assert_eq!(
        document
            .save_acknowledgement()
            .expect("save acknowledgement")
            .origin()
            .locator(),
        "saved/b.cir"
    );
    assert!(!document.is_dirty());
}

#[test]
fn stale_edit_save_validation_and_dependency_results_are_rejected() {
    let mut document = document();
    let generated_digest = document.content_digest();
    document.make_editable(generated_digest).expect("editable");
    document
        .replace_editable_source(
            generated_digest,
            b"edited\n.include new.lib\nR1 1 0 2k\n.end\n".to_vec(),
        )
        .expect("edit");
    let after_edit = document.clone();

    assert!(matches!(
        document.acknowledge_save(generated_digest, locator("stale.cir")),
        Err(DocumentError::ContentConflict { .. })
    ));
    assert!(matches!(
        document.acknowledge_validation(generated_digest, Vec::new()),
        Err(DocumentError::ContentConflict { .. })
    ));
    assert!(matches!(
        document.acknowledge_dependencies(generated_digest, Vec::new()),
        Err(DocumentError::ContentConflict { .. })
    ));
    assert_eq!(document, after_edit);
}

#[test]
fn generated_refresh_never_overwrites_user_owned_source() {
    let mut document = document();
    document
        .make_editable(document.content_digest())
        .expect("editable");
    let owned_source = "owned\nR7 n 0 7k\n.end\n";
    document
        .replace_editable_source(document.content_digest(), owned_source.as_bytes().to_vec())
        .expect("edit");
    let replacement = generated("new generated\nR2 n 0 2k\n.end\n", b"input-b");
    let replacement_digest = replacement.content_digest();
    document
        .update_generated_artifact(document.generated_artifact().content_digest(), replacement)
        .expect("refresh backing");

    assert_eq!(document.source(), owned_source);
    assert_eq!(document.ownership(), DocumentOwnership::Editable);
    assert_eq!(
        document.generated_artifact().content_digest(),
        replacement_digest
    );
    document
        .return_to_generated(document.content_digest())
        .expect("return generated");
    assert_eq!(document.content_digest(), replacement_digest);
    assert_eq!(document.ownership(), DocumentOwnership::Generated);
}

#[test]
fn editable_copy_is_a_distinct_unsaved_document_and_primary_is_unchanged() {
    let primary = document();
    let primary_before = primary.clone();
    let owned_id = NetlistDocumentId::try_from_uuid(
        Uuid::parse_str("9702bcf0-e52b-4eb5-bdc1-1421d11205c1").expect("UUID"),
    )
    .expect("identity");
    let owned = primary
        .create_editable_copy(owned_id, primary.content_digest())
        .expect("editable copy");

    assert_eq!(primary, primary_before);
    assert_eq!(owned.id(), owned_id);
    assert_ne!(owned.id(), primary.id());
    assert_eq!(owned.ownership(), DocumentOwnership::Editable);
    assert_eq!(owned.source_bytes(), primary.source_bytes());
    assert_eq!(
        owned.generated_artifact().content_digest(),
        primary.generated_artifact().content_digest()
    );
    assert!(owned.is_dirty());
    assert!(owned.validation().is_none());
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
fn source_changes_invalidate_validation_and_dependency_resolution() {
    let mut document = document();
    let resolved = document.dependencies()[0]
        .clone()
        .resolve_utf8(b"model member\n.model core r r=1k\n".to_vec())
        .expect("resolved dependency");
    document
        .acknowledge_dependencies(document.content_digest(), vec![resolved])
        .expect("dependency acknowledgement");
    document.validate_syntax().expect("syntax validation");
    assert!(document.validation().is_some());

    document
        .make_editable(document.content_digest())
        .expect("editable");
    document
        .replace_editable_source(
            document.content_digest(),
            b"changed\n.include other.lib\nR1 1 0 1k\n.end\n".to_vec(),
        )
        .expect("edit");
    assert!(document.validation().is_none());
    assert!(matches!(
        document.dependencies()[0].resolution(),
        DependencyResolution::Unresolved
    ));
    assert_eq!(document.dependencies()[0].locator().locator(), "other.lib");
}

#[test]
fn syntax_validation_is_bound_to_exact_content() {
    let mut document = document();
    document
        .make_editable(document.content_digest())
        .expect("editable");
    document
        .replace_editable_source(
            document.content_digest(),
            b"broken\nR1 only-one-node\n.end\n".to_vec(),
        )
        .expect("edit");
    document.validate_syntax().expect("validation transaction");
    let report = document.validation().expect("report");
    assert_eq!(report.content_digest(), document.content_digest());
    assert!(!report.is_valid());
    assert_eq!(report.error_count(), 1);
    assert!(report.diagnostics()[0].position().line() >= 1);
}

#[test]
fn diagnostic_positions_are_unicode_scalar_columns_and_checked() {
    let mut document = document();
    document
        .make_editable(document.content_digest())
        .expect("editable");
    document
        .replace_editable_source(
            document.content_digest(),
            "unicode\nRμ μ 0 1k\n.end\n".as_bytes().to_vec(),
        )
        .expect("edit");
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
fn dependency_metadata_requires_exact_direct_relationships() {
    let mut document = document();
    let wrong = DependencyMetadata::unresolved_direct(0, locator("models/wrong.lib"));
    let before = document.clone();
    assert!(matches!(
        document.acknowledge_dependencies(document.content_digest(), vec![wrong]),
        Err(DocumentError::InvalidDependency(_))
    ));
    assert_eq!(document, before);

    let parent = locator("models/core.lib");
    let resolved_parent = document.dependencies()[0]
        .clone()
        .resolve_utf8(b"core models\n.include models/transistors.lib\n".to_vec())
        .expect("resolved parent");
    let child =
        DependencyMetadata::unresolved_transitive(parent, 0, locator("models/transistors.lib"))
            .with_resolution(DependencyResolution::Missing {
                reason: "not present in sealed source bundle".to_owned(),
            })
            .expect("missing dependency");
    document
        .acknowledge_validation(document.content_digest(), Vec::new())
        .expect("validation receipt");
    assert!(document.validation().is_some());
    document
        .acknowledge_dependencies(document.content_digest(), vec![resolved_parent, child])
        .expect("valid transitive metadata");
    assert_eq!(document.dependencies().len(), 2);
    assert!(
        document.validation().is_none(),
        "dependency identity changes must revoke source validation"
    );
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
    let initial_revision = document.revision();
    let receipt = document
        .return_to_generated(document.content_digest())
        .expect("no-op return");
    assert_eq!(receipt.previous_revision(), initial_revision);
    assert_eq!(receipt.revision(), initial_revision);
    assert_eq!(document.revision(), initial_revision);

    let receipt = document
        .make_editable(document.content_digest())
        .expect("transition");
    assert_eq!(receipt.document_id(), id);
    assert_eq!(document.id(), id);
    assert_eq!(receipt.previous_ownership(), DocumentOwnership::Generated);
    assert_eq!(receipt.ownership(), DocumentOwnership::Editable);
    assert!(receipt.revision() > receipt.previous_revision());
}

#[test]
fn serde_round_trip_rebuilds_derived_navigation_without_losing_state() {
    let mut document = document();
    document
        .import_source(
            document.content_digest(),
            locator("imports/unicode.cir"),
            "Imported μ\n.include μ.lib\nRμ μ 0 1k\n.end\n"
                .as_bytes()
                .to_vec(),
        )
        .expect("import");
    document
        .make_editable(document.content_digest())
        .expect("editable");
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
    document.validate_syntax().expect("validate");
    let mut value = serde_json::to_value(&document).expect("serialize");
    value["schema_version"] = Value::from(99);
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());

    let mut value = serde_json::to_value(&document).expect("serialize");
    value["validation"]["content_digest"] = Value::String(content_digest(b"other").to_string());
    assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
}

#[test]
fn generated_source_cannot_be_saved_or_edited_directly() {
    let mut document = document();
    assert_eq!(
        document
            .acknowledge_save(document.content_digest(), locator("out.cir"))
            .expect_err("read only"),
        DocumentError::GeneratedSourceIsReadOnly
    );
    assert!(matches!(
        document.replace_editable_source(document.content_digest(), b"replacement".to_vec()),
        Err(DocumentError::SourceIsNotEditable(
            DocumentOwnership::Generated
        ))
    ));
    assert!(matches!(
        document.replace_editable_matches(
            document.content_digest(),
            "R1",
            "R2",
            FindOptions::default(),
            ReplaceScope::All,
        ),
        Err(DocumentError::SourceIsNotEditable(
            DocumentOwnership::Generated
        ))
    ));
}

#[test]
fn editable_regex_replacement_is_one_atomic_source_transition() {
    let mut document = document();
    document
        .make_editable(document.content_digest())
        .expect("editable");
    let before_revision = document.revision();
    let before_digest = document.content_digest();
    let (receipt, outcome) = document
        .replace_editable_matches(
            before_digest,
            r"R(?P<number>\d+)",
            "X${number}",
            FindOptions {
                regular_expression: true,
                ..FindOptions::default()
            },
            ReplaceScope::All,
        )
        .expect("replace");
    assert_eq!(outcome.replacement_count(), 1);
    assert!(document.source().contains("X1 in 0 1k"));
    assert_eq!(receipt.previous_revision(), before_revision);
    assert_eq!(receipt.previous_digest(), before_digest);
    assert_eq!(receipt.content_digest(), document.content_digest());
    assert!(receipt.revision() > before_revision);
    assert!(document.validation().is_none());
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
