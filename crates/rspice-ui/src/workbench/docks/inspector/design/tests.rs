//! Tests for inspector editing and layout contracts.
//!
//! The cases pin that an inline edit session folds all its keystrokes into one
//! undo entry, that choosing the default section removes the instance
//! override rather than writing it, and that the panel geometry matches its
//! design contract.

use super::*;
use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
use crate::state::{
    Cell, Library, LibraryCellInstance, Point, PortDirection, SchematicProbe, View, ViewType,
};

fn state_with_two_components() -> AppState {
    let mut state = AppState::default();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(40, 0));
    state
}

fn app_with_model_bound_instance() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let mut library = ModelLibrary::new("vendor_analog");
    library.add_model(DeviceModel::new("OPA189_A", ModelType::Other));
    library.add_model(DeviceModel::new("OPA189_B", ModelType::Other));
    library.add_model(DeviceModel::new("unrelated_nmos", ModelType::Nmos));
    app.state.model_library_manager.add_library(library);

    let mut binding = LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
    binding.module_name = Some("OPA189_A".to_owned());
    binding.netlist_template = Some("X{name} {nodes} {model} {params}".to_owned());
    binding.model_section = Some("tt".to_owned());
    let component = Component::new(41, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("XU1", "OPA189");
    app.state.schematic.components.push(component);
    app.state.schematic.init_undo_history();
    (app, 41)
}

fn validation_slot_height(editing: bool, reason: Option<&str>) -> f32 {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut height = 0.0;
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(312.0);
            height = rejection_slot(ui, editing, reason).height();
        });
    });
    height
}

#[test]
fn inspector_validation_slot_never_reflows_while_a_field_is_typed_into() {
    assert_eq!(validation_slot_height(true, None), INLINE_VALIDATION_SLOT_H);
    assert_eq!(
        validation_slot_height(
            true,
            Some(
                "This deliberately long validation message must truncate without growing the slot"
            )
        ),
        INLINE_VALIDATION_SLOT_H
    );
}

#[test]
fn a_closed_edit_session_leaves_no_reserved_validation_strip() {
    assert_eq!(validation_slot_height(false, None), 0.0);
    assert_eq!(
        validation_slot_height(false, Some("stated rejections are still shown")),
        INLINE_VALIDATION_SLOT_H
    );
}

#[test]
fn each_editable_group_reserves_its_strip_only_for_its_own_fields() {
    let (mut app, id) = app_with_model_bound_instance();
    assert!(!editing_identity_field(&app.state, id));
    assert!(!editing_parameter_field(&app.state, id));

    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the model-bound instance")
        .clone();
    begin_edit(
        &mut app,
        &component,
        InlineEditField::Value,
        component.value.clone(),
    );
    assert!(editing_identity_field(&app.state, id));
    assert!(!editing_parameter_field(&app.state, id));

    begin_edit(
        &mut app,
        &component,
        InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
        String::new(),
    );
    assert!(!editing_identity_field(&app.state, id));
    assert!(editing_parameter_field(&app.state, id));

    commit_edit(&mut app, "edit instance temperature");
    assert!(!editing_identity_field(&app.state, id));
    assert!(!editing_parameter_field(&app.state, id));
}

fn accesskit_nodes(
    mut add_contents: impl FnMut(&mut egui::Ui),
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
    })
    .platform_output
    .accesskit_update
    .expect("AccessKit tree update")
    .nodes
}

#[test]
fn an_empty_selection_inspects_the_sheet() {
    let state = AppState::default();
    assert_eq!(subject(&state, &[]), DesignSubject::Sheet);
}

#[test]
fn one_instance_beats_the_set_and_several_fall_through_to_multi() {
    let mut state = state_with_two_components();
    let ids: Vec<u64> = state
        .schematic
        .components
        .iter()
        .map(|component| component.id)
        .collect();

    state.schematic.selection.select_only_component(ids[0]);
    assert_eq!(subject(&state, &[]), DesignSubject::Component(ids[0]));

    state.schematic.selection.select_component(ids[1]);
    assert_eq!(subject(&state, &[]), DesignSubject::Multi);
}

#[test]
fn one_probe_has_its_own_inspector_and_mixed_selection_falls_back_to_multi() {
    let mut state = AppState::default();
    state.schematic.probes.push(
        SchematicProbe::new(73, Point::new(10, 20), "V(out)", Some("V(out)".to_owned())).unwrap(),
    );
    state.schematic.selection.select_only_probe(73);

    assert_eq!(subject(&state, &[]), DesignSubject::Probe(73));

    state.schematic.selection.select_wire(9);
    assert_eq!(subject(&state, &[]), DesignSubject::Multi);
}

#[test]
fn conductors_on_one_net_inspect_that_net_and_a_split_falls_back_to_multi() {
    let mut state = AppState::default();
    let nets = vec![
        DesignNet {
            name: "vout".to_owned(),
            authored_name: true,
            class: NetClass::Signal,
            terminals: Vec::new(),
            port: None,
            wire_ids: vec![7, 8],
        },
        DesignNet {
            name: "vin".to_owned(),
            authored_name: true,
            class: NetClass::Signal,
            terminals: Vec::new(),
            port: None,
            wire_ids: vec![9],
        },
    ];

    state.schematic.selection.select_wire(7);
    state.schematic.selection.select_wire(8);
    assert_eq!(
        subject(&state, &nets),
        DesignSubject::Net("vout".to_owned())
    );

    state.schematic.selection.select_wire(9);
    assert_eq!(subject(&state, &nets), DesignSubject::Multi);
}

#[test]
fn wireless_semantic_net_selection_routes_to_the_exact_net_inspector() {
    let mut state = AppState::default();
    state.schematic.components.push(Component::new(
        71,
        ComponentType::CellInstance,
        Point::origin(),
    ));
    state.schematic.selection.select_only_component(71);
    state
        .schematic
        .net_highlight
        .highlight_named_wires("PORT_OUT", HashSet::new());
    let nets = vec![DesignNet {
        name: "PORT_OUT".to_owned(),
        authored_name: true,
        class: NetClass::Signal,
        terminals: vec![crate::simulation::netlist_gen::NetTerminal {
            component_id: 71,
            reference: "X1".to_owned(),
            pin: "OUT".to_owned(),
        }],
        port: Some(PortDirection::Out),
        wire_ids: Vec::new(),
    }];

    assert_eq!(
        subject(&state, &nets),
        DesignSubject::Net("PORT_OUT".to_owned())
    );
    state.schematic.net_highlight.clear();
    assert_eq!(subject(&state, &nets), DesignSubject::Component(71));
}

#[test]
fn explicit_junction_selection_resolves_its_live_net() {
    let mut state = AppState::default();
    state.schematic.wires.push(crate::state::Wire::segment(
        7,
        Point::new(-20, 0),
        Point::new(20, 0),
    ));
    state
        .schematic
        .selection
        .select_only_junction(Point::origin());
    let nets = vec![DesignNet {
        name: "BIAS".to_owned(),
        authored_name: true,
        class: NetClass::Signal,
        terminals: Vec::new(),
        port: None,
        wire_ids: vec![7],
    }];

    assert_eq!(
        subject(&state, &nets),
        DesignSubject::Net("BIAS".to_owned())
    );
}

#[test]
fn a_selected_interface_port_routes_to_the_shared_net_inspector() {
    let mut state = AppState::default();
    let port =
        Component::new(77, ComponentType::Port, Point::origin()).with_name_value("P1", "VIN");
    state.schematic.components.push(port);
    state.schematic.selection.select_only_component(77);
    let nets = vec![DesignNet {
        name: "VIN".to_owned(),
        authored_name: true,
        class: NetClass::Signal,
        terminals: Vec::new(),
        port: Some(PortDirection::In),
        wire_ids: Vec::new(),
    }];

    assert_eq!(subject(&state, &nets), DesignSubject::Net("VIN".to_owned()));
}

#[test]
fn a_conductor_with_no_resolved_net_never_claims_one() {
    let mut state = AppState::default();
    state.schematic.selection.select_wire(42);

    // One unresolved wire is a single selected object, not a net.
    assert_eq!(subject(&state, &[]), DesignSubject::Multi);
}

#[test]
fn isolated_instance_terminals_are_open_not_bound_to_synthetic_nodes() {
    let mut state = AppState::default();
    state
        .schematic
        .components
        .push(Component::new(42, ComponentType::Resistor, Point::origin()));

    let sheet = sheet_connectivity(&state);
    let terminals = sheet.terminals.get(&42).expect("resistor terminals");

    assert_eq!(terminals.len(), 2);
    assert!(
        terminals.iter().all(|(_, net)| net.is_none()),
        "isolated terminals must not claim the netlister's synthetic node names"
    );
}

#[test]
fn open_terminal_rows_are_disabled_while_bound_rows_remain_actionable() {
    let nodes = accesskit_nodes(|ui| {
        terminal_row(ui, "1", None);
        terminal_row(ui, "2", Some("VOUT"));
    });

    let open = nodes
        .iter()
        .find(|(_, node)| node.label() == Some("1, open"))
        .map(|(_, node)| node)
        .expect("open terminal accessibility node");
    let bound = nodes
        .iter()
        .find(|(_, node)| node.label() == Some("2, VOUT"))
        .map(|(_, node)| node)
        .expect("bound terminal accessibility node");

    assert!(open.is_disabled());
    assert!(!bound.is_disabled());
}

#[test]
fn checks_are_stale_until_they_run_against_the_current_topology() {
    let mut state = AppState::default();
    assert!(!checks_current(&state));
    assert_eq!(checks_status(&state), "stale");

    state.dialogs.drc_results = Some(crate::services::drc::DrcResult::new());
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    assert!(checks_current(&state));
    assert_eq!(checks_status(&state), "0 errors");
}

#[test]
fn sheet_check_rows_report_real_unconnected_and_floating_counts() {
    let mut state = AppState::default();
    let mut result = crate::services::drc::DrcResult::new();
    result.add_violation(crate::services::drc::DrcViolation::new(
        1,
        crate::services::drc::DrcViolationType::UnconnectedPin,
        "R1.+ is open",
        crate::services::drc::DrcLocation::Component {
            id: 1,
            name: "R1".to_owned(),
        },
    ));
    result.add_violation(crate::services::drc::DrcViolation::new(
        2,
        crate::services::drc::DrcViolationType::FloatingNode,
        "net OUT is floating",
        crate::services::drc::DrcLocation::Point { x: 0.0, y: 0.0 },
    ));
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_checked_version = state.schematic.topology_version();

    assert_eq!(
        current_violation_count(
            &state,
            crate::services::drc::DrcViolationType::UnconnectedPin
        ),
        1
    );
    assert_eq!(
        current_violation_count(&state, crate::services::drc::DrcViolationType::FloatingNode),
        1
    );
}

#[test]
fn soa_display_reports_real_worst_margin_and_failures() {
    let pass = crate::state::SoaEvaluationEvidence {
        device_id: "M1".to_owned(),
        parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        limit_value: 5.0,
        worst_actual_value: 3.0,
        worst_time_s: 2.0e-6,
        sample_count: 101,
        unit: "V".to_owned(),
        description: "Drain-source voltage".to_owned(),
        verdict: SoaRuleVerdictEvidence::Pass,
    };
    let warning = crate::state::SoaEvaluationEvidence {
        device_id: "M1".to_owned(),
        parameter: crate::state::SoaParameterEvidence::PowerDissipation,
        limit_value: 1.0,
        worst_actual_value: 0.75,
        worst_time_s: 3.0e-6,
        sample_count: 101,
        unit: "W".to_owned(),
        description: "Power dissipation".to_owned(),
        verdict: SoaRuleVerdictEvidence::Warning,
    };

    let display = component_soa_display(&[&pass, &warning], 0);
    assert_eq!(display.label, "warning · 25.0% margin");
    assert_eq!(display.tone, ComponentSoaTone::Warning);

    let failed = component_soa_display(&[&pass], 2);
    assert_eq!(failed.label, "2 retained violations");
    assert_eq!(failed.tone, ComponentSoaTone::Failure);
}

#[test]
fn soa_display_does_not_invent_evidence() {
    let display = component_soa_display(&[], 0);
    assert_eq!(display.label, "No retained device evidence");
    assert_eq!(display.tone, ComponentSoaTone::NoEvidence);
}

#[test]
fn writing_a_parameter_preserves_the_other_entries_and_their_order() {
    assert_eq!(write_param("w=2u l=180n", "l", "220n"), "w=2u l=220n");
    assert_eq!(write_param("w=2u l=180n", "m", "4"), "w=2u l=180n m=4");
    assert_eq!(write_param("", "temp", "85"), "temp=85");
}

#[test]
fn clearing_a_parameter_removes_it_so_the_instance_inherits_again() {
    assert_eq!(
        write_param("w=2u temp=85 l=180n", "temp", ""),
        "w=2u l=180n"
    );
    assert_eq!(write_param("temp=85", "temp", "   "), "");
    // A bare flag with no value is left untouched by an unrelated write.
    assert_eq!(write_param("off w=2u", "w", "3u"), "off w=3u");
}

#[test]
fn a_parameter_key_matches_case_insensitively_and_is_written_back_once() {
    assert_eq!(write_param("TEMP=85", "temp", "27"), "temp=27");
    assert_eq!(write_param("W=2u w=3u", "w", "4u"), "w=4u w=4u");
}

#[test]
fn inherited_temperature_materializes_one_undoable_instance_override() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.schematic.init_undo_history();
    let component = app.state.schematic.components[0].clone();
    let field = InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned());

    assert_eq!(field_value(&component, &field), "");
    begin_edit(&mut app, &component, field.clone(), String::new());
    assert!(apply_field(&mut app.state, id, &field, "85"));
    commit_edit(&mut app, &edit_description(&field));

    assert_eq!(
        crate::state::parse_params_string(&app.state.schematic.components[0].params)
            .get(TEMPERATURE_PARAM)
            .map(String::as_str),
        Some("85")
    );
    assert!(app.state.schematic.undo());
    assert!(
        !crate::state::parse_params_string(&app.state.schematic.components[0].params)
            .contains_key(TEMPERATURE_PARAM)
    );
    assert!(!app.state.schematic.can_undo());
}

#[test]
fn free_form_parameters_edit_is_atomic_and_undoable() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::CellInstance, Point::origin());
    app.state.schematic.init_undo_history();
    let component = app.state.schematic.components[0].clone();
    let field = InlineEditField::Parameters;

    assert_eq!(field_value(&component, &field), "");
    begin_edit(&mut app, &component, field.clone(), String::new());
    for candidate in ["m=2", "m=2 tc1=0.01"] {
        assert!(apply_field(&mut app.state, id, &field, candidate));
    }
    commit_edit(&mut app, &edit_description(&field));

    assert_eq!(app.state.schematic.components[0].params, "m=2 tc1=0.01");
    assert!(app.state.schematic.undo());
    assert!(app.state.schematic.components[0].params.is_empty());
    assert!(
        !app.state.schematic.can_undo(),
        "one inline session must create exactly one undo entry"
    );
}

#[test]
fn an_instance_rename_is_rejected_when_it_collides_or_is_empty() {
    let mut state = AppState::default();
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    state.schematic.components[0].name = "R1".to_owned();
    state.schematic.components[1].name = "R2".to_owned();
    let subject = state.schematic.components[0].clone();

    assert!(field_rejection(&state, &subject, &InlineEditField::Instance, "R7").is_none());
    assert!(
        field_rejection(&state, &subject, &InlineEditField::Instance, "R1").is_none(),
        "its own name is not a collision"
    );
    let collision = field_rejection(&state, &subject, &InlineEditField::Instance, "r2")
        .expect("case-insensitive collision is rejected");
    assert!(collision.contains("already exists"), "was {collision}");
    assert!(field_rejection(&state, &subject, &InlineEditField::Instance, "  ").is_some());
}

#[test]
fn an_instance_rename_still_obeys_the_family_designator_rule() {
    let mut state = state_with_two_components();
    state.schematic.components[0].name = "R1".to_owned();
    let resistor = state.schematic.components[0].clone();

    let rejected = field_rejection(&state, &resistor, &InlineEditField::Instance, "C1")
        .expect("a resistor cannot take a capacitor designator");
    assert!(rejected.contains('R'), "was {rejected}");
}

#[test]
fn declared_parameters_are_typed_while_unknown_extensions_remain_lossless() {
    let state = state_with_two_components();
    let subject = state.schematic.components[0].clone();

    assert!(field_rejection(&state, &subject, &InlineEditField::Value, "10k").is_none());
    assert!(field_rejection(&state, &subject, &InlineEditField::Value, "").is_none());
    assert!(
        field_rejection(
            &state,
            &subject,
            &InlineEditField::Parameter("noisy".to_owned()),
            "maybe"
        )
        .is_some()
    );
    assert!(
        field_rejection(
            &state,
            &subject,
            &InlineEditField::Parameter("noisy".to_owned()),
            "yes"
        )
        .is_none()
    );
    assert!(
        field_rejection(
            &state,
            &subject,
            &InlineEditField::Parameter("vendor_extension".to_owned()),
            "arbitrary source text"
        )
        .is_none()
    );
}

#[test]
fn fresh_standard_instances_project_their_complete_typed_parameter_contract() {
    let state = AppState::default();
    let component = Component::new(7, ComponentType::Resistor, Point::origin());
    let contract = inline_parameter_contract(&state, &component, &HashMap::new());
    let keys = contract
        .iter()
        .map(|parameter| parameter.key.as_str())
        .collect::<Vec<_>>();

    assert!(keys.contains(&"m"));
    assert!(keys.contains(&"tc1"));
    assert!(keys.contains(&"noisy"));
    assert!(!keys.contains(&"r"), "the primary value is owned by Value");
    assert!(
        contract
            .iter()
            .find(|parameter| parameter.key == "m")
            .is_some_and(|parameter| parameter.label == "Multiplier" && parameter.hint == "1")
    );
}

#[test]
fn component_identity_reports_occurrence_and_graphical_execution_views() {
    let mut state = AppState::default();
    state.workspace.descend_into(
        "XAFE".to_owned(),
        CellViewRef::new("user", "afe_core", "schematic"),
        ViewType::Schematic,
    );
    let mut binding = LibraryCellInstance::new("user", "precision_r", "symbol");
    binding.netlist_template = Some("R{name} {nodes} {model} {params}".to_owned());
    let component = Component::new(9, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("RGAIN", "499");

    assert_eq!(
        component_occurrence_path(&state, &component),
        "/top/XAFE/RGAIN"
    );
    assert_eq!(component_view_contract(&component), "symbol · spice");
}

#[test]
fn literal_value_tuning_stages_a_typed_variable_without_mutating_authority() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.schematic.components[0].name = "RLOAD".to_owned();
    app.state.schematic.components[0].value = "10k".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let variables_before = app
        .state
        .workspace
        .plan_data(plan_id)
        .unwrap()
        .design_variables
        .len();
    let topology_before = app.state.schematic.topology_version();

    stage_component_tuning(&mut app, component_id).expect("resistor literal is tunable");

    assert_eq!(app.state.schematic.components[0].value, "10k");
    assert_eq!(app.state.schematic.topology_version(), topology_before);
    assert_eq!(
        app.state
            .workspace
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .len(),
        variables_before
    );
    let binding = app
        .state
        .workbench
        .verification
        .tuning_instance_binding
        .as_ref()
        .expect("transient instance binding");
    assert!(binding.creates_variable);
    assert_eq!(binding.variable.name, "RLOAD_VALUE");
    assert_eq!(
        binding.variable.quantity,
        crate::state::DesignVariableQuantity::Resistance
    );
    assert!(binding.variable.allowed_range.is_none());
    assert_eq!(binding.binding_expression, "{RLOAD_VALUE}");
    assert!(
        app.state
            .workbench
            .verification
            .tuning_variables
            .iter()
            .any(|draft| draft.variable_id == binding.variable.id && draft.proposed)
    );
}

#[test]
fn parameter_bound_value_tuning_selects_the_existing_typed_variable() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.schematic.components[0].value = "{RGAIN}".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let variable = crate::state::DesignVariable::new(
        "RGAIN",
        "499 ohm",
        crate::state::DesignVariableQuantity::Resistance,
        crate::state::DesignVariableScope::Project,
        "Gain resistor",
        None,
        crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
        crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .unwrap();
    let variable_id = variable.id;
    app.state
        .workspace
        .add_design_variable(plan_id, variable)
        .unwrap();

    stage_component_tuning(&mut app, component_id)
        .expect("existing typed parameter reference is tunable");

    let session = &app.state.workbench.verification;
    let binding = session
        .tuning_instance_binding
        .as_ref()
        .expect("instance context is retained");
    assert!(!binding.creates_variable);
    assert_eq!(binding.variable.id, variable_id);
    assert_eq!(session.tuning_selected_variable, Some(variable_id));
    assert_eq!(session.tuning_focus_variable, Some(variable_id));
    assert!(session.tuning_variables.iter().all(|draft| !draft.proposed));
    assert_eq!(
        app.state
            .workspace
            .plan_data(plan_id)
            .unwrap()
            .design_variables
            .len(),
        1
    );
}

#[test]
fn value_tuning_fails_closed_when_no_truthful_quantity_exists() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .schematic
        .add_component(ComponentType::Inductor, Point::origin());
    app.state.schematic.components[0].value = "10u".to_owned();

    let error = stage_component_tuning(&mut app, component_id)
        .expect_err("inductance is not representable by the current typed variable schema");

    assert!(error.contains("truthful typed design-variable mapping"));
    assert!(
        app.state
            .workbench
            .verification
            .tuning_instance_binding
            .is_none()
    );
}

#[test]
fn unsupported_value_tuning_action_is_disabled_with_the_staging_reason() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .add_component(ComponentType::Inductor, Point::origin());
    app.state.schematic.components[0].value = "10u".to_owned();
    let component = app.state.schematic.components[0].clone();

    let reason = component_tuning_action_block_reason(&app, &component)
        .expect("an unsupported typed quantity cannot expose an enabled Tune action");

    assert_eq!(
        reason,
        "Inductor values do not have a truthful typed design-variable mapping"
    );
    assert_eq!(
        prepare_component_tuning(&app, &component)
            .err()
            .expect("staging preflight must reject the same unsupported quantity"),
        reason
    );
}

#[test]
fn applying_a_field_reports_whether_the_design_actually_changed() {
    let mut state = state_with_two_components();
    let id = state.schematic.components[0].id;
    let before = state.schematic.topology_version();

    assert!(apply_field(&mut state, id, &InlineEditField::Value, "10k"));
    assert_eq!(state.schematic.components[0].value, "10k");
    assert!(state.schematic.topology_version() > before);

    let settled = state.schematic.topology_version();
    assert!(
        !apply_field(&mut state, id, &InlineEditField::Value, "10k"),
        "rewriting the same text is not a change"
    );
    assert_eq!(
        state.schematic.topology_version(),
        settled,
        "an unchanged write must not advance topology"
    );
}

#[test]
fn model_choices_stay_inside_the_bound_library_and_device_family() {
    let (app, id) = app_with_model_bound_instance();
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("bound component");
    let choices = bound_model_choices(&app.state, component, "OPA189_A");
    let values = choices
        .iter()
        .map(|(value, _)| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(values, ["OPA189_A", "OPA189_B"]);
    assert_eq!(
        catalog_model_location(&app.state, component),
        Some(("vendor_analog".to_owned(), "OPA189_A".to_owned()))
    );
}

#[test]
fn bound_model_choices_reject_other_family_and_polarity_collisions() {
    let (mut app, id) = app_with_model_bound_instance();
    let library = app
        .state
        .model_library_manager
        .get_library_mut("vendor_analog")
        .expect("fixture library");
    let mut njf = DeviceModel::new("NJF_MODEL", ModelType::Other);
    njf.spice_type = Some("NJF".to_owned());
    library.add_model(njf);
    let mut pjf = DeviceModel::new("PJF_MODEL", ModelType::Other);
    pjf.spice_type = Some("PJF".to_owned());
    library.add_model(pjf);
    app.state.schematic.components[0]
        .library_cell
        .as_mut()
        .expect("fixture binding")
        .module_name = Some("NJF_MODEL".to_owned());

    let component = &app.state.schematic.components[0];
    let choices = bound_model_choices(&app.state, component, "NJF_MODEL");
    assert_eq!(
        choices
            .iter()
            .map(|(value, _)| value.as_str())
            .collect::<Vec<_>>(),
        ["NJF_MODEL"]
    );
    let before = app.state.schematic.topology_version();
    let error = apply_bound_model_choice(&mut app, id, "PJF_MODEL")
        .expect_err("P-channel JFET must not replace an N-channel JFET");
    assert!(error.contains("incompatible"), "{error}");
    assert_eq!(app.state.schematic.topology_version(), before);
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
    app.state.model_library_manager.add_library(library);

    let mut binding = LibraryCellInstance::new("junctions", "junction", "spice");
    binding.module_name = Some("DIODE_A".to_owned());
    binding.netlist_template = Some("D{name} {nodes} {model} {params}".to_owned());
    let component = Component::new(52, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("D1", "junction");
    app.state.schematic.components.push(component);
    app.state.schematic.init_undo_history();

    let choices = bound_model_choices(&app.state, &app.state.schematic.components[0], "DIODE_A");
    assert_eq!(
        choices
            .iter()
            .map(|(value, _)| value.as_str())
            .collect::<Vec<_>>(),
        ["DIODE_A", "VARACTOR_B"]
    );
    assert!(
        apply_bound_model_choice(&mut app, 52, "VARACTOR_B")
            .expect("same-family model transition validates")
    );
}

#[test]
fn a_veriloga_cell_opens_its_exact_code_source_target() {
    let mut state = AppState::default();
    let mut cell = Cell::new("sensor_bridge");
    cell.add_view(View::new("veriloga", ViewType::VerilogA));
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    state.library_manager.add_library(library);
    let binding = LibraryCellInstance::new("behavioral", "sensor_bridge", "veriloga");
    let component = Component::new(7, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("XBRIDGE", "sensor_bridge");

    assert_eq!(
        component_model_source_target(&state, &component),
        Some(ComponentModelSourceTarget::VerilogA(CellViewRef::new(
            "behavioral",
            "sensor_bridge",
            "veriloga"
        )))
    );
}

#[test]
fn selecting_a_bound_model_is_atomic_undoable_and_netlist_authoritative() {
    let (mut app, id) = app_with_model_bound_instance();
    let before = app.state.schematic.topology_version();
    assert!(
        apply_bound_model_choice(&mut app, id, "OPA189_B")
            .expect("same opaque cell-model family is compatible")
    );
    let binding = app.state.schematic.components[0]
        .library_cell
        .as_ref()
        .expect("binding");
    assert_eq!(binding.module_name.as_deref(), Some("OPA189_B"));
    assert!(app.state.schematic.topology_version() > before);
    assert!(app.state.schematic.can_undo());

    assert!(app.state.schematic.undo());
    assert_eq!(
        app.state.schematic.components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.module_name.as_deref()),
        Some("OPA189_A")
    );
}

#[test]
fn the_default_section_choice_removes_the_instance_override() {
    let (mut app, id) = app_with_model_bound_instance();
    apply_bound_model_section(&mut app, id, "");
    assert_eq!(
        app.state.schematic.components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.as_deref()),
        None
    );
    assert!(app.state.schematic.can_undo());
}

#[test]
fn an_inline_session_folds_its_keystrokes_into_one_undo_entry() {
    let mut state = state_with_two_components();
    let id = state.schematic.components[0].id;
    state.schematic.init_undo_history();
    let before = crate::state::SchematicSnapshot::capture(&state.schematic);

    for text in ["1", "1k", "1k5"] {
        apply_field(&mut state, id, &InlineEditField::Value, text);
    }
    assert!(
        state
            .schematic
            .commit_undo_from(before, "edit instance value")
    );
    assert_eq!(state.schematic.components[0].value, "1k5");

    assert!(state.schematic.undo());
    assert_ne!(state.schematic.components[0].value, "1k5");
    assert!(
        !state.schematic.can_undo(),
        "three keystrokes produced more than one undo step"
    );
}

#[test]
fn the_hero_band_matches_the_mockup_geometry() {
    assert_eq!(HERO_H, 82.0);
    assert_eq!(HERO_PREVIEW_W, 82.0);
    assert_eq!(HERO_BASELINES, [12.0, 31.0, 49.0, 68.0]);
    assert!(HERO_BASELINES.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(HERO_BASELINES[3] < HERO_H);
}

#[test]
fn operating_point_summary_matches_the_upgraded_inset_contract() {
    assert_eq!(OP_SUMMARY_MARGIN_X, 8.0);
    assert_eq!(OP_SUMMARY_PADDING, 9.0);
    assert_eq!(OP_SUMMARY_ROW_H, 22.0);
}

/// Lay one annotation row out at the docked inspector's real inner width.
fn measured_operating_point_row(label: &str, value: &str) -> (f32, f32, f32, usize) {
    const PANEL_W: f32 = 312.0;
    let inner_width = PANEL_W - 2.0 * OP_SUMMARY_MARGIN_X - 2.0 * OP_SUMMARY_PADDING;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut measured = (0.0, 0.0, 0.0, 0);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(PANEL_W);
            let row = operating_point_row(ui, inner_width, label, value);
            // Both galleys are placed against the card's inner box: the label
            // from its left edge, the right-aligned value back from its right.
            let label_right = row.label.size().x;
            let value_left = inner_width - row.value.size().x;
            measured = (label_right, value_left, row.height, row.value.rows.len());
        });
    });
    measured
}

#[test]
fn a_long_operating_point_value_wraps_instead_of_covering_its_label() {
    let (label_right, value_left, height, rows) =
        measured_operating_point_row("Selection", "No retained device operating point");

    assert!(
        value_left >= label_right,
        "the value column started at {value_left} and the label ran to {label_right}"
    );
    assert!(rows > 1, "the value was not wrapped: {rows} row(s)");
    assert!(
        height > OP_SUMMARY_ROW_H,
        "a wrapped row must grow the card, measured {height}"
    );
}

#[test]
fn a_short_operating_point_row_keeps_the_base_band_and_one_line() {
    let (label_right, value_left, height, rows) = measured_operating_point_row("Region", "sat");

    assert!(value_left > label_right + OP_SUMMARY_GAP);
    assert_eq!(rows, 1);
    assert_eq!(height, OP_SUMMARY_ROW_H);
}

/// Measure the space a section body leaves above its first block and below
/// its last, for a body drawn by `content`.
fn section_body_padding(mut content: impl FnMut(&mut Ui)) -> (f32, f32) {
    use super::super::{begin_inspector_sections, finish_inspector_sections};
    use crate::workbench::design_system::PANEL_SECTION_H;

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut measured = (0.0, 0.0);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(312.0);
            ui.spacing_mut().item_spacing.y = 0.0;
            begin_inspector_sections(ui);
            let before_header = ui.cursor().top();
            section_header(ui, "Identity", Some("editable"));
            let body_top = ui.cursor().top();
            content(ui);
            let body_bottom = ui.cursor().top();
            finish_inspector_sections(ui);
            measured = (
                body_top - before_header - PANEL_SECTION_H,
                ui.cursor().top() - body_bottom,
            );
        });
    });
    measured
}

#[test]
fn every_inspector_section_frames_its_body_with_the_same_step() {
    use super::super::INSPECTOR_SECTION_PADDING;

    let list = section_body_padding(|ui| {
        property_row(ui, "Instance", "R1");
        property_row(ui, "Value", "1k");
    });
    assert_eq!(list, (INSPECTOR_SECTION_PADDING, INSPECTOR_SECTION_PADDING));

    let tree = section_body_padding(|ui| {
        terminal_row(ui, "+", Some("net1"));
        terminal_row(ui, "-", None);
    });
    assert_eq!(tree, list, "a tree body is framed like a property list");

    let annotation = section_body_padding(|ui| {
        operating_point_summary(ui, &[("Region".to_owned(), "saturation".to_owned())]);
    });
    assert_eq!(
        annotation, list,
        "an annotation card is framed like a property list"
    );

    let actions = section_body_padding(|ui| {
        action_stack(ui, |ui| {
            let _ = Button::new("Run schematic checks").show(ui);
        });
    });
    assert_eq!(
        actions, list,
        "an action row is framed like a property list"
    );

    let list_then_actions = section_body_padding(|ui| {
        property_row(ui, "Instance", "R1");
        action_stack(ui, |ui| {
            let _ = Button::new("Edit symbol…").ghost().show(ui);
        });
    });
    assert_eq!(
        list_then_actions, list,
        "adding a second block must not change how the body is framed"
    );
}

#[test]
fn terminal_rows_start_at_the_panel_inset_with_no_empty_caret_column() {
    // The table has no parent row and no pin expands, so the row leads with
    // its status icon at the same 10 px inset as the section title and the
    // property labels — not behind a nesting inset and a blank caret column.
    assert_eq!(TERMINAL_ROW_PAD_X, 10.0);
    assert_eq!(TERMINAL_ROW_LABEL_X, 31.0);
}

#[test]
fn drawing_sheet_inspector_uses_resolved_geometry_and_canonical_labels() {
    let state = AppState::default();
    let sheet = ActiveDrawingSheet::resolve(&state);

    assert_eq!(
        drawing_sheet_source_label(sheet.format.inheritance),
        "inherited · project default"
    );
    assert_eq!(sheet.format_label(), "ISO A4 · landscape");
    assert_eq!(
        sheet.format.display_unit.format_size_um(
            sheet.geometry.physical.paper.width_um,
            sheet.geometry.physical.paper.height_um
        ),
        "297 × 210 mm"
    );
    assert_eq!(
        drawing_sheet_margins_label(&sheet.format),
        "10 · 10 · 10 · 20 mm"
    );
    assert_eq!(
        drawing_sheet_border_label(&sheet),
        "standard border with zones · 4 × 4 zones"
    );
    assert_eq!(
        drawing_sheet_title_block_label(&sheet),
        "RSpice compact · bottom right"
    );
    assert_eq!(sheet.page_label, "1 of 1");
}
