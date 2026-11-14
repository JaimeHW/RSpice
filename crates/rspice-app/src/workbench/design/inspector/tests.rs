//! Tests for inspector editing and layout contracts.
//!
//! The cases pin that an inline edit session folds all its keystrokes into one
//! undo entry, that choosing the default section removes the instance
//! override rather than writing it, and that the panel geometry matches its
//! design contract.

mod off_sheet_connectors;
pub(crate) mod veriloga_modules;

use super::*;
use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
use crate::state::{
    Cell, Library, LibraryCellInstance, Point, PortDirection, SchematicProbe, View, ViewType,
};
use crate::workbench::docks::inspector::shared::{
    checks_status, sheet_floating_net_findings, sheet_open_pin_findings,
};
use crate::workbench::{AppState, RSpiceApp};
use rspice_design::drc::DrcSeverity;

pub(crate) fn state_with_two_components() -> AppState {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    state
        .model
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(40, 0));
    state
}

pub(crate) fn app_with_model_bound_instance() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let mut library = ModelLibrary::new("vendor_analog");
    library.add_model(DeviceModel::new("OPA189_A", ModelType::Other));
    library.add_model(DeviceModel::new("OPA189_B", ModelType::Other));
    library.add_model(DeviceModel::new("unrelated_nmos", ModelType::Nmos));
    app.state.model.model_library_manager.add_library(library);

    let mut binding = LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
    binding.module_name = Some("OPA189_A".to_owned());
    binding.netlist_template = Some("X{name} {nodes} {model} {params}".to_owned());
    binding.model_section = Some("tt".to_owned());
    let component = Component::new(41, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("XU1", "OPA189");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state.model.schematic.init_undo_history();
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

pub(crate) fn accesskit_nodes(
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

/// A rejected value reads from the panel's left edge, under the row labels,
/// rather than floating in the middle of its strip.
#[test]
fn a_rejection_starts_at_the_panels_inset() {
    use egui::accesskit::Role;

    let mut inset = 0.0;
    let nodes = accesskit_nodes(|ui| {
        inset = crate::workbench::design_system::panel_inset(ui);
        rejection_slot(ui, true, Some("pa takes 2 values; this has 3"));
    });
    let left = |role: Role| {
        nodes
            .iter()
            .find(|(_, node)| node.role() == role)
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| panic!("a {role:?} node with bounds"))
            .x0
    };
    let offset = left(Role::Label) - left(Role::Status);
    assert!(
        (offset - f64::from(inset)).abs() < 0.5,
        "the message starts {offset} into its strip, not at the {inset} inset"
    );
}

/// Everything one inspector panel paints, at the dock's real width.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn painted_panel(mut contents: impl FnMut(&mut Ui)) -> String {
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
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .probes
        .push(
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
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [PROBE])
        .expect("the fixture cell view takes a sheet catalog");
    if sheets > 1 {
        let catalog = app
            .state
            .model
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
    let painted = painted_panel(|ui| probe_panel(ui, &mut multi.design_cx(), probe));
    assert!(
        painted.contains("Sheet 2 \u{00b7} Output stage"),
        "the row names the ordinal and the name the catalog carries: {painted}"
    );

    let (mut single, probe) = app_with_probe_on_last_sheet(1);
    let painted = painted_panel(|ui| probe_panel(ui, &mut single.design_cx(), probe));
    assert!(
        !painted.contains("Input stage"),
        "a single-sheet cell view is its sheet, so the row is absent: {painted}"
    );
}

#[test]
fn an_empty_selection_inspects_the_sheet() {
    let state = AppState::default();
    assert_eq!(subject(&state.model, &[]), DesignSubject::Sheet);
}

#[test]
fn one_instance_beats_the_set_and_several_fall_through_to_multi() {
    let mut state = state_with_two_components();
    let ids: Vec<u64> = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|component| component.id)
        .collect();

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(ids[0]);
    assert_eq!(subject(&state.model, &[]), DesignSubject::Component(ids[0]));

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(ids[1]);
    assert_eq!(subject(&state.model, &[]), DesignSubject::Multi);
}

#[test]
fn one_probe_has_its_own_inspector_and_mixed_selection_falls_back_to_multi() {
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(73, Point::new(10, 20), "V(out)", Some("V(out)".to_owned())).unwrap(),
    );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_probe(73);

    assert_eq!(subject(&state.model, &[]), DesignSubject::Probe(73));

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(9);
    assert_eq!(subject(&state.model, &[]), DesignSubject::Multi);
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

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(7);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(8);
    assert_eq!(
        subject(&state.model, &nets),
        DesignSubject::Net("vout".to_owned())
    );

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(9);
    assert_eq!(subject(&state.model, &nets), DesignSubject::Multi);
}

#[test]
fn wireless_semantic_net_selection_routes_to_the_exact_net_inspector() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(
            71,
            ComponentType::CellInstance,
            Point::origin(),
        ));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(71);
    state
        .model
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
        subject(&state.model, &nets),
        DesignSubject::Net("PORT_OUT".to_owned())
    );
    state.model.schematic.session.editor.net_highlight.clear();
    assert_eq!(subject(&state.model, &nets), DesignSubject::Component(71));
}

#[test]
fn explicit_junction_selection_resolves_its_live_net() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            7,
            Point::new(-20, 0),
            Point::new(20, 0),
        ));
    state
        .model
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
        subject(&state.model, &nets),
        DesignSubject::Net("BIAS".to_owned())
    );
}

#[test]
fn a_selected_interface_port_routes_to_the_shared_net_inspector() {
    let mut state = AppState::default();
    let port =
        Component::new(77, ComponentType::Port, Point::origin()).with_name_value("P1", "VIN");
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(port);
    state
        .model
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

    assert_eq!(
        subject(&state.model, &nets),
        DesignSubject::Net("VIN".to_owned())
    );
}

#[test]
fn a_conductor_with_no_resolved_net_never_claims_one() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(42);

    // One unresolved wire is a single selected object, not a net.
    assert_eq!(subject(&state.model, &[]), DesignSubject::Multi);
}

#[test]
fn isolated_instance_terminals_are_open_not_bound_to_synthetic_nodes() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(42, ComponentType::Resistor, Point::origin()));

    let sheet = sheet_connectivity(&state.model);
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
    assert_eq!(net_engine_name(&state.model, "OUT").as_deref(), Some("out"));
    state.model.workspace.descend_into(
        "X1".to_owned(),
        CellViewRef::new("user", "inverter", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(
        net_engine_name(&state.model, "OUT").as_deref(),
        Some("x1.out")
    );
}

/// The Checks header is "not run" before any check, "stale" once the drawing
/// changes under one, and says nothing for a current, clean check.
#[test]
fn checks_header_speaks_only_when_the_check_is_not_current_and_clean() {
    let mut state = AppState::default();
    assert!(!checks_current(&state.model));
    assert_eq!(checks_status(&state.model).as_deref(), Some("not run"));

    state.model.design_checks.drc_results = Some(rspice_design::drc::DrcResult::new());
    state.model.design_checks.drc_checked_version =
        state.model.schematic.topology_version().wrapping_add(1);
    assert!(!checks_current(&state.model));
    assert_eq!(checks_status(&state.model).as_deref(), Some("stale"));

    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();
    assert!(checks_current(&state.model));
    assert_eq!(checks_status(&state.model), None);
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
            rspice_vocabulary::hierarchy_path::InstancePath::parse(&format!("/X{occurrence}"))
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
    state.model.design_checks.drc_results = Some(result);
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version();

    assert_eq!(sheet_open_pin_findings(&state.model).len(), 1);
    assert_eq!(sheet_floating_net_findings(&state.model).len(), 1);

    // A check of an earlier drawing reports nothing about this one.
    state.model.design_checks.drc_checked_version = state.model.schematic.topology_version() + 1;
    assert!(sheet_open_pin_findings(&state.model).is_empty());
}

/// A component's own findings are the ones located on it, or involving it on
/// this drawing. A finding inside a placed cell involves that cell's
/// components: `amp`'s `R1` is not this sheet's `R1`, and neither is another
/// instance on this sheet that happens to share its name.
#[test]
fn a_component_does_not_claim_a_placed_cells_finding_of_the_same_name() {
    use rspice_design::drc::{DrcLocation, DrcRelated, DrcRelation};

    let state = state_with_two_components();
    let resistor = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Resistor)
        .expect("the fixture resistor")
        .clone();
    let leg = |id: u64| DrcLocation::Terminal {
        component_id: id,
        component_name: resistor.name.clone(),
        pin_name: "1".to_owned(),
        point: crate::state::Point::new(0, 0),
    };
    let node = DrcLocation::Node {
        net_name: "n1".to_owned(),
        terminals: Vec::new(),
    };
    let inside = |location: DrcLocation| DrcLocation::InsideInstance {
        path: rspice_vocabulary::hierarchy_path::InstancePath::parse("/X1")
            .expect("an occurrence path"),
        view: crate::state::CellViewRef::new("work", "amp", "schematic"),
        inner: Box::new(location),
    };
    let floating = |location, on_node| {
        rspice_design::drc::DrcViolation::new(
            1,
            rspice_design::drc::DrcViolationType::FloatingNode,
            "floating",
            location,
        )
        .with_related(vec![DrcRelated::new(on_node, DrcRelation::OnNode)])
    };

    assert!(violation_targets_component(
        &floating(node.clone(), leg(resistor.id)),
        &resistor
    ));
    assert!(!violation_targets_component(
        &floating(node.clone(), leg(resistor.id + 1000)),
        &resistor
    ));
    assert!(!violation_targets_component(
        &floating(inside(node), inside(leg(resistor.id))),
        &resistor
    ));
}

#[test]
fn an_instance_rename_is_rejected_when_it_collides_or_is_empty() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    state.model.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
    state.model.schematic.document_mut_for_test().components[1].name = "R2".to_owned();
    let subject = state.model.schematic.document().components[0].clone();

    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Instance,
            "R7"
        )
        .is_none()
    );
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Instance,
            "R1"
        )
        .is_none(),
        "its own name is not a collision"
    );
    let collision = field_rejection(
        &state.design,
        &state.model,
        &state.workbench,
        &subject,
        &InlineEditField::Instance,
        "r2",
    )
    .expect("case-insensitive collision is rejected");
    assert!(collision.contains("already exists"), "was {collision}");
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Instance,
            "  "
        )
        .is_some()
    );
}

#[test]
fn an_instance_rename_still_obeys_the_family_designator_rule() {
    let mut state = state_with_two_components();
    state.model.schematic.document_mut_for_test().components[0].name = "R1".to_owned();
    let resistor = state.model.schematic.document().components[0].clone();

    let rejected = field_rejection(
        &state.design,
        &state.model,
        &state.workbench,
        &resistor,
        &InlineEditField::Instance,
        "C1",
    )
    .expect("a resistor cannot take a capacitor designator");
    assert!(rejected.contains('R'), "was {rejected}");
}

#[test]
fn declared_parameters_are_typed_while_unknown_extensions_remain_lossless() {
    let state = state_with_two_components();
    let subject = state.model.schematic.document().components[0].clone();

    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Value,
            "10k"
        )
        .is_none()
    );
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Value,
            ""
        )
        .is_none()
    );
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Parameter("noisy".to_owned()),
            "maybe"
        )
        .is_some()
    );
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &InlineEditField::Parameter("noisy".to_owned()),
            "yes"
        )
        .is_none()
    );
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
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
        .model
        .schematic
        .add_component(ComponentType::VoltageSourcePwl, Point::new(0, 0));
    let subject = state.model.schematic.document().components[0].clone();
    let field = InlineEditField::Parameter("td".to_owned());

    let inline = field_rejection(
        &state.design,
        &state.model,
        &state.workbench,
        &subject,
        &field,
        "-1ns",
    )
    .expect("a negative PWL delay is refused");

    // What the same delay says through the shared gate, read there rather than
    // restated here. The parameter is spelled without its unit because this
    // stands in for the editor's *typed* draft: both surfaces resolve the field
    // to a number before the waveform rules see it, which is the whole point.
    let mut committed = subject.clone();
    committed.params = "td=-1e-9".to_owned();
    let sheet = state
        .design
        .property_registry
        .get(committed.kind)
        .expect("a PWL source has a registered property sheet");
    let values = crate::properties::property_bridge::collect_properties_from_component(
        &committed,
        &state.design.property_registry,
    );
    let editor =
        crate::properties::property_bridge::source_commit_refusal(&committed, &values, sheet)
            .expect("the editor refuses the same value");

    assert_eq!(inline, editor, "one refusal, two surfaces");
    assert!(inline.contains("PWL TD"), "was {inline}");
    assert!(
        field_rejection(
            &state.design,
            &state.model,
            &state.workbench,
            &subject,
            &field,
            "1ns"
        )
        .is_none(),
        "a non-negative delay still commits"
    );
}

#[test]
fn fresh_standard_instances_project_their_complete_typed_parameter_contract() {
    let state = AppState::default();
    let component = Component::new(7, ComponentType::Resistor, Point::origin());
    let contract =
        inline_parameter_contract(&state.design, &state.model, &component, &HashMap::new());
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
    assert_eq!(containing_occurrence_path(&state.model), "/");

    state.model.workspace.descend_into(
        "XAFE".to_owned(),
        CellViewRef::new("user", "afe_core", "schematic"),
        ViewType::Schematic,
    );
    let mut binding = LibraryCellInstance::new("user", "precision_r", "symbol");
    binding.netlist_template = Some("R{name} {nodes} {model} {params}".to_owned());
    let component = Component::new(9, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("RGAIN", "499");

    assert_eq!(containing_occurrence_path(&state.model), "/XAFE");
    state.model.workspace.descend_into(
        "XBIAS".to_owned(),
        CellViewRef::new("user", "bias", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(containing_occurrence_path(&state.model), "/XAFE/XBIAS");
    assert_eq!(instance_view(&component), Some("symbol"));
    let resistor = Component::new(10, ComponentType::Resistor, Point::origin());
    assert_eq!(instance_view(&resistor), None, "a primitive binds no view");
}

#[test]
fn model_choices_stay_inside_the_bound_library_and_device_family() {
    let (app, id) = app_with_model_bound_instance();
    let component = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("bound component");
    let choices = bound_model_choices(&app.state.model, component, "OPA189_A");
    let values = choices
        .iter()
        .map(|(value, _)| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(values, ["OPA189_A", "OPA189_B"]);
    assert_eq!(
        catalog_model_location(&app.state.model, component),
        Some(("vendor_analog".to_owned(), "OPA189_A".to_owned()))
    );
}

#[test]
fn a_veriloga_cell_opens_its_exact_code_source_target() {
    let mut state = AppState::default();
    let mut cell = Cell::new("sensor_bridge");
    cell.add_view(View::new("veriloga", ViewType::VerilogA));
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    let binding = LibraryCellInstance::new("behavioral", "sensor_bridge", "veriloga");
    let component = Component::new(7, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("XBRIDGE", "sensor_bridge");

    assert_eq!(
        component_model_source_target(&state.model, &component),
        Some(ComponentModelSourceTarget::VerilogA(CellViewRef::new(
            "behavioral",
            "sensor_bridge",
            "veriloga"
        )))
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
    let rows = operating_point_rows(
        &entry,
        crate::quantity::QuantityPresentationPolicy::default(),
    );
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
    rspice_app_state::workbench::examples::load_example(
        "CMOS Inverter",
        &mut state.model.schematic,
    );
    let mut component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == "MN")
        .expect("the inverter has an MN")
        .clone();

    let declared = crate::state::parse_params_string(&component.params);
    let rows = inline_parameter_contract(&state.design, &state.model, &component, &declared);
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
    let offered = keys(&inline_parameter_contract(
        &state.design,
        &state.model,
        &component,
        &declared,
    ));
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
        device_engine_name(&state.model, &component).as_deref(),
        Some("mn")
    );
    state.model.workspace.descend_into(
        "X1".to_owned(),
        CellViewRef::new("user", "inverter", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(
        device_engine_name(&state.model, &component).as_deref(),
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

/// Give the workspace an active configuration that cannot resolve: its DUT
/// path names an instance the expanded hierarchy has not got. The design
/// projection refuses such a configuration, so a surface that still shows
/// connectivity afterwards is showing the editor buffer's.
fn unresolve_configuration(state: &mut AppState) {
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
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
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");
}

#[test]
fn the_inspector_states_an_unresolved_configuration_instead_of_buffer_connectivity() {
    let mut state = AppState::default();
    state
        .model
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
        !sheet_connectivity(&state.model).nets.is_empty(),
        "the fixture sheet has connectivity while its configuration resolves"
    );

    unresolve_configuration(&mut state);

    let sheet = sheet_connectivity(&state.model);
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            FIRST_WIRE,
            Point::new(0, 0),
            Point::new(40, 0),
        ));
    state
        .model
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
        rspice_design::connectivity::summary::design_nets(&state.model.schematic).len(),
        1,
        "the editor buffer holds one coordinate space, so its pages overlap"
    );

    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Page 1", [FIRST_WIRE])
        .expect("a fresh cell view accepts its first governed sheet");
    let catalog = state
        .model
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

    let sheet = sheet_connectivity(&state.model);
    assert!(sheet.unresolved.is_none(), "the fixture design resolves");
    assert_eq!(
        sheet.nets.len(),
        2,
        "the inspector reads the projected document, not the editor buffer"
    );
}

/// A plan's run reads the library it binds, so the instance states that
/// provider's card even when another drawing's library defines the same name.
#[test]
fn a_model_the_plan_binds_reads_as_resolved_beside_another_drawings_copy() {
    let mut state = AppState::default();
    state.model.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let bytes = format!(".model shared_diode D(Is=1n) ; {name}\n");
        state
            .model
            .model_library_manager
            .load_library_bytes(&format!("{name}.lib"), bytes.into_bytes(), None)
            .expect("the fixture source parses");
    }
    let component = Component::new(77, ComponentType::Diode, Point::origin())
        .with_name_value("D77", "shared_diode");
    let ambiguous = super::component_model_evidence(&state.model, &component)
        .problem
        .expect("two libraries and no plan binding is a contest");
    assert_eq!(ambiguous.0, Problem::Failure);

    let binding = state
        .model
        .model_library_manager
        .simulation_plan_binding("beta")
        .expect("the fixture library is bindable");
    state.model.sim_setup.model_bindings.push(binding);
    let evidence = super::component_model_evidence(&state.model, &component);
    assert_eq!(evidence.problem, None, "the run reads beta alone");
}

#[test]
fn a_resolved_contested_model_reads_as_resolved_and_keeps_its_source_jump() {
    use crate::state::model_library::ModelConsumerScope;

    let mut state = AppState::default();
    state.model.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
        state.model.model_library_manager.add_library(library);
    }
    let component = Component::new(77, ComponentType::Diode, Point::origin())
        .with_name_value("D77", "shared_diode");

    // Two providers and no decision: nothing resolves it, so the row says so
    // and there is nowhere to jump.
    let contested = super::component_model_evidence(&state.model, &component);
    let (problem, text) = contested
        .problem
        .expect("an unresolved contest is a problem");
    assert_eq!(problem, Problem::Failure);
    assert!(
        text.starts_with("Model shared_diode ambiguous"),
        "an unresolved contest is ambiguous: {text}"
    );
    assert_eq!(catalog_model_location(&state.model, &component), None);

    // The project resolves it. Execution and the Models workspace now both run
    // `beta`, so the inspector must stop calling the same reference ambiguous
    // and must offer the source of the copy that actually wins.
    state
        .model
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "shared_diode",
            "beta",
            "Test selects the executable provider.",
        )
        .expect("two providers accept a project-global decision");
    let resolved = super::component_model_evidence(&state.model, &component);
    assert_eq!(
        resolved.problem, None,
        "a recorded decision resolves the reference"
    );
    assert_eq!(
        catalog_model_location(&state.model, &component),
        Some(("beta".to_owned(), "shared_diode".to_owned())),
        "the jump follows the effective provider"
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
    rspice_app_state::workbench::examples::load_example(
        "CMOS Inverter",
        &mut state.model.schematic,
    );
    for (name, model) in [("MP", "RSPICE_PMOS"), ("MN", "RSPICE_NMOS")] {
        let component = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == name)
            .unwrap_or_else(|| panic!("the inverter has no {name}"));
        assert_eq!(
            component_model_evidence(&state.model, component),
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
