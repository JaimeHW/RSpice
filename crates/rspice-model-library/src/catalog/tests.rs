//! Headless catalog publication and snapshot atomicity.

use super::*;
use crate::CapturedHdlSources;
use crate::qualification::ModelQualificationState;
use crate::source_bundle::ImportLimits;
use rspice_core::library::{LibParseResult, LibParser, ResolvedLibDependency, ResolvedLibSource};
use std::{path::Path, sync::Arc};

fn captured_parse(root: &Path, source: &str) -> LibParseResult {
    let mut parsed = LibParser::new(root.parent().unwrap()).parse_string(source);
    assert!(parsed.is_ok());
    parsed.resolved_sources = vec![ResolvedLibSource {
        path: root.to_path_buf(),
        bytes: Arc::from(source.as_bytes()),
        content: Arc::from(source),
    }];
    parsed
}

#[test]
fn external_refresh_keeps_the_catalog_on_projection_limits_or_name_failure() {
    let root = Path::new("/models/owned.lib");
    let parsed = captured_parse(root, ".model nch NMOS (LEVEL=1 VTH0=0.4)\n");
    let limits = ImportLimits {
        max_files: 10,
        max_total_bytes: 4096,
    };
    let mut catalog = ModelCatalog::default();
    catalog
        .import_external_library(root, parsed.clone(), None, limits, |_| {
            panic!("no HDL root")
        })
        .unwrap();
    let before = serde_json::to_value(&catalog).unwrap();
    assert!(
        catalog
            .import_external_library(root, parsed.clone(), Some("missing"), limits, |_| panic!(
                "no HDL root"
            ))
            .is_err()
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    assert!(
        catalog
            .import_external_library(
                root,
                parsed.clone(),
                None,
                ImportLimits {
                    max_files: 0,
                    ..limits
                },
                |_| panic!("no HDL root")
            )
            .unwrap_err()
            .contains("exceeds the project limit")
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    let other = Path::new("/other/owned.lib");
    let other_parse = captured_parse(other, ".model nch NMOS (LEVEL=1)\n");
    assert!(
        catalog
            .import_external_library(other, other_parse, None, limits, |_| panic!("no HDL root"))
            .unwrap_err()
            .contains("already owned by a different model source")
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
}

#[test]
fn hdl_capture_rejection_does_not_publish_a_partial_import() {
    let root = Path::new("/models/owned.lib");
    let hdl = Path::new("/models/device.va");
    let source = ".model nch NMOS (LEVEL=1)\n";
    let mut parsed = captured_parse(root, source);
    let content = format!("{source}.hdl \"device.va\"\n");
    parsed.resolved_sources[0].bytes = Arc::from(content.as_bytes());
    parsed.resolved_sources[0].content = Arc::from(content);
    parsed.resolved_sources.push(ResolvedLibSource {
        path: hdl.to_path_buf(),
        bytes: Arc::from(b"module device; endmodule\n".as_slice()),
        content: Arc::from("module device; endmodule\n"),
    });
    parsed.resolved_dependencies.push(ResolvedLibDependency {
        owner: root.to_path_buf(),
        requested_path: "device.va".to_owned(),
        target: hdl.to_path_buf(),
    });
    let mut catalog = ModelCatalog::default();
    catalog.add_library(ModelLibrary::new("retained"));
    let before = serde_json::to_value(&catalog).unwrap();
    let limits = ImportLimits {
        max_files: 10,
        max_total_bytes: 4096,
    };
    let error = catalog
        .import_external_library(root, parsed.clone(), None, limits, |requested| {
            assert_eq!(requested, hdl);
            Err("host rejected HDL".to_owned())
        })
        .unwrap_err();
    assert_eq!(error, "host rejected HDL");
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    let error = catalog
        .import_external_library(root, parsed, None, limits, |_| {
            Ok(CapturedHdlSources {
                sources: vec![(hdl.to_path_buf(), "module changed; endmodule\n".to_owned())],
                dependencies: vec![],
            })
        })
        .unwrap_err();
    assert!(error.contains("changed while its closure was captured"));
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
}

fn definition(value: f64) -> ProjectModelRevisionDefinition {
    let base = ProjectModelDefinition {
        name: "nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: String::new(),
        numeric_parameters: BTreeMap::from([("vth0".to_owned(), value)]),
        string_parameters: BTreeMap::new(),
    };
    let metadata = base.reconcile_metadata(None).unwrap();
    ProjectModelRevisionDefinition::new(base, metadata)
}

#[test]
fn guarded_revision_publishes_only_after_complete_validation() {
    let mut catalog = ModelCatalog::default();
    let created = catalog
        .create_project_model_revision(
            "owned",
            &definition(0.4),
            &ModelQualificationState::default(),
        )
        .unwrap();
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest,
    } = created.after.source_authority
    else {
        panic!("project-owned fixture");
    };
    let target = ProjectModelTarget {
        library_name: "owned",
        source_id,
        library_revision: revision,
        model_revision: revision,
        model_name: "nch",
        model_digest: digest,
    };
    let before = serde_json::to_value(&catalog).unwrap();
    for stale in [
        ProjectModelTarget {
            source_id: ModelSourceId::new(),
            ..target
        },
        ProjectModelTarget {
            model_digest: ContentDigest::from_bytes([0; 32]),
            ..target
        },
    ] {
        assert!(
            catalog
                .replace_project_model_revision_in_library(
                    stale,
                    &definition(0.5),
                    &ModelQualificationState::default(),
                )
                .unwrap_err()
                .contains("changed after")
        );
        assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    }

    let mut invalid = definition(0.5);
    invalid
        .base
        .numeric_parameters
        .insert("vth0".to_owned(), f64::NAN);
    assert!(
        catalog
            .replace_project_model_revision_in_library(
                target,
                &invalid,
                &ModelQualificationState::default(),
            )
            .unwrap_err()
            .contains("must be finite")
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);

    let commit = catalog
        .replace_project_model_revision_in_library(
            target,
            &definition(0.5),
            &ModelQualificationState::default(),
        )
        .unwrap();
    assert_eq!(commit.after.models["nch"].parameters["vth0"], 0.5);
    assert_eq!(
        commit.after.project_source_revision(),
        Some(revision.next().unwrap())
    );
    assert!(commit.affects_execution);
    assert_eq!(
        serde_json::to_value(commit.before.unwrap()).unwrap(),
        before["owned"]
    );
}

#[test]
fn snapshot_replacement_preserves_retained_state_and_rejects_duplicates_atomically() {
    let mut catalog = ModelCatalog::default();
    let mut original = ModelLibrary::new("owned");
    original.expanded = true;
    catalog.add_library(original);
    let before = serde_json::to_value(&catalog).unwrap();
    assert!(
        catalog
            .replace_library_snapshot(vec![ModelLibrary::new("next"), ModelLibrary::new("next")])
            .is_err()
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    catalog
        .replace_library_snapshot(vec![ModelLibrary::new("owned")])
        .unwrap();
    assert!(catalog.get_library("owned").unwrap().expanded);
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
}
