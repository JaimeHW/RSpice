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

fn role(path: &str, source_role: ProjectSourceRole) -> ProjectSourceRoleBinding {
    ProjectSourceRoleBinding::try_new(path, source_role).unwrap()
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
fn revision_history_round_trips_and_restores_as_a_new_monotonic_revision() {
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "models/root.va",
        "module root; endmodule\n",
        [],
        [],
    )
    .unwrap();
    let initial_revision = bundle.revision();
    let initial_digest = bundle.closure_digest();
    bundle
        .replace_file_content(
            "models/root.va",
            "module root; real changed; endmodule\n".to_owned(),
        )
        .unwrap();
    bundle
        .add_file(
            "models/root.va",
            dependency_file("models/helper.vams", "`define VALUE 1\n"),
        )
        .unwrap();
    assert_eq!(bundle.revision().get(), 3);
    assert_eq!(bundle.revision_history().len(), 2);
    assert_eq!(
        bundle
            .revision_snapshot(initial_revision)
            .unwrap()
            .closure_digest(),
        initial_digest
    );

    let encoded = serde_json::to_vec(&bundle).unwrap();
    let mut restored: ProjectSourceBundle = serde_json::from_slice(&encoded).unwrap();
    restored.validate().unwrap();
    assert!(
        restored
            .restore_revision(restored.revision(), initial_revision)
            .unwrap()
    );
    assert_eq!(restored.revision().get(), 4);
    assert_eq!(restored.root().content(), "module root; endmodule\n");
    assert!(!restored.contains_file("models/helper.vams"));
    assert_eq!(restored.revision_history().len(), 3);
    assert!(matches!(
        restored.restore_revision(
            rspice_vocabulary::product::ObjectRevision::new(3).unwrap(),
            initial_revision,
        ),
        Err(ProjectSourceError::StaleRevisionRestore { .. })
    ));
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
    let dependency_id = bundle.document_id("shared/constants.vams").unwrap();
    let dependency_revision = bundle.document_revision("shared/constants.vams").unwrap();

    assert!(
        bundle
            .rename_file("shared/constants.vams", "shared/physical.vams")
            .unwrap()
    );
    assert!(bundle.contains_file("shared/physical.vams"));
    assert!(!bundle.contains_file("shared/constants.vams"));
    assert_eq!(
        bundle.document_id("shared/physical.vams"),
        Some(dependency_id)
    );
    assert!(bundle.document_revision("shared/physical.vams").unwrap() > dependency_revision);
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
fn automation_authoring_uses_persisted_roles_without_veriloga_include_injection() {
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
        ProjectSourceLanguage::RSpiceAutomation,
        "flows/main.py",
        "from helpers import label\nprint(label())\n",
        [],
        [],
    )
    .unwrap();
    let entry_before = bundle.root().content().to_owned();

    for (path, content) in [
        ("flows/helpers.py", "def label():\n    return 'tt'\n"),
        ("plans/signoff.data", "schema: rspice.run-plan/v1\n"),
        (
            "environment/pinned.toml",
            "format = \"rspice-python-lock/v1\"\n",
        ),
        (
            "security/project-policy.toml",
            "project_files = \"read\"\nnetwork = \"deny\"\n",
        ),
    ] {
        bundle
            .add_file("flows/main.py", dependency_file(path, content))
            .unwrap();
    }
    assert_eq!(bundle.root().content(), entry_before);
    assert!(!bundle.root().content().contains("`include"));

    for (path, role) in [
        ("plans/signoff.data", ProjectSourceRole::AutomationRunPlan),
        (
            "environment/pinned.toml",
            ProjectSourceRole::AutomationEnvironmentLock,
        ),
        (
            "security/project-policy.toml",
            ProjectSourceRole::AutomationPermissionManifest,
        ),
    ] {
        assert!(bundle.set_non_entry_role(path, Some(role)).unwrap());
    }
    let digest_before_rename = bundle.closure_digest();

    bundle
        .rename_file("plans/signoff.data", "plans/release.data")
        .unwrap();
    assert_eq!(
        bundle.role_for_path("plans/release.data"),
        Some(ProjectSourceRole::AutomationRunPlan)
    );
    assert_ne!(bundle.closure_digest(), digest_before_rename);
    assert!(matches!(
        bundle.remove_file("plans/release.data"),
        Err(ProjectSourceError::RoleBoundFile { .. })
    ));
    assert!(
        bundle
            .set_non_entry_role("plans/release.data", None)
            .unwrap()
    );
    assert!(bundle.remove_file("plans/release.data").unwrap());
    assert_eq!(bundle.root().content(), entry_before);
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
fn multi_file_replacement_is_atomic_and_rejects_duplicate_paths() {
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "`include \"defs.vams\"\nmodule root; endmodule\n",
        [dependency_file("defs.vams", "// old definitions\n")],
        [dependency("root.va", "defs.vams")],
    )
    .unwrap();
    let bundle_id = bundle.id();
    let original_revision = bundle.revision();
    let original_root = bundle.root().content().to_owned();
    let original_dependency = bundle.file_content("defs.vams").unwrap().to_owned();
    let mut registry = ProjectSourceRegistry::default();
    registry.insert_bundle(bundle).unwrap();

    let changed = registry
        .replace_bundle_files_transactionally(
            bundle_id,
            [
                (
                    "root.va".to_owned(),
                    "`include \"defs.vams\"\nmodule updated; endmodule\n".to_owned(),
                ),
                ("defs.vams".to_owned(), "// new definitions\n".to_owned()),
            ],
        )
        .unwrap();
    assert_eq!(changed, 2);
    let committed = registry.get_bundle(bundle_id).unwrap();
    assert_eq!(committed.revision().get(), original_revision.get() + 1);
    assert_eq!(committed.revision_history().len(), 1);
    assert_eq!(
        committed.revision_history()[0].revision(),
        original_revision
    );
    assert!(committed.root().content().contains("module updated"));
    assert_eq!(
        committed.file_content("defs.vams"),
        Some("// new definitions\n")
    );

    let edited_revision = committed.revision();
    assert!(
        registry
            .restore_bundle_revision(bundle_id, edited_revision, original_revision)
            .unwrap()
    );
    let restored = registry.get_bundle(bundle_id).unwrap();
    assert_eq!(restored.root().content(), original_root);
    assert_eq!(
        restored.file_content("defs.vams"),
        Some(original_dependency.as_str())
    );
    assert!(restored.revision().get() > edited_revision.get());

    let before = registry.clone();
    assert!(matches!(
        registry.replace_bundle_files_transactionally(
            bundle_id,
            [
                ("root.va".to_owned(), "module first; endmodule\n".to_owned()),
                (
                    "ROOT.VA".to_owned(),
                    "module second; endmodule\n".to_owned()
                ),
            ],
        ),
        Err(ProjectSourceError::DuplicateLogicalPath { .. })
    ));
    assert_eq!(registry, before);

    assert!(
        registry
            .replace_bundle_files_transactionally(
                bundle_id,
                [
                    (
                        "root.va".to_owned(),
                        "module partial; endmodule\n".to_owned()
                    ),
                    (
                        "missing.va".to_owned(),
                        "module missing; endmodule\n".to_owned()
                    ),
                ],
            )
            .is_err()
    );
    assert_eq!(registry, before);
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
fn closure_accepts_the_exact_ten_thousand_file_contract_limit() {
    let files = (0..MAX_PROJECT_SOURCE_FILES - 1)
        .map(|index| dependency_file(&format!("deps/{index:04}.vams"), ""))
        .collect::<Vec<_>>();
    let dependencies = (0..MAX_PROJECT_SOURCE_FILES - 1)
        .map(|index| dependency("root.va", &format!("deps/{index:04}.vams")))
        .collect::<Vec<_>>();
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "root.va",
        "root",
        files,
        dependencies,
    )
    .unwrap();
    assert_eq!(bundle.files().len() + 1, MAX_PROJECT_SOURCE_FILES);
    bundle.validate().unwrap();
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
fn schema_v1_automation_role_heuristics_are_one_time_and_persist_as_schema_v2() {
    let bundle = ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
        ProjectSourceLanguage::RSpiceAutomation,
        "flows/main.py",
        "print('ok')\n",
        [
            dependency_file("plans/release.yaml", "schema: rspice.run-plan/v1\n"),
            dependency_file(
                "environment/pinned.toml",
                "format = \"rspice-python-lock/v1\"\n",
            ),
            dependency_file(
                "security/policy.toml",
                "project_files = \"read\"\nnetwork = \"deny\"\n",
            ),
        ],
        [
            dependency("flows/main.py", "plans/release.yaml"),
            dependency("flows/main.py", "environment/pinned.toml"),
            dependency("flows/main.py", "security/policy.toml"),
        ],
        [role("flows/main.py", ProjectSourceRole::AutomationEntry)],
    )
    .unwrap();
    let registry = ProjectSourceRegistry::try_from_bundles([bundle]).unwrap();
    let mut persisted = serde_json::to_value(registry).unwrap();
    persisted["schema_version"] = serde_json::json!(1);
    persisted["bundles"][0]
        .as_object_mut()
        .unwrap()
        .remove("roles");
    persisted["bundles"][0]["root"]
        .as_object_mut()
        .unwrap()
        .remove("id");
    for file in persisted["bundles"][0]["files"].as_array_mut().unwrap() {
        file.as_object_mut().unwrap().remove("id");
    }

    let migrated: ProjectSourceRegistry = serde_json::from_value(persisted).unwrap();
    let migrated = migrated
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .unwrap();
    assert_eq!(
        migrated.role_for_path("flows/main.py"),
        Some(ProjectSourceRole::AutomationEntry)
    );
    assert_eq!(
        migrated.role_for_path("plans/release.yaml"),
        Some(ProjectSourceRole::AutomationRunPlan)
    );
    assert_eq!(
        migrated.role_for_path("environment/pinned.toml"),
        Some(ProjectSourceRole::AutomationEnvironmentLock)
    );
    assert_eq!(
        migrated.role_for_path("security/policy.toml"),
        Some(ProjectSourceRole::AutomationPermissionManifest)
    );
    let canonical =
        serde_json::to_value(ProjectSourceRegistry::try_from_bundles([migrated.clone()]).unwrap())
            .unwrap();
    assert_eq!(
        canonical["schema_version"],
        PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION
    );
    assert!(canonical["bundles"][0]["roles"].is_array());
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

    let mut value = serde_json::to_value(&registry).unwrap();
    value["bundles"][0]["root"]["id"] = serde_json::json!(Uuid::nil());
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
    assert_ne!(
        copied_bundle.root().id(),
        registry.get_bundle(original_id).unwrap().root().id()
    );
    for copied_file in copied_bundle.files() {
        let original_file = registry
            .get_bundle(original_id)
            .unwrap()
            .files()
            .iter()
            .find(|file| file.logical_path() == copied_file.logical_path())
            .unwrap();
        assert_ne!(copied_file.id(), original_file.id());
    }

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
    // A deck names the module the engineer wrote, with a short tag beside it.
    let tag = alias
        .strip_prefix("ota_core_")
        .unwrap_or_else(|| panic!("the alias leads with the module: {alias}"));
    assert_eq!(tag.len(), 8, "{alias}");
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

/// The project-wide Verilog-A bundle a v1 registry carried, as the legacy
/// singleton field it was persisted in.
fn legacy_veriloga_registry() -> ProjectSourceRegistry {
    let source = ProjectSourceDocument::try_new(
        "sensor.va",
        ProjectSourceLanguage::VerilogA,
        "module sensor(p, n); inout p, n; electrical p, n; endmodule\n",
    )
    .unwrap();
    serde_json::from_value(serde_json::json!({ "verilog_a": source, "automation": null })).unwrap()
}

/// A Code Workspace bundle given to a cell view is the same source: same
/// identity, same bytes, same validation. Only the owner and the revision
/// that records the move are new, and the move cannot happen twice.
#[test]
fn a_code_workspace_bundle_moves_to_a_cell_view_keeping_its_identity_and_validation() {
    let mut registry = legacy_veriloga_registry();
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
    let before = registry.bundle_for_owner(&owner).unwrap().id();
    registry.mark_bundle_validated(before).unwrap();
    let original = registry.get_bundle(before).unwrap().clone();
    let reference = CellViewRef::new("work", "sensor", "veriloga");

    let moved = registry
        .move_code_workspace_bundle_to_cell_view(ProjectSourceLanguage::VerilogA, reference.clone())
        .unwrap();

    assert_eq!(moved, Some(before));
    assert!(registry.bundle_for_owner(&owner).is_none());
    let bundle = registry
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .unwrap();
    assert_eq!(bundle.id(), before);
    assert_eq!(bundle.root().content(), original.root().content());
    assert!(bundle.revision() > original.revision());
    assert_ne!(bundle.closure_digest(), original.closure_digest());
    assert!(bundle.validation_is_current());
    assert!(bundle.root().validation_is_current());
    assert_eq!(
        bundle
            .revision_history()
            .last()
            .map(ProjectSourceRevisionSnapshot::closure_digest),
        Some(original.closure_digest())
    );
    assert_eq!(
        registry
            .move_code_workspace_bundle_to_cell_view(ProjectSourceLanguage::VerilogA, reference)
            .unwrap(),
        None
    );
}

/// A key written before the move still names the bundle and still names its
/// bytes, so it reads back as the current key. Once the bytes change it names
/// nothing current, though it still names whose source it was.
#[test]
fn a_key_written_before_a_move_reads_back_as_the_current_key_until_the_bytes_change() {
    let project = ProjectId::new();
    let mut registry = legacy_veriloga_registry();
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
    let id = registry.bundle_for_owner(&owner).unwrap().id();
    let legacy_key =
        project_veriloga_bundle_source_key(project, registry.get_bundle(id).unwrap(), "sensor")
            .unwrap();
    assert_eq!(
        registry.current_veriloga_source_key(project, &legacy_key),
        Some(legacy_key.clone()),
        "a current key is its own current key"
    );

    let reference = CellViewRef::new("work", "sensor", "veriloga");
    registry
        .move_code_workspace_bundle_to_cell_view(ProjectSourceLanguage::VerilogA, reference)
        .unwrap();
    let current_key =
        project_veriloga_bundle_source_key(project, registry.get_bundle(id).unwrap(), "sensor")
            .unwrap();
    assert_ne!(legacy_key, current_key);
    assert_eq!(
        registry.current_veriloga_source_key(project, &legacy_key),
        Some(current_key)
    );
    assert_eq!(
        registry
            .veriloga_bundle_for_source_key(project, &legacy_key)
            .map(ProjectSourceBundle::id),
        Some(id)
    );
    assert!(
        registry
            .current_veriloga_source_key(ProjectId::new(), &legacy_key)
            .is_none(),
        "a key is read only against the project it was written for"
    );

    registry
        .replace_bundle_file_content(
            id,
            "sensor.va",
            "module sensor(p, n); inout p, n; electrical p, n; real edited; endmodule\n".to_owned(),
        )
        .unwrap();
    assert!(
        registry
            .current_veriloga_source_key(project, &legacy_key)
            .is_none()
    );
    assert!(
        registry
            .veriloga_bundle_for_source_key(project, &legacy_key)
            .is_some()
    );
}

/// A Verilog-AMS root is a `.vams` file as often as a `.va` one; any other
/// extension, or an extension alone, is refused and the refusal names both.
#[test]
fn a_veriloga_root_may_be_a_va_or_a_vams_file() {
    let owner = || ProjectSourceOwner::cell_view(CellViewRef::new("lib", "amp", "veriloga"));
    for root in ["amp.va", "models/amp.VAMS"] {
        let bundle = ProjectSourceBundle::try_new(
            owner(),
            ProjectSourceLanguage::VerilogA,
            root,
            "module amp; endmodule
",
            [],
            [],
        );
        assert!(bundle.is_ok(), "{root}: {bundle:?}");
    }
    for root in ["amp.v", ".vams", "amp.vams.txt"] {
        let refused = ProjectSourceBundle::try_new(
            owner(),
            ProjectSourceLanguage::VerilogA,
            root,
            "module amp; endmodule
",
            [],
            [],
        )
        .expect_err(root)
        .to_string();
        assert_eq!(
            refused, "Verilog-A source file name must end in '.va' or '.vams'",
            "{root}"
        );
    }
}

/// One source, two views: each module of a two-module file is a view of its
/// own cell, both served by the same bundle. The views find it as their own,
/// no other source may claim either, sharing is outside the digest and
/// survives a save, and the source outlives its owner's view while another
/// view still uses it.
#[test]
fn a_shared_source_serves_each_of_its_views_and_outlives_its_owner() {
    let first = CellViewRef::new("lib", "first", "veriloga");
    let second = CellViewRef::new("lib", "second", "veriloga");
    let mut bundle = owned_bundle("lib", "first", "veriloga");
    let digest = bundle.closure_digest();
    let revision = bundle.revision();
    bundle.share_with(second.clone()).unwrap();
    assert_eq!(
        bundle.closure_digest(),
        digest,
        "sharing changes no source byte"
    );
    assert!(matches!(
        bundle.clone().share_with(first.clone()),
        Err(ProjectSourceError::DuplicateOwner { .. })
    ));
    let id = bundle.id();
    let mut registry = ProjectSourceRegistry::try_from_bundles([bundle]).unwrap();

    for view in [&first, &second] {
        assert_eq!(
            registry
                .bundle_for_owner(&ProjectSourceOwner::cell_view(view.clone()))
                .map(ProjectSourceBundle::id),
            Some(id),
            "{}",
            view.display_path()
        );
    }
    assert!(matches!(
        registry
            .clone()
            .insert_bundle(owned_bundle("lib", "second", "veriloga")),
        Err(ProjectSourceError::DuplicateOwner { .. })
    ));
    let restored: ProjectSourceRegistry =
        serde_json::from_slice(&serde_json::to_vec(&registry).unwrap()).unwrap();
    assert_eq!(restored, registry, "the views a source serves are saved");

    let renamed = CellViewRef::new("lib", "renamed", "veriloga");
    assert_eq!(
        registry
            .rename_cell_view_bundles("lib", "second", "renamed")
            .unwrap(),
        Vec::<ProjectSourceId>::new(),
        "renaming a view the source is shared with changes no evidence"
    );
    let copied = registry
        .clone_cell_view_bundles("lib", "renamed", "lib", "copy")
        .unwrap();
    assert_eq!(copied.len(), 1, "a copy of a sharing cell owns a source");
    assert_ne!(copied[0], id);

    assert_eq!(
        registry.remove_cell_view_bundles("lib", "first", None),
        [id]
    );
    let survivor = registry
        .bundle_for_owner(&ProjectSourceOwner::cell_view(renamed.clone()))
        .expect("the source outlives its owner's view");
    assert_eq!(survivor.id(), id);
    assert_eq!(survivor.owner(), &ProjectSourceOwner::cell_view(renamed));
    assert!(survivor.shared_with().is_empty());
    assert!(
        survivor.revision() > revision,
        "passing the source on is a revision"
    );
    assert_ne!(survivor.closure_digest(), digest);
    assert!(
        survivor.digest_names_current_graph(digest),
        "a reference made under the old owner still names the source"
    );
    assert_eq!(
        registry.remove_cell_view_bundles("lib", "renamed", None),
        [id]
    );
    assert!(registry.get_bundle(id).is_none());
}

fn compile_attempt(bundle: &ProjectSourceBundle) -> ProjectSourceQualificationAttempt {
    let digest = ContentDigest::from_bytes([7; 32]);
    ProjectSourceQualificationAttempt {
        attempt_id: Uuid::new_v4(),
        recorded_at_unix_ms: 1,
        source_revision: bundle.revision().get(),
        source_closure_digest: bundle.closure_digest(),
        profile_digest: digest,
        package_name: "package".to_owned(),
        package_version: "1".to_owned(),
        selected_module: "module".to_owned(),
        compiler_version: "1".to_owned(),
        disposition: ProjectSourceQualificationDisposition::Passed,
        report_digest: digest,
        targets: Vec::new(),
        checks: Vec::new(),
        error_count: 0,
        warning_count: 0,
    }
}

fn veriloga_bundle() -> ProjectSourceBundle {
    ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(CellViewRef::new("lib", "cell", "veriloga")),
        ProjectSourceLanguage::VerilogA,
        "m.va",
        "module m; endmodule\n",
        [],
        [],
    )
    .unwrap()
}

/// A compile is never refused for its bookkeeping: past what a bundle keeps,
/// the oldest attempt gives way and the numbering goes on.
#[test]
fn the_oldest_compile_attempt_gives_way_to_the_newest() {
    let mut bundle = veriloga_bundle();
    let attempts = RETAINED_QUALIFICATION_RECORDS as u64 + 3;
    for _ in 0..attempts {
        bundle
            .append_qualification(compile_attempt(&bundle))
            .unwrap();
    }
    let kept = bundle.qualifications();
    assert_eq!(kept.len(), RETAINED_QUALIFICATION_RECORDS);
    assert_eq!(kept[0].sequence(), 4);
    assert_eq!(kept[kept.len() - 1].sequence(), attempts);
    bundle.validate().unwrap();
}

/// What a compile proves is kept with the bundle, and does not make the
/// bundle differ from the one that was saved.
#[test]
fn compile_evidence_is_not_authored_content() {
    let mut saved = ProjectSourceRegistry::try_from_bundles([veriloga_bundle()]).unwrap();
    let bundle_id = saved.iter_bundles().next().unwrap().id();
    let mut working = saved.clone();
    working.mark_bundle_validated(bundle_id).unwrap();
    let attempt = compile_attempt(working.get_bundle(bundle_id).unwrap());
    working
        .append_bundle_qualification(bundle_id, attempt)
        .unwrap();
    assert_ne!(working, saved, "the evidence is held");
    assert!(working.same_authored_content(&saved));

    saved
        .replace_bundle_file_content(bundle_id, "m.va", "module n; endmodule\n".to_owned())
        .unwrap();
    assert!(
        !working.same_authored_content(&saved),
        "an edit is authored"
    );
}
