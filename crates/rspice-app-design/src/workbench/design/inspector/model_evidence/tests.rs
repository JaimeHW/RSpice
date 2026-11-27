//! Model evidence names the model an instance runs, and states a problem only
//! when its binding has one.

use super::*;
use crate::state::ComponentType;
use crate::workbench::design::test_host::DesignTestHost;

#[test]
fn native_component_never_claims_a_resolved_catalog_model() {
    let state = DesignTestHost::default();
    let component = Component::new(1, ComponentType::Resistor, crate::state::Point::origin());

    let evidence = component_model_evidence(&state.model, &component);

    assert_eq!(evidence.model, None, "a resistor names no model");
    assert_eq!(evidence.problem, None);
}

#[test]
fn missing_explicit_model_is_reported_as_unresolved_review_evidence() {
    let state = DesignTestHost::default();
    let component = Component::new(2, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state.model, &component);

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    let (problem, text) = evidence.problem.expect("a missing model is a problem");
    assert_eq!(problem, Problem::Failure);
    assert_eq!(text, "Model vendor_npn not found in the loaded libraries");
}

#[test]
fn a_model_resolves_only_from_a_file_its_library_records() {
    let mut state = DesignTestHost::default();
    let path = std::path::PathBuf::from("models/vendor.lib");
    let mut library = crate::state::model_library::ModelLibrary::new("vendor");
    library.root_path = Some(path.clone());
    library
        .source_closure
        .push(crate::state::model_library::ModelSourcePin {
            path: path.clone(),
            digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        });
    let mut model = crate::state::model_library::DeviceModel::new(
        "vendor_npn",
        crate::state::model_library::ModelType::Npn,
    );
    model.file_path = Some(path);
    library.add_model(model);
    state.model.model_library_manager.add_library(library);
    let component = Component::new(3, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state.model, &component);

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    assert_eq!(evidence.problem, None, "a recorded file says nothing more");

    let library = state
        .model
        .model_library_manager
        .get_library_mut("vendor")
        .expect("the fixture library");
    library.source_closure.clear();
    let evidence = component_model_evidence(&state.model, &component);
    let (problem, text) = evidence
        .problem
        .expect("a file the library does not record is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(text.contains("vendor.lib"), "{text}");
}

#[test]
fn bound_model_evidence_uses_the_instance_model_and_section_overrides() {
    let mut state = DesignTestHost::default();
    let mut library = crate::state::model_library::ModelLibrary::new("vendor_analog");
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_A",
        crate::state::model_library::ModelType::Other,
    ));
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_B",
        crate::state::model_library::ModelType::Other,
    ));
    library
        .model_definition_metadata
        .insert("OPA189_B".to_owned(), sectioned(&["high_accuracy"]));
    state.model.model_library_manager.add_library(library);

    let mut binding = crate::state::LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
    binding.module_name = Some("OPA189_B".to_owned());
    binding.model_section = Some("high_accuracy".to_owned());
    let component = Component::new(
        9,
        ComponentType::CellInstance,
        crate::state::Point::origin(),
    )
    .with_library_cell(binding)
    .with_name_value("XU1", "OPA189");

    let evidence = component_model_evidence(&state.model, &component);

    assert_eq!(evidence.model.as_deref(), Some("OPA189_B"));
    assert_eq!(evidence.problem, None);
}

/// Metadata stating `sections` and nothing else.
fn sectioned(sections: &[&str]) -> crate::state::model_library::ModelDefinitionMetadata {
    use crate::state::model_library::{ModelDefinitionMetadata, ModelSectionDefinition};
    ModelDefinitionMetadata {
        schema_version: 1,
        source_identity: None,
        parameters: Vec::new(),
        sections: sections
            .iter()
            .map(|name| ModelSectionDefinition {
                name: (*name).to_owned(),
                parent: None,
                overrides: std::collections::BTreeMap::new(),
                model_files: Vec::new(),
            })
            .collect(),
        statistics: Default::default(),
        temperature_laws: Vec::new(),
    }
}

/// A section the bound model does not define is a failure on the Model row,
/// and the Section row marks it as the one the model lacks.
#[test]
fn a_section_the_model_does_not_define_is_a_failure() {
    let mut state = DesignTestHost::default();
    let mut library = crate::state::model_library::ModelLibrary::new("vendor_analog");
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_B",
        crate::state::model_library::ModelType::Other,
    ));
    library
        .model_definition_metadata
        .insert("OPA189_B".to_owned(), sectioned(&["typical", "worst"]));
    state.model.model_library_manager.add_library(library);
    let mut binding = crate::state::LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
    binding.module_name = Some("OPA189_B".to_owned());
    binding.model_section = Some("fast".to_owned());
    let component = Component::new(
        9,
        ComponentType::CellInstance,
        crate::state::Point::origin(),
    )
    .with_library_cell(binding)
    .with_name_value("XU1", "OPA189");

    let evidence = component_model_evidence(&state.model, &component);
    assert_eq!(evidence.model.as_deref(), Some("OPA189_B"));
    assert_eq!(
        evidence.problem,
        Some((
            Problem::Failure,
            "Section fast is not defined for model OPA189_B".to_owned()
        ))
    );
    let choices =
        super::super::component::bound_model_section_choices(&state.model, &component, "OPA189_B");
    let labels = choices
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        ["default", "typical", "worst", "fast \u{b7} not defined"]
    );
}

/// A binned family is named by the family the instance binds, even when one
/// bin is at fault; the problem names that bin.
#[test]
fn a_binned_family_with_a_bad_bin_is_named_by_the_family() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelSourcePin, ModelType};
    let mut state = DesignTestHost::default();
    let path = std::path::PathBuf::from("models/nch.lib");
    let mut library = ModelLibrary::new("vendor");
    library.root_path = Some(path.clone());
    library.source_closure.push(ModelSourcePin {
        path: path.clone(),
        digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
    });
    for (bin, file) in [
        ("nch.1", path.clone()),
        ("nch.2", std::path::PathBuf::from("models/elsewhere.lib")),
    ] {
        let mut card = DeviceModel::new(bin, ModelType::Nmos);
        card.file_path = Some(file);
        library.add_model(card);
    }
    state.model.model_library_manager.add_library(library);
    let binding = crate::state::LibraryCellInstance::new("vendor", "nch", "spice");
    let component = Component::new(
        10,
        ComponentType::CellInstance,
        crate::state::Point::origin(),
    )
    .with_library_cell(binding)
    .with_name_value("XM1", "nch");

    let evidence = component_model_evidence(&state.model, &component);
    assert_eq!(evidence.model.as_deref(), Some("nch"));
    let (problem, text) = evidence.problem.expect("the stray bin is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(
        text.contains("elsewhere.lib") && text.contains("nch.2"),
        "{text}"
    );
}

/// A module with no current compile says which of three things is true: it
/// has never compiled, its last compile of this source failed, or it
/// compiled and has changed since.
#[test]
fn a_module_without_a_current_compile_says_why() {
    use crate::state::{
        Cell, Library, LibraryCellInstance, ProjectSourceBundle, ProjectSourceOwner, View, ViewType,
    };
    let mut state = DesignTestHost::default();
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("veriloga", ViewType::VerilogA));
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    let reference = CellViewRef::new("behavioral", "amp", "veriloga");
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        "amp.va".to_owned(),
        "module amp(p, n);\nendmodule\n".to_owned(),
        [],
        [],
    )
    .expect("a bundle");
    let id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("the bundle is kept");
    let component = Component::new(
        11,
        ComponentType::CellInstance,
        crate::state::Point::origin(),
    )
    .with_library_cell(LibraryCellInstance::new("behavioral", "amp", "veriloga"))
    .with_name_value("XAMP", "amp");
    let problem = |state: &DesignTestHost| {
        component_model_evidence(&state.model, &component)
            .problem
            .expect("an uncompiled module is a problem")
    };

    assert_eq!(
        problem(&state),
        (
            Problem::Failure,
            "Not compiled: compile the module to run it".to_owned()
        )
    );

    let attempt = |state: &DesignTestHost, passed: bool| {
        let bundle = state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(id)
            .expect("the bundle");
        crate::state::ProjectSourceQualificationAttempt {
            attempt_id: uuid::Uuid::from_u128(if passed { 2 } else { 1 }),
            recorded_at_unix_ms: 1,
            source_revision: bundle.revision().get(),
            source_closure_digest: bundle.closure_digest(),
            profile_digest: crate::product::ContentDigest::from_bytes([1; 32]),
            package_name: "amp".to_owned(),
            package_version: "0.1.0".to_owned(),
            selected_module: "amp".to_owned(),
            compiler_version: "test".to_owned(),
            disposition: if passed {
                crate::state::ProjectSourceQualificationDisposition::Passed
            } else {
                crate::state::ProjectSourceQualificationDisposition::Failed
            },
            report_digest: crate::product::ContentDigest::from_bytes([2; 32]),
            targets: Vec::new(),
            checks: Vec::new(),
            error_count: if passed { 0 } else { 2 },
            warning_count: 0,
        }
    };
    let failed = attempt(&state, false);
    state
        .model
        .workspace
        .content
        .append_project_source_qualification(id, failed)
        .expect("the attempt is recorded");
    assert_eq!(
        problem(&state),
        (Problem::Failure, "Compile failed: 2 errors".to_owned())
    );

    let passed = attempt(&state, true);
    state
        .model
        .workspace
        .content
        .append_project_source_qualification(id, passed)
        .expect("the attempt is recorded");
    state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(
            id,
            "amp.va",
            "module amp(p, n);\n// edited\nendmodule\n".to_owned(),
        )
        .expect("the edit applies");
    assert_eq!(
        problem(&state),
        (
            Problem::Warning,
            "Recompile needed: the module changed since its compile".to_owned()
        )
    );
}
