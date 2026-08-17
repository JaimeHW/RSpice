//! Tests for execution-binding migration and source availability.
//!
//! Migrating an older schema must not invent resolution edges, and a source
//! that is unavailable or has changed is retained and warned about with the
//! run blocked - never dropped or silently re-resolved.

use super::*;
use crate::product::ObjectRevision;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind, AnalysisLifecycleState};
use crate::state::model_library::{
    CornerSectionBinding, CornerSectionDomain, CorrelationDatasetClass, CorrelationDatasetRevision,
    CorrelationSuite,
};

fn project_id() -> ProjectId {
    ProjectId::from_namespace(
        uuid::Uuid::from_u128(0xe707_36ed_7eef_5205_b51e_9608_f55e_bd35),
        b"project-execution-tests",
    )
}

fn context_from_state(
    plan: &SimSetupState,
    manager: &ModelLibraryManager,
) -> Result<ProjectExecutionContext, String> {
    ProjectExecutionContext::from_state(project_id(), plan, manager)
}

#[cfg(not(target_arch = "wasm32"))]
fn model_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "rspice-project-execution-models-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("create model fixture directory");
    let path = directory.join("foundry.lib");
    std::fs::write(
        directory.join("shared.inc"),
        ".model helper NMOS (LEVEL=1 KP=5e-4)\n",
    )
    .expect("write transitive model fixture");
    std::fs::write(
            &path,
            ".include \"shared.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n",
        )
        .expect("write model fixture");
    (directory, path)
}

#[test]
fn transient_runtime_state_is_not_serialized() {
    let mut plan = SimSetupState::new();
    plan.options_open = true;
    plan.options_errors.push("not project data".to_owned());
    plan.palette_open = true;
    plan.palette_query = "noise".to_owned();
    let context = context_from_state(&plan, &ModelLibraryManager::new()).expect("valid context");

    let value = serde_json::to_value(context).expect("serialize context");
    let plan = &value["simulation_plan"];
    assert!(plan.get("options_open").is_none());
    assert!(plan.get("options_errors").is_none());
    assert!(plan.get("options_draft").is_none());
    assert!(plan.get("palette_open").is_none());
    assert!(plan.get("palette_query").is_none());
    for retired in [
        "enabled",
        "analysis_order",
        "listed",
        "tran",
        "ac",
        "dc",
        "noise",
        "op",
        "pss",
        "disto_f2_over_f1",
    ] {
        assert!(
            plan.get(retired).is_none(),
            "current schema must omit retired singleton field {retired}"
        );
    }
    assert!(plan.get("analysis_plan").is_some());
}

#[test]
fn source_qualified_provider_decision_round_trips_without_reauthorization() {
    let mut manager = ModelLibraryManager::new();
    let winner = manager
        .load_library_bytes(
            "approved.lib",
            b".model shared NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
            None,
        )
        .expect("approved source imports");
    manager
        .load_library_bytes(
            "alternate.lib",
            b".model shared NMOS (LEVEL=1 KP=2e-3)\n".to_vec(),
            None,
        )
        .expect("alternate source imports");
    let record = manager
        .resolve_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            "shared",
            &winner,
            "Model-owner review selected the released characterization source.",
        )
        .expect("provider decision records");
    let plan = manager
        .seal_execution_sources()
        .expect("retained providers seal")
        .reference_model_execution_plan(crate::product::ProcessCorner::TT)
        .expect("provider decision produces one exact plan");
    let validation = manager
        .issue_model_validation_receipt(
            ObjectRevision::INITIAL,
            plan.digest(),
            None,
            PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            vec![crate::state::model_library::ModelValidationFinding {
                code: "SPICE_NAMESPACE_COMPILED".to_owned(),
                severity: crate::state::model_library::ModelValidationFindingSeverity::Information,
                message: "The frozen persisted namespace compiled.".to_owned(),
            }],
        )
        .expect("validation receipt records");

    let context = context_from_state(&SimSetupState::new(), &manager)
        .expect("provider decision saves in execution context");
    assert_eq!(context.model_resolution_records, vec![record.clone()]);
    assert_eq!(context.model_validation_receipt, Some(validation.clone()));
    let encoded = serde_json::to_vec(&context).expect("execution context serializes");
    let restored: ProjectExecutionContext =
        serde_json::from_slice(&encoded).expect("execution context deserializes");
    let (_, restored, _) = restored
        .into_state(project_id())
        .expect("provider decision restores against exact retained sources");
    assert_eq!(
        restored
            .model_resolution_record(
                crate::state::model_library::ModelConsumerScope::PrimitiveModel,
                "shared"
            )
            .cloned(),
        Some(record)
    );
    assert_eq!(
        restored.model_validation_receipt().cloned(),
        Some(validation)
    );
}

#[test]
fn retained_subcircuit_interfaces_round_trip_migrate_and_reject_tampering() {
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "browser-subcircuits.lib",
            b".subckt AMP inp inn out params: GAIN=100 MODE=\"low noise\"\n\
              e1 out 0 inp inn {GAIN}\n\
              .ends AMP\n"
                .to_vec(),
            None,
        )
        .expect("subcircuit source imports");
    let context =
        context_from_state(&SimSetupState::new(), &manager).expect("interface context saves");
    let value = serde_json::to_value(&context).expect("context serializes");
    assert_eq!(
        value["model_libraries"][0]["subcircuits"]["AMP"]["ports"],
        serde_json::json!(["inp", "inn", "out"])
    );
    assert_eq!(
        value["model_libraries"][0]["subcircuits"]["AMP"]["parameter_defaults"]["MODE"],
        "\"low noise\""
    );

    let restored: ProjectExecutionContext =
        serde_json::from_value(value.clone()).expect("context deserializes");
    let (_, restored_manager, _) = restored
        .into_state(project_id())
        .expect("exact interface restores");
    assert_eq!(
        restored_manager
            .get_library("browser-subcircuits")
            .and_then(|library| library.subcircuits.get("AMP"))
            .map(|interface| interface.ports.clone()),
        Some(vec!["inp".to_owned(), "inn".to_owned(), "out".to_owned()])
    );

    let mut tampered = value.clone();
    tampered["model_libraries"][0]["subcircuits"]["AMP"]["ports"][0] =
        serde_json::json!("forged_terminal");
    let tampered: ProjectExecutionContext =
        serde_json::from_value(tampered).expect("tampered shape deserializes");
    let error = tampered
        .validate()
        .expect_err("interface metadata cannot diverge from authenticated source");
    assert!(
        error.contains("not the exact interface projection"),
        "{error}"
    );

    let mut legacy = value;
    legacy["schema_version"] =
        serde_json::json!(EXPLICIT_MODEL_DEFINITION_RESOLUTION_SCHEMA_VERSION);
    legacy["model_libraries"][0]
        .as_object_mut()
        .expect("library is an object")
        .remove("subcircuits");
    let mut migrated: ProjectExecutionContext =
        serde_json::from_value(legacy).expect("schema-13 context deserializes");
    migrated
        .migrate_to_current(project_id())
        .expect("schema-13 interfaces rebuild from retained bytes");
    assert_eq!(
        migrated.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    assert_eq!(
        migrated.model_libraries[0].subcircuits["AMP"].ports,
        ["inp", "inn", "out"]
    );
    migrated.validate().expect("migrated context validates");
}

#[test]
fn active_model_section_provenance_round_trips_migrates_and_rejects_tampering() {
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_bytes(
            "sectioned-cards.lib",
            b".model helper NMOS (LEVEL=1 KP=5e-4)\n\
              .lib TT\n\
              .model nch NMOS (LEVEL=1 KP=1e-3)\n\
              .endl TT\n\
              .lib FF\n\
              .model nch NMOS (LEVEL=1 KP=2e-3)\n\
              .endl FF\n"
                .to_vec(),
            Some("TT"),
        )
        .expect("sectioned cards import");
    let context =
        context_from_state(&SimSetupState::new(), &manager).expect("sectioned context saves");
    let value = serde_json::to_value(&context).expect("context serializes");
    assert_eq!(
        value["model_libraries"][0]["models"]["nch"]["section"],
        "TT"
    );
    assert!(
        value["model_libraries"][0]["models"]["helper"]
            .get("section")
            .is_none(),
        "top-level cards omit section provenance"
    );
    assert_eq!(
        value["model_libraries"][0]["section_models"]["FF"]["nch"]["parameters"]["kp"],
        2.0e-3
    );

    let restored: ProjectExecutionContext =
        serde_json::from_value(value.clone()).expect("current context deserializes");
    restored.validate().expect("exact section validates");
    let (_, mut restored_manager, warnings) = restored
        .clone()
        .into_state(project_id())
        .expect("complete section catalog restores");
    assert!(warnings.is_empty());
    let restored_library = restored_manager
        .get_library_mut("sectioned-cards")
        .expect("restored sectioned library");
    assert!(restored_library.select_corner("FF"));
    assert_eq!(
        restored_library.models["nch"].parameters.get("kp"),
        Some(&2.0e-3)
    );

    let mut schema_17 = value.clone();
    schema_17["schema_version"] =
        serde_json::json!(EXPLICIT_SIMULATION_PLAN_MODEL_BINDINGS_SCHEMA_VERSION);
    schema_17["model_libraries"][0]
        .as_object_mut()
        .expect("library is an object")
        .remove("top_level_models");
    schema_17["model_libraries"][0]
        .as_object_mut()
        .expect("library is an object")
        .remove("section_models");
    let mut schema_17: ProjectExecutionContext =
        serde_json::from_value(schema_17).expect("schema-17 context deserializes");
    schema_17
        .migrate_to_current(project_id())
        .expect("complete catalog rebuilds from authenticated bytes");
    assert_eq!(
        schema_17.model_libraries[0].section_models["FF"]["nch"]
            .parameters
            .get("kp"),
        Some(&2.0e-3)
    );
    schema_17.validate().expect("schema-17 migration validates");

    let mut legacy = value.clone();
    legacy["schema_version"] = serde_json::json!(RETAINED_SUBCIRCUIT_INTERFACE_SCHEMA_VERSION);
    legacy["model_libraries"][0]["models"]["nch"]
        .as_object_mut()
        .expect("model is an object")
        .remove("section");
    let mut migrated: ProjectExecutionContext =
        serde_json::from_value(legacy).expect("schema-14 context deserializes");
    migrated
        .migrate_to_current(project_id())
        .expect("schema-14 section is recovered from authenticated bytes");
    assert_eq!(
        migrated.model_libraries[0].models["nch"].section.as_deref(),
        Some("TT")
    );
    assert_eq!(migrated.model_libraries[0].models["helper"].section, None);
    migrated.validate().expect("migrated context validates");

    let mut tampered = value.clone();
    tampered["model_libraries"][0]["models"]["nch"]
        .as_object_mut()
        .expect("model is an object")
        .remove("section");
    let tampered: ProjectExecutionContext =
        serde_json::from_value(tampered).expect("tampered shape deserializes");
    let error = tampered
        .validate()
        .expect_err("current schema cannot discard active-card section provenance");
    assert!(error.contains("not an exact projection"), "{error}");

    let mut tampered_section = value.clone();
    tampered_section["model_libraries"][0]["section_models"]["FF"]["nch"]["parameters"]["kp"] =
        serde_json::json!(9.0e-3);
    let tampered_section: ProjectExecutionContext = serde_json::from_value(tampered_section)
        .expect("tampered complete section catalog deserializes");
    let error = tampered_section
        .validate()
        .expect_err("complete section catalog cannot diverge from retained source bytes");
    assert!(error.contains("section_models is not the exact"), "{error}");

    let mut duplicate = value;
    let mut alias = duplicate["model_libraries"][0]["models"]["nch"].clone();
    alias["name"] = serde_json::json!("NCH");
    duplicate["model_libraries"][0]["models"]
        .as_object_mut()
        .expect("models is an object")
        .insert("NCH".to_owned(), alias);
    let duplicate: ProjectExecutionContext =
        serde_json::from_value(duplicate).expect("duplicate model shape deserializes");
    let error = duplicate
        .validate()
        .expect_err("SPICE model identifiers are case-insensitive");
    assert!(
        error.contains("case-insensitive duplicate names"),
        "{error}"
    );
}

#[test]
fn current_schema_rejects_every_retired_singleton_analysis_field() {
    let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
        .expect("baseline context validates");
    let baseline = serde_json::to_value(context).expect("context serializes");

    for field in RETIRED_SINGLETON_ANALYSIS_FIELDS {
        let mut value = baseline.clone();
        value["simulation_plan"]
            .as_object_mut()
            .expect("simulation plan is an object")
            .insert((*field).to_owned(), serde_json::Value::Null);
        let error = serde_json::from_value::<ProjectExecutionContext>(value)
            .expect_err("current schema must reject retired singleton input")
            .to_string();
        assert!(
            error.contains(&format!("retired singleton field `{field}`")),
            "{error}"
        );
    }
}

#[test]
fn schema_three_still_accepts_singletons_only_for_load_time_migration() {
    let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
        .expect("baseline context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(SINGLETON_ANALYSIS_PLAN_SCHEMA_VERSION);
    let persisted_plan = value["simulation_plan"]
        .as_object_mut()
        .expect("simulation plan is an object");
    persisted_plan.remove("analysis_plan");
    persisted_plan.insert("enabled".to_owned(), serde_json::json!([1]));
    persisted_plan.insert("analysis_order".to_owned(), serde_json::json!([1]));
    persisted_plan.insert("listed".to_owned(), serde_json::json!([1]));
    persisted_plan.insert(
        "tran".to_owned(),
        serde_json::to_value(crate::workbench::app_state::TranSetup::default())
            .expect("legacy draft serializes"),
    );

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema 3 accepts its legacy fields");
    restored
        .migrate_to_current(project_id())
        .expect("schema 3 migrates at the load boundary");

    assert_eq!(
        restored.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    assert!(restored.simulation_plan.stable_analysis_plan().is_ok());
    restored.validate().expect("migrated context validates");
}

#[test]
fn schema_four_promotes_the_single_stable_plan_into_the_named_catalog() {
    let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
        .expect("baseline context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(STABLE_ANALYSIS_PLAN_SCHEMA_VERSION);
    let persisted_plan = value["simulation_plan"]
        .as_object_mut()
        .expect("simulation plan is an object");
    persisted_plan.remove("active_plan_name");
    persisted_plan.remove("active_plan_lineage");
    persisted_plan.remove("inactive_plans");
    persisted_plan["analysis_plan"]
        .as_object_mut()
        .expect("stable analysis plan is an object")
        .remove("configuration_receipts");

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema 4 remains readable");
    restored
        .migrate_to_current(project_id())
        .expect("schema 4 promotes deterministically");

    assert_eq!(
        restored.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    assert_eq!(
        restored.simulation_plan.active_plan_name().as_str(),
        "Lab characterization"
    );
    assert_eq!(restored.simulation_plan.plan_count(), 1);
    assert_eq!(
        restored.simulation_plan.active_plan_lineage(),
        crate::workbench::app_state::SimulationPlanLineage::root()
    );
    restored.validate().expect("promoted context validates");
}

#[test]
fn schema_sixteen_migrates_global_model_selection_into_every_plan() {
    let mut manager = ModelLibraryManager::new();
    let library = manager
        .load_library_bytes(
            "foundry.lib",
            b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n"
                .to_vec(),
            Some("FF"),
        )
        .expect("legacy global model selection loads");
    let mut plan = SimSetupState::new();
    plan.clone_active_plan(
        "Legacy second plan",
        crate::workbench::app_state::SimulationPlanCloneOptions::default(),
    )
    .expect("second plan exists");
    let context = context_from_state(&plan, &manager).expect("current context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(SOURCE_QUALIFIED_MODEL_RESOLUTION_SCHEMA_VERSION);
    let persisted_plan = value["simulation_plan"]
        .as_object_mut()
        .expect("simulation plan is an object");
    persisted_plan.remove("model_bindings");
    for stored in persisted_plan["inactive_plans"]
        .as_array_mut()
        .expect("inactive plans are an array")
    {
        stored
            .as_object_mut()
            .expect("stored plan is an object")
            .remove("model_bindings");
    }

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema sixteen remains readable");
    restored
        .migrate_to_current(project_id())
        .expect("global selections migrate to explicit plan bindings");

    assert_eq!(restored.simulation_plan.model_bindings.len(), 1);
    assert_eq!(
        restored.simulation_plan.model_bindings[0].library_name,
        library
    );
    assert_eq!(
        restored.simulation_plan.model_bindings[0]
            .selected_corner
            .as_deref(),
        Some("FF")
    );
    assert_eq!(
        restored.simulation_plan.inactive_plans()[0].model_bindings(),
        restored.simulation_plan.model_bindings.as_slice(),
        "the former global closure applied to every plan and must migrate without behavioral drift"
    );
    restored.validate().expect("migrated context validates");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn schema_six_classifies_legacy_sources_without_inventing_edit_authority() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load external source");
    manager.add_library(ModelLibrary::new("built-in-catalog"));
    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(RETAINED_MODEL_SOURCE_BYTES_SCHEMA_VERSION);
    for library in value["model_libraries"]
        .as_array_mut()
        .expect("libraries array")
    {
        library
            .as_object_mut()
            .expect("library object")
            .remove("source_authority");
    }

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema six remains readable");
    restored
        .migrate_to_current(project_id())
        .expect("schema six migrates");
    restored.validate().expect("migrated context validates");
    assert!(restored.model_libraries.iter().any(|library| {
        library.name == "foundry" && library.source_authority == ModelSourceAuthority::External
    }));
    assert!(restored.model_libraries.iter().any(|library| {
        library.name == "built-in-catalog"
            && library.source_authority == ModelSourceAuthority::BuiltIn
    }));

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[test]
fn schema_seven_migrates_without_inventing_model_authoring_records() {
    let mut manager = ModelLibraryManager::new();
    manager.add_library(ModelLibrary::new("legacy-catalog"));
    let context =
        context_from_state(&SimSetupState::new(), &manager).expect("baseline context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(MODEL_SOURCE_AUTHORITY_SCHEMA_VERSION);
    for library in value["model_libraries"]
        .as_array_mut()
        .expect("libraries are an array")
    {
        library
            .as_object_mut()
            .expect("library is an object")
            .remove("model_definition_metadata");
        library
            .as_object_mut()
            .expect("library is an object")
            .remove("model_qualification");
        library
            .as_object_mut()
            .expect("library is an object")
            .remove("model_correlation");
    }

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema seven remains readable");
    restored
        .migrate_to_current(project_id())
        .expect("schema seven migrates");

    assert_eq!(
        restored.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    assert!(restored.model_libraries.iter().all(|library| {
        library.model_definition_metadata.is_empty()
            && library.model_qualification.is_empty()
            && library.model_correlation.is_empty()
    }));
    restored.validate().expect("migrated context validates");
}

#[test]
fn schema_eight_migrates_without_inventing_correlation_records() {
    let mut manager = ModelLibraryManager::new();
    manager.add_library(ModelLibrary::new("legacy-qualified-catalog"));
    let context =
        context_from_state(&SimSetupState::new(), &manager).expect("baseline context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(MODEL_AUTHORING_QUALIFICATION_SCHEMA_VERSION);
    for library in value["model_libraries"]
        .as_array_mut()
        .expect("libraries are an array")
    {
        library
            .as_object_mut()
            .expect("library is an object")
            .remove("model_correlation");
    }

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema eight remains readable");
    restored
        .migrate_to_current(project_id())
        .expect("schema eight migrates");

    assert_eq!(
        restored.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    assert!(
        restored
            .model_libraries
            .iter()
            .all(|library| library.model_correlation.is_empty())
    );
    restored.validate().expect("migrated context validates");
}

#[test]
fn project_owned_model_round_trip_preserves_authority_bytes_and_revision() {
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "owned_nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Persisted project model".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("kp".to_owned(), 0.001),
        ]),
        string_parameters: std::collections::BTreeMap::from([(
            "revision_tag".to_owned(),
            "r1".to_owned(),
        )]),
    };
    let mut manager = ModelLibraryManager::new();
    let committed = manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");
    let expected_bytes = committed.after.source_contents[0].bytes.clone();
    let expected_authority = committed.after.source_authority;
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: _,
        ..
    } = expected_authority
    else {
        panic!("project model must be source-bound");
    };
    let model = &committed.after.models["owned_nch"];
    let definition = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(model),
        committed.after.model_definition_metadata["owned_nch"].clone(),
    );
    let canonical = definition.canonical_source().unwrap();
    let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
    let model_revision = definition
        .project_source_identity()
        .unwrap()
        .expect("project source identity")
        .revision;
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        "owned_nch",
        source_id,
        model_digest,
        model_revision,
    )
    .unwrap();
    let reference = CorrelationDatasetRevision::try_from_csv(
        "bench-reference",
        crate::product::ObjectRevision::INITIAL,
        "Bench reference",
        CorrelationDatasetClass::BenchMeasurement,
        "test lab",
        "lot-1",
        "fixture-1",
        "calibration-1",
        "bench.csv",
        b"id,quantity,value,unit\nr1,gain,1,V\n".to_vec(),
        None,
    )
    .unwrap();
    let suite = CorrelationSuite::try_new(
        "owned-nch-correlation",
        crate::product::ObjectRevision::INITIAL,
        "Owned NCH correlation",
        "model-owner",
        source,
        vec![reference],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let expected_correlation = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
    manager
        .get_library_mut("owned-models")
        .unwrap()
        .model_correlation
        .insert("owned_nch".to_owned(), expected_correlation.clone());
    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    let mut foreign_evidence = context.clone();
    foreign_evidence.model_libraries[0]
        .model_correlation
        .get_mut("owned_nch")
        .unwrap()
        .suites[0]
        .source
        .source_id = Some(crate::product::ModelSourceId::new());
    let error = foreign_evidence
        .validate()
        .expect_err("foreign project source identity must be rejected");
    assert!(
        error.contains("different project source identity"),
        "{error}"
    );
    let json = serde_json::to_string(&context).expect("context serializes");
    let restored: ProjectExecutionContext =
        serde_json::from_str(&json).expect("context deserializes");
    let (_, restored_manager, warnings) = restored
        .into_state(project_id())
        .expect("project-owned model restores");

    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(
        restored_manager
            .get_library("owned-models")
            .unwrap()
            .model_correlation
            .get("owned_nch"),
        Some(&expected_correlation)
    );
    let library = restored_manager
        .get_library("owned-models")
        .expect("library restored");
    assert_eq!(library.source_authority, expected_authority);
    assert_eq!(library.source_contents[0].bytes, expected_bytes);
    let metadata = library
        .model_definition_metadata
        .get("owned_nch")
        .expect("typed model metadata restored");
    assert_eq!(metadata.parameters.len(), 3);
    metadata.validate().expect("restored metadata validates");
    restored_manager
        .seal_execution_sources()
        .expect("desktop execution consumes retained project bytes");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn retained_import_round_trip_executes_from_authenticated_bytes_after_source_disappears() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    let library_name = manager
        .load_library_file(&path, Some("TT"))
        .expect("load imported source");
    let library = manager
        .get_library_mut(&library_name)
        .expect("loaded library");
    let root = library.root_path.clone().expect("root identity");
    let digest = library
        .source_closure
        .iter()
        .find(|source| source.path == root)
        .expect("root pin")
        .digest;
    library.source_authority = ModelSourceAuthority::RetainedImport {
        source_id: crate::product::ModelSourceId::new(),
        digest,
    };

    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    std::fs::remove_dir_all(directory).expect("remove live imported source");
    let (_, restored, warnings) = context
        .into_state(project_id())
        .expect("retained import restores");
    assert!(warnings.is_empty());
    let restored_library = restored
        .get_library(&library_name)
        .expect("library restores");
    assert!(matches!(
        restored_library.source_authority,
        ModelSourceAuthority::RetainedImport { .. }
    ));
    assert!(!restored_library.source_contents.is_empty());
    let cards = restored
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect("retained bytes execute without live file");
    assert!(cards.join("\n").to_ascii_lowercase().contains(".model nch"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn project_owned_multifile_closure_restores_distinct_member_identities() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load authenticated multi-file fixture");
    let source_id = crate::product::ModelSourceId::new();
    let library = manager
        .get_library_mut("foundry")
        .expect("fixture library exists");
    let root = library.root_path.clone().expect("fixture has a root");
    let root_digest = library
        .source_closure
        .iter()
        .find(|source| source.path == root)
        .expect("root is pinned")
        .digest;
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: crate::product::ObjectRevision::INITIAL,
        digest: root_digest,
    };
    let context = context_from_state(&SimSetupState::new(), &manager)
        .expect("multi-file project-owned closure validates");
    std::fs::remove_dir_all(directory).expect("retained restore must not need fixture files");

    let (_, restored, warnings) = context
        .into_state(project_id())
        .expect("multi-file project-owned closure restores from retained bytes");
    assert!(warnings.is_empty(), "{warnings:?}");
    let library = restored.get_library("foundry").expect("library restores");
    assert_eq!(library.source_closure.len(), 2);
    assert_eq!(library.source_contents.len(), 2);
    assert_eq!(library.source_edges.len(), 1);
    let restored_root = library.root_path.as_ref().expect("root restores");
    assert_eq!(&library.source_edges[0].owner, restored_root);
    assert_ne!(library.source_edges[0].target, *restored_root);
    assert_eq!(
        library.models["helper"].file_path.as_ref(),
        Some(&library.source_edges[0].target)
    );
    assert_eq!(library.models["helper"].source_line, Some(1));
    restored
        .seal_execution_sources()
        .expect("execution seals the complete retained project closure");
}

#[test]
fn project_model_identity_is_bound_to_its_member_digest_and_independent_revision() {
    let definition = ProjectModelDefinition {
        name: "member_nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Included canonical model".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("kp".to_owned(), 0.001),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    };
    let mut manager = ModelLibraryManager::new();
    let base = manager
        .create_project_model("base-model", &definition)
        .expect("base metadata is synthesized");
    let mut metadata = base.after.model_definition_metadata["member_nch"].clone();
    metadata
        .sections
        .push(crate::state::model_library::ModelSectionDefinition {
            name: "TT".to_owned(),
            parent: None,
            overrides: std::collections::BTreeMap::new(),
            model_files: Vec::new(),
            qualification: ModelSectionQualification::Unqualified,
        });
    let revision = ProjectModelRevisionDefinition::new(definition, metadata);
    manager
        .create_project_model_revision(
            "sectioned-model",
            &revision,
            &ModelQualificationState::default(),
        )
        .expect("sectioned source is published");
    let library = manager
        .get_library_mut("sectioned-model")
        .expect("sectioned library exists");
    let root = library
        .root_path
        .clone()
        .expect("project source has a root");
    let member = root.with_file_name("model-member.lib");
    let member_bytes = library.source_contents[0].bytes.clone();
    let member_digest = ContentDigest::from_bytes(Sha256::digest(&member_bytes).into());
    let root_bytes = b".include \"model-member.lib\"\n".to_vec();
    let root_digest = ContentDigest::from_bytes(Sha256::digest(&root_bytes).into());
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        ..
    } = library.source_authority
    else {
        panic!("fixture is project-owned");
    };
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        digest: root_digest,
    };
    library.source_closure = vec![
        ModelSourcePin {
            path: root.clone(),
            digest: root_digest,
        },
        ModelSourcePin {
            path: member.clone(),
            digest: member_digest,
        },
    ];
    library
        .source_closure
        .sort_by(|left, right| left.path.cmp(&right.path));
    library.source_contents = vec![
        ModelSourceContent {
            path: root.clone(),
            bytes: root_bytes,
        },
        ModelSourceContent {
            path: member.clone(),
            bytes: member_bytes,
        },
    ];
    library
        .source_contents
        .sort_by(|left, right| left.path.cmp(&right.path));
    library.source_edges = vec![ModelSourceEdge {
        owner: root,
        requested_path: "model-member.lib".to_owned(),
        target: member.clone(),
    }];
    library
        .models
        .get_mut("member_nch")
        .expect("model projection exists")
        .file_path = Some(member.clone());
    for corner in library.corners.values_mut() {
        corner.file_path = Some(member.clone());
    }
    let _ = library;
    let library = manager
        .get_library_mut("sectioned-model")
        .expect("sectioned library still exists");
    let model_revision = crate::product::ObjectRevision::new(7).expect("fixture revision");
    let metadata = library
        .model_definition_metadata
        .get_mut("member_nch")
        .expect("typed metadata exists");
    metadata
        .source_identity
        .as_mut()
        .expect("base model identity is retained")
        .revision = model_revision.get();
    for section in &mut metadata.sections {
        section.model_files[0].revision = model_revision.get();
    }

    let context = context_from_state(&SimSetupState::new(), &manager).expect(
            "model identity may use its canonical member digest and revision independently of the library root",
        );
    let persisted = context
        .model_libraries
        .iter()
        .find(|library| library.name == "sectioned-model")
        .expect("sectioned model persists");
    let ModelSourceAuthority::ProjectOwned {
        revision: persisted_library_revision,
        digest: persisted_root_digest,
        ..
    } = persisted.source_authority
    else {
        panic!("persisted fixture is project-owned");
    };
    let identity = persisted.model_definition_metadata["member_nch"]
        .source_identity
        .as_ref()
        .expect("model identity persists");
    assert_eq!(identity.revision, model_revision.get());
    assert_ne!(identity.revision, persisted_library_revision.get());
    assert_eq!(identity.content_digest, member_digest.to_string());
    assert_ne!(member_digest, persisted_root_digest);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn project_owned_load_rejects_a_tampered_serialized_model_projection() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load authenticated multi-file fixture");
    let source_id = crate::product::ModelSourceId::new();
    let library = manager
        .get_library_mut("foundry")
        .expect("fixture library exists");
    let root = library.root_path.clone().expect("fixture has a root");
    let root_digest = library
        .source_closure
        .iter()
        .find(|source| source.path == root)
        .expect("root is pinned")
        .digest;
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: crate::product::ObjectRevision::INITIAL,
        digest: root_digest,
    };
    let mut context = context_from_state(&SimSetupState::new(), &manager)
        .expect("untampered projection validates");
    std::fs::remove_dir_all(directory).expect("remove model fixture");
    context.model_libraries[0]
        .models
        .get_mut("helper")
        .expect("helper projection exists")
        .parameters
        .insert("kp".to_owned(), 0.75);

    let error = context
        .validate()
        .expect_err("serialized projection cannot diverge from retained model cards");
    assert!(error.contains("not an exact projection"), "{error}");
}

#[test]
fn project_load_rejects_qualified_section_without_exact_retained_evidence() {
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "owned_nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Persisted project model".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("kp".to_owned(), 0.001),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    };
    let mut manager = ModelLibraryManager::new();
    manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");
    let mut context =
        context_from_state(&SimSetupState::new(), &manager).expect("base context validates");
    let library = context
        .model_libraries
        .iter_mut()
        .find(|library| library.name == "owned-models")
        .expect("project model library persists");
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest,
    } = library.source_authority
    else {
        panic!("fixture source is project-owned");
    };
    library
        .model_definition_metadata
        .get_mut("owned_nch")
        .expect("fixture metadata")
        .sections
        .push(crate::state::model_library::ModelSectionDefinition {
            name: "TT".to_owned(),
            parent: None,
            overrides: std::collections::BTreeMap::new(),
            model_files: vec![crate::state::model_library::ModelFileIdentity {
                source_id: source_id.to_string(),
                revision: revision.get(),
                content_digest: digest.to_string(),
                display_name: "definition.model".to_owned(),
            }],
            qualification: ModelSectionQualification::Qualified {
                evidence_digest: Some("0".repeat(64)),
            },
        });
    let bound = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(&library.models["owned_nch"]),
        library.model_definition_metadata["owned_nch"].clone(),
    )
    .bind_project_source_identity(source_id, revision, "definition.model")
    .expect("fixture section identity binds to its canonical model digest");
    let canonical_source = bound
        .canonical_source()
        .expect("fixture canonical source renders")
        .into_bytes();
    let identity = bound
        .project_source_identity()
        .expect("fixture source identity validates")
        .expect("sectioned fixture has a source identity");
    library
        .model_definition_metadata
        .insert("owned_nch".to_owned(), bound.metadata);
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest: identity.content_digest,
    };
    library.source_closure[0].digest = identity.content_digest;
    library.source_contents[0].bytes = canonical_source;

    let error = context
        .validate()
        .expect_err("a qualified section cannot invent its evidence digest");
    assert!(
        error.contains("claims qualified evidence without a retained qualification record"),
        "{error}"
    );
}

#[test]
fn current_state_save_fails_closed_without_a_stable_plan() {
    let mut plan = SimSetupState::new();
    plan.analysis_plan = None;

    let error = context_from_state(&plan, &ModelLibraryManager::new())
        .expect_err("current-state persistence must never invoke legacy migration");

    assert!(error.contains("legacy singleton migration is load-only"));
    assert!(plan.analysis_plan.is_none());
}

#[test]
fn restored_execution_lifecycle_never_retains_runner_authority() {
    let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
        .expect("baseline context validates");
    let baseline = serde_json::to_value(context).expect("context serializes");

    for lifecycle in ["queued", "running", "paused"] {
        let mut value = baseline.clone();
        value["simulation_plan"]["analysis_plan"]["instances"][0]["lifecycle"] =
            serde_json::json!(lifecycle);
        let mut restored: ProjectExecutionContext =
            serde_json::from_value(value).expect("persisted lifecycle deserializes");
        restored
            .migrate_to_current(project_id())
            .expect("current context restores");
        let instance = &restored
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan restored")
            .instances()[0];
        let id = instance.id();
        assert_eq!(instance.lifecycle(), AnalysisLifecycleState::Draft);
        restored.validate().expect("normalized context validates");

        let (mut setup, _, _) = restored
            .into_state(project_id())
            .expect("normalized context enters application state");
        setup
            .stable_analysis_plan_mut()
            .expect("stable plan remains present")
            .edit(id, |_| ())
            .expect("stale runner lifecycle cannot lock the restored draft");
    }
}

#[test]
fn incomplete_disabled_and_enabled_analysis_drafts_round_trip_losslessly() {
    let mut plan = SimSetupState::new();
    let stable = plan
        .analysis_plan
        .as_mut()
        .expect("current setup owns a stable plan");
    let transient_id = stable.instances()[0].id();
    stable
        .edit(transient_id, |draft| {
            let AnalysisDraft::Transient(transient) = draft else {
                panic!("default instance must be transient");
            };
            transient.stop = "also unfinished".to_owned();
        })
        .expect("transient draft edit commits");
    let (pss_id, _) = stable.insert(AnalysisKind::Pss).expect("PSS inserts");
    stable
        .edit(pss_id, |draft| {
            let AnalysisDraft::Pss(pss) = draft else {
                panic!("inserted instance must be PSS");
            };
            pss.fund_freq = "unfinished-expression(".to_owned();
        })
        .expect("PSS draft edit commits");
    // PSS proves a disabled draft is retained; Transient proves an enabled
    // invalid draft is persistable but remains blocked by run validation.
    stable
        .set_enabled(pss_id, false)
        .expect("PSS disables without losing its position");

    let context = context_from_state(&plan, &ModelLibraryManager::new())
        .expect("draft validity is a run concern, not a persistence concern");
    let serialized = serde_json::to_string(&context).expect("context serializes");
    let restored: ProjectExecutionContext =
        serde_json::from_str(&serialized).expect("context deserializes");
    let (restored, _, _) = restored.into_state(project_id()).expect("context restores");
    let restored = restored
        .stable_analysis_plan()
        .expect("v4 restores a stable plan");
    assert_eq!(
        restored
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>(),
        vec![transient_id, pss_id]
    );
    let transient = restored.instance(transient_id).expect("transient retained");
    let AnalysisDraft::Transient(transient_draft) = transient.draft() else {
        panic!("transient identity must retain its kind");
    };
    assert_eq!(transient_draft.stop, "also unfinished");
    assert!(transient.enabled());
    let pss = restored.instance(pss_id).expect("PSS retained");
    let AnalysisDraft::Pss(pss_draft) = pss.draft() else {
        panic!("PSS identity must retain its kind");
    };
    assert_eq!(pss_draft.fund_freq, "unfinished-expression(");
    assert!(!pss.enabled());
    assert!(pss.dependencies().is_empty());
}

#[test]
fn duplicate_and_unknown_stable_analysis_data_fail_precisely() {
    let context = context_from_state(&SimSetupState::new(), &ModelLibraryManager::new())
        .expect("baseline context validates");
    let mut duplicate_value = serde_json::to_value(&context).expect("context serializes");
    let instances = duplicate_value["simulation_plan"]["analysis_plan"]["instances"]
        .as_array_mut()
        .expect("v4 instances are an array");
    let duplicate_instance = instances[0].clone();
    instances.push(duplicate_instance);
    let duplicate: ProjectExecutionContext =
        serde_json::from_value(duplicate_value).expect("shape deserializes before validation");
    let error = duplicate
        .validate()
        .expect_err("duplicate identity must fail");
    assert!(error.contains("appears more than once"));

    let mut unknown_value = serde_json::to_value(context).expect("context serializes");
    unknown_value["simulation_plan"]["analysis_plan"]["instances"][0]["kind"] =
        serde_json::Value::String("future-analysis".to_owned());
    let error = serde_json::from_value::<ProjectExecutionContext>(unknown_value)
        .expect_err("unknown stable analysis kind must fail closed")
        .to_string();
    assert!(
        error.contains("unknown variant `future-analysis`"),
        "unexpected unknown-analysis diagnostic: {error}"
    );
}

#[test]
fn legacy_context_migrates_to_sorted_execution_order() {
    let mut plan = SimSetupState::new();
    plan.ensure_initialized();
    plan.analysis_plan = None;
    plan.enabled.extend([4, 0]);
    plan.analysis_order.clear();
    let mut context = ProjectExecutionContext {
        schema_version: LEGACY_EXECUTION_CONTEXT_SCHEMA_VERSION,
        simulation_plan: plan,
        model_libraries: Vec::new(),
        model_resolution_records: Vec::new(),
        model_validation_receipt: None,
    };

    context
        .migrate_to_current(project_id())
        .expect("legacy migration");

    let migrated = context
        .simulation_plan
        .stable_analysis_plan()
        .expect("legacy context migrates to stable identity");
    assert_eq!(migrated.instances().len(), AnalysisKind::ALL.len());
    assert_eq!(
        migrated
            .instances()
            .iter()
            .take(3)
            .map(|instance| (instance.kind(), instance.enabled()))
            .collect::<Vec<_>>(),
        vec![
            (AnalysisKind::OperatingPoint, true),
            (AnalysisKind::Transient, true),
            (AnalysisKind::Noise, true),
        ]
    );
    let noise = migrated
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Noise)
        .expect("noise migrated");
    let op = migrated
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::OperatingPoint)
        .expect("OP migrated");
    assert_eq!(noise.dependencies().len(), 1);
    assert_eq!(noise.dependencies()[0].target(), op.id());
    context.validate().expect("migrated context validates");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn model_source_and_section_bindings_round_trip_without_substitution() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("FF"))
        .expect("load FF section");
    {
        let corner = manager
            .get_library_mut("foundry")
            .and_then(|library| library.corners.get_mut("FF"))
            .expect("FF corner exists");
        corner
            .section_bindings
            .push(CornerSectionBinding::new(CornerSectionDomain::Aging, "FF"));
        corner.required_domains.push(CornerSectionDomain::Aging);
        corner.minimum_temperature_c = Some(-40.0);
        corner.maximum_temperature_c = Some(150.0);
    }
    let expected = manager
        .reference_process_model_cards(crate::product::ProcessCorner::FF)
        .expect("source FF binding");
    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    let canonical_root = context.model_libraries[0]
        .root_path
        .clone()
        .expect("external library keeps canonical root");
    let json = serde_json::to_string(&context).expect("context serializes");
    let restored_context: ProjectExecutionContext =
        serde_json::from_str(&json).expect("context deserializes");

    let (_, restored_manager, warnings) = restored_context
        .into_state(project_id())
        .expect("available source restores");

    assert!(warnings.is_empty());
    assert_eq!(
        restored_manager
            .reference_process_model_cards(crate::product::ProcessCorner::FF,)
            .expect("restored FF binding"),
        expected
    );
    assert_eq!(
        restored_manager
            .get_library("foundry")
            .expect("library restored")
            .selected_corner
            .as_deref(),
        Some("FF")
    );
    let restored_corner = restored_manager
        .get_library("foundry")
        .and_then(|library| library.corners.get("FF"))
        .expect("typed FF corner restores");
    assert_eq!(
        restored_corner
            .section_bindings
            .iter()
            .find(|binding| binding.domain == CornerSectionDomain::Aging)
            .map(|binding| binding.section.as_str()),
        Some("FF")
    );
    assert!(
        restored_corner
            .required_domains
            .contains(&CornerSectionDomain::Aging)
    );
    assert_eq!(restored_corner.minimum_temperature_c, Some(-40.0));
    assert_eq!(restored_corner.maximum_temperature_c, Some(150.0));
    assert_eq!(
        restored_manager
            .get_library("foundry")
            .expect("library restored")
            .source_closure
            .iter()
            .find(|source| source.path == canonical_root)
            .expect("root pin restored")
            .digest,
        ModelLibraryManager::calculate_source_digest(&path).expect("fixture digest computes")
    );
    assert_eq!(
        restored_manager
            .get_library("foundry")
            .expect("library restored")
            .source_closure
            .len(),
        2
    );
    let restored_library = restored_manager
        .get_library("foundry")
        .expect("library restored");
    assert_eq!(restored_library.source_edges.len(), 1);
    assert_eq!(restored_library.source_edges[0].owner, canonical_root);
    assert_eq!(
        restored_library.source_edges[0].requested_path,
        "shared.inc"
    );

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn schema_ten_corner_names_migrate_to_required_composite_contracts() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load legacy-shaped source");
    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    value["schema_version"] = serde_json::json!(MODEL_BIN_AUDIT_SCHEMA_VERSION);
    for library in value["model_libraries"]
        .as_array_mut()
        .expect("libraries array")
    {
        for corner in library["corners"]
            .as_object_mut()
            .expect("corner map")
            .values_mut()
        {
            let corner = corner.as_object_mut().expect("corner object");
            corner.remove("section_bindings");
            corner.remove("required_domains");
            corner.remove("minimum_temperature_c");
            corner.remove("maximum_temperature_c");
        }
    }

    let mut restored: ProjectExecutionContext =
        serde_json::from_value(value).expect("schema ten shape decodes");
    restored
        .migrate_to_current(project_id())
        .expect("schema ten migrates");
    assert_eq!(
        restored.schema_version,
        PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION
    );
    let (_, restored_manager, _) = restored
        .into_state(project_id())
        .expect("migrated state restores");
    let corner = restored_manager
        .get_library("foundry")
        .and_then(|library| library.corners.get("TT"))
        .expect("TT corner restores");
    assert_eq!(
        corner.effective_required_domains(),
        vec![CornerSectionDomain::Composite]
    );
    assert_eq!(
        corner.effective_section_bindings(),
        vec![CornerSectionBinding::new(
            CornerSectionDomain::Composite,
            "TT"
        )]
    );

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn schema_one_external_source_migrates_unpinned_and_stays_blocked_until_refresh() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load source");
    let mut context =
        context_from_state(&SimSetupState::new(), &manager).expect("current context validates");
    context.schema_version = UNPINNED_MODEL_SOURCE_SCHEMA_VERSION;
    context.model_libraries[0].source_closure.clear();

    let (_, restored, warnings) = context
        .into_state(project_id())
        .expect("legacy unpinned catalog remains recoverable");

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("legacy binding is not content-pinned"));
    let blocked = restored
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("unpinned legacy source must not run");
    assert!(blocked.contains("is not content-pinned"));

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn schema_two_multifile_source_migrates_without_inventing_resolution_edges() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load multifile source");
    let mut context =
        context_from_state(&SimSetupState::new(), &manager).expect("current context validates");
    context.schema_version = PATH_PINNED_MODEL_SOURCE_SCHEMA_VERSION;
    context.model_libraries[0].source_edges.clear();

    let (_, restored, warnings) = context
        .into_state(project_id())
        .expect("schema-two catalog remains repairable");
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("no authenticated dependency-resolution graph"));
    let blocked = restored
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("missing legacy graph must block a multifile source");
    assert!(
        blocked.contains("no authenticated resolution edge"),
        "{blocked}"
    );

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn unavailable_or_changed_model_source_is_retained_warned_and_run_blocked() {
    let (directory, path) = model_fixture();
    let mut manager = ModelLibraryManager::new();
    manager
        .load_library_file(&path, Some("TT"))
        .expect("load model source");
    let context = context_from_state(&SimSetupState::new(), &manager).expect("context validates");
    let canonical_root = context.model_libraries[0]
        .root_path
        .clone()
        .expect("external library keeps canonical root");

    std::fs::remove_file(&path).expect("remove source");
    let (_, retained, warnings) = context
        .clone()
        .into_state(project_id())
        .expect("missing source is retained for repair");
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("is unavailable"));
    assert_eq!(
        retained
            .get_library("foundry")
            .expect("binding retained")
            .root_path
            .as_deref(),
        Some(canonical_root.as_path())
    );
    let unavailable = retained
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("missing source must block binding");
    assert!(unavailable.contains("is unavailable"));

    std::fs::write(
            &path,
            ".include \"shared.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=9e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=8e-3)\n.endl FF\n",
        )
        .expect("write changed source");
    let (_, retained, warnings) = context
        .into_state(project_id())
        .expect("changed source must not discard persisted catalog");
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("differs from the explicitly accepted SHA-256"));
    let changed = retained
        .reference_process_model_cards(crate::product::ProcessCorner::TT)
        .expect_err("changed source must block binding");
    assert!(changed.contains("dependency changed at"));

    std::fs::remove_dir_all(directory).expect("remove model fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn foreign_platform_source_binding_is_retained_without_filesystem_probe() {
    #[cfg(windows)]
    let root = PathBuf::from("/opt/foundry/models/device.lib");
    #[cfg(not(windows))]
    let root = PathBuf::from(r"C:\Foundry\Models\device.lib");

    assert!(is_portable_absolute_path(&root));
    assert!(is_foreign_platform_absolute_path(&root));

    let context = ProjectExecutionContext {
        schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
        simulation_plan: SimSetupState::new(),
        model_libraries: vec![ProjectModelLibrary {
            name: "foreign-foundry".to_owned(),
            pdk_name: String::new(),
            technology_node: String::new(),
            pack_id: None,
            pack_pin: None,
            root_path: Some(root.clone()),
            source_authority: ModelSourceAuthority::External,
            source_closure: vec![ModelSourcePin {
                path: root.clone(),
                digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
            }],
            source_contents: Vec::new(),
            source_edges: Vec::new(),
            models: HashMap::new(),
            top_level_models: HashMap::new(),
            section_models: HashMap::new(),
            subcircuits: HashMap::new(),
            model_definition_metadata: HashMap::new(),
            model_qualification: HashMap::new(),
            model_correlation: HashMap::new(),
            corners: HashMap::new(),
            selected_corner: None,
            version: String::new(),
        }],
        model_resolution_records: Vec::new(),
        model_validation_receipt: None,
    };

    context
        .validate()
        .expect("foreign desktop syntax remains valid project metadata");
    let (_, manager, warnings) = context
        .into_state(project_id())
        .expect("foreign binding remains retained for repair");
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("foreign-platform"), "{:?}", warnings);

    let blocked = manager
        .seal_execution_sources()
        .expect_err("execution must fail before probing a foreign path");
    assert!(
        blocked.contains("foreign-platform") || blocked.contains("non-canonical"),
        "{blocked}"
    );
}

#[test]
fn disconnected_source_subgraph_is_rejected_even_when_every_member_has_an_edge() {
    let directory = std::env::temp_dir().join("rspice-disconnected-persisted-graph");
    let root = directory.join("root.lib");
    let reachable = directory.join("reachable.inc");
    let orphan = directory.join("orphan.inc");
    let digest = crate::product::ContentDigest::from_bytes([0x33; 32]);
    let mut source_closure = vec![
        ModelSourcePin {
            path: root.clone(),
            digest,
        },
        ModelSourcePin {
            path: reachable.clone(),
            digest,
        },
        ModelSourcePin {
            path: orphan.clone(),
            digest,
        },
    ];
    source_closure.sort_by(|left, right| left.path.cmp(&right.path));
    let mut source_edges = vec![
        ModelSourceEdge {
            owner: root.clone(),
            requested_path: "reachable.inc".to_owned(),
            target: reachable,
        },
        ModelSourceEdge {
            owner: orphan.clone(),
            requested_path: "orphan.inc".to_owned(),
            target: orphan.clone(),
        },
    ];
    source_edges.sort();
    let context = ProjectExecutionContext {
        schema_version: PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
        simulation_plan: SimSetupState::new(),
        model_libraries: vec![ProjectModelLibrary {
            name: "disconnected".to_owned(),
            pdk_name: String::new(),
            technology_node: String::new(),
            pack_id: None,
            pack_pin: None,
            root_path: Some(root),
            source_authority: ModelSourceAuthority::External,
            source_closure,
            source_contents: Vec::new(),
            source_edges,
            models: HashMap::new(),
            top_level_models: HashMap::new(),
            section_models: HashMap::new(),
            subcircuits: HashMap::new(),
            model_definition_metadata: HashMap::new(),
            model_qualification: HashMap::new(),
            model_correlation: HashMap::new(),
            corners: HashMap::new(),
            selected_corner: None,
            version: String::new(),
        }],
        model_resolution_records: Vec::new(),
        model_validation_receipt: None,
    };

    let error = context
        .validate()
        .expect_err("all closure members must be root-reachable");
    assert!(error.contains("not reachable from root_path"), "{error}");
}
