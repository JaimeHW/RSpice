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
///
/// Asked of the excitation rail, which is where a source is listed: named
/// signals is the plan's saved probes, and a probe has no sheet of its own.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_narrows_the_object_rails_to_the_active_sheet() {
    let mut app = two_sheet_named_signals();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    sheet_visibility::set_sheet_scope(&ctx, SheetScope::AllSheets);
    let every_sheet = painted_panel(&ctx, |ui| excitation_section(ui, &mut app));
    assert!(
        every_sheet.contains("VIN") && every_sheet.contains("VBIAS"),
        "the default scope lists the whole cell view: {every_sheet}"
    );

    sheet_visibility::set_sheet_scope(&ctx, SheetScope::ActiveSheet);
    let this_sheet = painted_panel(&ctx, |ui| excitation_section(ui, &mut app));
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

/// The object menu's find row answers about the object it was raised over:
/// the rail comes back filtering by that object's own name, not by nothing.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn finding_references_seeds_the_navigator_filter_with_the_object_name() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.design_panel = DesignPanel::ComponentShelf;

    find_navigator_object_references(
        &mut app,
        &NavigatorObject::Net {
            name: "VOUT".to_owned(),
            wire_ids: Vec::new(),
            component_ids: Vec::new(),
            position: None,
        },
    );

    assert_eq!(
        app.state
            .workbench
            .navigator_trees
            .filter(Workspace::Design),
        "VOUT"
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.workbench.design_panel, DesignPanel::Navigator);
    assert!(app.state.workbench.navigator_visible);
    assert!(app.state.workbench.focus_navigator_search);
}

/// The keyboard find is a different promise: it raises an empty box to type
/// into, so whatever the rail was already filtering by survives it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_plain_find_command_leaves_the_navigator_filter_alone() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    *app.state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Design) = "VOUT".to_owned();

    Command::FindInDesign.execute(&mut app);

    assert_eq!(
        app.state
            .workbench
            .navigator_trees
            .filter(Workspace::Design),
        "VOUT"
    );
    assert!(app.state.workbench.focus_navigator_search);
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
            DesignNavigatorSection::Excitations,
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
        .flat_map(|(_, _, _, sections)| primitive_entries(sections))
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

/// Every run of text a frame painted: what it says, where it landed, and the
/// colour it was set in.
///
/// The rail is read from its shapes rather than its accessibility tree: a
/// section that resolves paints selectable rows and one that does not
/// paints a plain row, and only the shapes carry both. The colour is here
/// because a meta column that states a hazard has to be told apart from one
/// that states a count, and that difference is only ever a colour.
#[cfg(not(target_arch = "wasm32"))]
fn painted_runs(output: &egui::FullOutput) -> Vec<(String, egui::Rect, egui::Color32)> {
    fn walk(shape: &egui::epaint::Shape, into: &mut Vec<(String, egui::Rect, egui::Color32)>) {
        match shape {
            egui::epaint::Shape::Text(painted) => into.push((
                painted.galley.job.text.clone(),
                egui::Rect::from_min_size(painted.pos, painted.galley.size()),
                painted.override_text_color.unwrap_or_else(|| {
                    painted
                        .galley
                        .job
                        .sections
                        .first()
                        .map_or(egui::Color32::PLACEHOLDER, |section| section.format.color)
                }),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }

    let mut runs = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut runs);
    }
    runs
}

/// Everything a painted frame carries, as text.
#[cfg(not(target_arch = "wasm32"))]
fn painted_text(output: &egui::FullOutput) -> String {
    painted_runs(output)
        .into_iter()
        .fold(String::new(), |mut text, (run, _, _)| {
            text.push_str(&run);
            text.push('\n');
            text
        })
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
    // The catalog's word for a subcircuit, not a second one: the same column
    // carries pack rows, and the class chip that narrows it spells it this way.
    assert_eq!(macromodel.meta, "subckt · in project");
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
    let index = crate::state::model_hub::provider::part_index(
        &libraries,
        &[],
        None,
        &crate::state::model_hub::Recalls::default(),
        None,
    );

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
    let mut index = crate::state::model_hub::provider::part_index(
        &libraries,
        &[],
        None,
        &crate::state::model_hub::Recalls::default(),
        None,
    );
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

// ---------------------------------------------------------- rails that fold

/// One interface pin, drawn as the typed port editor writes it.
fn port(
    id: u64,
    name: &str,
    netlist_order: Option<usize>,
    direction: PortDirection,
    documentation: &str,
) -> crate::state::Component {
    use crate::state::{PortContract, PortDiscipline, PortSignalType};

    let mut component = crate::state::Component::new(
        id,
        ComponentType::Port,
        crate::state::Point::new(20, 20 + id as i32),
    );
    component.value = name.to_owned();
    component.params = PortContract {
        direction,
        signal_type: PortSignalType::Analog,
        discipline: PortDiscipline::Electrical,
        netlist_order,
        documentation: documentation.to_owned(),
    }
    .encoded_params();
    component
}

/// A cell whose interface is declared out of document order: the pin the
/// contract puts second is the one drawn first.
fn interface_design() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    for (id, name, order, direction) in [
        (401, "BETA", 2, PortDirection::In),
        (402, "ALPHA", 1, PortDirection::In),
        (403, "GAMMA", 3, PortDirection::Out),
    ] {
        app.state.schematic.components.push(port(
            id,
            name,
            Some(order),
            direction,
            "carries the proving signal",
        ));
    }
    app.state.sync_active_schematic_to_workspace();
    app
}

/// A press and a release at one point, which is what egui reads as a click.
#[cfg(not(target_arch = "wasm32"))]
fn click_events(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// The titles the navigator's bands carry, in the order it stacks them.
#[cfg(not(target_arch = "wasm32"))]
const SECTION_TITLES: [&str; 6] = [
    "Masters",
    "Occurrences",
    "Ports",
    "Nets",
    "Excitations",
    "Named signals",
];

/// The navigator held open across frames, so a press and the frame that reads
/// it belong to one session.
///
/// A disclosure is only worth anything if it survives the frame it was set in,
/// and a case that wrote the persisted flag itself would prove nothing about
/// the band the reader actually presses.
#[cfg(not(target_arch = "wasm32"))]
struct NavigatorPanel {
    ctx: egui::Context,
    app: RSpiceApp,
    /// Every control the frame announced: what it says, where it is, and the
    /// disclosure position it publishes.
    controls: Vec<(String, egui::Rect, Option<bool>)>,
    /// Every run of text the frame painted.
    runs: Vec<(String, egui::Rect, egui::Color32)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NavigatorPanel {
    fn open(app: RSpiceApp) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut panel = Self {
            ctx,
            app,
            controls: Vec::new(),
            runs: Vec::new(),
        };
        // Twice: the first pass builds the font set and the second lays out
        // against it, and a band measured before the fonts exist is not the
        // band the header ends up in.
        panel.pass(Vec::new());
        panel.pass(Vec::new());
        panel
    }

    /// One rendered pass, and what it published.
    fn pass(&mut self, events: Vec<egui::Event>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(260.0, 1600.0),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.set_width(260.0);
                        navigator(ui, app);
                    });
            },
        );
        self.runs = painted_runs(&output);
        self.controls = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                            node.is_expanded(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    /// The one band announcing `title`.
    fn band(&self, title: &str) -> (egui::Rect, Option<bool>) {
        let hits = self
            .controls
            .iter()
            .filter(|(label, _, _)| label == title)
            .collect::<Vec<_>>();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly one control announcing {title:?}, found {}; the rail announces: \
             {:#?}",
            hits.len(),
            self.controls
                .iter()
                .map(|(label, _, _)| label.as_str())
                .collect::<Vec<_>>()
        );
        (hits[0].1, hits[0].2)
    }

    /// The disclosure position the band publishes to a screen reader.
    fn expanded(&self, title: &str) -> Option<bool> {
        self.band(title).1
    }

    /// Press the band, then settle the frame the press produced.
    fn click(&mut self, title: &str) {
        let at = self.band(title).0.center();
        self.pass(click_events(at));
        self.pass(Vec::new());
    }

    /// The count the section states, read from the band its own title is
    /// painted in rather than from the frame at large.
    fn stated_count(&self, title: &str) -> Option<String> {
        let heading = title.to_uppercase();
        let band = self
            .runs
            .iter()
            .find(|(run, _, _)| run.as_str() == heading)?
            .1;
        self.runs
            .iter()
            .find(|(run, rect, _)| {
                run.as_str() != heading && (rect.center().y - band.center().y).abs() <= 2.0
            })
            .map(|(run, _, _)| run.clone())
    }

    /// Every run one section painted under its own band.
    ///
    /// Read from the band down to the next one rather than from the frame at
    /// large, because the rails answer about one design and say the same words
    /// about it: a port named `ALPHA` puts `ALPHA` in the ports rail and in the
    /// nets rail both, and a search of the whole frame would call a folded
    /// section open on the strength of its neighbour.
    fn rows_under(&self, title: &str) -> Vec<String> {
        let band = self.band(title).0;
        let next = SECTION_TITLES
            .iter()
            .filter(|other| **other != title)
            .map(|other| self.band(other).0.top())
            .filter(|top| *top > band.top())
            .fold(f32::INFINITY, f32::min);
        self.runs
            .iter()
            .filter(|(_, rect, _)| rect.top() >= band.bottom() && rect.bottom() <= next)
            .map(|(run, _, _)| run.clone())
            .collect()
    }
}

/// A section folds on a press, stays folded into the next frame, and goes on
/// stating what it holds while folded.
///
/// The caret was decoration before this: every band pointed down and nothing
/// answered a press, so a reader working in a long rail had no way to put one
/// group aside. What makes it honest is the pair — the rows go, the count
/// stays — because a header that hid its count would trade a rail you cannot
/// shorten for one that will not say what it is hiding.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_navigator_section_folds_on_a_press_and_keeps_stating_what_it_holds() {
    let mut panel = NavigatorPanel::open(interface_design());

    assert_eq!(
        panel.expanded("Ports"),
        Some(true),
        "sections are open until a reader folds one"
    );
    assert_eq!(panel.stated_count("Ports").as_deref(), Some("3"));
    assert!(
        panel.rows_under("Ports").contains(&"ALPHA".to_owned()),
        "an open rail paints its rows: {:?}",
        panel.rows_under("Ports")
    );

    panel.click("Ports");
    assert_eq!(panel.expanded("Ports"), Some(false));
    assert_eq!(
        panel.rows_under("Ports"),
        Vec::<String>::new(),
        "a folded rail paints no rows"
    );
    assert_eq!(
        panel.stated_count("Ports").as_deref(),
        Some("3"),
        "a folded rail still states what it holds"
    );

    // The frame after the press: a position that did not survive it would be a
    // caret that flickers rather than a section that folds.
    panel.pass(Vec::new());
    assert_eq!(panel.expanded("Ports"), Some(false));
    assert_eq!(panel.rows_under("Ports"), Vec::<String>::new());

    panel.click("Ports");
    assert_eq!(panel.expanded("Ports"), Some(true));
    assert!(
        panel.rows_under("Ports").contains(&"ALPHA".to_owned()),
        "a second press unfolds it again"
    );
}

/// Folding one section leaves every other one alone.
///
/// The position is held per title, so a rail that keyed them together would
/// fold the whole navigator on one press.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn folding_one_navigator_section_leaves_the_others_open() {
    let mut panel = NavigatorPanel::open(interface_design());

    panel.click("Ports");

    assert_eq!(panel.expanded("Ports"), Some(false));
    for title in [
        "Masters",
        "Occurrences",
        "Nets",
        "Excitations",
        "Named signals",
    ] {
        assert_eq!(
            panel.expanded(title),
            Some(true),
            "{title} was not the section that was pressed"
        );
    }
}

/// The ports rail lists the interface in the order the deck has it, and numbers
/// every pin with the position it occupies there.
///
/// Document order is the order the pins happened to be drawn in, which is no
/// order at all: the `.SUBCKT` line and the node order of every instance of
/// this cell come from `netlist_order`. A rail numbering rows by their place in
/// the file would call a pin `#1` that the deck calls third.
#[test]
fn the_ports_rail_lists_the_interface_in_the_order_the_deck_has_it() {
    let app = interface_design();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");

    assert_eq!(
        rows.iter()
            .map(|row| (row.spec.name.as_str(), row.meta()))
            .collect::<Vec<_>>(),
        [
            ("ALPHA", "#1 \u{00b7} in".to_owned()),
            ("BETA", "#2 \u{00b7} in".to_owned()),
            ("GAMMA", "#3 \u{00b7} out".to_owned()),
        ]
    );
}

/// A pin whose name declares a vector states the conductors it carries.
///
/// The name is the declaration, so `DATA[7:0]` is one row standing for eight
/// wires. A rail that listed it as one wire would under-count the interface by
/// seven every time it was read.
#[test]
fn a_vector_pin_states_the_conductors_its_name_declares() {
    let mut app = RSpiceApp::test_instance();
    app.state.schematic.components.push(port(
        404,
        "DATA[7:0]",
        Some(1),
        PortDirection::In,
        "the sampled word",
    ));
    app.state.sync_active_schematic_to_workspace();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");

    assert_eq!(rows[0].meta(), "#1 \u{00b7} in \u{00b7} [8]");
    let tooltip = rows[0].tooltip();
    assert!(
        tooltip.contains("8 conductors") && tooltip.contains("the sampled word"),
        "the tooltip carries the whole contract: {tooltip}"
    );
}

/// A name declared twice is marked on every row that declares it, in the tone a
/// hazard is stated in rather than the tone a count is.
///
/// The interface takes the first declaration and the rest add no pin, so the
/// rail is the only place the reader is told that the sheet shows two ports and
/// the deck has one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_port_name_declared_twice_is_marked_on_both_rows() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    // Folded for case, because the netlist folds it: `SENSE` and `sense` are
    // one node, declared twice.
    app.state
        .schematic
        .components
        .push(port(405, "SENSE", Some(1), PortDirection::In, ""));
    app.state
        .schematic
        .components
        .push(port(406, "sense", Some(2), PortDirection::In, ""));
    app.state
        .schematic
        .components
        .push(port(407, "CLEAN", Some(3), PortDirection::Out, ""));
    app.state.sync_active_schematic_to_workspace();

    let rows = port_rows(&app.state, SheetScope::AllSheets, "");
    assert_eq!(
        rows.iter()
            .map(|row| (row.spec.name.as_str(), row.duplicated))
            .collect::<Vec<_>>(),
        [("SENSE", true), ("sense", true), ("CLEAN", false)],
        "the repetition is a fact about both declarations, not about the second"
    );
    assert!(
        rows[0].tooltip().contains("declared more than once"),
        "{}",
        rows[0].tooltip()
    );

    // And the marking reaches the screen: a warning painted in the faint tone
    // every other meta column uses is not a warning.
    let panel = NavigatorPanel::open(app);
    let warn = Tokens::get(&panel.ctx).color.warn;
    let tone = |meta: &str| {
        panel
            .runs
            .iter()
            .find(|(run, _, _)| run.as_str() == meta)
            .map(|(_, _, color)| *color)
    };
    assert_eq!(tone("#1 \u{00b7} in"), Some(warn));
    assert_eq!(tone("#2 \u{00b7} in"), Some(warn));
    assert_ne!(
        tone("#3 \u{00b7} out"),
        Some(warn),
        "the pin declared once states no hazard"
    );
}

/// An excitation row stands for the instance it names, which is what the shared
/// object menu acts on.
///
/// This was the one rail whose rows answered to the pointer alone: they could
/// be selected and nothing else, while every other rail carried open, rename
/// and find on the same gesture.
#[test]
fn an_excitation_row_stands_for_the_instance_the_object_menu_acts_on() {
    let mut app = RSpiceApp::test_instance();
    let mut source = crate::state::Component::new(
        501,
        ComponentType::VoltageSource,
        crate::state::Point::new(60, 80),
    );
    source.name = "VIN".to_owned();
    app.state.schematic.components.push(source);
    app.state.sync_active_schematic_to_workspace();

    let sources = crate::simulation::placed_sources::placed_sources(
        &app.state.schematic,
        app.state.sim_setup.analysis_plan.as_ref(),
    );
    let placed = sources
        .iter()
        .find(|source| source.reference == "VIN")
        .expect("the fixture source is placed");

    assert_eq!(
        excitation_object(&app.state, placed),
        Some(NavigatorObject::Component {
            id: 501,
            label: "VIN".to_owned(),
            position: crate::state::Point::new(60, 80),
        })
    );

    // A source the sheet holds no instance for is offered no menu rather than a
    // menu of commands that would act on nothing.
    app.state.schematic.components.clear();
    assert_eq!(excitation_object(&app.state, placed), None);
}

// ------------------------------------------------------------ shelf identity

/// Every glyph the Component shelf can paint, deduplicated: each placeable
/// palette entry's, each primitive group band's, and the marks the XSPICE,
/// Verilog-A and library sections use.
#[cfg(not(target_arch = "wasm32"))]
fn every_shelf_glyph() -> Vec<ShelfGlyph> {
    let mut glyphs = vec![
        // Built-in XSPICE rows and their band.
        ShelfGlyph::Event,
        // Generated Verilog-A rows and their band.
        ShelfGlyph::Text("VA"),
        // Library parts and project-library bands.
        ShelfGlyph::Icon(WorkbenchIcon::Models),
    ];
    glyphs.extend(PRIMITIVE_GROUPS.iter().map(|(_, glyph, _, _)| *glyph));
    for section in component_palette() {
        for entry in section.entries {
            glyphs.push(primitive_shelf_glyph(entry.kind));
        }
    }
    let mut distinct = Vec::new();
    for glyph in glyphs {
        if !distinct.contains(&glyph) {
            distinct.push(glyph);
        }
    }
    distinct
}

/// One glyph rasterized alone in the 15 px slot the shelf paints it in.
#[cfg(not(target_arch = "wasm32"))]
fn glyph_canvas(glyph: ShelfGlyph) -> crate::ui::raster::Canvas {
    crate::ui::raster::render(egui::vec2(24.0, 24.0), move |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                glyph.paint(
                    ui.painter(),
                    egui::Rect::from_center_size(egui::pos2(12.0, 12.0), egui::vec2(15.0, 15.0)),
                    egui::Color32::WHITE,
                );
            });
    })
}

/// Every glyph in the shelf table puts real ink in the glyph slot, and no
/// text glyph is the replacement box the text layouter substitutes for a
/// character the bundled faces lack.
///
/// The tofu reference is rendered through the same paint path from two
/// characters no bundled face holds; that the pair renders identically is
/// what proves a missing glyph has one shape, which every table glyph must
/// then differ from. This is the gate that catches a future table entry
/// reaching for ◯, ▷, ⊞, Σ, Ω or anything else outside the Latin-subset
/// Plex cuts: it would compile, paint a plausible box of ink, and fail here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_shelf_glyph_paints_ink_and_no_text_glyph_is_a_tofu_box() {
    let pattern = |glyph: ShelfGlyph| -> Vec<egui::Color32> {
        let canvas = glyph_canvas(glyph);
        canvas
            .pixels_in(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(24.0, 24.0),
            ))
            .collect()
    };
    let background = glyph_canvas(ShelfGlyph::Text(" ")).background();
    let tofu = pattern(ShelfGlyph::Text("\u{2603}"));
    assert_eq!(
        tofu,
        pattern(ShelfGlyph::Text("\u{E001}")),
        "two characters the faces lack must share one replacement shape for \
         the tofu comparison to mean anything"
    );

    let glyphs = every_shelf_glyph();
    assert!(
        glyphs.len() >= 20,
        "the table lost its vocabulary: {glyphs:?}"
    );
    for glyph in glyphs {
        let painted = pattern(glyph);
        assert!(
            painted.iter().any(|pixel| *pixel != background),
            "{glyph:?} paints no ink in the glyph slot"
        );
        if let ShelfGlyph::Text(text) = glyph {
            assert_ne!(
                painted, tofu,
                "'{text}' is not in the bundled faces and would reach the \
                 shelf as a tofu box"
            );
        }
    }
}

/// One painted frame of the primitive catalog: every text run it painted and
/// every control it announced, with the disclosure position each publishes.
#[cfg(not(target_arch = "wasm32"))]
fn shelf_frame(
    ctx: &egui::Context,
    app: &RSpiceApp,
    events: Vec<egui::Event>,
) -> (String, Vec<(String, egui::Rect, Option<bool>)>) {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 1600.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    primitive_catalog(ui, app);
                });
        },
    );
    let controls = output
        .platform_output
        .accesskit_update
        .as_ref()
        .map(|update| {
            update
                .nodes
                .iter()
                .filter_map(|(_, node)| {
                    let label = node.label()?.to_owned();
                    let bounds = node.bounds()?;
                    Some((
                        label,
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                        node.is_expanded(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    (painted_text(&output), controls)
}

/// A fresh install's shelf leads with the Passives rows: that one band opens
/// by default, every other band folds — and states its count — until the
/// reader unfolds it. The reader's own position then outlives the shipped
/// default, because the default sits only behind the persisted flag.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_fresh_shelf_opens_passives_and_folds_the_rest_until_the_reader_moves_one() {
    let app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();

    // Two settling frames: the first builds the font set, the second lays
    // out against it, and only then are the band rects worth clicking.
    let _ = shelf_frame(&ctx, &app, Vec::new());
    let (text, controls) = shelf_frame(&ctx, &app, Vec::new());

    for passive in ["Resistor", "Capacitor", "Transmission Line"] {
        assert!(
            text.contains(passive),
            "a fresh shelf shows the Passives rows, missing {passive}: {text}"
        );
    }
    for folded in ["Voltage Source", "NMOS", "AND Gate", "Op-Amp"] {
        assert!(
            !text.contains(folded),
            "{folded} belongs to a band a fresh install keeps folded: {text}"
        );
    }
    let expanded = |controls: &[(String, egui::Rect, Option<bool>)], band: &str| {
        let hits = controls
            .iter()
            .filter(|(label, _, _)| label == band)
            .collect::<Vec<_>>();
        assert_eq!(hits.len(), 1, "exactly one control announces {band:?}");
        (hits[0].1, hits[0].2)
    };
    assert_eq!(expanded(&controls, "Passives").1, Some(true));
    for band in ["Sources", "Analog", "Mixed signal / XSPICE"] {
        assert_eq!(
            expanded(&controls, band).1,
            Some(false),
            "{band} starts folded"
        );
    }

    // The reader folds Passives; the press must beat the shipped default in
    // the frame it lands in and in every frame after it.
    let at = expanded(&controls, "Passives").0.center();
    let _ = shelf_frame(&ctx, &app, click_events(at));
    let (text, controls) = shelf_frame(&ctx, &app, Vec::new());
    assert_eq!(
        expanded(&controls, "Passives").1,
        Some(false),
        "the reader's fold outlives the default-open position"
    );
    assert!(
        !text.contains("Resistor"),
        "a folded Passives band paints no rows: {text}"
    );
}

/// The shelf's meta column states the designator prefix and the default the
/// placed instance will carry, spelled through the crate's engineering
/// formatter — and only what a part actually has: a modelled device keeps its
/// prefix alone, a structural row says nothing.
#[test]
fn shelf_meta_states_the_prefix_and_the_placed_default() {
    for (kind, meta) in [
        (ComponentType::Resistor, Some("R \u{00b7} 1k")),
        (ComponentType::Capacitor, Some("C \u{00b7} 1u")),
        (ComponentType::Inductor, Some("L \u{00b7} 1m")),
        (ComponentType::VoltageSource, Some("V \u{00b7} 5")),
        (ComponentType::OpAmp, Some("E \u{00b7} 100k")),
        // The one default not authored in the formatter's spelling: the
        // formatter's decade wins, as it does in the property editor.
        (ComponentType::CoupledInductor, Some("K \u{00b7} 990m")),
        // A default no engineering parser reads is stated as authored.
        (ComponentType::BehavioralSource, Some("B \u{00b7} V=0")),
        // Modelled devices have no meaningful default value.
        (ComponentType::Diode, Some("D")),
        (ComponentType::Nmos, Some("M")),
        (ComponentType::Memristor, Some("MR")),
        // Structural objects own neither a designator nor a value.
        (ComponentType::Ground, None),
        (ComponentType::Port, None),
    ] {
        assert_eq!(
            primitive_shelf_meta(kind).as_deref(),
            meta,
            "meta column of {kind:?}"
        );
    }
}

/// Rows of different families paint different identity glyphs.
///
/// The label and meta are pinned to one string so the glyph slot is the only
/// thing that can differ — which is exactly what one shared icon, or two
/// missing characters both rendering as the replacement box, would fail.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shelf_rows_of_different_families_paint_distinct_glyph_slots() {
    let row = |kind: ComponentType| {
        crate::ui::raster::render(egui::vec2(160.0, 24.0), move |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    shelf_part_row(ui, primitive_shelf_glyph(kind), "PART", false, Some("m"), 0);
                });
        })
    };
    // The identity slot: the 15 px glyph rect at the schematic row's icon
    // column, with a pixel of air on each side.
    let slot = egui::Rect::from_center_size(egui::pos2(33.5, 12.0), egui::vec2(17.0, 17.0));
    let slot_pixels = |canvas: &crate::ui::raster::Canvas| -> Vec<egui::Color32> {
        canvas.pixels_in(slot).collect()
    };

    let resistor = row(ComponentType::Resistor);
    let capacitor = row(ComponentType::Capacitor);
    let xspice = row(ComponentType::XspiceGain);
    for (name, canvas) in [
        ("resistor", &resistor),
        ("capacitor", &capacitor),
        ("XSPICE", &xspice),
    ] {
        assert!(
            slot_pixels(canvas)
                .iter()
                .any(|pixel| *pixel != canvas.background()),
            "the {name} row's glyph slot is empty"
        );
    }
    assert_ne!(
        slot_pixels(&resistor),
        slot_pixels(&capacitor),
        "a resistor row and a capacitor row paint one glyph"
    );
    assert_ne!(
        slot_pixels(&resistor),
        slot_pixels(&xspice),
        "a resistor row and an XSPICE row paint one glyph"
    );
    assert_ne!(
        slot_pixels(&capacitor),
        slot_pixels(&xspice),
        "a capacitor row and an XSPICE row paint one glyph"
    );
}
