//! Model evidence names the model an instance runs, and states a problem only
//! when its binding has one.

use super::*;
use crate::state::ComponentType;

#[test]
fn native_component_never_claims_a_resolved_catalog_model() {
    let state = AppState::default();
    let component = Component::new(1, ComponentType::Resistor, crate::state::Point::origin());

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model, None, "a resistor names no model");
    assert_eq!(evidence.problem, None);
}

#[test]
fn missing_explicit_model_is_reported_as_unresolved_review_evidence() {
    let state = AppState::default();
    let component = Component::new(2, ComponentType::NpnBjt, crate::state::Point::origin())
        .with_name_value("Q1", "vendor_npn");

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    let (problem, text) = evidence.problem.expect("a missing model is a problem");
    assert_eq!(problem, Problem::Failure);
    assert_eq!(text, "Model vendor_npn not found in the loaded libraries");
}

#[test]
fn a_model_resolves_only_from_a_file_its_library_records() {
    let mut state = AppState::default();
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

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model.as_deref(), Some("vendor_npn"));
    assert_eq!(evidence.problem, None, "a recorded file says nothing more");

    let library = state
        .model
        .model_library_manager
        .get_library_mut("vendor")
        .expect("the fixture library");
    library.source_closure.clear();
    let evidence = component_model_evidence(&state, &component);
    let (problem, text) = evidence
        .problem
        .expect("a file the library does not record is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(text.contains("vendor.lib"), "{text}");
}

#[test]
fn bound_model_evidence_uses_the_instance_model_and_section_overrides() {
    let mut state = AppState::default();
    let mut library = crate::state::model_library::ModelLibrary::new("vendor_analog");
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_A",
        crate::state::model_library::ModelType::Other,
    ));
    library.add_model(crate::state::model_library::DeviceModel::new(
        "OPA189_B",
        crate::state::model_library::ModelType::Other,
    ));
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

    let evidence = component_model_evidence(&state, &component);

    assert_eq!(evidence.model.as_deref(), Some("OPA189_B"));
    assert_eq!(evidence.problem, None);
}
