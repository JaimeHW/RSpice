//! Tests for source ownership across copy, rename, and delete.
//!
//! The cases pin that identity is canonical for lookup, that a collision rolls
//! back every mutation it touched, and that no operation leaves stale evidence
//! pointing at a source that has moved.

use super::*;

fn dependency_file(path: &str, content: &str) -> ProjectSourceFile {
    ProjectSourceFile::try_new(path, content).unwrap()
}

fn dependency(importer: &str, imported: &str) -> ProjectSourceDependency {
    ProjectSourceDependency::try_new(importer, imported).unwrap()
}

fn bundle_fixture() -> ProjectSourceBundle {
    ProjectSourceBundle::try_new_with_id(
        "6d3c50a8-dd02-46da-beca-160517eea8a9".parse().unwrap(),
        ProjectSourceOwner::cell_view(CellViewRef::new("user", "ota", "veriloga")),
        ProjectSourceLanguage::VerilogA,
        "models/ota.va",
        "`include \"shared/constants.vams\"\nmodule ota; endmodule\r\n",
        [
            dependency_file("shared/constants.vams", "`define VT 0.026\n"),
            dependency_file("shared/physics.vams", "// δ exact UTF-8\n"),
        ],
        [
            dependency("models/ota.va", "shared/constants.vams"),
            dependency("shared/constants.vams", "shared/physics.vams"),
        ],
    )
    .unwrap()
}

fn owned_bundle(library: &str, cell: &str, view: &str) -> ProjectSourceBundle {
    ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(CellViewRef::new(library, cell, view)),
        ProjectSourceLanguage::VerilogA,
        format!("{cell}.va"),
        "module device; endmodule\n",
        [],
        [],
    )
    .unwrap()
}

#[test]
fn canonical_round_trip_preserves_identity_owner_graph_and_exact_bytes() {
    let bundle = bundle_fixture();
    let digest = bundle.closure_digest();
    let registry = ProjectSourceRegistry::try_from_bundles([bundle.clone()]).unwrap();

    let encoded = serde_json::to_vec(&registry).unwrap();
    let restored: ProjectSourceRegistry = serde_json::from_slice(&encoded).unwrap();
    let restored_bundle = restored.get_bundle(bundle.id()).unwrap();

    assert_eq!(restored, registry);
    assert_eq!(restored_bundle.owner(), bundle.owner());
    assert_eq!(restored_bundle.closure_digest(), digest);
    assert_eq!(
        restored_bundle.root().exact_bytes(),
        bundle.root().exact_bytes()
    );
    assert_eq!(
        restored_bundle.files()[1].exact_bytes(),
        "// δ exact UTF-8\n".as_bytes()
    );
}

#[test]
fn closure_digest_is_order_independent_after_canonical_construction_and_length_framed() {
    let first = bundle_fixture();
    let second = ProjectSourceBundle::try_new_with_id(
        first.id(),
        first.owner().clone(),
        first.language(),
        first.root().logical_path(),
        first.root().content(),
        first.files().iter().cloned().rev(),
        first.dependencies().iter().cloned().rev(),
    )
    .unwrap();
    assert_eq!(first.closure_digest(), second.closure_digest());

    let split_a = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "a.va",
        "bc",
        [],
        [],
    )
    .unwrap();
    let split_b = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "ab.va",
        "c",
        [],
        [],
    )
    .unwrap();
    assert_ne!(split_a.closure_digest(), split_b.closure_digest());
}

#[test]
fn edits_advance_bundle_revision_and_invalidate_root_and_closure_evidence() {
    let mut bundle = bundle_fixture();
    let first = bundle.mark_validated().unwrap();
    assert!(bundle.validation_is_current());
    assert!(bundle.root().validation_is_current());

    assert!(
        bundle
            .replace_file_content("SHARED/PHYSICS.VAMS", "changed\r\n".to_owned())
            .unwrap()
    );
    assert_eq!(bundle.revision().get(), 2);
    assert!(bundle.validated_identity().is_none());
    assert!(bundle.root().validated_identity().is_none());
    assert_ne!(bundle.closure_digest(), first.closure_digest());

    let revision = bundle.revision();
    assert!(
        !bundle
            .replace_file_content("shared/physics.vams", "changed\r\n".to_owned())
            .unwrap()
    );
    assert_eq!(bundle.revision(), revision);
}

#[test]
fn authoring_add_rename_and_delete_keep_source_and_dependency_graph_atomic() {
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "models/root.va",
        "module root; endmodule\n",
        [],
        [],
    )
    .unwrap();
    bundle.mark_validated().unwrap();
    let initial_revision = bundle.revision();

    assert!(
        bundle
            .add_file(
                "models/root.va",
                dependency_file("shared/constants.vams", "`define GAIN 2\n"),
            )
            .unwrap()
    );
    assert!(bundle.revision() > initial_revision);
    assert!(!bundle.validation_is_current());
    assert!(
        bundle
            .root()
            .content()
            .starts_with("`include \"shared/constants.vams\"\n")
    );
    assert_eq!(
        bundle.dependencies(),
        [dependency("models/root.va", "shared/constants.vams")]
    );

    assert!(
        bundle
            .rename_file("shared/constants.vams", "shared/physical.vams")
            .unwrap()
    );
    assert!(bundle.contains_file("shared/physical.vams"));
    assert!(!bundle.contains_file("shared/constants.vams"));
    assert!(
        bundle
            .root()
            .content()
            .starts_with("`include \"shared/physical.vams\"\n")
    );
    assert_eq!(
        bundle.dependencies(),
        [dependency("models/root.va", "shared/physical.vams")]
    );

    assert!(bundle.remove_file("shared/physical.vams").unwrap());
    assert_eq!(bundle.root().content(), "module root; endmodule\n");
    assert!(bundle.files().is_empty());
    assert!(bundle.dependencies().is_empty());
    bundle.validate().unwrap();
}

#[test]
fn authoring_rejects_invalid_or_cascading_edits_without_partial_mutation() {
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "`include \"a.vams\"\nmodule root; endmodule\n",
        [
            dependency_file("a.vams", "`include \"b.vams\"\n"),
            dependency_file("b.vams", "// leaf\n"),
        ],
        [
            dependency("root.va", "a.vams"),
            dependency("a.vams", "b.vams"),
        ],
    )
    .unwrap();
    let before = bundle.clone();
    assert!(matches!(
        bundle.remove_file("a.vams"),
        Err(ProjectSourceError::FileHasDependencies { .. })
    ));
    assert_eq!(bundle, before);

    assert!(bundle.rename_file("b.vams", "../escape.vams").is_err());
    assert_eq!(bundle, before);
    assert!(matches!(
        bundle.remove_file("root.va"),
        Err(ProjectSourceError::CannotRemoveBundleRoot { .. })
    ));
    assert_eq!(bundle, before);
}

#[test]
fn root_rename_preserves_explicit_root_ownership_and_outgoing_edges() {
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "models/root.va",
        "`include \"shared/defs.vams\"\nmodule root; endmodule\n",
        [dependency_file("shared/defs.vams", "// definitions\n")],
        [dependency("models/root.va", "shared/defs.vams")],
    )
    .unwrap();

    assert!(
        bundle
            .rename_file("models/root.va", "models/renamed.va")
            .unwrap()
    );
    assert_eq!(bundle.root().logical_path(), "models/renamed.va");
    assert_eq!(
        bundle.dependencies(),
        [dependency("models/renamed.va", "shared/defs.vams")]
    );
    bundle.validate().unwrap();
}

#[test]
fn paths_reject_absolute_traversal_backslashes_nul_and_case_duplicates() {
    for path in [
        "/absolute/model.va",
        "C:/absolute/model.va",
        "../model.va",
        "models/../model.va",
        "models\\model.va",
        "models//model.va",
    ] {
        assert!(ProjectSourceFile::try_new(path, "x").is_err(), "{path}");
    }
    assert!(matches!(
        ProjectSourceFile::try_new("models/a.vams", "a\0b"),
        Err(ProjectSourceError::NulInContent { .. })
    ));
    let result = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "models/root.va",
        "root",
        [dependency_file("MODELS/ROOT.VA", "duplicate")],
        [],
    );
    assert!(matches!(
        result,
        Err(ProjectSourceError::DuplicateLogicalPath { .. })
    ));
}

#[test]
fn dependency_graph_rejects_missing_cycles_and_unreachable_files() {
    let missing = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "root",
        [],
        [dependency("root.va", "missing.vams")],
    );
    assert!(matches!(
        missing,
        Err(ProjectSourceError::MissingDependencyEndpoint { .. })
    ));

    let cyclic = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "root",
        [dependency_file("a.vams", "a")],
        [
            dependency("root.va", "a.vams"),
            dependency("a.vams", "root.va"),
        ],
    );
    assert!(matches!(
        cyclic,
        Err(ProjectSourceError::DependencyCycle { .. })
    ));

    let unreachable = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "root",
        [dependency_file("orphan.vams", "orphan")],
        [],
    );
    assert!(matches!(
        unreachable,
        Err(ProjectSourceError::UnreachableFile { .. })
    ));
}

#[test]
fn closure_bounds_are_enforced() {
    let files = (0..MAX_PROJECT_SOURCE_FILES)
        .map(|index| dependency_file(&format!("deps/{index:04}.vams"), ""))
        .collect::<Vec<_>>();
    assert!(matches!(
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
            "root.va",
            "root",
            files,
            [],
        ),
        Err(ProjectSourceError::TooManyFiles { .. })
    ));

    let too_long = "a".repeat(MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES + 1);
    assert!(matches!(
        ProjectSourceFile::try_new(too_long, ""),
        Err(ProjectSourceError::LogicalPathTooLong { .. })
    ));
}

#[test]
fn legacy_singletons_migrate_deterministically_and_keep_accessors() {
    let source = ProjectSourceDocument::try_new(
        "sensor.va",
        ProjectSourceLanguage::VerilogA,
        "module sensor; endmodule\r\n",
    )
    .unwrap();
    let legacy = serde_json::json!({ "verilog_a": source, "automation": null });
    let first: ProjectSourceRegistry = serde_json::from_value(legacy.clone()).unwrap();
    let second: ProjectSourceRegistry = serde_json::from_value(legacy).unwrap();

    assert_eq!(
        first.iter_bundles().next().unwrap().id(),
        second.iter_bundles().next().unwrap().id()
    );
    assert_eq!(
        first
            .get(ProjectSourceLanguage::VerilogA)
            .unwrap()
            .content(),
        "module sensor; endmodule\r\n"
    );
    let canonical = serde_json::to_value(&first).unwrap();
    assert_eq!(
        canonical["schema_version"],
        PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION
    );
    assert!(canonical.get("verilog_a").is_none());
}

#[test]
fn persisted_corruption_fails_closed_during_deserialization() {
    let mut bundle = bundle_fixture();
    bundle.mark_validated().unwrap();
    let registry = ProjectSourceRegistry::try_from_bundles([bundle]).unwrap();
    let mut value = serde_json::to_value(&registry).unwrap();
    value["bundles"][0]["files"][0]["content"] = serde_json::json!("tampered");
    assert!(serde_json::from_value::<ProjectSourceRegistry>(value).is_err());

    let mut value = serde_json::to_value(&registry).unwrap();
    value["bundles"][0]["id"] = serde_json::json!(Uuid::nil());
    assert!(serde_json::from_value::<ProjectSourceRegistry>(value).is_err());
}

#[test]
fn cell_view_sources_follow_copy_rename_and_delete_without_stale_evidence() {
    let mut original = bundle_fixture();
    original.mark_validated().unwrap();
    let original_id = original.id();
    let mut registry = ProjectSourceRegistry::try_from_bundles([original]).unwrap();

    let copied = registry
        .clone_cell_view_bundles("user", "ota", "user", "ota_copy")
        .unwrap();
    assert_eq!(copied.len(), 1);
    assert_ne!(copied[0], original_id);
    let copied_owner =
        ProjectSourceOwner::cell_view(CellViewRef::new("user", "ota_copy", "veriloga"));
    let copied_bundle = registry.bundle_for_owner(&copied_owner).unwrap();
    assert!(!copied_bundle.validation_is_current());
    assert_eq!(
        copied_bundle.root().content(),
        bundle_fixture().root().content()
    );

    let before_revision = registry.get_bundle(original_id).unwrap().revision();
    let renamed = registry
        .rename_cell_view_bundles("user", "ota", "ota_renamed")
        .unwrap();
    assert_eq!(renamed, [original_id]);
    let renamed_owner =
        ProjectSourceOwner::cell_view(CellViewRef::new("user", "ota_renamed", "veriloga"));
    let renamed_bundle = registry.bundle_for_owner(&renamed_owner).unwrap();
    assert_eq!(renamed_bundle.id(), original_id);
    assert!(renamed_bundle.revision() > before_revision);
    assert!(!renamed_bundle.validation_is_current());

    assert_eq!(
        registry.remove_cell_view_bundles("user", "ota_renamed", Some("veriloga")),
        [original_id]
    );
    assert!(registry.get_bundle(original_id).is_none());
    assert!(registry.bundle_for_owner(&copied_owner).is_some());
}

#[test]
fn accented_owner_identity_is_canonical_for_uniqueness_lookup_and_scope_operations() {
    assert_eq!(
        canonical_cell_view_owner_key("Biblioth\u{e8}que", "\u{c9}tage", "Mod\u{e8}le"),
        canonical_cell_view_owner_key("BIBLIOTH\u{c8}QUE", "E\u{301}TAGE", "MOD\u{c8}LE",)
    );
    let original = owned_bundle("Biblioth\u{e8}que", "\u{c9}tage", "Mod\u{e8}le");
    let original_id = original.id();
    let mut registry = ProjectSourceRegistry::try_from_bundles([original]).unwrap();

    let decomposed_alias = ProjectSourceOwner::cell_view(CellViewRef::new(
        "BIBLIOTH\u{c8}QUE",
        "E\u{301}TAGE",
        "MOD\u{c8}LE",
    ));
    assert_eq!(
        registry
            .bundle_for_owner(&decomposed_alias)
            .expect("accented case alias resolves")
            .id(),
        original_id
    );

    let before_duplicate = registry.clone();
    let duplicate = owned_bundle("BIBLIOTH\u{c8}QUE", "\u{e9}TAGE", "MOD\u{c8}LE");
    assert!(matches!(
        registry.insert_bundle(duplicate),
        Err(ProjectSourceError::DuplicateOwner { .. })
    ));
    assert_eq!(registry, before_duplicate);

    let copied = registry
        .clone_cell_view_bundles("BIBLIOTH\u{c8}QUE", "e\u{301}TAGE", "work", "copie")
        .unwrap();
    assert_eq!(copied.len(), 1);
    let renamed = registry
        .rename_cell_view_bundles("biblioth\u{e8}que", "E\u{301}TAGE", "renomm\u{e9}")
        .unwrap();
    assert_eq!(renamed, [original_id]);
    assert_eq!(
        registry.remove_cell_view_bundles(
            "BIBLIOTH\u{c8}QUE",
            "RENOMME\u{301}",
            Some("mod\u{e8}le"),
        ),
        [original_id]
    );

    let retained = [CellViewRef::new("WORK", "COPIE", "mod\u{e8}le")];
    assert!(registry.retain_cell_view_bundles_for(retained).is_empty());
    assert!(registry.get_bundle(copied[0]).is_some());
}

#[test]
fn canonical_copy_and_rename_collisions_roll_back_every_source_mutation() {
    let source_behavior = owned_bundle("work", "\u{c9}tage", "behavior");
    let source_model = owned_bundle("work", "\u{c9}tage", "model");
    let occupied_target = owned_bundle("WORK", "CIBLE", "MODEL");
    let mut registry =
        ProjectSourceRegistry::try_from_bundles([source_behavior, source_model, occupied_target])
            .unwrap();

    let before_copy = registry.clone();
    assert!(matches!(
        registry.clone_cell_view_bundles("WORK", "e\u{301}TAGE", "work", "cible"),
        Err(ProjectSourceError::DuplicateOwner { .. })
    ));
    assert_eq!(registry, before_copy);

    let before_rename = registry.clone();
    assert!(matches!(
        registry.rename_cell_view_bundles("WORK", "e\u{301}TAGE", "cible"),
        Err(ProjectSourceError::DuplicateOwner { .. })
    ));
    assert_eq!(registry, before_rename);
}

#[test]
fn veriloga_runtime_keys_and_aliases_are_deterministic_and_identity_complete() {
    let project = "2af7bdb9-1843-4b07-9524-d783567e015b"
        .parse::<ProjectId>()
        .unwrap();
    let bundle = bundle_fixture();
    let key = project_veriloga_bundle_source_key(project, &bundle, "ota_core").unwrap();
    let alias = project_veriloga_bundle_alias(&bundle, "ota_core").unwrap();

    assert_eq!(
        key,
        project_veriloga_bundle_source_key(project, &bundle, "ota_core").unwrap()
    );
    assert_eq!(
        alias,
        project_veriloga_bundle_alias(&bundle, "ota_core").unwrap()
    );
    assert!(key.starts_with(&format!("__rspice_project__/{project}/")));
    assert!(key.contains(&bundle.id().to_string()));
    assert!(key.contains(&bundle.closure_digest().to_string()));
    assert!(key.ends_with("/models/ota.va"));
    assert!(
        alias
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    );
    assert!(
        alias
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic())
    );
}

#[test]
fn veriloga_runtime_identities_separate_project_bundle_and_selected_module() {
    let first_project = ProjectId::new();
    let second_project = ProjectId::new();
    let first_bundle = bundle_fixture();
    let mut second_bundle = bundle_fixture();
    second_bundle.id = ProjectSourceId::new();

    let first_key =
        project_veriloga_bundle_source_key(first_project, &first_bundle, "ota_core").unwrap();
    assert_ne!(
        first_key,
        project_veriloga_bundle_source_key(second_project, &first_bundle, "ota_core").unwrap()
    );
    assert_ne!(
        first_key,
        project_veriloga_bundle_source_key(first_project, &second_bundle, "ota_core").unwrap()
    );
    assert_ne!(
        first_key,
        project_veriloga_bundle_source_key(first_project, &first_bundle, "ota_aux").unwrap()
    );
    assert_ne!(
        project_veriloga_bundle_alias(&first_bundle, "ota_core").unwrap(),
        project_veriloga_bundle_alias(&second_bundle, "ota_core").unwrap()
    );
    assert_ne!(
        project_veriloga_bundle_alias(&first_bundle, "ota_core").unwrap(),
        project_veriloga_bundle_alias(&first_bundle, "ota_aux").unwrap()
    );
    assert!(matches!(
        project_veriloga_bundle_alias(&first_bundle, "invalid-module"),
        Err(ProjectSourceError::InvalidModuleName { .. })
    ));
}
