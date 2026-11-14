//! The design inspector drawn through the design context a whole application
//! lends, or through the whole frame: the component, net, sheet and selection
//! cards and what their edits do.

use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
use crate::state::{
    CellViewRef, Component, ComponentType, LibraryCellInstance, Point, SchematicProbe, ViewType,
};
use crate::workbench::app::actions::workspace::inline_edit_authority;
use crate::workbench::design::inspector::component::{
    TEMPERATURE_PARAM, apply_bound_model_choice, apply_bound_model_section, bound_model_choices,
    component_checks, component_panel, editing_identity_field, editing_parameter_field,
    terminals_section,
};
use crate::workbench::design::inspector::inline_instance::{
    begin_edit, component_tuning_action_block_reason, field_value, net_panel, update_edit,
};
use crate::workbench::design::inspector::model_evidence::component_model_evidence;
use crate::workbench::design::inspector::tests::{
    accesskit_nodes, app_with_model_bound_instance, painted_panel, seed_two_components,
};
use crate::workbench::design::inspector::*;
use crate::workbench::framework::feature::SimulatePort as _;
use crate::workbench::state::{InlineEditField, InlineEditSession, Workspace};
use crate::workbench::{AppState, RSpiceApp};
use std::collections::HashMap;

fn state_with_two_components() -> AppState {
    let mut state = AppState::default();
    seed_two_components(&mut state.model);
    state
}

#[test]
fn each_editable_group_reserves_its_strip_only_for_its_own_fields() {
    let (mut app, id) = app_with_model_bound_instance();
    assert!(!editing_identity_field(&app.state.workbench, id));
    assert!(!editing_parameter_field(&app.state.workbench, id));

    assert!(begin_edit(&mut app.design_cx(), id, InlineEditField::Value));
    assert!(editing_identity_field(&app.state.workbench, id));
    assert!(!editing_parameter_field(&app.state.workbench, id));

    assert!(begin_edit(
        &mut app.design_cx(),
        id,
        InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
    ));
    assert!(!editing_identity_field(&app.state.workbench, id));
    assert!(editing_parameter_field(&app.state.workbench, id));

    app.state.commit_inline_component_edit().unwrap();
    assert!(!editing_identity_field(&app.state.workbench, id));
    assert!(!editing_parameter_field(&app.state.workbench, id));
}

/// A net names where it reaches and what it connects. Its class, wire count
/// and a terminal count beside the terminal list are not restated.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_net_panel_states_scope_and_pins_only() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            1,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    app.state.sync_active_schematic_to_workspace();
    let nets = sheet_connectivity(&app.state.model).nets;
    let name = nets.first().expect("the wire is a net").name.clone();
    let painted = painted_panel(|ui| net_panel(ui, &mut app.design_cx(), &name, &nets));
    for gone in [
        "Class",
        "Conductors",
        "Terminals\n",
        "Operating point",
        "Checks",
    ] {
        assert!(!painted.contains(gone), "{gone}: {painted}");
    }
    assert!(painted.contains("No pins on this net."), "{painted}");
}

/// The Terminals header counts only open pins, since the rows count
/// themselves, and an instance without terminals has no section at all.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn terminals_head_only_open_pins_and_vanish_without_terminals() {
    let mut app = RSpiceApp::test_instance();
    let component = Component::new(42, ComponentType::Resistor, Point::origin());
    let mut sheet = SheetConnectivity {
        nets: std::sync::Arc::new(Vec::new()),
        terminals: HashMap::new(),
        unresolved: None,
    };
    let painted =
        painted_panel(|ui| terminals_section(ui, &mut app.design_cx(), &component, &sheet));
    assert!(painted.is_empty(), "{painted}");

    sheet.terminals.insert(
        42,
        vec![
            ("1".to_owned(), Some("in".to_owned())),
            ("2".to_owned(), Some("out".to_owned())),
        ],
    );
    let painted =
        painted_panel(|ui| terminals_section(ui, &mut app.design_cx(), &component, &sheet));
    assert!(
        painted.starts_with("Terminals\n") && !painted.contains("bound"),
        "{painted}"
    );

    sheet.terminals.insert(
        42,
        vec![
            ("1".to_owned(), None),
            ("2".to_owned(), Some("out".to_owned())),
        ],
    );
    let painted =
        painted_panel(|ui| terminals_section(ui, &mut app.design_cx(), &component, &sheet));
    assert!(painted.lines().any(|line| line == "1 open"), "{painted}");
}

/// An instance's Checks section lists only what is wrong with it. A clean
/// instance, or one whose check predates the drawing, has no section: the
/// sheet's own Checks section owns the check.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_instance_has_a_checks_section_only_for_its_own_findings() {
    let (mut app, id) = app_with_model_bound_instance();
    let component = app.state.model.schematic.document().components[0].clone();
    let painted = painted_panel(|ui| component_checks(ui, &app.design_cx(), &component));
    assert!(painted.is_empty(), "unchecked: {painted}");

    let mut result = rspice_design::drc::DrcResult::new();
    app.state.model.design_checks.drc_results = Some(result.clone());
    app.state.model.design_checks.drc_checked_version =
        app.state.model.schematic.topology_version();
    let painted = painted_panel(|ui| component_checks(ui, &app.design_cx(), &component));
    assert!(painted.is_empty(), "clean: {painted}");

    result.add_violation(rspice_design::drc::DrcViolation::new(
        1,
        rspice_design::drc::DrcViolationType::UnconnectedPin,
        "XU1.1 is open",
        rspice_design::drc::DrcLocation::Terminal {
            component_id: id,
            component_name: "XU1".to_owned(),
            pin_name: "1".to_owned(),
            point: crate::state::Point::new(10, 20),
        },
    ));
    app.state.model.design_checks.drc_results = Some(result);
    let painted = painted_panel(|ui| component_checks(ui, &app.design_cx(), &component));
    assert_eq!(painted, "Checks\nConnectivity\n1 finding\n");
}

#[test]
fn inherited_temperature_materializes_one_undoable_instance_override() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.model.schematic.init_undo_history();
    let component = app.state.model.schematic.document().components[0].clone();
    let field = InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned());

    assert_eq!(field_value(&component, &field), "");
    assert!(begin_edit(&mut app.design_cx(), id, field.clone()));
    update_edit(&mut app.design_cx(), "85°C".to_owned());
    assert_eq!(
        app.state.model.schematic.document().components[0],
        component
    );
    assert!(app.state.commit_inline_component_edit().unwrap());

    assert_eq!(
        crate::state::parse_params_string(
            &app.state.model.schematic.document().components[0].params
        )
        .get(TEMPERATURE_PARAM)
        .map(String::as_str),
        Some("85")
    );
    assert!(app.state.model.schematic.undo());
    assert!(
        !crate::state::parse_params_string(
            &app.state.model.schematic.document().components[0].params
        )
        .contains_key(TEMPERATURE_PARAM)
    );
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn free_form_parameters_edit_is_atomic_and_undoable() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::CellInstance, Point::origin());
    app.state.model.schematic.init_undo_history();
    let component = app.state.model.schematic.document().components[0].clone();
    let field = InlineEditField::Parameters;

    assert_eq!(field_value(&component, &field), "");
    assert!(begin_edit(&mut app.design_cx(), id, field.clone()));
    for candidate in ["m=2", "m=2 tc1=0.01"] {
        update_edit(&mut app.design_cx(), candidate.to_owned());
        assert_eq!(
            app.state.model.schematic.document().components[0],
            component
        );
    }
    assert!(app.state.commit_inline_component_edit().unwrap());

    assert_eq!(
        app.state.model.schematic.document().components[0].params,
        "m=2 tc1=0.01"
    );
    assert!(app.state.model.schematic.undo());
    assert!(
        app.state.model.schematic.document().components[0]
            .params
            .is_empty()
    );
    assert!(
        !app.state.model.schematic.can_undo(),
        "one inline session must create exactly one undo entry"
    );
}

#[test]
fn unsupported_value_tuning_action_is_disabled_with_the_staging_reason() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Inductor, Point::origin());
    app.state.model.schematic.document_mut_for_test().components[0].value = "10u".to_owned();
    let component = app.state.model.schematic.document().components[0].clone();

    let reason = component_tuning_action_block_reason(&app.design_cx(), &component)
        .expect("an unsupported typed quantity cannot expose an enabled Tune action");

    assert_eq!(
        reason,
        "Inductor values do not have a truthful typed design-variable mapping"
    );
    assert_eq!(
        app.state
            .simulate
            .component_tuning_block_reason(&app.state.model, &component)
            .expect("staging preflight must reject the same unsupported quantity"),
        reason
    );
}

#[test]
fn applying_a_field_reports_whether_the_design_actually_changed() {
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_two_components();
    let id = app.state.model.schematic.document().components[0].id;
    let before = app.state.model.schematic.topology_version();

    assert!(begin_edit(&mut app.design_cx(), id, InlineEditField::Value));
    update_edit(&mut app.design_cx(), "10k".to_owned());
    assert!(app.state.commit_inline_component_edit().unwrap());
    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        "10k"
    );
    assert!(app.state.model.schematic.topology_version() > before);

    let settled = app.state.model.schematic.topology_version();
    assert!(begin_edit(&mut app.design_cx(), id, InlineEditField::Value));
    update_edit(&mut app.design_cx(), "10k".to_owned());
    assert!(
        !app.state.commit_inline_component_edit().unwrap(),
        "rewriting the same text is not a change"
    );
    assert_eq!(
        app.state.model.schematic.topology_version(),
        settled,
        "an unchanged write must not advance topology"
    );
}

#[test]
fn bound_model_choices_reject_other_family_and_polarity_collisions() {
    let (mut app, id) = app_with_model_bound_instance();
    let library = app
        .state
        .model
        .model_library_manager
        .get_library_mut("vendor_analog")
        .expect("fixture library");
    let mut njf = DeviceModel::new("NJF_MODEL", ModelType::Other);
    njf.spice_type = Some("NJF".to_owned());
    library.add_model(njf);
    let mut pjf = DeviceModel::new("PJF_MODEL", ModelType::Other);
    pjf.spice_type = Some("PJF".to_owned());
    library.add_model(pjf);
    app.state.model.schematic.document_mut_for_test().components[0]
        .library_cell
        .as_mut()
        .expect("fixture binding")
        .module_name = Some("NJF_MODEL".to_owned());

    let component = &app.state.model.schematic.document().components[0];
    let choices = bound_model_choices(&app.state.model, component, "NJF_MODEL");
    assert_eq!(
        choices
            .iter()
            .map(|(value, _)| value.as_str())
            .collect::<Vec<_>>(),
        ["NJF_MODEL"]
    );
    let before = app.state.model.schematic.topology_version();
    let error = apply_bound_model_choice(&mut app.design_cx(), id, "PJF_MODEL")
        .expect_err("P-channel JFET must not replace an N-channel JFET");
    assert!(error.contains("incompatible"), "{error}");
    assert_eq!(app.state.model.schematic.topology_version(), before);
}

#[test]
fn bound_model_choices_accept_same_family_across_catalog_model_types() {
    let mut app = RSpiceApp::test_instance();
    let mut library = ModelLibrary::new("junctions");
    let mut diode = DeviceModel::new("DIODE_A", ModelType::Diode);
    diode.spice_type = Some("D".to_owned());
    library.add_model(diode);
    let mut varactor = DeviceModel::new("VARACTOR_B", ModelType::Varactor);
    varactor.spice_type = Some("D".to_owned());
    library.add_model(varactor);
    app.state.model.model_library_manager.add_library(library);

    let mut binding = LibraryCellInstance::new("junctions", "junction", "spice");
    binding.module_name = Some("DIODE_A".to_owned());
    binding.netlist_template = Some("D{name} {nodes} {model} {params}".to_owned());
    let component = Component::new(52, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("D1", "junction");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state.model.schematic.init_undo_history();

    let choices = bound_model_choices(
        &app.state.model,
        &app.state.model.schematic.document().components[0],
        "DIODE_A",
    );
    assert_eq!(
        choices
            .iter()
            .map(|(value, _)| value.as_str())
            .collect::<Vec<_>>(),
        ["DIODE_A", "VARACTOR_B"]
    );
    assert!(
        apply_bound_model_choice(&mut app.design_cx(), 52, "VARACTOR_B")
            .expect("same-family model transition validates")
    );
}

/// A binned family is one model choice, named as an instance binds it. An
/// instance pinned to one bin keeps that bin as its current choice, so the row
/// says what it runs, and choosing the family binds the family's name.
#[test]
fn bound_model_choices_offer_a_binned_family_once() {
    let mut app = RSpiceApp::test_instance();
    let mut kit = ModelLibrary::new("kit");
    for name in ["nch.1", "nch.2", "nch_hv"] {
        let mut card = DeviceModel::new(name, ModelType::Nmos);
        card.spice_type = Some("NMOS".to_owned());
        kit.add_model(card);
    }
    app.state.model.model_library_manager.add_library(kit);
    let mut binding = LibraryCellInstance::new("kit", "nch_cell", "spice");
    binding.module_name = Some("nch.1".to_owned());
    binding.netlist_template = Some("M{name} {nodes} {model} {params}".to_owned());
    let component = Component::new(61, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("XM1", "nch_cell");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state.model.schematic.init_undo_history();

    let component = app.state.model.schematic.document().components[0].clone();
    let choices = bound_model_choices(&app.state.model, &component, "nch.1");
    let owned = |pairs: &[(&str, &str)]| {
        pairs
            .iter()
            .map(|(value, label)| ((*value).to_owned(), (*label).to_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        choices,
        owned(&[
            ("nch", "nch · 2 bins"),
            ("nch.1", "nch.1 · bin of nch"),
            ("nch_hv", "nch_hv"),
        ])
    );

    assert!(
        apply_bound_model_choice(&mut app.design_cx(), 61, "nch").expect("the family validates")
    );
    let component = app.state.model.schematic.document().components[0].clone();
    assert_eq!(
        component
            .library_cell
            .as_ref()
            .and_then(|binding| binding.module_name.as_deref()),
        Some("nch")
    );
    assert_eq!(
        bound_model_choices(&app.state.model, &component, "nch"),
        owned(&[("nch", "nch · 2 bins"), ("nch_hv", "nch_hv")]),
        "once the family is bound, no bin is offered beside it"
    );
    let evidence = component_model_evidence(&app.state.model, &component);
    assert_eq!(evidence.model.as_deref(), Some("nch"));
    assert!(evidence.problem.is_none(), "{:?}", evidence.problem);
}

#[test]
fn selecting_a_bound_model_is_atomic_undoable_and_netlist_authoritative() {
    let (mut app, id) = app_with_model_bound_instance();
    let before = app.state.model.schematic.topology_version();
    assert!(
        apply_bound_model_choice(&mut app.design_cx(), id, "OPA189_B")
            .expect("same opaque cell-model family is compatible")
    );
    let binding = app.state.model.schematic.document().components[0]
        .library_cell
        .as_ref()
        .expect("binding");
    assert_eq!(binding.module_name.as_deref(), Some("OPA189_B"));
    assert!(app.state.model.schematic.topology_version() > before);
    assert!(app.state.model.schematic.can_undo());

    assert!(app.state.model.schematic.undo());
    assert_eq!(
        app.state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.module_name.as_deref()),
        Some("OPA189_A")
    );
}

#[test]
fn the_default_section_choice_removes_the_instance_override() {
    let (mut app, id) = app_with_model_bound_instance();
    apply_bound_model_section(&mut app.design_cx(), id, "");
    assert_eq!(
        app.state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.as_deref()),
        None
    );
    assert!(app.state.model.schematic.can_undo());
}

#[test]
fn an_inline_session_folds_its_keystrokes_into_one_undo_entry() {
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_two_components();
    let id = app.state.model.schematic.document().components[0].id;
    app.state.model.schematic.init_undo_history();
    let before = app.state.model.schematic.document().components[0].clone();
    assert!(begin_edit(&mut app.design_cx(), id, InlineEditField::Value));

    for text in ["1", "1k", "15k"] {
        update_edit(&mut app.design_cx(), text.to_owned());
        assert_eq!(app.state.model.schematic.document().components[0], before);
    }
    assert!(app.state.commit_inline_component_edit().unwrap());
    assert_eq!(
        app.state.model.schematic.document().components[0].value,
        "15k"
    );

    assert!(app.state.model.schematic.undo());
    assert_eq!(app.state.model.schematic.document().components[0], before);
    assert!(
        !app.state.model.schematic.can_undo(),
        "three keystrokes produced more than one undo step"
    );
}

/// A probe states the signal it saves, and an unbound one is the problem to
/// act on. Its stable ID, raw anchor and the toolbar's own actions are gone.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_unbound_probe_states_its_missing_signal_as_a_problem() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .probes
        .push(
            SchematicProbe::new(5, Point::new(10, 20), "P1", None)
                .expect("an unbound probe is well formed"),
        );
    let nodes = accesskit_nodes(|ui| probe_panel(ui, &mut app.design_cx(), 5));
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("Signal") && node.description() == Some("Warning")
        }),
        "the unbound signal carries a warning"
    );
    let painted = painted_panel(|ui| probe_panel(ui, &mut app.design_cx(), 5));
    for gone in [
        "PROBE-5",
        "Anchor",
        "Center marker",
        "Delete probe",
        "Show in results",
    ] {
        assert!(!painted.contains(gone), "{gone}: {painted}");
    }
}

/// The component panel opens on its title and names the occurrence only below
/// the design root, where it says something.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_component_title_states_the_occurrence_only_below_the_root() {
    let (mut app, id) = app_with_model_bound_instance();
    let sheet = sheet_connectivity(&app.state.model);
    let painted = painted_panel(|ui| component_panel(ui, &mut app.design_cx(), id, &sheet));
    let first = painted.lines().next().unwrap_or_default();
    assert!(
        first.starts_with("XU1") && !first.contains('/'),
        "the title leads the panel and says nothing of the root: {painted}"
    );

    app.state.model.workspace.descend_into(
        "XAFE".to_owned(),
        CellViewRef::new("user", "afe_core", "schematic"),
        ViewType::Schematic,
    );
    let painted = painted_panel(|ui| component_panel(ui, &mut app.design_cx(), id, &sheet));
    assert!(
        painted
            .lines()
            .next()
            .is_some_and(|line| line.ends_with("/XAFE")),
        "{painted}"
    );
}

/// Two resistors on the design workspace, nothing selected.
#[cfg(not(target_arch = "wasm32"))]
fn two_resistors() -> (RSpiceApp, u64, u64) {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let first = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    let second = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(200, 0));
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.session.editor.selection.clear();
    (app, first, second)
}

/// The name the document holds for the instance `id`.
#[cfg(not(target_arch = "wasm32"))]
fn instance_name(app: &RSpiceApp, id: u64) -> String {
    app.state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the fixture instance exists")
        .name
        .clone()
}

/// An inline rename of the instance `id` begun with `buffer` typed: legal
/// when `rename` holds the name it would take, refused otherwise.
#[cfg(not(target_arch = "wasm32"))]
fn begin_rename(app: &mut RSpiceApp, id: u64, buffer: &str, rename: Option<&str>) {
    let expected = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the fixture instance exists")
        .clone();
    let candidate = rename.map(|name| {
        let mut candidate = expected.clone();
        candidate.name = name.to_owned();
        candidate
    });
    let authority = inline_edit_authority(&app.state.model);
    app.state.workbench.inline_edit.begin(InlineEditSession {
        expected,
        field: InlineEditField::Instance,
        authority,
        description: "rename instance".to_owned(),
        buffer: buffer.to_owned(),
        error: rename.is_none().then(|| "Invalid name".to_owned()),
        candidate,
        widget: None,
    });
}

/// ASK 11-1 (b): an inline edit the selection has moved off is committed
/// before the shell lends the inspector its context, so the panel it then
/// draws reads the committed design. With both instances selected, the
/// selection's list names the renamed one in the same frame.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_inspector_draws_on_the_inline_edit_the_selection_moved_off() {
    use crate::workbench::framework::host::{Host as _, InspectorScope, Point as HostPoint};

    let (mut app, first, second) = two_resistors();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    begin_rename(&mut app, first, "R99", Some("R99"));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(second);

    let painted = painted_panel(|ui| {
        app.contribute(
            HostPoint::Inspector(InspectorScope::Workspace(Workspace::Design)),
            ui,
        );
    });

    assert!(
        app.state.workbench.inline_edit.session().is_none(),
        "the edit was committed"
    );
    assert_eq!(instance_name(&app, first), "R99");
    assert!(
        painted.lines().any(|line| line == "R99"),
        "the panel drawn after the commit lists the renamed instance: {painted}"
    );
}

/// A commit that fails selects its instance again and keeps it in the
/// inspector, whatever the selection then reads as, so the draft can be
/// corrected.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_failed_inline_commit_keeps_its_instance_in_the_inspector() {
    use crate::workbench::framework::host::{Host as _, InspectorScope, Point as HostPoint};

    let (mut app, first, second) = two_resistors();
    let name = instance_name(&app, first);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    begin_rename(&mut app, first, "1 bad", None);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(second);

    let painted = painted_panel(|ui| {
        app.contribute(
            HostPoint::Inspector(InspectorScope::Workspace(Workspace::Design)),
            ui,
        );
    });

    assert!(
        app.state.workbench.inline_edit.session().is_some(),
        "the refused draft is kept"
    );
    assert_eq!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(first),
        "the edited instance is selected again"
    );
    assert_eq!(app.state.design.inspector_holds_failed_edit, Some(first));
    assert!(
        painted.starts_with(name.as_str()),
        "the inspector's title names the edited instance: {painted}"
    );
    assert!(
        painted.lines().any(|line| line == "1 bad") && painted.contains("Invalid name"),
        "the refused draft stays on show, with why it was refused: {painted}"
    );
}

/// Each pass of the inspector says afresh whether the sheet's section takes
/// its place: with nothing selected it does, and a pass with an instance
/// selected clears what an earlier pass left. The shell takes the answer as
/// it draws the sheet's section.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn each_inspector_pass_says_afresh_whether_the_sheet_takes_its_place() {
    use crate::workbench::framework::host::{Host as _, InspectorScope, Point as HostPoint};

    let (mut app, first, _) = two_resistors();
    let _ = painted_panel(|ui| show(ui, &mut app.design_cx()));
    assert!(
        app.state.design.inspector_yields_to_sheet,
        "with nothing selected the sheet's section takes the inspector's place"
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    let _ = painted_panel(|ui| show(ui, &mut app.design_cx()));
    assert!(
        !app.state.design.inspector_yields_to_sheet,
        "a pass that inspects an instance must not keep an earlier pass's answer"
    );

    app.state.model.schematic.session.editor.selection.clear();
    let _ = painted_panel(|ui| {
        app.contribute(
            HostPoint::Inspector(InspectorScope::Workspace(Workspace::Design)),
            ui,
        );
    });
    assert!(
        !app.state.design.inspector_yields_to_sheet,
        "the shell takes the answer as it draws the sheet's section"
    );
}
