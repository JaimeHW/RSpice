use super::*;

fn technology_binding_fixture() -> ProjectTechnologyBinding {
    let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
    ProjectTechnologyBinding {
        schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
        package_name: "Qualified analog models".to_owned(),
        package_version: Some("2026.07".to_owned()),
        technology_node: Some("180 nm".to_owned()),
        model_library: "qualified_analog".to_owned(),
        root_source: root.clone(),
        source_closure: vec![rspice_model_library::ModelSourcePin {
            path: root,
            digest: rspice_app_types::product::ContentDigest::from_bytes([0x4a; 32]),
        }],
        source_edges: Vec::new(),
        model_count: 14,
        process_sections: vec!["ff".to_owned(), "ss".to_owned(), "tt".to_owned()],
        signed_package: None,
    }
}

#[test]
fn project_identity_is_stable_and_rename_is_atomic() {
    let mut project = ProjectDescriptor::default();
    let id = project.id();
    let initial_revision = project.revision();

    let renamed_revision = project
        .rename("Precision ΔΣ ADC")
        .expect("valid Unicode name");

    assert_eq!(project.id(), id);
    assert_eq!(project.name(), "Precision ΔΣ ADC");
    assert_eq!(renamed_revision.get(), initial_revision.get() + 1);
    assert_eq!(
        project.rename("Precision ΔΣ ADC").expect("no-op rename"),
        renamed_revision
    );

    let rejected = project.rename("bad/name");
    assert!(matches!(
        rejected,
        Err(ProjectDescriptorError::PathSeparator('/'))
    ));
    assert_eq!(project.name(), "Precision ΔΣ ADC");
    assert_eq!(project.revision(), renamed_revision);
    assert_eq!(project.id(), id);
}

#[test]
fn legacy_project_descriptor_identity_migration_is_deterministic() {
    let original = ProjectDescriptor::default();
    let mut legacy = serde_json::to_value(&original).expect("descriptor serializes");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("id");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("schema_version");
    legacy
        .as_object_mut()
        .expect("descriptor is an object")
        .remove("revision");
    let legacy_json = serde_json::to_string(&legacy).expect("legacy descriptor serializes");

    let first: ProjectDescriptor =
        serde_json::from_str(&legacy_json).expect("legacy descriptor restores");
    let second: ProjectDescriptor =
        serde_json::from_str(&legacy_json).expect("legacy descriptor restores again");

    assert_eq!(first.id(), second.id());
    assert!(!first.id().as_uuid().is_nil());
    assert_ne!(first.id(), original.id());

    let persisted = serde_json::to_value(&first).expect("migrated descriptor serializes");
    assert_eq!(
        persisted.get("id"),
        Some(&serde_json::to_value(first.id()).expect("identity serializes"))
    );
}

#[test]
fn versioned_or_explicitly_null_project_identity_and_schema_are_rejected() {
    let project = ProjectDescriptor::default();
    let mut missing = serde_json::to_value(&project).expect("descriptor serializes");
    missing
        .as_object_mut()
        .expect("descriptor object")
        .remove("id");
    let missing_error = serde_json::from_value::<ProjectDescriptor>(missing)
        .expect_err("versioned descriptor must retain identity");
    assert!(
        missing_error
            .to_string()
            .contains("missing its stable identity")
    );

    let mut null = serde_json::to_value(&project).expect("descriptor serializes");
    null["id"] = serde_json::Value::Null;
    let null_error = serde_json::from_value::<ProjectDescriptor>(null)
        .expect_err("explicit null identity is not legacy absence");
    assert!(
        null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut unversioned_null = serde_json::to_value(&project).expect("descriptor serializes");
    unversioned_null
        .as_object_mut()
        .expect("descriptor object")
        .remove("schema_version");
    unversioned_null["id"] = serde_json::Value::Null;
    let unversioned_null_error = serde_json::from_value::<ProjectDescriptor>(unversioned_null)
        .expect_err("unversioned explicit null is not genuine legacy absence");
    assert!(
        unversioned_null_error
            .to_string()
            .contains("must not be explicitly null")
    );

    let mut null_schema = serde_json::to_value(&project).expect("descriptor serializes");
    null_schema["schema_version"] = serde_json::Value::Null;
    let null_schema_error = serde_json::from_value::<ProjectDescriptor>(null_schema)
        .expect_err("explicit null schema is not an unversioned descriptor");
    assert!(
        null_schema_error
            .to_string()
            .contains("schema version must not be explicitly null")
    );
}

#[test]
fn project_name_contract_counts_graphemes_and_rejects_unsafe_text() {
    let family = "👨‍👩‍👧‍👦";
    assert!(ProjectDescriptor::validate_name(&family.repeat(120)).is_ok());
    assert!(matches!(
        ProjectDescriptor::validate_name(&family.repeat(121)),
        Err(ProjectDescriptorError::NameTooLong {
            grapheme_count: 121
        })
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name(" leading"),
        Err(ProjectDescriptorError::SurroundingWhitespace)
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name("line\nfeed"),
        Err(ProjectDescriptorError::ControlCharacter('\n'))
    ));
    assert!(matches!(
        ProjectDescriptor::validate_name("path\\name"),
        Err(ProjectDescriptorError::PathSeparator('\\'))
    ));
}

#[test]
fn changing_source_path_does_not_rename_an_existing_project() {
    let mut project = ProjectDescriptor::default();
    project.set_path(PathBuf::from("first-save.rspiceproj"));
    let revision = project.revision();

    assert_eq!(project.name(), "first-save");
    project.set_path(PathBuf::from("moved-copy.rspiceproj"));

    assert_eq!(project.name(), "first-save");
    assert_eq!(project.revision(), revision);
    assert_eq!(
        project.path.as_deref(),
        Some(Path::new("moved-copy.rspiceproj"))
    );
}

#[test]
fn project_copy_has_independent_identity_without_rebinding_source() {
    let mut source = ProjectDescriptor::default();
    source
        .rename("Precision reference")
        .expect("source name is valid");
    source.set_path(PathBuf::from("source.rspiceproj"));
    let source_id = source.id();
    let source_revision = source.revision();
    let source_path = source.path.clone();

    let copy = source.fork_copy_at(PathBuf::from("copy.rspiceproj"));

    assert_ne!(copy.id(), source_id);
    assert_eq!(copy.revision(), ObjectRevision::INITIAL);
    assert_eq!(copy.name(), source.name());
    assert_eq!(copy.path.as_deref(), Some(Path::new("copy.rspiceproj")));
    assert_eq!(source.id(), source_id);
    assert_eq!(source.revision(), source_revision);
    assert_eq!(source.path, source_path);
}

#[test]
fn signed_technology_pin_is_strictly_validated_and_round_trips_with_the_project_binding() {
    let binding = technology_binding_fixture();
    let mut value = serde_json::to_value(&binding).expect("binding serializes");
    value["signed_package"] = serde_json::json!({
        "schema_version": PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
        "package_id": "demo180",
        "revision": "2.3.1",
        "manifest_digest": rspice_app_types::product::ContentDigest::from_bytes([0x11; 32]),
        "archive_digest": rspice_app_types::product::ContentDigest::from_bytes([0x22; 32]),
        "technology_name": "Demo 180 nm",
        "publisher_id": "rspice-foundry-demo",
        "signing_key_id": "ceremony-01",
        "process_node_nm": 180,
        "stack_name": "1P2M",
        "execution_targets": ["desktop", "web-assembly", "mobile"]
    });
    let restored: ProjectTechnologyBinding =
        serde_json::from_value(value.clone()).expect("strict signed pin deserializes");
    restored.validate().expect("signed binding validates");
    assert_eq!(
        serde_json::from_str::<ProjectTechnologyBinding>(
            &serde_json::to_string(&restored).expect("binding serializes")
        )
        .expect("binding round trips"),
        restored
    );

    value["signed_package"]["execution_targets"] = serde_json::json!(["desktop", "desktop"]);
    let duplicate_targets: ProjectTechnologyBinding =
        serde_json::from_value(value).expect("shape deserializes");
    assert!(matches!(
        duplicate_targets.validate(),
        Err(TechnologyBindingError::InvalidSignedPackage(_))
    ));
}

#[test]
fn technology_attachment_is_atomic_revisioned_and_idempotent() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let binding = technology_binding_fixture();

    let committed = project
        .attach_technology(binding.clone())
        .expect("valid binding commits");
    assert_eq!(committed.get(), initial_revision.get() + 1);
    assert_eq!(project.technology_binding(), Some(&binding));
    assert_eq!(
        project.technology.as_deref(),
        Some(binding.display_label().as_str())
    );
    assert_eq!(
        project
            .attach_technology(binding)
            .expect("identical binding is a no-op"),
        committed
    );

    let mut rejected = technology_binding_fixture();
    rejected.model_count = 0;
    let before = project.clone();
    assert!(matches!(
        project.attach_technology(rejected),
        Err(ProjectDescriptorError::Technology(
            TechnologyBindingError::NoModels
        ))
    ));
    assert_eq!(project.revision(), before.revision());
    assert_eq!(project.technology, before.technology);
    assert_eq!(project.technology_binding(), before.technology_binding());
}

#[test]
fn cloud_publication_binding_is_revisioned_idempotent_and_validated() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let binding = ProjectCloudPublicationBinding::new(
        "0198c1c2-aaaa-7000-8000-000000000001".to_owned(),
        "0198c1c2-bbbb-7000-8000-000000000002".to_owned(),
    )
    .expect("service identifiers validate");

    let committed = project
        .bind_cloud_publication(binding.clone())
        .expect("binding commits");
    assert_eq!(committed.get(), initial_revision.get() + 1);
    assert_eq!(project.cloud_publication(), Some(&binding));
    project.validate().expect("bound project validates");
    assert_eq!(
        project
            .bind_cloud_publication(binding.clone())
            .expect("identical binding is a no-op"),
        committed
    );

    let mut value = serde_json::to_value(&project).expect("descriptor serializes");
    let restored: ProjectDescriptor =
        serde_json::from_value(value.clone()).expect("descriptor round trips");
    assert_eq!(restored.cloud_publication(), Some(&binding));
    // Legacy descriptors without the field restore to an unbound project.
    value
        .as_object_mut()
        .expect("descriptor object")
        .remove("cloud_publication");
    let unbound: ProjectDescriptor =
        serde_json::from_value(value).expect("legacy shape deserializes");
    assert_eq!(unbound.cloud_publication(), None);

    assert!(matches!(
        ProjectCloudPublicationBinding::new(String::new(), "circuit".to_owned()),
        Err(ProjectDescriptorError::InvalidCloudPublicationBinding(
            "workspace_id"
        ))
    ));
    assert!(matches!(
        ProjectCloudPublicationBinding::new("workspace".to_owned(), "has space".to_owned()),
        Err(ProjectDescriptorError::InvalidCloudPublicationBinding(
            "circuit_id"
        ))
    ));
}

#[test]
fn technology_change_receipts_are_checkpoint_bound_atomic_and_tamper_evident() {
    let mut project = ProjectDescriptor::default();
    let initial_revision = project.revision();
    let authority = ProjectTechnologyChangeAuthority::new(
        "engineer.james",
        "project-technology-admin",
        "Attach the qualified tapeout technology",
    )
    .expect("authority validates");
    let context = ProjectTechnologyChangeContext::new(
        authority,
        Uuid::new_v4(),
        initial_revision,
        1_785_430_000_000,
        rspice_app_types::product::ContentDigest::from_bytes([0x71; 32]),
        4_096,
    )
    .expect("checkpoint context validates");
    let binding = technology_binding_fixture();
    let (attached_revision, first_receipt) = project
        .attach_technology_audited(binding.clone(), context)
        .expect("audited attachment commits");
    assert_eq!(first_receipt.sequence(), 1);
    assert_eq!(
        first_receipt.action(),
        ProjectTechnologyChangeAction::Attach
    );
    assert_eq!(attached_revision.get(), initial_revision.get() + 1);
    assert_eq!(project.technology_change_audit().len(), 1);
    project.validate().expect("audited project validates");

    let mut bypass = binding.clone();
    bypass.package_version = Some("2026.08".to_owned());
    assert_eq!(
        project.attach_technology(bypass.clone()),
        Err(ProjectDescriptorError::TechnologyAuditRequired)
    );
    assert_eq!(project.technology_binding(), Some(&binding));

    project
        .rename("Audited technology project")
        .expect("unrelated project revision advances");
    let replacement_from = project.revision();
    let replacement_context = ProjectTechnologyChangeContext::new(
        ProjectTechnologyChangeAuthority::new(
            "engineer.james",
            "project-technology-admin",
            "Adopt qualified model update",
        )
        .expect("replacement authority validates"),
        Uuid::new_v4(),
        replacement_from,
        1_785_430_100_000,
        rspice_app_types::product::ContentDigest::from_bytes([0x72; 32]),
        8_192,
    )
    .expect("replacement checkpoint context validates");
    let (_, replacement_receipt) = project
        .attach_technology_audited(bypass.clone(), replacement_context)
        .expect("audited replacement commits");
    assert_eq!(replacement_receipt.sequence(), 2);
    assert_eq!(
        replacement_receipt.action(),
        ProjectTechnologyChangeAction::Replace
    );
    assert_eq!(
        replacement_receipt.source_project_revision(),
        replacement_from
    );
    assert_eq!(project.technology_binding(), Some(&bypass));
    project.validate().expect("replacement chain validates");

    let encoded = serde_json::to_vec(&project).expect("descriptor serializes");
    let restored: ProjectDescriptor =
        serde_json::from_slice(&encoded).expect("descriptor round trips");
    restored.validate().expect("restored audit validates");
    assert_eq!(
        serde_json::to_value(&restored).expect("restored descriptor serializes"),
        serde_json::to_value(&project).expect("source descriptor serializes")
    );

    let mut tampered = serde_json::to_value(&project).expect("descriptor serializes to value");
    tampered["technology_change_audit"][0]["reason"] =
        serde_json::Value::String("tampered reason".to_owned());
    let tampered: ProjectDescriptor =
        serde_json::from_value(tampered).expect("tampered shape deserializes");
    assert!(matches!(
        tampered.validate(),
        Err(ProjectDescriptorError::TechnologyAuditCorrupted(_))
    ));

    let copied = project.fork_copy_at(PathBuf::from(r"C:\projects\fork.rspice"));
    assert!(copied.technology_change_audit().is_empty());
    copied
        .validate()
        .expect("independent copy starts fresh history");
}

#[test]
fn attached_technology_detects_exact_catalog_drift() {
    let root = PathBuf::from(r"C:\qualified-pdk\models.lib");
    let bytes = b".model nch nmos level=1\n".to_vec();
    let digest =
        rspice_app_types::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into());
    let mut library = rspice_model_library::ModelLibrary::new("qualified_analog")
        .with_technology("Qualified analog models", "180 nm");
    library.version = "2026.07".to_owned();
    library.root_path = Some(root.clone());
    library.source_closure = vec![rspice_model_library::ModelSourcePin {
        path: root.clone(),
        digest,
    }];
    library.source_contents = vec![rspice_model_library::ModelSourceContent { path: root, bytes }];
    library.add_model(rspice_model_library::DeviceModel::new(
        "nch",
        rspice_model_library::ModelType::Nmos,
    ));
    let binding = ProjectTechnologyBinding::from_model_library(&library)
        .expect("exact retained source is attachable");
    binding
        .validate_model_library(&library)
        .expect("unchanged catalog matches");

    library.version = "2026.08".to_owned();
    assert!(matches!(
        binding.validate_model_library(&library),
        Err(TechnologyBindingError::CatalogDrift { .. })
    ));
}

#[test]
fn project_library_mutation_receipts_are_exact_hash_chained_and_tamper_evident() {
    let mut project = ProjectDescriptor::default();
    let first = project
        .prepare_library_mutation(
            ProjectLibraryMutation::CreateCell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            },
            7,
        )
        .expect("prepare cell creation");
    assert_eq!(first.minimum_library_revision(), 8);
    let first = project
        .publish_library_mutation(first, 8)
        .expect("publish cell creation");

    let second = project
        .prepare_library_mutation(
            ProjectLibraryMutation::CreateView {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
                view: "symbol".to_owned(),
            },
            8,
        )
        .expect("prepare view creation");
    let second = project
        .publish_library_mutation(second, 10)
        .expect("publish view creation");

    assert_eq!(project.revision().get(), 3);
    assert_eq!(project.library_mutation_audit().len(), 2);
    assert_eq!(first.sequence(), 1);
    assert_eq!(second.sequence(), 2);
    assert_eq!(first.source_project_revision().get(), 1);
    assert_eq!(first.target_project_revision().get(), 2);
    assert_eq!(second.source_project_revision().get(), 2);
    assert_eq!(second.target_project_revision().get(), 3);
    assert_eq!(first.source_library_revision(), 7);
    assert_eq!(first.target_library_revision(), 8);
    assert_eq!(second.source_library_revision(), 8);
    assert_eq!(second.target_library_revision(), 10);
    assert_eq!(
        second.previous_receipt_digest(),
        Some(first.receipt_digest())
    );
    project.validate().expect("valid receipt chain");

    let json = serde_json::to_string(&project).expect("serialize project descriptor");
    let restored: ProjectDescriptor =
        serde_json::from_str(&json).expect("restore project descriptor");
    restored
        .validate()
        .expect("restored receipt chain is valid");

    let mut tampered: serde_json::Value =
        serde_json::from_str(&json).expect("parse project descriptor");
    tampered["library_mutation_audit"][0]["mutation"]["cell"] =
        serde_json::Value::String("tampered".to_owned());
    let tampered: ProjectDescriptor =
        serde_json::from_value(tampered).expect("receipt schema remains structurally valid");
    assert!(matches!(
        tampered.validate(),
        Err(ProjectDescriptorError::LibraryAuditCorrupted(_))
    ));
}

#[test]
fn library_mutation_preflight_rejects_noops_and_revision_exhaustion_atomically() {
    let project = ProjectDescriptor::default();
    let revision = project.revision();

    assert!(matches!(
        project.prepare_library_mutation(
            ProjectLibraryMutation::RenameCell {
                library: "work".to_owned(),
                from_cell: "amp".to_owned(),
                to_cell: "AMP".to_owned(),
            },
            4,
        ),
        Err(ProjectDescriptorError::LibraryAuditCorrupted(_))
    ));
    assert!(matches!(
        project.prepare_library_mutation(
            ProjectLibraryMutation::DeleteCell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            },
            u64::MAX,
        ),
        Err(ProjectDescriptorError::LibraryRevisionExhausted)
    ));
    assert_eq!(project.revision(), revision);
    assert!(project.library_mutation_audit().is_empty());
}

#[test]
fn include_search_paths_round_trip_and_default_to_empty() {
    let mut project = ProjectDescriptor::default();
    assert!(project.include_search_paths().is_empty());

    // Absent from a stored descriptor is the empty chain, not a failure.
    let stored = serde_json::to_value(&project).expect("descriptor serializes");
    assert!(
        stored.get("include_search_paths").is_none(),
        "an empty chain must not be written into the project file"
    );
    let restored: ProjectDescriptor =
        serde_json::from_value(stored).expect("descriptor without a chain restores");
    assert!(restored.include_search_paths().is_empty());

    project
        .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
        .expect("an ordered chain of distinct directories is accepted");
    let wire = serde_json::to_value(&project).expect("descriptor serializes");
    let restored: ProjectDescriptor =
        serde_json::from_value(wire).expect("descriptor with a chain restores");
    assert_eq!(
        restored.include_search_paths(),
        [PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")],
        "order is the setting and must survive the round trip"
    );
    restored.validate().expect("a persisted chain validates");
}

#[test]
fn a_chain_rejects_an_empty_or_repeated_entry() {
    let mut project = ProjectDescriptor::default();
    assert!(matches!(
        project.set_include_search_paths(vec![PathBuf::from("  ")]),
        Err(ProjectDescriptorError::EmptyIncludeSearchPath)
    ));
    assert!(matches!(
        project.set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("models")]),
        Err(ProjectDescriptorError::DuplicateIncludeSearchPath(_))
    ));
    assert!(project.include_search_paths().is_empty());
}
