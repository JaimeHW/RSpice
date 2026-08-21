//! What the design navigator claims about the design it is reading.
//!
//! The hierarchy rows are the load-bearing ones: an unfolded node builds its
//! children and a folded one does not, a master that repeats on its own
//! ancestry is an inert leaf, and an occurrence click lands the session on
//! that occurrence. The rest hold the mockup geometry and the rail contracts
//! the sections were built to.

use super::hierarchy_tree::{DesignTreeRow, OccurrenceState};
use super::*;
use crate::state::{CellViewRef, InstancePath, SchematicState};
use crate::workbench::state::{NavigatorTreeNode, NavigatorTreeState, Workspace};

/// Give the project one more schematic master: the library cell view, and the
/// content the projection will materialize for it.
fn add_master(
    state: &mut crate::workbench::app_state::AppState,
    library: &str,
    cell: &str,
    schematic: SchematicState,
) {
    use crate::state::{Library, View, ViewType};

    if state.library_manager.get_library(library).is_none() {
        state.library_manager.add_library(Library::new(library));
    }
    let owner = state
        .library_manager
        .get_library_mut(library)
        .expect("the fixture library exists");
    let target = owner.get_or_create_cell(cell);
    if target.get_view("schematic").is_none() {
        target.add_view(View::new("schematic", ViewType::Schematic));
    }
    state.workspace.schematic_buffers.insert(
        CellViewRef::new(library, cell, "schematic").key(),
        schematic,
    );
}

/// One placed hierarchical instance, under the name the design gives it.
fn placed(id: u64, name: &str, library: &str, cell: &str) -> crate::state::Component {
    let mut component = crate::state::Component::new(
        id,
        ComponentType::CellInstance,
        crate::state::Point::new(40, 40),
    )
    .with_library_cell(crate::state::LibraryCellInstance::new(
        library,
        cell,
        "schematic",
    ));
    component.name = name.to_owned();
    component
}

/// A design that recurses. The root places `XA` of `work/child`, and
/// `work/child` places a leaf beside a second instance of the root cell — so
/// `/XA/XLOOP` is the occurrence the resolver refuses to descend.
fn recursive_design() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let root = app.state.workspace.active_view.clone();
    app.state
        .schematic
        .components
        .push(placed(101, "XA", "work", "child"));
    app.state.sync_active_schematic_to_workspace();

    let mut child = SchematicState::default();
    child.components.push(placed(201, "XLEAF", "work", "leaf"));
    child
        .components
        .push(placed(202, "XLOOP", &root.library, &root.cell));
    add_master(&mut app.state, "work", "child", child);
    add_master(&mut app.state, "work", "leaf", SchematicState::default());
    app
}

/// A design whose active cell view holds two sheets: the named source `VIN`
/// on the sheet the session is on, `VBIAS` on the one it is not.
#[cfg(not(target_arch = "wasm32"))]
fn two_sheet_named_signals() -> RSpiceApp {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    for (id, name) in [(301, "VIN"), (302, "VBIAS")] {
        let mut source = crate::state::Component::new(
            id,
            ComponentType::VoltageSource,
            crate::state::Point::new(20, 20),
        );
        source.name = name.to_owned();
        app.state.schematic.components.push(source);
    }
    app.state.sync_active_schematic_to_workspace();

    let key = app.state.workspace.active_schematic_reference().key();
    let first = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input stage", [301, 302])
        .expect("the fixture cell view takes a sheet catalog");
    let catalog = app
        .state
        .workspace
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
        .assign_objects(catalog.revision(), second, [302])
        .expect("the off-sheet source takes its assignment");
    catalog
        .set_active(first)
        .expect("the first sheet is active");
    app
}

/// Everything one navigator surface paints, at the dock's real width.
#[cfg(not(target_arch = "wasm32"))]
fn painted_panel(ctx: &egui::Context, mut contents: impl FnMut(&mut Ui)) -> String {
    painted_text(&ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                contents(ui);
            });
    }))
}

/// The object rails answer about the sheet on screen when the reader asks
/// them to, and about the whole cell view until then.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_narrows_the_object_rails_to_the_active_sheet() {
    let mut app = two_sheet_named_signals();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    sheet_visibility::set_sheet_scope(&ctx, SheetScope::AllSheets);
    let every_sheet = painted_panel(&ctx, |ui| named_signal_section(ui, &mut app));
    assert!(
        every_sheet.contains("VIN") && every_sheet.contains("VBIAS"),
        "the default scope lists the whole cell view: {every_sheet}"
    );

    sheet_visibility::set_sheet_scope(&ctx, SheetScope::ActiveSheet);
    let this_sheet = painted_panel(&ctx, |ui| named_signal_section(ui, &mut app));
    assert!(
        this_sheet.contains("VIN"),
        "an object on the active sheet is kept: {this_sheet}"
    );
    assert!(
        !this_sheet.contains("VBIAS"),
        "an object owned by another sheet is filtered out: {this_sheet}"
    );
}

/// A drawing with one sheet has nothing to scope, so the control is absent
/// rather than offered with one position that means anything.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_control_is_offered_only_above_one_sheet() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    let mut single = RSpiceApp::test_instance();
    let one_sheet = painted_panel(&ctx, |ui| navigator(ui, &mut single));
    assert!(
        !one_sheet.contains(SheetScope::ActiveSheet.label())
            && !one_sheet.contains(SheetScope::AllSheets.label()),
        "a single-sheet cell view offers no scope control: {one_sheet}"
    );

    let mut multi = two_sheet_named_signals();
    let two_sheets = painted_panel(&ctx, |ui| navigator(ui, &mut multi));
    assert!(
        two_sheets.contains(SheetScope::ActiveSheet.label())
            && two_sheets.contains(SheetScope::AllSheets.label()),
        "both positions are offered above one sheet: {two_sheets}"
    );
}

/// The scope is a reading position, not an edit: moving it must leave the
/// governed catalog and the project revision exactly where they were.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn moving_the_sheet_scope_never_touches_the_project() {
    let mut app = two_sheet_named_signals();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let revision = app.state.workspace.project.revision().get();
    let catalog = app.state.workspace.design_management.clone();

    for scope in SheetScope::OPTIONS {
        sheet_visibility::set_sheet_scope(&ctx, scope);
        let _ = painted_panel(&ctx, |ui| navigator(ui, &mut app));
    }

    assert_eq!(app.state.workspace.project.revision().get(), revision);
    assert_eq!(app.state.workspace.design_management, catalog);
}

/// The occurrence rows of one app against one reading position.
fn occurrence_rows(app: &RSpiceApp, tree: &NavigatorTreeState) -> Vec<DesignTreeRow> {
    let projection = app
        .state
        .workspace
        .design_projection(
            &app.state.library_manager,
            &app.state.workspace.active_view,
            &app.state.schematic,
        )
        .expect("the fixture design projects");
    hierarchy_tree::occurrence_rows(
        &hierarchy_tree::TreeSource {
            projection: projection.as_ref(),
            sheets: &app.state.workspace.design_management,
            query: "",
        },
        tree,
    )
}

/// Every occurrence the rows carry, with how far it can be read.
fn occurrence_states(rows: &[DesignTreeRow]) -> Vec<(InstancePath, OccurrenceState)> {
    rows.iter()
        .filter_map(|row| match row {
            DesignTreeRow::Occurrence(row) => Some((row.path.clone(), row.state)),
            DesignTreeRow::Sheet { .. } | DesignTreeRow::Truncated => None,
        })
        .collect()
}

/// Children are built by unfolding a node and by nothing else, and the
/// design's own cycle ends in a leaf that says why.
#[test]
fn navigator_expansion_is_lazy_and_cycle_guarded() {
    let app = recursive_design();
    let root = InstancePath::root();
    let instance = root.child("XA").expect("the fixture instance is nameable");
    let leaf = instance.child("XLEAF").expect("the leaf is nameable");
    let recursion = instance.child("XLOOP").expect("the cycle is nameable");
    let mut tree = NavigatorTreeState::default();

    assert_eq!(
        occurrence_states(&occurrence_rows(&app, &tree)),
        vec![(root.clone(), OccurrenceState::Collapsed)],
        "a folded root costs one row, not one row per instance below it"
    );

    tree.expand(NavigatorTreeNode::Occurrence(root.fold_key()));
    assert_eq!(
        occurrence_states(&occurrence_rows(&app, &tree)),
        vec![
            (root.clone(), OccurrenceState::Expanded),
            (instance.clone(), OccurrenceState::Collapsed),
        ],
        "unfolding one level builds one level"
    );

    tree.expand(NavigatorTreeNode::Occurrence(instance.fold_key()));
    let states = occurrence_states(&occurrence_rows(&app, &tree));
    assert_eq!(
        states,
        vec![
            (root, OccurrenceState::Expanded),
            (instance, OccurrenceState::Expanded),
            (leaf, OccurrenceState::Leaf),
            (recursion, OccurrenceState::Recursive),
        ]
    );
    assert!(
        !OccurrenceState::Recursive.is_bound(),
        "a recursive occurrence has nothing to descend into, so its row is inert"
    );
}

/// The row click is the descent: it lands the active document on exactly the
/// occurrence the row names, and on the way back up it ascends to it.
#[test]
fn an_occurrence_row_lands_the_session_on_that_occurrence() {
    let mut app = recursive_design();
    let instance = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    let leaf = instance.child("XLEAF").expect("the leaf is nameable");

    hierarchy_tree::open_occurrence(&mut app.state, &leaf);
    assert_eq!(app.state.workspace.occurrence_path(), leaf);
    assert_eq!(
        app.state.workspace.active_view,
        CellViewRef::new("work", "leaf", "schematic")
    );

    hierarchy_tree::open_occurrence(&mut app.state, &instance);
    assert_eq!(app.state.workspace.occurrence_path(), instance);
    assert_eq!(
        app.state.workspace.active_view,
        CellViewRef::new("work", "child", "schematic")
    );
}

/// A cell view with one sheet *is* its sheet, so the tree offers a sheet node
/// only where entering one means something.
#[test]
fn sheet_nodes_appear_only_above_one_sheet() {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.workspace.active_view.clone();
    let key = reference.key();
    let first = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("the fixture cell view takes a sheet catalog");
    assert!(
        hierarchy_tree::sheets_of(&app.state.workspace.design_management, &reference).is_empty()
    );

    let catalog = app
        .state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the catalog was just bootstrapped");
    catalog
        .create_sheet(
            crate::state::SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            None,
        )
        .expect("a second sheet");
    catalog
        .set_active(first)
        .expect("the first sheet is active");

    let sheets = hierarchy_tree::sheets_of(&app.state.workspace.design_management, &reference);
    assert_eq!(
        sheets
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>(),
        ["Sheet 1", "Sheet 2"]
    );
}

/// The unfold is a reading position held per workspace, so a second frame
/// reads back what the first left and another workspace reads its own.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn navigator_tree_expansion_survives_a_re_render_and_is_per_workspace() {
    let mut app = recursive_design();
    app.state.workbench.activate(Workspace::Design);
    let root = NavigatorTreeNode::Occurrence(InstancePath::root().fold_key());
    let instance = NavigatorTreeNode::Occurrence(
        InstancePath::root()
            .child("XA")
            .expect("the fixture instance is nameable")
            .fold_key(),
    );
    for node in [root.clone(), instance.clone()] {
        app.state
            .workbench
            .navigator_trees
            .for_workspace(Workspace::Design)
            .expand(node);
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut frame = || {
        painted_text(&ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    hierarchy_tree::occurrences_section(ui, &mut app);
                });
        }))
    };
    let first = frame();
    let second = frame();
    assert!(
        first.contains("XLEAF") && second.contains("XLEAF"),
        "an unfolded node keeps its children across frames: {second}"
    );
    assert!(
        second.contains("XLOOP") && second.contains("recursive"),
        "the recursive occurrence is painted and marked: {second}"
    );

    assert!(
        app.state
            .workbench
            .navigator_trees
            .for_workspace(Workspace::Design)
            .is_expanded(&instance)
    );
    assert!(
        !app.state
            .workbench
            .navigator_trees
            .for_workspace(Workspace::Verify)
            .is_expanded(&root),
        "one workspace's unfold is not another's"
    );
}

#[test]
fn design_tabs_keep_the_mockup_horizontal_inset() {
    let outer = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(260.0, 33.0));
    let content = panel_tabs_content_rect(outer);

    assert_eq!(PANEL_TABS_PADDING_X, 8.0);
    assert_eq!(content.left(), 8.0);
    assert_eq!(content.right(), 252.0);
}

#[test]
fn design_tabs_flex_from_their_label_widths_like_the_mockup() {
    let widths = flexible_tab_widths(239.0, [59.0, 95.0]);
    assert!((widths[0] - 101.5).abs() <= 0.001);
    assert!((widths[1] - 137.5).abs() <= 0.001);
    assert!((widths.iter().sum::<f32>() - 239.0).abs() <= 0.001);
}

/// The hierarchy leads: a design is read as the masters it declares and the
/// occurrences that instantiate them, and the object rails below answer about
/// the one sheet on screen.
#[test]
fn design_navigator_sections_lead_with_the_hierarchy() {
    assert_eq!(
        DESIGN_NAVIGATOR_SECTION_ORDER,
        [
            DesignNavigatorSection::Masters,
            DesignNavigatorSection::Occurrences,
            DesignNavigatorSection::Ports,
            DesignNavigatorSection::Nets,
            DesignNavigatorSection::NamedSignals,
        ]
    );
}

#[test]
fn navigator_path_names_the_occurrence_and_the_master_bound_there() {
    let mut workspace = crate::state::ProjectWorkspace::default();
    let (occurrence, master, can_ascend) = navigator_path(&workspace);
    assert_eq!(occurrence, "/");
    assert_eq!(master, "user/top");
    assert!(!can_ascend);

    workspace.descend_into(
        "XAFE".to_owned(),
        crate::state::CellViewRef::new("user", "afe_core", "schematic"),
        crate::state::ViewType::Schematic,
    );
    let (occurrence, master, can_ascend) = navigator_path(&workspace);
    assert_eq!(occurrence, "/XAFE");
    assert_eq!(master, "user/afe_core");
    assert!(can_ascend);

    workspace.descend_into(
        "XBIAS".to_owned(),
        crate::state::CellViewRef::new("user", "bias", "schematic"),
        crate::state::ViewType::Schematic,
    );
    assert_eq!(navigator_path(&workspace).0, "/XAFE/XBIAS");
}

#[test]
fn mockup_primitive_groups_cover_every_placeable_palette_entry_once() {
    let entries = PRIMITIVE_GROUPS
        .iter()
        .flat_map(|(_, sections)| primitive_entries(sections))
        .collect::<Vec<_>>();
    let unique = entries
        .iter()
        .map(|entry| entry.kind)
        .collect::<HashSet<_>>();

    assert_eq!(entries.len(), primitive_entry_count());
    assert_eq!(unique.len(), entries.len());
}

#[test]
fn shelf_search_matches_labels_case_insensitively() {
    assert!(matches_query("nmos", &["NMOS", "Semiconductors"]));
    assert!(!matches_query("nmos", &["Resistor", "Passives"]));
}

#[test]
fn named_signal_sources_exclude_passive_and_interface_objects() {
    assert!(is_named_source(ComponentType::VoltageSourcePulse));
    assert!(is_named_source(ComponentType::CurrentSourceNoise));
    assert!(is_named_source(ComponentType::BehavioralSource));
    assert!(!is_named_source(ComponentType::Resistor));
    assert!(!is_named_source(ComponentType::Port));
}

#[test]
fn raw_probe_targets_cover_scalar_differential_and_current_navigation() {
    assert_eq!(
        raw_probe_target("V(afe_out)"),
        Some(RawProbeTarget::Voltage {
            positive: "afe_out",
            negative: None,
        })
    );
    assert_eq!(
        raw_probe_target(" v(VREF) "),
        Some(RawProbeTarget::Voltage {
            positive: "VREF",
            negative: None,
        })
    );
    assert_eq!(
        raw_probe_target("V(out, in)"),
        Some(RawProbeTarget::Voltage {
            positive: "out",
            negative: Some("in"),
        })
    );
    assert_eq!(
        raw_probe_target("I(VDD)"),
        Some(RawProbeTarget::Current("VDD"))
    );
    assert_eq!(raw_probe_target("gain"), None);
    assert_eq!(raw_probe_target("V(out,)"), None);
}

#[test]
fn wireless_navigator_net_selection_is_exact_and_self_invalidating() {
    let mut app = RSpiceApp::test_instance();
    let net = DesignNet {
        name: "PORT_OUT".to_owned(),
        authored_name: true,
        class: crate::simulation::netlist_gen::NetClass::Signal,
        terminals: vec![crate::simulation::netlist_gen::NetTerminal {
            component_id: 9,
            reference: "X1".to_owned(),
            pin: "OUT".to_owned(),
        }],
        port: Some(crate::state::PortDirection::Out),
        wire_ids: Vec::new(),
    };
    app.state.schematic.selection.select_only_component(9);
    app.state
        .schematic
        .net_highlight
        .highlight_named_wires(&net.name, HashSet::new());
    assert!(navigator_net_selection_matches(&app, &net));

    app.state.schematic.selection.select_only_component(10);
    app.state.schematic.net_highlight.clear();
    assert!(!navigator_net_selection_matches(&app, &net));
}

#[test]
fn shelf_match_count_drives_a_truthful_filtered_empty_state() {
    let app = RSpiceApp::test_instance();
    assert!(component_shelf_match_count(&app, "resistor") > 0);
    assert_eq!(
        component_shelf_match_count(&app, "no-such-component-or-cell"),
        0
    );
}

#[test]
fn palette_placement_cancels_every_unfinished_conductor_route() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .start_wire(crate::state::Point::origin());
    app.state
        .schematic
        .start_bus(crate::state::Point::new(2, 3), None)
        .unwrap();

    arm_primitive(&mut app, ComponentType::Resistor, &egui::Context::default());

    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Resistor)
    );
    assert!(!app.state.schematic.wire_drawing.active);
    assert!(!app.state.schematic.bus_drawing.active);
}

#[test]
fn port_shelf_entry_uses_the_typed_place_pin_transaction() {
    let mut app = RSpiceApp::test_instance();

    arm_primitive(&mut app, ComponentType::Port, &egui::Context::default());

    assert!(app.state.dialogs.pin_port.open);
    assert_eq!(app.state.schematic.tool, Tool::Select);
    assert!(app.state.schematic.pending_port.is_none());
    assert!(app.state.schematic.components.is_empty());
}

/// Give the workspace an active configuration that cannot resolve: its
/// DUT path names an instance the expanded hierarchy has not got. The
/// design projection refuses such a configuration, so a rail that still
/// lists nets afterwards is listing the editor buffer's.
fn unresolve_configuration(state: &mut crate::workbench::app_state::AppState) {
    let root = state.workspace.active_view.clone();
    state
        .workspace
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

/// Everything a painted frame carries, as text.
///
/// The rail is read from its shapes rather than its accessibility tree: a
/// section that resolves paints selectable rows and one that does not
/// paints a plain row, and only the shapes carry both.
#[cfg(not(target_arch = "wasm32"))]
fn painted_text(output: &egui::FullOutput) -> String {
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

    let mut text = String::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut text);
    }
    text
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_nets_section_states_an_unresolved_configuration_instead_of_buffer_nets() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.wires.push(crate::state::Wire::segment(
        1,
        crate::state::Point::new(0, 0),
        crate::state::Point::new(40, 0),
    ));
    app.state
        .schematic
        .net_labels
        .push(crate::state::NetLabel::new(
            2,
            crate::state::Point::new(0, 0),
            "VOUT",
        ));
    app.state.sync_active_schematic_to_workspace();

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    let resolved = painted_text(&ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                net_section(ui, &mut app);
            });
    }));
    assert!(
        resolved.contains("VOUT"),
        "the fixture sheet lists its net while the configuration resolves: {resolved}"
    );

    unresolve_configuration(&mut app.state);

    let refused = painted_text(&ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                net_section(ui, &mut app);
            });
    }));
    assert!(
        refused.contains("XABSENT"),
        "the rail must state the projection's own reason: {refused}"
    );
    assert!(
        !refused.contains("VOUT"),
        "an unresolved configuration must not fall back to the editor buffer: {refused}"
    );
}

/// A library of pinned bytes the project retained: one macromodel, one card a
/// device family draws, one card no family does, and one section-scoped
/// subcircuit key a netlist cannot reference by name.
fn retained_model_library(name: &str) -> ModelLibrary {
    use crate::state::model_library::{DeviceModel, ModelSubcircuitInterface, ModelType};

    let mut library = ModelLibrary::new(name);
    library.source_authority = ModelSourceAuthority::External;
    for key in ["PROVING_DIV", "SECTIONED\u{1f}LOCAL"] {
        library.subcircuits.insert(
            key.to_owned(),
            ModelSubcircuitInterface {
                name: key.to_owned(),
                ports: vec!["IN".to_owned(), "OUT".to_owned(), "GND".to_owned()],
                parameter_defaults: std::collections::BTreeMap::new(),
                description: None,
                file_path: None,
                source_line: None,
                section: None,
            },
        );
    }
    let mut zener = DeviceModel::new("RSPICE_ZENER", ModelType::Diode);
    zener.spice_type = Some("D".to_owned());
    library.add_model(zener);
    library.add_model(DeviceModel::new("VENDOR_PRIVATE", ModelType::Other));
    library
}

/// The canvas-side shelf can place what the project already holds: a retained
/// part arms the cursor directly instead of re-raising the pack confirmation.
/// This is the road that used to dead-end — the row was filtered out of this
/// section, and the Project-library section below reads the symbol library,
/// not the model-library manager, so an adopted pack part had no shelf door.
#[test]
fn a_part_the_project_retained_arms_from_the_component_shelf() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model_library_manager
        .add_library(retained_model_library("proving_parts"));

    let rows = library_part_rows(&app, "proving_parts");
    let row = |name: &str| {
        rows.iter()
            .find(|row| row.part_id == name)
            .unwrap_or_else(|| panic!("no shelf row for '{name}' in {rows:?}"))
    };

    let macromodel = row("PROVING_DIV");
    assert_eq!(macromodel.meta, "subcircuit · in project");
    let LibraryPartAction::Arm(placement) = &macromodel.action else {
        panic!(
            "a retained macromodel arms directly: {:?}",
            macromodel.action
        );
    };
    let PartPlacement::CellInstance(binding) = placement.as_ref() else {
        panic!("a macromodel places as a cell instance over its own ports");
    };
    assert_eq!(
        binding.terminal_order,
        ["IN", "OUT", "GND"].map(str::to_owned)
    );

    assert_eq!(
        row("RSPICE_ZENER").action,
        LibraryPartAction::Arm(Box::new(PartPlacement::NativeDevice {
            component_type: ComponentType::Diode,
            variant: None,
            model: "RSPICE_ZENER".to_owned(),
        }))
    );

    // The card no schematic device is drawn for stays listed, refused as a
    // sentence rather than hidden.
    let LibraryPartAction::Refused(reason) = &row("VENDOR_PRIVATE").action else {
        panic!("an undrawable card is refused");
    };
    assert!(
        reason.contains("VENDOR_PRIVATE") && reason.ends_with('.'),
        "{reason}"
    );

    // A section-scoped subcircuit key is not a part a reader picks.
    assert!(!rows.iter().any(|row| row.part_id.contains('\u{1f}')));
}

/// A compiled-in foundation part arms from its built-in library, and its meta
/// says it is built in rather than "installed".
#[test]
fn a_foundation_part_arms_from_its_built_in_library() {
    use crate::state::model_library::{DeviceModel, ModelType};

    let mut library = ModelLibrary::new("rspice_foundation_probe");
    let mut card = DeviceModel::new("RSPICE_PROBE_NPN", ModelType::Npn);
    card.spice_type = Some("NPN".to_owned());
    library.add_model(card);
    let libraries = vec![&library];
    let index = crate::state::model_hub::provider::part_index(&libraries, &[], None, None);

    let rows = shelf_rows(&index, &libraries, "");
    let row = rows
        .iter()
        .find(|row| row.part_id == "RSPICE_PROBE_NPN")
        .expect("the foundation card is on the shelf");
    assert!(row.meta.ends_with("built in"), "{}", row.meta);
    assert!(
        matches!(&row.action, LibraryPartAction::Arm(placement)
        if matches!(placement.as_ref(), PartPlacement::NativeDevice {
            component_type: ComponentType::NpnBjt,
            ..
        })),
        "{:?}",
        row.action
    );
}

/// A part the project adopted is offered once, as the armable retained row —
/// not a second time as its pack's "review and add" row.
#[test]
fn an_adopted_part_is_offered_once_as_the_retained_row() {
    use rspice_pack::PartKind;

    let library = retained_model_library("proving_parts");
    let libraries = vec![&library];
    let mut index = crate::state::model_hub::provider::part_index(&libraries, &[], None, None);
    index.push(ModelHubPartRow {
        part_id: "RSPICE_ZENER".to_owned(),
        kind: PartKind::Model,
        device: "diode".to_owned(),
        terminals: vec!["A".to_owned(), "K".to_owned()],
        provenance: PartProvenance::InstalledPack {
            pack_id: "rspice-diodes".to_owned(),
            version: "1.0.0".to_owned(),
        },
        state: PartState::Installed,
        pack_name: Some("RSpice diodes".to_owned()),
        source: None,
    });

    let rows = shelf_rows(&index, &libraries, "");
    let zener = rows
        .iter()
        .filter(|row| row.part_id == "RSPICE_ZENER")
        .collect::<Vec<_>>();
    assert_eq!(zener.len(), 1, "{rows:?}");
    assert!(matches!(zener[0].action, LibraryPartAction::Arm(_)));
}

/// Pack releases the project has not adopted keep their two states: review
/// for one this engine can run, and a refusal naming the missing capability
/// for one it cannot.
#[test]
fn an_unadopted_release_reviews_and_an_incompatible_one_refuses() {
    use rspice_pack::PartKind;

    let release = |part: &str, state: PartState| ModelHubPartRow {
        part_id: part.to_owned(),
        kind: PartKind::Subckt,
        device: "opamp".to_owned(),
        terminals: Vec::new(),
        provenance: PartProvenance::RemoteRelease {
            pack_id: "vendor-amps".to_owned(),
            version: "2.1.0".to_owned(),
        },
        state,
        pack_name: Some("Vendor amplifiers".to_owned()),
        source: None,
    };
    let index = vec![
        release("VENDOR_OA1", PartState::Available),
        release(
            "VENDOR_RF9",
            PartState::Incompatible {
                missing: vec!["harmonic-balance-2".to_owned()],
            },
        ),
    ];

    let rows = shelf_rows(&index, &[], "");
    assert_eq!(
        rows[0].action,
        LibraryPartAction::Review {
            pack_id: "vendor-amps".to_owned(),
            version: "2.1.0".to_owned(),
            pack_name: "Vendor amplifiers".to_owned(),
        }
    );
    let LibraryPartAction::Refused(reason) = &rows[1].action else {
        panic!("an incompatible release is refused: {:?}", rows[1].action);
    };
    assert!(
        reason.contains("harmonic-balance-2") && reason.contains("Vendor amplifiers"),
        "{reason}"
    );
    assert_eq!(rows[1].meta, "opamp · needs harmonic-balance-2");
}
