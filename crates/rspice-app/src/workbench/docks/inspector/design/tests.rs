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
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
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

    assert!(begin_edit(&mut app, id, InlineEditField::Value));
    assert!(editing_identity_field(&app.state, id));
    assert!(!editing_parameter_field(&app.state, id));

    assert!(begin_edit(
        &mut app,
        id,
        InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
    ));
    assert!(!editing_identity_field(&app.state, id));
    assert!(editing_parameter_field(&app.state, id));

    app.state.commit_inline_component_edit().unwrap();
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

/// Everything one inspector panel paints, at the dock's real width.
#[cfg(not(target_arch = "wasm32"))]
fn painted_panel(mut contents: impl FnMut(&mut Ui)) -> String {
    fn walk(shape: &egui::epaint::Shape, into: &mut String) {
        match shape {
            egui::epaint::Shape::Text(painted) => {
                into.push_str(&painted.galley.job.text);
                into.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(312.0);
                contents(ui);
            });
    });
    let mut text = String::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut text);
    }
    text
}

/// One retained probe on a cell view whose sheet catalog holds `sheets`
/// sheets. With two, the probe is assigned to the second one.
#[cfg(not(target_arch = "wasm32"))]
fn app_with_probe_on_last_sheet(sheets: usize) -> (RSpiceApp, u64) {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    const PROBE: u64 = 77;
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(
            PROBE,
            Point::new(10, 20),
            "V(OUT)",
            Some("V(OUT)".to_owned()),
        )
        .expect("the fixture probe is well formed"),
    );
    app.state.sync_active_schematic_to_workspace();

    let key = app
        .state
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = app
        .state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [PROBE])
        .expect("the fixture cell view takes a sheet catalog");
    if sheets > 1 {
        let catalog = app
            .state
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the catalog was just bootstrapped");
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output stage".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .expect("a second sheet");
        catalog
            .assign_objects(catalog.revision(), second, [PROBE])
            .expect("the probe takes its assignment");
        catalog
            .set_active(first)
            .expect("the first sheet is active");
    }
    (app, PROBE)
}

/// An inspected object states which sheet holds it, by the catalog's own page
/// number and name — and says nothing at all where there is one sheet, since
/// naming it would only repeat the document.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_inspected_object_names_its_sheet_only_above_one_sheet() {
    let (mut multi, probe) = app_with_probe_on_last_sheet(2);
    let painted = painted_panel(|ui| probe_panel(ui, &mut multi, probe));
    assert!(
        painted.contains("Sheet 2 \u{00b7} Output stage"),
        "the row names the ordinal and the name the catalog carries: {painted}"
    );

    let (mut single, probe) = app_with_probe_on_last_sheet(1);
    let painted = painted_panel(|ui| probe_panel(ui, &mut single, probe));
    assert!(
        !painted.contains("Input stage"),
        "a single-sheet cell view is its sheet, so the row is absent: {painted}"
    );
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
        .document()
        .components
        .iter()
        .map(|component| component.id)
        .collect();

    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(ids[0]);
    assert_eq!(subject(&state, &[]), DesignSubject::Component(ids[0]));

    state
        .schematic
        .session
        .editor
        .selection
        .select_component(ids[1]);
    assert_eq!(subject(&state, &[]), DesignSubject::Multi);
}

#[test]
fn one_probe_has_its_own_inspector_and_mixed_selection_falls_back_to_multi() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(73, Point::new(10, 20), "V(out)", Some("V(out)".to_owned())).unwrap(),
    );
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_probe(73);

    assert_eq!(subject(&state, &[]), DesignSubject::Probe(73));

    state.schematic.session.editor.selection.select_wire(9);
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

    state.schematic.session.editor.selection.select_wire(7);
    state.schematic.session.editor.selection.select_wire(8);
    assert_eq!(
        subject(&state, &nets),
        DesignSubject::Net("vout".to_owned())
    );

    state.schematic.session.editor.selection.select_wire(9);
    assert_eq!(subject(&state, &nets), DesignSubject::Multi);
}

#[test]
fn wireless_semantic_net_selection_routes_to_the_exact_net_inspector() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(
            71,
            ComponentType::CellInstance,
            Point::origin(),
        ));
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(71);
    state
        .schematic
        .session
        .editor
        .net_highlight
        .highlight_named_wires("PORT_OUT", HashSet::new());
    let nets = vec![DesignNet {
        name: "PORT_OUT".to_owned(),
        authored_name: true,
        class: NetClass::Signal,
        terminals: vec![rspice_design::connectivity::summary::NetTerminal {
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
    state.schematic.session.editor.net_highlight.clear();
    assert_eq!(subject(&state, &nets), DesignSubject::Component(71));
}

#[test]
fn explicit_junction_selection_resolves_its_live_net() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            7,
            Point::new(-20, 0),
            Point::new(20, 0),
        ));
    state
        .schematic
        .session
        .editor
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
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(port);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(77);
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
    state.schematic.session.editor.selection.select_wire(42);

    // One unresolved wire is a single selected object, not a net.
    assert_eq!(subject(&state, &[]), DesignSubject::Multi);
}

#[test]
fn isolated_instance_terminals_are_open_not_bound_to_synthetic_nodes() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
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

/// A net below the design root is read under its instance path, as the run
/// names its node; the bare name is the top sheet's node of that name.
#[test]
fn a_net_below_the_root_is_read_under_its_instance_path() {
    let mut state = AppState::default();
    assert_eq!(net_engine_name(&state, "OUT").as_deref(), Some("out"));
    state.workspace.descend_into(
        "X1".to_owned(),
        CellViewRef::new("user", "inverter", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(net_engine_name(&state, "OUT").as_deref(), Some("x1.out"));
}

/// A net names where it reaches and what it connects. Its class, wire count
/// and a terminal count beside the terminal list are not restated.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_net_panel_states_scope_and_pins_only() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            1,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    app.state.sync_active_schematic_to_workspace();
    let nets = sheet_connectivity(&app.state).nets;
    let name = nets.first().expect("the wire is a net").name.clone();
    let painted = painted_panel(|ui| net_panel(ui, &mut app, &name, &nets));
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
    let painted = painted_panel(|ui| terminals_section(ui, &mut app, &component, &sheet));
    assert!(painted.is_empty(), "{painted}");

    sheet.terminals.insert(
        42,
        vec![
            ("1".to_owned(), Some("in".to_owned())),
            ("2".to_owned(), Some("out".to_owned())),
        ],
    );
    let painted = painted_panel(|ui| terminals_section(ui, &mut app, &component, &sheet));
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
    let painted = painted_panel(|ui| terminals_section(ui, &mut app, &component, &sheet));
    assert!(painted.lines().any(|line| line == "1 open"), "{painted}");
}

/// The Checks header is "not run" before any check, "stale" once the drawing
/// changes under one, and says nothing for a current, clean check.
#[test]
fn checks_header_speaks_only_when_the_check_is_not_current_and_clean() {
    let mut state = AppState::default();
    assert!(!checks_current(&state));
    assert_eq!(checks_status(&state).as_deref(), Some("not run"));

    state.dialogs.drc_results = Some(rspice_design::drc::DrcResult::new());
    state.dialogs.drc_checked_version = state.schematic.topology_version().wrapping_add(1);
    assert!(!checks_current(&state));
    assert_eq!(checks_status(&state).as_deref(), Some("stale"));

    state.dialogs.drc_checked_version = state.schematic.topology_version();
    assert!(checks_current(&state));
    assert_eq!(checks_status(&state), None);
}

#[test]
fn sheet_check_rows_report_real_unconnected_and_floating_counts() {
    let mut state = AppState::default();
    let mut result = rspice_design::drc::DrcResult::new();
    result.add_violation(rspice_design::drc::DrcViolation::new(
        1,
        rspice_design::drc::DrcViolationType::UnconnectedPin,
        "R1.+ is open",
        rspice_design::drc::DrcLocation::Component {
            id: 1,
            name: "R1".to_owned(),
        },
    ));
    result.add_violation(rspice_design::drc::DrcViolation::new(
        2,
        rspice_design::drc::DrcViolationType::FloatingNode,
        "net OUT is floating",
        rspice_design::drc::DrcLocation::Point { x: 0.0, y: 0.0 },
    ));
    // A placed cell's open pin and floating node, stated at each of eight
    // occurrences, are that cell's findings and not this sheet's.
    for occurrence in 1..=8 {
        let path =
            rspice_app_types::hierarchy_path::InstancePath::parse(&format!("/X{occurrence}"))
                .expect("an occurrence path");
        let view = crate::state::CellViewRef::new("work", "amp", "schematic");
        for (id, violation_type, inner) in [
            (
                10 + occurrence,
                rspice_design::drc::DrcViolationType::UnconnectedPin,
                rspice_design::drc::DrcLocation::Terminal {
                    component_id: 1,
                    component_name: "R1".to_owned(),
                    pin_name: "1".to_owned(),
                    point: Point::new(0, 0),
                },
            ),
            (
                20 + occurrence,
                rspice_design::drc::DrcViolationType::FloatingNode,
                rspice_design::drc::DrcLocation::Node {
                    net_name: "n1".to_owned(),
                    terminals: Vec::new(),
                },
            ),
        ] {
            result.add_violation(rspice_design::drc::DrcViolation::new(
                id,
                violation_type,
                "inside amp",
                rspice_design::drc::DrcLocation::InsideInstance {
                    path: path.clone(),
                    view: view.clone(),
                    inner: Box::new(inner),
                },
            ));
        }
    }
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_checked_version = state.schematic.topology_version();

    assert_eq!(sheet_open_pin_findings(&state).len(), 1);
    assert_eq!(sheet_floating_net_findings(&state).len(), 1);

    // A check of an earlier drawing reports nothing about this one.
    state.dialogs.drc_checked_version = state.schematic.topology_version() + 1;
    assert!(sheet_open_pin_findings(&state).is_empty());
}

/// A component's own findings are the ones located on it, or naming it on
/// this drawing. A finding inside a placed cell names that cell's components:
/// `amp`'s `R1` is not this sheet's `R1`.
#[test]
fn a_component_does_not_claim_a_placed_cells_finding_of_the_same_name() {
    let state = state_with_two_components();
    let resistor = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Resistor)
        .expect("the fixture resistor")
        .clone();
    let named = |location| {
        rspice_design::drc::DrcViolation::new(
            1,
            rspice_design::drc::DrcViolationType::FloatingNode,
            "floating",
            location,
        )
        .with_related(vec![resistor.name.clone()])
    };

    assert!(violation_targets_component(
        &named(rspice_design::drc::DrcLocation::Node {
            net_name: "n1".to_owned(),
            terminals: Vec::new(),
        }),
        &resistor
    ));
    assert!(!violation_targets_component(
        &named(rspice_design::drc::DrcLocation::InsideInstance {
            path: rspice_app_types::hierarchy_path::InstancePath::parse("/X1")
                .expect("an occurrence path"),
            view: crate::state::CellViewRef::new("work", "amp", "schematic"),
            inner: Box::new(rspice_design::drc::DrcLocation::Node {
                net_name: "n1".to_owned(),
                terminals: Vec::new(),
            }),
        }),
        &resistor
    ));
}

#[test]
fn soa_display_reports_real_worst_margin_and_failures() {
    let pass = crate::state::SoaEvaluationEvidence {
        duration: None,
        thresholds: Default::default(),
        envelope: None,
        derating: None,
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
        duration: None,
        thresholds: Default::default(),
        envelope: None,
        derating: None,
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

    let display = component_soa_display(&[&pass, &warning], 0).expect("a verdict");
    assert_eq!(display.label, "25.0% margin");
    assert_eq!(display.problem, Some(Problem::Warning));

    let passed = component_soa_display(&[&pass], 0).expect("a verdict");
    assert_eq!(passed.label, "40.0% margin");
    assert_eq!(passed.problem, None, "a pass is neutral");

    let failed = component_soa_display(&[&pass], 2).expect("a verdict");
    assert_eq!(failed.label, "2 violations");
    assert_eq!(failed.problem, Some(Problem::Failure));
}

#[test]
fn soa_display_does_not_invent_evidence() {
    assert_eq!(component_soa_display(&[], 0), None);
}

/// An instance's Checks section lists only what is wrong with it. A clean
/// instance, or one whose check predates the drawing, has no section: the
/// sheet's own Checks section owns the check.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_instance_has_a_checks_section_only_for_its_own_findings() {
    let (mut app, id) = app_with_model_bound_instance();
    let component = app.state.schematic.document().components[0].clone();
    let painted = painted_panel(|ui| component_checks(ui, &app, &component));
    assert!(painted.is_empty(), "unchecked: {painted}");

    let mut result = rspice_design::drc::DrcResult::new();
    app.state.dialogs.drc_results = Some(result.clone());
    app.state.dialogs.drc_checked_version = app.state.schematic.topology_version();
    let painted = painted_panel(|ui| component_checks(ui, &app, &component));
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
    app.state.dialogs.drc_results = Some(result);
    let painted = painted_panel(|ui| component_checks(ui, &app, &component));
    assert_eq!(painted, "Checks\nConnectivity\n1 finding\n");
}

#[test]
fn inherited_temperature_materializes_one_undoable_instance_override() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.schematic.init_undo_history();
    let component = app.state.schematic.document().components[0].clone();
    let field = InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned());

    assert_eq!(field_value(&component, &field), "");
    assert!(begin_edit(&mut app, id, field.clone()));
    update_edit(&mut app, "85°C".to_owned());
    assert_eq!(app.state.schematic.document().components[0], component);
    assert!(app.state.commit_inline_component_edit().unwrap());

    assert_eq!(
        crate::state::parse_params_string(&app.state.schematic.document().components[0].params)
            .get(TEMPERATURE_PARAM)
            .map(String::as_str),
        Some("85")
    );
    assert!(app.state.schematic.undo());
    assert!(
        !crate::state::parse_params_string(&app.state.schematic.document().components[0].params)
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
    let component = app.state.schematic.document().components[0].clone();
    let field = InlineEditField::Parameters;

    assert_eq!(field_value(&component, &field), "");
    assert!(begin_edit(&mut app, id, field.clone()));
    for candidate in ["m=2", "m=2 tc1=0.01"] {
        update_edit(&mut app, candidate.to_owned());
        assert_eq!(app.state.schematic.document().components[0], component);
    }
    assert!(app.state.commit_inline_component_edit().unwrap());

    assert_eq!(
        app.state.schematic.document().components[0].params,
        "m=2 tc1=0.01"
    );
    assert!(app.state.schematic.undo());
    assert!(
        app.state.schematic.document().components[0]
            .params
            .is_empty()
    );
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
    state.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
    state.schematic.document_mut_for_test().components[1].name = "R2".to_owned();
    let subject = state.schematic.document().components[0].clone();

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
    state.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
    let resistor = state.schematic.document().components[0].clone();

    let rejected = field_rejection(&state, &resistor, &InlineEditField::Instance, "C1")
        .expect("a resistor cannot take a capacitor designator");
    assert!(rejected.contains('R'), "was {rejected}");
}

#[test]
fn declared_parameters_are_typed_while_unknown_extensions_remain_lossless() {
    let state = state_with_two_components();
    let subject = state.schematic.document().components[0].clone();

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

/// A source value the component editor refuses is refused by the inline field
/// too, in the same words.
///
/// The two commit paths ran different gates. The inline editor consulted only
/// the property-sheet type contract, and `td` on a PWL source is an untyped
/// expression there, so `TD=-1ns` — which the netlist parser refuses outright —
/// committed from the inspector while the editor beside it turned it away.
#[test]
fn an_inline_source_edit_is_refused_by_the_gate_the_editor_commits_through() {
    let mut state = AppState::default();
    state
        .schematic
        .add_component(ComponentType::VoltageSourcePwl, Point::new(0, 0));
    let subject = state.schematic.document().components[0].clone();
    let field = InlineEditField::Parameter("td".to_owned());

    let inline =
        field_rejection(&state, &subject, &field, "-1ns").expect("a negative PWL delay is refused");

    // What the same delay says through the shared gate, read there rather than
    // restated here. The parameter is spelled without its unit because this
    // stands in for the editor's *typed* draft: both surfaces resolve the field
    // to a number before the waveform rules see it, which is the whole point.
    let mut committed = subject.clone();
    committed.params = "td=-1e-9".to_owned();
    let sheet = state
        .property_registry
        .get(committed.kind)
        .expect("a PWL source has a registered property sheet");
    let values = crate::properties::property_bridge::collect_properties_from_component(
        &committed,
        &state.property_registry,
    );
    let editor =
        crate::properties::property_bridge::source_commit_refusal(&committed, &values, sheet)
            .expect("the editor refuses the same value");

    assert_eq!(inline, editor, "one refusal, two surfaces");
    assert!(inline.contains("PWL TD"), "was {inline}");
    assert!(
        field_rejection(&state, &subject, &field, "1ns").is_none(),
        "a non-negative delay still commits"
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
    assert_eq!(containing_occurrence_path(&state), "/");

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

    assert_eq!(containing_occurrence_path(&state), "/XAFE");
    state.workspace.descend_into(
        "XBIAS".to_owned(),
        CellViewRef::new("user", "bias", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(containing_occurrence_path(&state), "/XAFE/XBIAS");
    assert_eq!(instance_view(&component), Some("symbol"));
    let resistor = Component::new(10, ComponentType::Resistor, Point::origin());
    assert_eq!(instance_view(&resistor), None, "a primitive binds no view");
}

#[test]
fn literal_value_tuning_stages_a_typed_variable_without_mutating_authority() {
    let mut app = RSpiceApp::test_instance();
    let component_id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    app.state.schematic.document_mut_for_test().components[0].name = "RLOAD".to_owned();
    app.state.schematic.document_mut_for_test().components[0].value = "10k".to_owned();
    let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
    let variables_before = app
        .state
        .workspace
        .content
        .plan_data(plan_id)
        .unwrap()
        .design_variables
        .len();
    let topology_before = app.state.schematic.topology_version();

    stage_component_tuning(&mut app, component_id).expect("resistor literal is tunable");

    assert_eq!(app.state.schematic.document().components[0].value, "10k");
    assert_eq!(app.state.schematic.topology_version(), topology_before);
    assert_eq!(
        app.state
            .workspace
            .content
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
    app.state.schematic.document_mut_for_test().components[0].value = "{RGAIN}".to_owned();
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
        .content
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
            .content
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
    app.state.schematic.document_mut_for_test().components[0].value = "10u".to_owned();

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
    app.state.schematic.document_mut_for_test().components[0].value = "10u".to_owned();
    let component = app.state.schematic.document().components[0].clone();

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
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_two_components();
    let id = app.state.schematic.document().components[0].id;
    let before = app.state.schematic.topology_version();

    assert!(begin_edit(&mut app, id, InlineEditField::Value));
    update_edit(&mut app, "10k".to_owned());
    assert!(app.state.commit_inline_component_edit().unwrap());
    assert_eq!(app.state.schematic.document().components[0].value, "10k");
    assert!(app.state.schematic.topology_version() > before);

    let settled = app.state.schematic.topology_version();
    assert!(begin_edit(&mut app, id, InlineEditField::Value));
    update_edit(&mut app, "10k".to_owned());
    assert!(
        !app.state.commit_inline_component_edit().unwrap(),
        "rewriting the same text is not a change"
    );
    assert_eq!(
        app.state.schematic.topology_version(),
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
        .document()
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
    app.state.schematic.document_mut_for_test().components[0]
        .library_cell
        .as_mut()
        .expect("fixture binding")
        .module_name = Some("NJF_MODEL".to_owned());

    let component = &app.state.schematic.document().components[0];
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
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state.schematic.init_undo_history();

    let choices = bound_model_choices(
        &app.state,
        &app.state.schematic.document().components[0],
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
    let binding = app.state.schematic.document().components[0]
        .library_cell
        .as_ref()
        .expect("binding");
    assert_eq!(binding.module_name.as_deref(), Some("OPA189_B"));
    assert!(app.state.schematic.topology_version() > before);
    assert!(app.state.schematic.can_undo());

    assert!(app.state.schematic.undo());
    assert_eq!(
        app.state.schematic.document().components[0]
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
        app.state.schematic.document().components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.as_deref()),
        None
    );
    assert!(app.state.schematic.can_undo());
}

#[test]
fn an_inline_session_folds_its_keystrokes_into_one_undo_entry() {
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_two_components();
    let id = app.state.schematic.document().components[0].id;
    app.state.schematic.init_undo_history();
    let before = app.state.schematic.document().components[0].clone();
    assert!(begin_edit(&mut app, id, InlineEditField::Value));

    for text in ["1", "1k", "15k"] {
        update_edit(&mut app, text.to_owned());
        assert_eq!(app.state.schematic.document().components[0], before);
    }
    assert!(app.state.commit_inline_component_edit().unwrap());
    assert_eq!(app.state.schematic.document().components[0].value, "15k");

    assert!(app.state.schematic.undo());
    assert_eq!(app.state.schematic.document().components[0], before);
    assert!(
        !app.state.schematic.can_undo(),
        "three keystrokes produced more than one undo step"
    );
}

/// The title is one panel row, whatever it names: the name, what the object
/// is and where it sits share the line.
#[test]
fn the_title_takes_one_panel_row() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut measured = (0.0, 0.0);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_width(220.0);
            let top = ui.cursor().top();
            object_title(
                ui,
                "XBANDGAP_REFERENCE",
                Some("bandgap_reference_trimmed"),
                Some("/XAFE/XBIAS"),
            );
            measured = (
                ui.cursor().top() - top,
                crate::workbench::design_system::property_row_height(ui),
            );
        });
    });
    assert_eq!(measured.0, measured.1);
}

/// A MOS transistor shows the designer's set — region, the bias point and the
/// small-signal parameters — in engineering notation with units, and the
/// temperature its run solved it at. What the report carries beyond the set
/// (the gate and bulk currents, gmb) has no row.
#[test]
fn a_mos_operating_point_shows_the_designer_set_in_engineering_units() {
    use rspice_core::op_label::OpLabel;

    let entry = rspice_core::circuit::DeviceOpEntry::new(
        "mn".to_owned(),
        OpLabel::MOSFET,
        Some(OpLabel::SATURATION),
        vec![
            (OpLabel::ID, 96.86112e-6),
            (OpLabel::IG, 0.0),
            (OpLabel::VGS, 0.9),
            (OpLabel::VDS, 0.85),
            (OpLabel::VTH, 0.45),
            (OpLabel::VDSAT, 0.45),
            (OpLabel::GM, 430.5e-6),
            (OpLabel::GDS, 4.6e-6),
            (OpLabel::GMB, 1.0e-6),
            (OpLabel::TEMP, 85.0),
        ],
    );
    let rows = operating_point_rows(&entry);
    let rows = rows
        .iter()
        .map(|(label, value)| (label.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        [
            ("Region", "saturation"),
            ("Id", "96.86 µA"),
            ("Vgs", "900 mV"),
            ("Vds", "850 mV"),
            ("Vth", "450 mV"),
            ("Vdsat", "450 mV"),
            ("gm", "430.5 µS"),
            ("gds", "4.6 µS"),
            ("Temperature", "85 °C"),
        ]
    );
}

/// The MOSFET property sheet is the union of every MOS level's instance set,
/// so a LEVEL=1 transistor used to be offered BSIM4 geometry modes, stress
/// distances, mismatch terms and body squares that nothing in its deck reads.
/// It is offered what its card reads; a value the instance sets anyway is
/// still the instance's own and keeps its row.
#[test]
fn a_level_1_mosfet_is_offered_only_the_parameters_its_card_reads() {
    let mut state = AppState::default();
    crate::workbench::examples::load_example("CMOS Inverter", &mut state.schematic);
    let mut component = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter has an MN")
        .clone();

    let declared = crate::state::parse_params_string(&component.params);
    let rows = inline_parameter_contract(&state, &component, &declared);
    let keys = |rows: &[InlineParameterContract]| {
        rows.iter()
            .map(|row| row.key.to_ascii_lowercase())
            .collect::<Vec<_>>()
    };
    let offered = keys(&rows);
    for unread in [
        "geomod", "rgeomod", "min", "nf", "sa", "sb", "sc", "sd", "sca", "scb", "scc", "delvto",
        "mulu0", "nrb",
    ] {
        assert!(
            !offered.iter().any(|key| key == unread),
            "LEVEL=1 does not read {unread}: {offered:?}"
        );
    }
    for read in [
        "w", "l", "m", "ad", "as", "pd", "ps", "nrd", "nrs", "ic_vds",
    ] {
        assert!(
            offered.iter().any(|key| key == read),
            "LEVEL=1 reads {read}: {offered:?}"
        );
    }

    // Set one of the dropped parameters and it comes back: it is a value on
    // the instance, not a field the card invited.
    component.params = format!("{} sa=1u", component.params);
    let declared = crate::state::parse_params_string(&component.params);
    let offered = keys(&inline_parameter_contract(&state, &component, &declared));
    assert!(offered.iter().any(|key| key == "sa"), "{offered:?}");
}

/// A device below the design root is reported under its instance path. The
/// inspector used to look the bare card name up, which below the root read
/// another instance's operating point, or the top sheet's device of that name.
#[test]
fn a_device_below_the_root_is_read_under_its_instance_path() {
    let mut state = AppState::default();
    let component =
        Component::new(7, ComponentType::Nmos, Point::origin()).with_name_value("MN", "nch");
    assert_eq!(
        device_engine_name(&state, &component).as_deref(),
        Some("mn")
    );
    state.workspace.descend_into(
        "X1".to_owned(),
        CellViewRef::new("user", "inverter", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(
        device_engine_name(&state, &component).as_deref(),
        Some("x1.mn")
    );
}

/// Measure, in the dock's panel body, the space between a section header and
/// the first block of its body, and between the body's last block and the
/// next section's header, for a body drawn by `content`.
fn section_body_padding(mut content: impl FnMut(&mut Ui)) -> (f32, f32) {
    use crate::workbench::design_system::{PANEL_HEADER_GAP, PANEL_SECTION_H, panel_body};

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut measured = (0.0, 0.0);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(egui::vec2(312.0, 800.0), |ui| {
                panel_body(ui, |ui| {
                    let before_header = ui.cursor().top();
                    section_header(ui, "Identity", Some("editable"));
                    let body_top = ui.cursor().top();
                    content(ui);
                    let body_bottom = ui.cursor().top();
                    section_header(ui, "Checks", None);
                    let next_header = ui.cursor().top() - PANEL_HEADER_GAP - PANEL_SECTION_H;
                    measured = (
                        body_top - before_header - PANEL_SECTION_H,
                        next_header - body_bottom,
                    );
                });
            });
        });
    });
    measured
}

#[test]
fn every_inspector_section_frames_its_body_with_the_same_step() {
    use crate::workbench::design_system::{PANEL_HEADER_GAP, PANEL_SECTION_GAP};

    let list = section_body_padding(|ui| {
        property_row(ui, "Instance", "R1");
        property_row(ui, "Value", "1k");
    });
    assert_eq!(list, (PANEL_HEADER_GAP, PANEL_SECTION_GAP));

    let tree = section_body_padding(|ui| {
        terminal_row(ui, "+", Some("net1"));
        terminal_row(ui, "-", None);
    });
    assert_eq!(tree, list, "a tree body is framed like a property list");

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
    // its status icon on the panel inset, like the section title and the
    // property labels — not behind a nesting inset and a blank caret column —
    // and its net ends on the inset on the right.
    use crate::workbench::design_system::{PANEL_INSET, panel_body};
    fn texts(shape: &egui::epaint::Shape, into: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::epaint::Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::epaint::Shape::Vec(shapes) => shapes.iter().for_each(|shape| texts(shape, into)),
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.allocate_ui(egui::vec2(312.0, 100.0), |ui| {
                    panel_body(ui, |ui| {
                        let _ = terminal_row(ui, "PLUS", Some("net1"));
                    });
                });
            });
    });
    let mut painted = Vec::new();
    for clipped in &output.shapes {
        texts(&clipped.shape, &mut painted);
    }
    let find = |wanted: &str| {
        painted
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted: {painted:?}"))
    };
    let pin = find("PLUS");
    let net = find("net1");
    assert_eq!(
        pin.left(),
        PANEL_INSET + TERMINAL_ROW_ICON_W + TERMINAL_ROW_ICON_GAP
    );
    assert!((net.right() - (312.0 - PANEL_INSET)).abs() < 0.5, "{net:?}");
}

/// With nothing selected the panel holds what the sheet needs from the
/// reader: the route to Page Setup and the schematic checks. What the tab,
/// the status bar and the navigator already say about the sheet, and the print
/// format Page Setup owns, are not restated.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_panel_holds_only_page_setup_and_checks() {
    let mut app = RSpiceApp::test_instance();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app));
    assert_eq!(
        painted,
        "Drawing sheet\nPage setup…\nChecks\nnot run\nCheck cellview\n"
    );
}

/// A probe states the signal it saves, and an unbound one is the problem to
/// act on. Its stable ID, raw anchor and the toolbar's own actions are gone.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_unbound_probe_states_its_missing_signal_as_a_problem() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(5, Point::new(10, 20), "P1", None)
            .expect("an unbound probe is well formed"),
    );
    let nodes = accesskit_nodes(|ui| probe_panel(ui, &mut app, 5));
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("Signal") && node.description() == Some("Warning")
        }),
        "the unbound signal carries a warning"
    );
    let painted = painted_panel(|ui| probe_panel(ui, &mut app, 5));
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

/// The canvas can draw that a name crosses sheets but not where it lands, so
/// the inspector is the only surface that can answer "which page". A net no
/// connector names must not grow the rows at all.
#[test]
fn the_net_inspector_states_the_sheets_a_connector_partners_with() {
    use crate::state::{
        CrossSheetPortDirection, NetLabel, NetLabelKind, SheetDefinition, SheetPortPolicy,
        SheetTemplate,
    };

    let mut state = AppState::default();
    let key = state.workspace.content.active_schematic_reference().key();
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "LOCAL"));
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::off_sheet(
            2,
            Point::new(20, 0),
            "BIAS",
            CrossSheetPortDirection::Output,
        ));
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::off_sheet(
            3,
            Point::new(40, 0),
            "BIAS",
            CrossSheetPortDirection::Input,
        ));

    // A plain label declares no crossing, so the panel stays as it was.
    assert!(off_sheet_declaration(&state, "LOCAL").is_none());
    assert!(off_sheet_declaration(&state, "never_drawn").is_none());

    // Before any sheet catalog exists the crossing has nowhere to land.
    let lone = off_sheet_declaration(&state, "BIAS").expect("a connector declares itself");
    assert_eq!(lone.direction, "input, output");
    assert_eq!(lone.partners, "none \u{2014} no partner on another sheet");

    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [1, 2])
        .expect("sheet ownership");
    let catalog = state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Output".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [3])
        .expect("the partner connector moves to the second sheet");
    catalog
        .set_active(first)
        .expect("the first sheet is active");

    let paired = off_sheet_declaration(&state, "BIAS").expect("a connector declares itself");
    assert_eq!(paired.direction, "input, output");
    assert_eq!(paired.partners, "p2", "the partner's page, not its own");

    // Read from the far side the answer is the other page, never both.
    state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog")
        .set_active(second)
        .expect("the second sheet is active");
    assert_eq!(
        off_sheet_declaration(&state, "BIAS")
            .expect("a connector declares itself")
            .partners,
        "p1"
    );

    assert_eq!(
        NetLabelKind::direction_label(CrossSheetPortDirection::InOut),
        "bidirectional",
        "the inspector and the placement dialog share one direction vocabulary"
    );
    assert_eq!(
        NetLabelKind::default(),
        NetLabelKind::Local,
        "a label makes no crossing claim until one is authored"
    );
}

/// Give the workspace an active configuration that cannot resolve: its DUT
/// path names an instance the expanded hierarchy has not got. The design
/// projection refuses such a configuration, so a surface that still shows
/// connectivity afterwards is showing the editor buffer's.
fn unresolve_configuration(state: &mut AppState) {
    let root = state.workspace.content.active_view.clone();
    state
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Unresolvable DUT".to_owned(),
            root,
            dut_path: "/XABSENT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");
}

#[test]
fn the_inspector_states_an_unresolved_configuration_instead_of_buffer_connectivity() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            1,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    state.sync_active_schematic_to_workspace();
    assert!(
        !sheet_connectivity(&state).nets.is_empty(),
        "the fixture sheet has connectivity while its configuration resolves"
    );

    unresolve_configuration(&mut state);

    let sheet = sheet_connectivity(&state);
    let reason = sheet
        .unresolved
        .expect("the projection refuses an unresolvable configuration");
    assert!(
        reason.contains("XABSENT"),
        "the inspector must state the projection's own reason: {reason}"
    );
    assert!(
        sheet.nets.is_empty() && sheet.terminals.is_empty(),
        "an unresolved configuration must not fall back to the editor buffer"
    );
}

#[test]
fn the_inspector_reads_the_projection_so_coincident_pages_stay_two_nets() {
    const FIRST_WIRE: u64 = 101;
    const SECOND_WIRE: u64 = 102;

    // The same conductor geometry on two governed sheets. One coordinate
    // space holds both, so the editor buffer merges them; the projection
    // namespaces the pages apart before anything electrical is read. The
    // inspector therefore reports one net or two depending entirely on which
    // document it read, which is what this pins.
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            FIRST_WIRE,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            SECOND_WIRE,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    state.sync_active_schematic_to_workspace();
    assert_eq!(
        rspice_design::connectivity::summary::design_nets(&state.schematic).len(),
        1,
        "the editor buffer holds one coordinate space, so its pages overlap"
    );

    let key = state.workspace.content.active_schematic_reference().key();
    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Page 1", [FIRST_WIRE])
        .expect("a fresh cell view accepts its first governed sheet");
    let catalog = state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the sheet catalog was just created");
    let second = catalog
        .create_sheet(
            crate::state::SheetDefinition {
                name: "Page 2".to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("a second governed sheet inserts after the first");
    catalog
        .assign_objects(catalog.revision(), second, [SECOND_WIRE])
        .expect("the second conductor belongs to the second page");

    let sheet = sheet_connectivity(&state);
    assert!(sheet.unresolved.is_none(), "the fixture design resolves");
    assert_eq!(
        sheet.nets.len(),
        2,
        "the inspector reads the projected document, not the editor buffer"
    );
}

#[test]
fn a_resolved_contested_model_reads_as_resolved_and_keeps_its_source_jump() {
    use crate::state::model_library::ModelConsumerScope;

    let mut state = AppState::default();
    state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
        state.model_library_manager.add_library(library);
    }
    let component = Component::new(77, ComponentType::Diode, Point::origin())
        .with_name_value("D77", "shared_diode");

    // Two providers and no decision: nothing resolves it, so the row says so
    // and there is nowhere to jump.
    let contested = super::super::component_model_evidence(&state, &component);
    let (problem, text) = contested
        .problem
        .expect("an unresolved contest is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(
        text.starts_with("Model shared_diode ambiguous"),
        "an unresolved contest is ambiguous: {text}"
    );
    assert_eq!(catalog_model_location(&state, &component), None);

    // The project resolves it. Execution and the Models workspace now both run
    // `beta`, so the inspector must stop calling the same reference ambiguous
    // and must offer the source of the copy that actually wins.
    state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "shared_diode",
            "beta",
            "Test selects the executable provider.",
        )
        .expect("two providers accept a project-global decision");
    let resolved = super::super::component_model_evidence(&state, &component);
    assert_eq!(
        resolved.problem, None,
        "a recorded decision resolves the reference"
    );
    assert_eq!(
        catalog_model_location(&state, &component),
        Some(("beta".to_owned(), "shared_diode".to_owned())),
        "the jump follows the effective provider"
    );
}

/// A compiled module parameter, as a clean compile states it.
fn module_parameter(
    name: &str,
    default: Option<&str>,
    units: Option<&str>,
    description: Option<&str>,
) -> rspice_design::library::metadata::CellParameter {
    use rspice_design::library::metadata::{
        CellParameter, ParameterDeclaration, ParameterScope, ParameterValueType,
    };
    CellParameter {
        name: name.to_owned(),
        aliases: Vec::new(),
        required: false,
        default_value: default.map(str::to_owned),
        declaration: Some(ParameterDeclaration {
            value_type: ParameterValueType::Real,
            scope: ParameterScope::Instance,
            range: None,
            units: units.map(str::to_owned),
            description: description.map(str::to_owned),
        }),
    }
}

/// An instance `X1` setting `params` of `behavioral/amp`, whose `veriloga`
/// view's compile stated `contract`.
fn app_with_compiled_veriloga_instance(
    contract: &[rspice_design::library::metadata::CellParameter],
    params: &str,
) -> (RSpiceApp, Component) {
    let mut app = RSpiceApp::test_instance();
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "amp".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    view.metadata.insert(
        rspice_design::library::metadata::VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(),
        rspice_design::library::metadata::encode_veriloga_parameter_contract(contract)
            .expect("a compiled contract encodes"),
    );
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);
    let mut binding = LibraryCellInstance::new("behavioral", "amp", "veriloga");
    binding.terminal_order = vec!["p".to_owned(), "n".to_owned()];
    let mut component = Component::new(51, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("X1", "amp");
    component.params = params.to_owned();
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(component.clone());
    app.state.schematic.init_undo_history();
    (app, component)
}

/// The unit a parameter is in sits beside its field and what it is sits on
/// its label's hover, for a compiled module's parameter and for a native
/// device's alike; a module's computed default reads as such.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn parameter_rows_carry_units_and_descriptions_for_modules_and_native_devices() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[
            module_parameter("gain", Some("2.5"), Some("S"), Some("Transconductance")),
            module_parameter("vmax", None, Some("V"), None),
        ],
        "",
    );
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    let row = |key: &str| {
        rows.iter()
            .find(|row| row.key == key)
            .unwrap_or_else(|| panic!("a {key} row: {rows:?}"))
    };
    assert_eq!(row("gain").unit.as_deref(), Some("S"));
    assert_eq!(row("gain").hint, "2.5");
    assert!(
        row("gain")
            .description
            .as_deref()
            .is_some_and(|description| description.starts_with("Transconductance.")),
        "{:?}",
        row("gain").description
    );
    assert_eq!(row("vmax").hint, "computed");

    let evidence = super::super::component_model_evidence(&app.state, &component);
    let painted = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        painted.lines().any(|line| line == "S") && painted.lines().any(|line| line == "V"),
        "each unit is painted beside its field:\n{painted}"
    );
    let nodes = accesskit_nodes(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("gain")
                && node
                    .description()
                    .is_some_and(|description| description.starts_with("Transconductance."))
        }),
        "the field is described by the module's own words"
    );

    let capacitor = Component::new(9, ComponentType::Capacitor, Point::origin());
    let rows = inline_parameter_contract(&app.state, &capacitor, &HashMap::new());
    let tc1 = rows
        .iter()
        .find(|row| row.key == "tc1")
        .expect("a capacitor's temperature coefficient row");
    assert_eq!(tc1.unit.as_deref(), Some("1/°C"));
    assert_eq!(
        tc1.description.as_deref(),
        Some("First-order temperature coefficient")
    );
}

/// `m` is a module's `$mfactor`: a module instance keeps its multiplicity
/// row, and an edit there sets the typed multiplicity the deck emits, never
/// parameter text the netlister would refuse.
#[test]
fn m_on_a_module_instance_is_its_typed_multiplicity() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    assert!(rows.iter().any(|row| row.key == "m"), "{rows:?}");

    let field = InlineEditField::Parameter("m".to_owned());
    assert!(begin_edit(&mut app, component.id, field.clone()));
    update_edit(&mut app, "2".to_owned());
    app.state
        .commit_inline_component_edit()
        .expect("a multiplicity of 2 commits");
    let edited = app.state.schematic.document().components[0].clone();
    assert_eq!(
        edited
            .multiplicity
            .map(crate::state::InstanceMultiplicity::value),
        Some(2.0)
    );
    assert!(
        crate::state::parse_params_string(&edited.params).is_empty(),
        "m is never written as parameter text: {}",
        edited.params
    );
    assert_eq!(field_value(&edited, &field), "2");
}

/// A module reads `$temperature`, so an instance of one is offered `temp`
/// only as a parameter its module declares, as that parameter's own row.
#[test]
fn temp_is_offered_on_a_module_instance_only_as_a_declared_parameter() {
    let (app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    assert!(!supports_instance_temperature(&app.state, &component));
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    assert!(
        rows.iter().all(|row| row.key != TEMPERATURE_PARAM),
        "{rows:?}"
    );

    let (app, component) = app_with_compiled_veriloga_instance(
        &[
            module_parameter("gain", Some("2.5"), None, None),
            module_parameter("Temp", Some("27"), Some("C"), None),
        ],
        "",
    );
    assert!(!supports_instance_temperature(&app.state, &component));
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    let temp = rows
        .iter()
        .find(|row| row.key == TEMPERATURE_PARAM)
        .expect("the module's own temp row");
    assert_eq!((temp.label.as_str(), temp.hint.as_str()), ("Temp", "27"));

    // A native device still takes the instance temperature.
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert!(supports_instance_temperature(&app.state, &resistor));
}

/// A value for a parameter the module no longer declares is shown, flagged,
/// and removable, and the remove is one undoable edit that touches nothing
/// else.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_undeclared_parameter_value_is_flagged_and_removable() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "gain=3 offset=1m",
    );
    let declared = crate::state::parse_params_string(&component.params);
    let rows = inline_parameter_contract(&app.state, &component, &declared);
    let offset = rows
        .iter()
        .find(|row| row.key == "offset")
        .expect("the stale value has a row");
    assert!(offset.undeclared && !offset.editable, "{offset:?}");
    assert!(
        rows.iter()
            .filter(|row| row.key == "gain")
            .all(|row| !row.undeclared)
    );

    let evidence = super::super::component_model_evidence(&app.state, &component);
    let painted = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        painted.contains(UNDECLARED_PARAMETER_NOTE)
            && painted.contains(REMOVE_UNDECLARED_PARAMETER),
        "{painted}"
    );

    assert!(
        app.state
            .remove_undeclared_instance_parameter(component.id, "gain")
            .is_err(),
        "a declared parameter is edited, not removed"
    );
    assert_eq!(
        app.state
            .remove_undeclared_instance_parameter(component.id, "offset"),
        Ok(true)
    );
    let removed = &app.state.schematic.document().components[0];
    assert_eq!(removed.params, "gain=3");
    assert!(app.state.schematic.can_undo());
}

/// [`app_with_compiled_veriloga_instance`] whose view owns its module's
/// source, compiled, and whose cell has the symbol a compile generates.
fn app_with_owned_veriloga_instance() -> (RSpiceApp, Component) {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("10"), Some("V/V"), None)],
        "gain=5",
    );
    let reference = CellViewRef::new("behavioral", "amp", "veriloga");
    let ports = [
        crate::state::PortSpec {
            name: "p".to_owned(),
            direction: PortDirection::InOut,
        },
        crate::state::PortSpec {
            name: "n".to_owned(),
            direction: PortDirection::InOut,
        },
    ];
    let mut symbol = View::new("symbol", ViewType::Symbol);
    rspice_design::symbol::generated::store_generated_symbol(&mut symbol, &ports, |_| None)
        .expect("the symbol stores");
    app.state
        .library_manager
        .get_library_mut("behavioral")
        .and_then(|library| library.get_cell_mut("amp"))
        .expect("the cell")
        .add_view(symbol);
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(reference),
        crate::state::ProjectSourceLanguage::VerilogA,
        "amp.va",
        "module amp(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real gain = 10;\n  analog I(p, n) <+ gain * V(p, n);\nendmodule\n",
        [],
        [],
    )
    .expect("a cell-view bundle");
    let id = bundle.id();
    let sources = &mut app.state.workspace.content.project_sources;
    sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    sources
        .mark_bundle_validated(id)
        .expect("the module compiled");
    (app, component)
}

/// An instance of a module says what it is and nothing a model library
/// would: its view is a Verilog-A module, it runs that module, and no model
/// card or library section is offered. A current compile is not a warning.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_module_instance_states_its_view_and_module_and_no_model_library_rows() {
    let (mut app, component) = app_with_owned_veriloga_instance();

    let evidence = super::super::component_model_evidence(&app.state, &component);
    assert_eq!(evidence.model.as_deref(), Some("amp"));
    assert_eq!(evidence.problem, None, "a current compile says nothing");

    let identity = painted_panel(|ui| identity_section(ui, &mut app, &component));
    assert!(
        identity.lines().any(|line| line == "veriloga"),
        "{identity}"
    );
    assert!(!identity.contains("spice"), "{identity}");
    assert!(
        identity.lines().any(|line| line == "Open Verilog-A view")
            && !identity.contains("model source"),
        "{identity}"
    );
    let parameters = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    let labels = parameters.lines().collect::<Vec<_>>();
    assert!(
        !labels.contains(&"Module"),
        "the module is the cell the Library cell row names: {parameters}"
    );
    assert!(
        !labels.contains(&"editable"),
        "a writable document is the normal state: {parameters}"
    );
    assert!(
        !labels.contains(&"Model") && !labels.contains(&"Section"),
        "{parameters}"
    );

    // An edit since the compile is worth a look, and says so.
    let bundle = app
        .state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
            CellViewRef::new("behavioral", "amp", "veriloga"),
        ))
        .expect("the bundle")
        .id();
    app.state
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(
            bundle,
            "amp.va",
            "module amp(p, n);\nendmodule\n".to_owned(),
        )
        .expect("the edit applies");
    let evidence = super::super::component_model_evidence(&app.state, &component);
    assert_eq!(
        evidence.problem.as_ref().map(|(problem, _)| *problem),
        Some(Problem::Warning)
    );
    let parameters = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(parameters.contains("Recompile needed"), "{parameters}");
}

/// A component's title says what it is: a cell instance by its cell, a
/// primitive by its device kind.
#[test]
fn a_components_title_names_what_it_instantiates() {
    let (_, module) = app_with_owned_veriloga_instance();
    assert_eq!(component_title_kind(&module), "amp");
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert_eq!(component_title_kind(&resistor), "Resistor");
}

/// The component panel opens on its title and names the occurrence only below
/// the design root, where it says something.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_component_title_states_the_occurrence_only_below_the_root() {
    let (mut app, id) = app_with_model_bound_instance();
    let sheet = sheet_connectivity(&app.state);
    let painted = painted_panel(|ui| component_panel(ui, &mut app, id, &sheet));
    let first = painted.lines().next().unwrap_or_default();
    assert!(
        first.starts_with("XU1") && !first.contains('/'),
        "the title leads the panel and says nothing of the root: {painted}"
    );

    app.state.workspace.descend_into(
        "XAFE".to_owned(),
        CellViewRef::new("user", "afe_core", "schematic"),
        ViewType::Schematic,
    );
    let painted = painted_panel(|ui| component_panel(ui, &mut app, id, &sheet));
    assert!(
        painted
            .lines()
            .next()
            .is_some_and(|line| line.ends_with("/XAFE")),
        "{painted}"
    );
}

/// Each of the shipped inverter's transistors names a model the built-in
/// library resolves, and keeps W and L as instance parameters. Its value once
/// read "NMOS W=1u L=0.18u": the Model property holding the instance
/// parameters too, which the inspector read whole as a model name and
/// reported missing.
#[test]
fn the_cmos_inverter_example_binds_models_the_library_resolves() {
    let mut state = AppState::default();
    crate::workbench::examples::load_example("CMOS Inverter", &mut state.schematic);
    for (name, model) in [("MP", "RSPICE_PMOS"), ("MN", "RSPICE_NMOS")] {
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("the inverter has no {name}"));
        assert_eq!(
            component_model_evidence(&state, component),
            ComponentModelEvidence::resolved(model),
            "{name}'s model binding"
        );
        let params = rspice_design::parameters::parse_params_string(&component.params);
        assert!(
            params.contains_key("w") && params.contains_key("l"),
            "{name} keeps W and L as instance parameters: {params:?}"
        );
    }
}

/// A waived error is a decision, not a failure: the sheet's row marks the
/// findings it counts as the Checks header and the run gate count them, so
/// errors that are all waived read as findings to note rather than as a
/// failure the header says is not there.
#[test]
fn a_sheet_row_of_waived_errors_is_not_a_failure() {
    let mut error = DrcViolation::new(
        0,
        rspice_design::drc::DrcViolationType::UnconnectedPin,
        "open",
        DrcLocation::Global,
    );
    error.severity = DrcSeverity::Error;
    assert_eq!(findings_problem(&[&error]), Problem::Failure);

    error.waiver = Some(rspice_design::drc::DrcWaiver {
        rule: "DRC-UNCONNECTED-PIN".to_owned(),
        object: "global".to_owned(),
        reason: "a spare pin".to_owned(),
        author: "ana".to_owned(),
        waived_at_ms: 1,
    });
    assert_eq!(findings_problem(&[&error]), Problem::Warning);
}
