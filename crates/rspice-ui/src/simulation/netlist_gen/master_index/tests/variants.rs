//! The frozen hierarchy and generated deck must describe the active variant.

use super::*;
use crate::state::{
    AssemblyVariantDraft, ComponentSubstitution, SchematicObjectKey, VariantInheritance,
    VariantObjectOverride, VariantQualificationPlan, VariantQualificationState,
};

fn fixture(configured: bool) -> (ProjectWorkspace, LibraryManager, SchematicState) {
    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);
    let mut work = Library::new("work");
    for name in ["div", "alternate"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        work.add_cell(cell);
    }
    libraries.add_library(work);
    let mut workspace = ProjectWorkspace::default();
    for (name, value) in [("div", "1k"), ("alternate", "7k")] {
        workspace.schematic_buffers.insert(
            CellViewRef::new("work", name, "schematic").key(),
            two_port_master(value),
        );
    }
    let top = top_with_instance(&["a", "b"]);
    workspace
        .schematic_buffers
        .insert(workspace.active_view.key(), top.clone());
    if configured {
        workspace
            .configuration_sets
            .create(ConfigurationSetDefinition {
                name: "Variant testbench".to_owned(),
                root: workspace.active_view.clone(),
                dut_path: "/".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "test".to_owned(),
            })
            .unwrap();
    }
    (workspace, libraries, top)
}

fn activate(
    workspace: &mut ProjectWorkspace,
    reference: &CellViewRef,
    component: u64,
    change: VariantObjectOverride,
) {
    let variant = workspace
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "Qualified variant".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([(
                SchematicObjectKey::new(&reference.key(), component).unwrap(),
                change,
            )]),
        })
        .unwrap();
    workspace
        .design_management
        .variants_mut()
        .set_active(variant)
        .unwrap();
}

fn replacement() -> VariantObjectOverride {
    VariantObjectOverride::Substitute {
        replacement: ComponentSubstitution {
            library: "work".to_owned(),
            cell: "alternate".to_owned(),
            view: "schematic".to_owned(),
            value_override: None,
            model_section: None,
            port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
            qualification: VariantQualificationState::Current,
        },
    }
}

fn omission() -> VariantObjectOverride {
    VariantObjectOverride::DoNotPopulate {
        approval_reference: "reviewed omission".to_owned(),
    }
}

fn connect_testbench(
    top: &mut SchematicState,
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
) {
    top.wires.clear();
    top.net_labels.clear();
    // Labels provide ground. Keep the supply outside the swept symbol bounds
    // so an orientation cannot accidentally short a DUT terminal to it.
    top.components
        .retain(|component| component.kind != ComponentType::Ground);
    for component in &mut top.components {
        if component.kind == ComponentType::VoltageSource {
            component.pos = Point::new(-1000, 1000);
        }
    }
    let resolver = crate::state::SymbolResolver::new(libraries, &workspace.schematic_buffers);
    for component in &top.components {
        if component.kind != ComponentType::CellInstance
            && component.kind != ComponentType::VoltageSource
        {
            continue;
        }
        let symbol = component
            .library_cell
            .as_ref()
            .and_then(|binding| resolver.resolve_binding(binding));
        for (index, (_, point)) in component
            .terminal_positions_resolved(symbol.as_ref())
            .into_iter()
            .enumerate()
        {
            top.net_labels.push(NetLabel::new(
                1000 + top.net_labels.len() as u64,
                point,
                if index == 0 { "drive" } else { "0" },
            ));
        }
    }
}

fn source_current(
    projection: &crate::state::workspace::DesignProjection,
    libraries: &LibraryManager,
) -> f64 {
    let hierarchy = HierarchySource::from_design_projection(libraries, projection);
    let generated =
        generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
    assert!(
        generated.errors.is_empty(),
        "{:?}\n{}",
        generated.errors,
        generated.netlist
    );
    let netlist = rspice_core::Netlist::parse(&generated.netlist).unwrap();
    let result = rspice_core::engine::Engine::new(rspice_core::SimulationConfig::default())
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("{error}\n{}", generated.netlist));
    let branch = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("V1"))
        .expect("testbench voltage source");
    result.branch_currents[branch]
}

#[test]
fn a_replacement_cannot_infer_the_pins_of_an_unresolved_source_instance() {
    let (mut workspace, libraries, mut top) = fixture(false);
    let source = top.components[0].library_cell.as_mut().unwrap();
    source.cell = "missing".to_owned();
    source.terminal_order.clear();
    let active = workspace.active_view.clone();
    activate(&mut workspace, &active, top.components[0].id, replacement());
    let error = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("source instance has no resolved pin contract"),
        "{error}"
    );
}

#[test]
fn a_live_master_interface_owns_the_source_pins_during_replacement() {
    let (mut workspace, libraries, top) = fixture(false);
    let active = workspace.active_view.clone();
    activate(&mut workspace, &active, top.components[0].id, replacement());
    let master = CellViewRef::new("work", "div", "schematic");
    let mut live = workspace.schematic_buffers[&master.key()].clone();
    place_port(&mut live, "bias", Point::new(200, 0));
    place_port(
        workspace
            .schematic_buffers
            .get_mut("work/alternate/schematic")
            .unwrap(),
        "bias",
        Point::new(300, 0),
    );
    let error = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("source has 2 terminals but replacement has 3"),
        "{error}"
    );
    let raw_buffers = serde_json::to_value(&workspace.schematic_buffers).unwrap();
    let projected = workspace
        .design_projection(&libraries, &master, &live)
        .unwrap();
    let placed = &projected.root_schematic().unwrap().components[0];
    assert_eq!(
        placed.library_cell.as_ref().unwrap().terminal_order,
        ["a", "b", "bias"]
    );
    assert_eq!(placed.execution_terminal_layout.as_ref().unwrap().len(), 3);
    assert_eq!(
        serde_json::to_value(&workspace.schematic_buffers).unwrap(),
        raw_buffers
    );
    let same = workspace
        .design_projection(&libraries, &master, &live)
        .unwrap();
    assert!(std::sync::Arc::ptr_eq(&projected, &same));
    live.components.pop();
    assert!(
        workspace
            .design_projection(&libraries, &master, &live)
            .is_err()
    );
}

#[test]
fn replacement_keeps_vector_terminals_and_their_conductor_order() {
    let (mut workspace, libraries, mut top) = fixture(false);
    for name in ["div", "alternate"] {
        workspace
            .schematic_buffers
            .get_mut(&format!("work/{name}/schematic"))
            .unwrap()
            .components[0]
            .value = "DATA[3:0]".to_owned();
    }
    top.components[0]
        .library_cell
        .as_mut()
        .unwrap()
        .bind_interface(&[
            PortSpec {
                name: "DATA[3:0]".to_owned(),
                direction: PortDirection::InOut,
            },
            PortSpec {
                name: "b".to_owned(),
                direction: PortDirection::InOut,
            },
        ]);
    let resolver = crate::state::SymbolResolver::new(&libraries, &workspace.schematic_buffers);
    let symbol = resolver.resolve_binding(top.components[0].library_cell.as_ref().unwrap());
    let original_pins = top.components[0].terminal_positions_resolved(symbol.as_ref());
    workspace
        .schematic_buffers
        .get_mut("work/alternate/schematic")
        .unwrap()
        .components
        .reverse();
    let active = workspace.active_view.clone();
    activate(&mut workspace, &active, top.components[0].id, replacement());
    let projection = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    let placed = &projection.root_schematic().unwrap().components[0];
    let binding = placed.library_cell.as_ref().unwrap();
    assert_eq!(binding.terminal_order, ["b", "DATA[3:0]"]);
    assert_eq!(binding.terminal_widths(), [1, 4]);
    assert_eq!(binding.terminal_node_count(), 5);
    assert_eq!(
        placed.terminal_positions_resolved(None),
        [original_pins[1].clone(), original_pins[0].clone()]
    );
}

#[test]
fn named_replacement_terminals_keep_their_nets_when_the_master_order_changes() {
    for configured in [false, true] {
        let (mut workspace, libraries, mut top) = fixture(configured);
        connect_testbench(&mut top, &workspace, &libraries);
        workspace
            .schematic_buffers
            .get_mut("work/alternate/schematic")
            .unwrap()
            .components
            .reverse();
        let active = workspace.active_view.clone();
        activate(&mut workspace, &active, top.components[0].id, replacement());
        let projection = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let deck =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(deck.errors.is_empty(), "{:?}", deck.errors);
        assert!(
            deck.netlist.contains(".subckt alternate b a"),
            "{}",
            deck.netlist
        );
        assert_eq!(instance_line(&deck.netlist, "X1"), "X1 0 drive alternate");
        assert!((source_current(&projection, &libraries) + 5.0 / 7000.0).abs() < 1e-12);
    }
}

#[test]
fn incompatible_replacements_refuse_without_changing_the_source_or_prior_projection() {
    for (nested, defect, expected) in [
        (
            false,
            "count",
            "source has 2 terminals but replacement has 1",
        ),
        (false, "name", "has no matching source terminal"),
        (true, "width", "changes conductor width"),
    ] {
        let (mut workspace, libraries, mut top) = fixture(false);
        connect_testbench(&mut top, &workspace, &libraries);
        let active = workspace.active_view.clone();
        let owner = if nested {
            CellViewRef::new("work", "div", "schematic")
        } else {
            active.clone()
        };
        let id = if nested {
            workspace.schematic_buffers[&owner.key()].components[1].id
        } else {
            top.components[0].id
        };
        activate(&mut workspace, &owner, id, replacement());
        let original = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        let original_master = workspace.schematic_buffers["work/alternate/schematic"].clone();
        let target = workspace
            .schematic_buffers
            .get_mut("work/alternate/schematic")
            .unwrap();
        match defect {
            "count" => target.components.retain(|component| component.value != "b"),
            "name" => target.components[0].value = "different".to_owned(),
            "width" => target.components[0].value = "a[3:0]".to_owned(),
            _ => unreachable!(),
        }
        let source = serde_json::to_value((&top, &workspace.schematic_buffers)).unwrap();
        let error = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap_err();
        assert!(error.to_string().contains(expected), "{defect}: {error}");
        assert_eq!(
            serde_json::to_value((&top, &workspace.schematic_buffers)).unwrap(),
            source
        );
        assert!((source_current(&original, &libraries) + 5.0 / 7000.0).abs() < 1e-12);
        workspace
            .schematic_buffers
            .insert("work/alternate/schematic".to_owned(), original_master);
        let restored = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        assert!(std::sync::Arc::ptr_eq(&original, &restored));
    }
}

#[test]
fn replacement_connectivity_follows_the_projected_sheet_translation() {
    use crate::state::{
        MoveBoundaryResolution, MoveSelectionRequest, SheetDefinition, SheetPortPolicy,
        SheetTemplate,
    };
    let (mut workspace, libraries, mut top) = fixture(true);
    connect_testbench(&mut top, &workspace, &libraries);
    let owner = CellViewRef::new("work", "div", "schematic");
    let objects = workspace.schematic_buffers[&owner.key()]
        .components
        .iter()
        .map(|component| component.id)
        .collect::<Vec<_>>();
    let source_id = objects[1];
    let first = workspace
        .design_management
        .bootstrap_for_cell_view(&owner.key(), "First", objects.clone())
        .unwrap();
    let sheets = workspace
        .design_management
        .sheet_catalog_mut(&owner.key())
        .unwrap();
    let second = sheets
        .create_sheet(
            SheetDefinition {
                name: "Second".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .unwrap();
    sheets
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: sheets.revision(),
            object_ids: objects,
            destination_sheet_id: second,
            boundary_resolution: MoveBoundaryResolution::VerifiedNoBoundaryNets,
        })
        .unwrap();
    activate(&mut workspace, &owner, source_id, replacement());
    let projection = workspace
        .design_projection(&libraries, &workspace.active_view, &top)
        .unwrap();
    let placed = &projection.schematic_buffers()[&owner.key()].components[1];
    assert_ne!(
        placed.pos,
        workspace.schematic_buffers[&owner.key()].components[1].pos
    );
    assert!((source_current(&projection, &libraries) + 5.0 / 7000.0).abs() < 1e-12);
}

#[test]
fn replacement_pin_layout_is_rebuilt_when_the_authored_symbol_changes() {
    let (mut workspace, mut libraries, mut top) = fixture(false);
    connect_testbench(&mut top, &workspace, &libraries);
    let active = workspace.active_view.clone();
    activate(&mut workspace, &active, top.components[0].id, replacement());
    let original = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    let document = SymbolDocument {
        origin: Point::new(10, 20),
        pins: vec![
            SymbolPin::new("a", PortDirection::InOut, Some(Point::new(-80, 30))),
            SymbolPin::new("b", PortDirection::InOut, Some(Point::new(90, -30))),
        ],
        ..SymbolDocument::default()
    };
    let mut view = View::new("symbol", ViewType::Symbol);
    document.store_in_view(&mut view).unwrap();
    libraries
        .get_library_mut("work")
        .unwrap()
        .get_cell_mut("div")
        .unwrap()
        .add_view(view);
    let updated = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    assert!(!std::sync::Arc::ptr_eq(&original, &updated));
    let layout = &updated.root_schematic().unwrap().components[0].execution_terminal_layout;
    assert_eq!(
        layout.as_deref().unwrap(),
        &[
            ("a".to_owned(), Point::new(-90, 10)),
            ("b".to_owned(), Point::new(80, -50))
        ]
    );
    assert_ne!(
        *layout,
        original.root_schematic().unwrap().components[0].execution_terminal_layout
    );
    connect_testbench(&mut top, &workspace, &libraries);
    let rewired = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    assert!((source_current(&rewired, &libraries) + 5.0 / 7000.0).abs() < 1e-12);
}

#[test]
fn variant_replacements_preserve_the_electrical_load_through_all_orientations() {
    use crate::state::{Component, Rotation};
    for configured in [false, true] {
        for authored in [false, true] {
            for nested in [false, true] {
                for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                    for mirror_h in [false, true] {
                        for mirror_v in [false, true] {
                            let (mut workspace, mut libraries, mut top) = fixture(configured);
                            if authored {
                                for (cell, left, right) in [
                                    ("div", Point::new(-50, 20), Point::new(60, -30)),
                                    ("alternate", Point::new(-150, 50), Point::new(200, -50)),
                                ] {
                                    let document = SymbolDocument {
                                        pins: vec![
                                            SymbolPin::new("a", PortDirection::InOut, Some(left)),
                                            SymbolPin::new("b", PortDirection::InOut, Some(right)),
                                        ],
                                        ..SymbolDocument::default()
                                    };
                                    let mut view = View::new("symbol", ViewType::Symbol);
                                    document.store_in_view(&mut view).unwrap();
                                    libraries
                                        .get_library_mut("work")
                                        .unwrap()
                                        .get_cell_mut(cell)
                                        .unwrap()
                                        .add_view(view);
                                }
                            }
                            let owner = if nested {
                                CellViewRef::new("work", "div", "schematic")
                            } else {
                                workspace.active_view.clone()
                            };
                            let source_document = if nested {
                                workspace.schematic_buffers.get_mut(&owner.key()).unwrap()
                            } else {
                                &mut top
                            };
                            let source_kind = if nested {
                                ComponentType::Resistor
                            } else {
                                ComponentType::CellInstance
                            };
                            let component = source_document
                                .components
                                .iter_mut()
                                .find(|component| component.kind == source_kind)
                                .unwrap();
                            component.rotation = rotation;
                            component.mirror_h = mirror_h;
                            component.mirror_v = mirror_v;
                            let source_id = component.id;
                            if nested {
                                let pins = component.terminal_positions();
                                for (port, (_, point)) in source_document
                                    .components
                                    .iter_mut()
                                    .filter(|component| component.kind == ComponentType::Port)
                                    .zip(pins)
                                {
                                    port.pos = point - ComponentType::Port.terminal_offsets()[0].1;
                                }
                            }
                            connect_testbench(&mut top, &workspace, &libraries);
                            let active = workspace.active_view.clone();
                            let original = workspace
                                .design_projection(&libraries, &active, &top)
                                .unwrap();
                            assert!(
                                (source_current(&original, &libraries) + 5.0 / 1000.0).abs()
                                    < 1e-12
                            );
                            let source_top = serde_json::to_value(&top).unwrap();
                            let source_buffers =
                                serde_json::to_value(&workspace.schematic_buffers).unwrap();
                            activate(&mut workspace, &owner, source_id, replacement());
                            let projection = workspace
                                .design_projection(&libraries, &active, &top)
                                .unwrap();
                            let actual = source_current(&projection, &libraries);
                            assert!(
                                (actual + 5.0 / 7000.0).abs() < 1e-12,
                                "configured={configured} authored={authored} nested={nested} rotation={rotation:?} mirrors={mirror_h}/{mirror_v}: {actual}"
                            );
                            let placed = projection.schematic_buffers()[&owner.key()]
                                .components
                                .iter()
                                .find(|component| component.id == source_id)
                                .unwrap();
                            assert!(placed.execution_terminal_layout.is_some());
                            let encoded = serde_json::to_value(placed).unwrap();
                            assert!(encoded.get("execution_terminal_layout").is_none());
                            let restored: Component = serde_json::from_value(encoded).unwrap();
                            assert!(restored.execution_terminal_layout.is_none());
                            assert_eq!(serde_json::to_value(&top).unwrap(), source_top);
                            assert_eq!(
                                serde_json::to_value(&workspace.schematic_buffers).unwrap(),
                                source_buffers
                            );
                            assert!(
                                (source_current(&original, &libraries) + 5.0 / 1000.0).abs()
                                    < 1e-12
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_primitive_replacement_uses_the_requested_schematic_views_ports() {
    let (mut workspace, mut libraries, mut top) = fixture(false);
    libraries
        .get_library_mut("work")
        .unwrap()
        .get_cell_mut("alternate")
        .unwrap()
        .add_view(View::new("rf_impl", ViewType::Schematic));
    let mut target = two_port_master("9k");
    for (port, name) in target
        .components
        .iter_mut()
        .filter(|component| component.kind == ComponentType::Port)
        .zip(["p", "n"])
    {
        port.value = name.to_owned();
    }
    workspace
        .schematic_buffers
        .insert("work/alternate/rf_impl".to_owned(), target);
    connect_testbench(&mut top, &workspace, &libraries);
    let owner = CellViewRef::new("work", "div", "schematic");
    let source_id = workspace.schematic_buffers[&owner.key()]
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Resistor)
        .unwrap()
        .id;
    let mut change = replacement();
    if let VariantObjectOverride::Substitute { replacement } = &mut change {
        replacement.view = "rf_impl".to_owned();
    }
    activate(&mut workspace, &owner, source_id, change);
    let projection = workspace
        .design_projection(&libraries, &workspace.active_view, &top)
        .unwrap();
    let placed = projection.schematic_buffers()[&owner.key()]
        .components
        .iter()
        .find(|component| component.id == source_id)
        .unwrap();
    assert_eq!(
        placed.library_cell.as_ref().unwrap().terminal_order,
        ["p", "n"]
    );
    assert!((source_current(&projection, &libraries) + 5.0 / 9000.0).abs() < 1e-12);
}

#[test]
fn a_variant_replacement_owns_the_plan_and_emitted_master() {
    for configured in [false, true] {
        let (mut workspace, libraries, top) = fixture(configured);
        let active = workspace.active_view.clone();
        let original = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        let instance = top.components.iter().find(|c| c.name == "X1").unwrap().id;
        activate(&mut workspace, &active, instance, replacement());
        let projection = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        assert!(!std::sync::Arc::ptr_eq(&original, &projection));
        let placed = projection.plan().binding(&instance_path("/X1")).unwrap();
        assert_eq!(placed.resolved_reference().cell, "alternate");
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(
            subckt_headers(&result.netlist),
            vec![".subckt alternate a b"]
        );
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with('R') && line.ends_with(" 7k"))
        );
        assert!(instance_line(&result.netlist, "X1").ends_with(" alternate"));
        assert_eq!(
            original
                .plan()
                .binding(&instance_path("/X1"))
                .unwrap()
                .resolved_reference()
                .cell,
            "div"
        );
        assert_eq!(
            top.components
                .iter()
                .find(|c| c.id == instance)
                .unwrap()
                .library_cell
                .as_ref()
                .unwrap()
                .cell,
            "div"
        );
        assert!(std::sync::Arc::ptr_eq(
            &projection,
            &workspace
                .design_projection(&libraries, &active, &top)
                .unwrap()
        ));
    }
}

#[test]
fn a_primitive_substitution_inside_a_buffered_master_enters_the_hierarchy() {
    for configured in [false, true] {
        let (mut workspace, libraries, top) = fixture(configured);
        let child = CellViewRef::new("work", "div", "schematic");
        let resistor = workspace.schematic_buffers[&child.key()]
            .components
            .iter()
            .find(|c| c.kind == ComponentType::Resistor)
            .unwrap()
            .id;
        activate(&mut workspace, &child, resistor, replacement());
        let projection = workspace
            .design_projection(&libraries, &workspace.active_view, &top)
            .unwrap();
        let nested = projection
            .plan()
            .binding(&instance_path("/X1/R1"))
            .expect("the replacement introduces a child instance");
        assert_eq!(nested.resolved_reference().cell, "alternate");
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line == "XR1 a b alternate"),
            "{}",
            result.netlist
        );
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with('R') && line.ends_with(" 7k"))
        );
        assert_eq!(
            workspace.schematic_buffers[&child.key()]
                .components
                .iter()
                .find(|c| c.id == resistor)
                .unwrap()
                .kind,
            ComponentType::Resistor
        );
    }
}

#[test]
fn omitted_instances_do_not_require_or_emit_their_missing_master() {
    for configured in [false, true] {
        let (mut workspace, libraries, mut top) = fixture(configured);
        let active = workspace.active_view.clone();
        let omitted =
            top.add_library_cell_component(Point::new(300, 0), binding("absent", &["a", "b"]));
        activate(&mut workspace, &active, omitted, omission());
        let projection = workspace
            .design_projection(&libraries, &active, &top)
            .expect("an omitted instance cannot block the populated circuit");
        assert!(projection.plan().binding(&instance_path("/X2")).is_none());
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(subckt_headers(&result.netlist), vec![".subckt div a b"]);
        assert!(!result.netlist.lines().any(|line| line.starts_with("X2 ")));
        assert!(top.components.iter().any(|c| c.id == omitted));
    }
}

#[test]
fn a_variant_cannot_silently_remove_the_configured_dut() {
    let (mut workspace, libraries, top) = fixture(true);
    let active = workspace.active_view.clone();
    let configuration = workspace.configuration_sets.active().unwrap().clone();
    let mut definition = configuration.definition().clone();
    definition.dut_path = "/X1".to_owned();
    workspace
        .configuration_sets
        .update(configuration.id(), configuration.revision(), definition)
        .unwrap();
    let original = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    let instance = top.components.iter().find(|c| c.name == "X1").unwrap().id;
    activate(&mut workspace, &active, instance, omission());
    let error = workspace
        .design_projection(&libraries, &active, &top)
        .expect_err("the selected DUT no longer exists in this variant");
    assert!(error.to_string().contains("configured DUT path"), "{error}");
    assert!(original.plan().binding(&instance_path("/X1")).is_some());
    assert!(top.components.iter().any(|c| c.id == instance));
}
