//! The inspector edits a binding, never a copy of one.
//!
//! The axis-range field's text and parser are pinned beside them, in
//! `documents/result_document/axis_range.rs`. Scroll memory is per subject and
//! bounded, so two schematic subjects cannot inherit each other's offset.

use super::design::bind_component_model_from_catalog;
use super::*;
use crate::state::{Component, ComponentType};
use crate::workbench::RSpiceApp;

fn catalog_test_diode(params: &str) -> (RSpiceApp, u64) {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager.clear();
    let mut library = ModelLibrary::new("models");
    library.add_model(DeviceModel::new("junction", ModelType::Diode));
    app.state.model.model_library_manager.add_library(library);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::origin());
    app.state.model.schematic.document_mut_for_test().components[0].params = params.to_owned();
    app.state.model.schematic.clear_undo_history();
    app.state.model.schematic.session.is_dirty = false;
    (app, id)
}

/// The Models workspace binds a binned family by its name, after checking
/// every bin suits the instance: any of them may be the one its size selects.
#[test]
fn catalog_binding_binds_a_binned_family_whose_every_bin_suits_the_instance() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager.clear();
    let mut kit = ModelLibrary::new("kit");
    for bin in ["nch.1", "nch.2"] {
        let mut card = DeviceModel::new(bin, ModelType::Nmos);
        card.spice_type = Some("NMOS".to_owned());
        kit.add_model(card);
    }
    app.state.model.model_library_manager.add_library(kit);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let nmos = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Nmos, crate::state::Point::origin());
    let pmos = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Pmos, crate::state::Point::new(200, 0));

    let refusal = validate_component_model_catalog_binding(&app.state, pmos, "kit", "nch")
        .expect_err("an NMOS family does not suit a PMOS instance");
    assert!(!refusal.contains("no longer present"), "{refusal}");
    bind_component_model_from_catalog(&mut app, nmos, "kit", "nch").expect("bind the family");
    let params = &app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == nmos)
        .expect("nmos")
        .params;
    assert!(params.ends_with("model=nch model_library=kit"), "{params}");
}

#[test]
fn catalog_binding_preserves_raw_parameters_and_refuses_malformed_text_atomically() {
    let original = r#"off note="a  b" expr={a + f(b,c)}"#;
    let (mut app, id) = catalog_test_diode(original);
    bind_component_model_from_catalog(&mut app, id, "models", "junction").unwrap();
    assert_eq!(
        app.state.model.schematic.document().components[0].params,
        format!("{original} model=junction model_library=models")
    );
    assert!(app.state.model.schematic.undo());
    assert_eq!(
        app.state.model.schematic.document().components[0].params,
        original
    );
    assert!(!app.state.model.schematic.can_undo());

    let (mut app, id) = catalog_test_diode("note='unterminated");
    let before = app.state.model.schematic.document().components[0].clone();
    assert!(
        bind_component_model_from_catalog(&mut app, id, "models", "junction")
            .unwrap_err()
            .contains("unterminated")
    );
    assert_eq!(app.state.model.schematic.document().components[0], before);
    assert!(!app.state.model.schematic.session.is_dirty);
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn catalog_binding_resolves_the_pending_inspector_draft_before_model_publication() {
    use crate::workbench::state::{InlineEditField, InlineEditSession};
    let (mut app, id) = catalog_test_diode(r#"note="a  b""#);
    let expected = app.state.model.schematic.document().components[0].clone();
    let mut candidate = expected.clone();
    candidate.name = "D42".to_owned();
    let authority = app.state.inline_edit_authority();
    app.state.workbench.inline_edit.begin(InlineEditSession {
        expected: expected.clone(),
        field: InlineEditField::Instance,
        authority,
        description: "rename instance".to_owned(),
        buffer: "invalid name".to_owned(),
        candidate: None,
        error: Some("Invalid name".to_owned()),
        widget: None,
    });
    assert!(bind_component_model_from_catalog(&mut app, id, "models", "junction").is_err());
    assert_eq!(app.state.model.schematic.document().components[0], expected);
    app.state
        .workbench
        .inline_edit
        .set_draft("D42".to_owned(), Ok(candidate));
    bind_component_model_from_catalog(&mut app, id, "models", "junction").unwrap();
    assert_eq!(
        app.state.model.schematic.document().components[0].name,
        "D42"
    );
    assert!(
        app.state.model.schematic.document().components[0]
            .params
            .starts_with(r#"note="a  b" "#)
    );
    assert!(app.state.workbench.inline_edit.session().is_none());
}

#[test]
fn catalog_binding_uses_the_resolved_project_global_provider() {
    use crate::state::model_library::{DeviceModel, ModelConsumerScope, ModelLibrary, ModelType};

    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager.clear();
    for library_name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(library_name);
        library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
        app.state.model.model_library_manager.add_library(library);
    }
    app.state
        .model
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "shared_diode",
            "beta",
            "Test selects the executable provider.",
        )
        .expect("provider decision");
    let component_id = 9_001;
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(
                component_id,
                ComponentType::Diode,
                crate::state::Point::origin(),
            )
            .with_name_value("D9001", ""),
        );

    bind_component_model_from_catalog(&mut app, component_id, "beta", "shared_diode")
        .expect("resolved provider binds");
    let params = crate::state::parse_params_string(
        &app.state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("component")
            .params,
    );
    assert_eq!(
        params.get("model").map(String::as_str),
        Some("shared_diode")
    );
    assert_eq!(
        params.get("model_library").map(String::as_str),
        Some("beta")
    );

    let error = bind_component_model_from_catalog(&mut app, component_id, "alpha", "shared_diode")
        .expect_err("losing provider cannot bind");
    assert!(error.contains("project-global provider 'beta'"));
}

#[test]
fn catalog_binding_rejects_an_incompatible_primitive_without_mutation() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager.clear();
    let mut library = ModelLibrary::new("models");
    library.add_model(DeviceModel::new("junction", ModelType::Diode));
    app.state.model.model_library_manager.add_library(library);
    let component_id = 9_002;
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(
                component_id,
                ComponentType::Resistor,
                crate::state::Point::origin(),
            )
            .with_name_value("R9002", "1k"),
        );
    let topology_before = app.state.model.schematic.topology_version();
    let params_before = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .expect("test resistor")
        .params
        .clone();

    let error = bind_component_model_from_catalog(&mut app, component_id, "models", "junction")
        .expect_err("a diode model cannot bind to a resistor");

    assert!(error.contains("incompatible with the selected Resistor instance"));
    assert_eq!(
        app.state.model.schematic.topology_version(),
        topology_before
    );
    assert_eq!(
        app.state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .expect("test resistor")
            .params,
        params_before
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inspector_header_exposes_its_workspace_heading() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    let nodes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                header(ui, &mut app);
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Inspector")
            && node.level() == Some(2)
    }));
}

#[test]
fn schematic_subjects_do_not_share_inspector_scroll_state() {
    let mut app = RSpiceApp::test_instance();
    let sheet = inspector_scroll_identity(&app);
    let component_id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, crate::state::Point::new(10, 10));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    let component = inspector_scroll_identity(&app);

    assert_ne!(sheet, component);
    app.state.model.schematic.session.editor.selection.clear();
    assert_eq!(sheet, inspector_scroll_identity(&app));
}

#[test]
fn inspector_scroll_memory_restores_subject_offsets_with_one_stable_widget() {
    let mut memory = InspectorScrollMemory::default();

    assert_eq!(memory.begin_subject("sheet"), Some(0.0));
    memory.record("sheet".to_owned(), 320.0);
    assert_eq!(memory.begin_subject("component:R1"), Some(0.0));
    memory.record("component:R1".to_owned(), 48.0);
    assert_eq!(memory.begin_subject("sheet"), Some(320.0));
    assert_eq!(memory.begin_subject("sheet"), None);
}

#[test]
fn inspector_scroll_memory_is_bounded_and_clamps_invalid_offsets() {
    let mut memory = InspectorScrollMemory::default();
    for index in 0..=INSPECTOR_SCROLL_HISTORY_LIMIT {
        memory.record(format!("subject:{index}"), index as f32);
    }
    memory.record("negative".to_owned(), -50.0);

    assert_eq!(memory.offsets.len(), INSPECTOR_SCROLL_HISTORY_LIMIT);
    assert_eq!(memory.begin_subject("negative"), Some(0.0));
    assert!(
        memory
            .offsets
            .iter()
            .all(|(_, offset)| offset.is_finite() && *offset >= 0.0)
    );
}

/// A compact panel row — an operating point, a node voltage, a trace's
/// statistics — holds four significant digits whatever precision the reader
/// set for the result tables, and still drops the zeros that only pad a
/// value out.
#[test]
fn a_compact_panel_readout_holds_four_significant_digits() {
    assert_eq!(compact_readout(96.86112e-6, "A"), "96.86 µA");
    assert_eq!(compact_readout(275.30774e-3, "V"), "275.3 mV");
    assert_eq!(compact_readout(310.10824e-6, "S"), "310.1 µS");
    assert_eq!(compact_readout(4.1, "V"), "4.1 V");
    assert_eq!(compact_readout(0.9, "V"), "900 mV");
}
