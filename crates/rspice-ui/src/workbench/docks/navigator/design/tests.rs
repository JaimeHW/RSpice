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
///
/// The RF ports rail sits with the interface pins rather than with the
/// excitations, and that placement is the claim: a port is read as a terminal
/// of the design — the thing an S-parameter matrix is indexed by — before it is
/// read as something that may or may not be driving. The rails below it are the
/// conductors and the stimulus those terminals carry.
#[test]
fn design_navigator_sections_lead_with_the_hierarchy() {
    assert_eq!(
        DESIGN_NAVIGATOR_SECTION_ORDER,
        [
            DesignNavigatorSection::Masters,
            DesignNavigatorSection::Occurrences,
            DesignNavigatorSection::Ports,
            DesignNavigatorSection::RfPorts,
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

    /// The one band announcing `title`, or nothing when this design draws no
    /// such rail.
    ///
    /// Absence is an answer — the RF ports band is drawn only above a design
    /// that places a port — but a title announced *twice* never is, so that
    /// stays a failure either way.
    fn band_if_present(&self, title: &str) -> Option<(egui::Rect, Option<bool>)> {
        let hits = self
            .controls
            .iter()
            .filter(|(label, _, _)| label == title)
            .collect::<Vec<_>>();
        assert!(
            hits.len() <= 1,
            "expected at most one control announcing {title:?}, found {}; the rail announces: \
             {:#?}",
            hits.len(),
            self.controls
                .iter()
                .map(|(label, _, _)| label.as_str())
                .collect::<Vec<_>>()
        );
        hits.first().map(|(_, rect, expanded)| (*rect, *expanded))
    }

    /// The one band announcing `title`, which this design is expected to draw.
    fn band(&self, title: &str) -> (egui::Rect, Option<bool>) {
        self.band_if_present(title).unwrap_or_else(|| {
            panic!(
                "expected a control announcing {title:?}; the rail announces: {:#?}",
                self.controls
                    .iter()
                    .map(|(label, _, _)| label.as_str())
                    .collect::<Vec<_>>()
            )
        })
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

    /// The accessible names of the controls `keep` admits, top to bottom.
    ///
    /// Ordered by where they were laid out rather than by the order the tree
    /// was published in: the accessibility update is a node set, and a rail's
    /// claim is about the order a reader meets its rows in.
    fn announced_rows(&self, keep: impl Fn(&str) -> bool) -> Vec<String> {
        let mut hits = self
            .controls
            .iter()
            .filter(|(label, _, _)| keep(label))
            .collect::<Vec<_>>();
        hits.sort_by(|(_, left, _), (_, right, _)| left.top().total_cmp(&right.top()));
        hits.into_iter()
            .map(|(label, _, _)| label.clone())
            .collect()
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
        let next = DESIGN_NAVIGATOR_SECTION_ORDER
            .iter()
            .map(|section| section.title())
            .filter(|other| *other != title)
            .filter_map(|other| self.band_if_present(other))
            .map(|(rect, _)| rect.top())
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
        placed_object(&app.state, placed.component_id, &placed.reference),
        Some(NavigatorObject::Component {
            id: 501,
            label: "VIN".to_owned(),
            position: crate::state::Point::new(60, 80),
        })
    );

    // A source the sheet holds no instance for is offered no menu rather than a
    // menu of commands that would act on nothing.
    app.state.schematic.components.clear();
    assert_eq!(
        placed_object(&app.state, placed.component_id, &placed.reference),
        None
    );
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
    app: &mut RSpiceApp,
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
    (painted_text(&output), announced_controls(&output))
}

/// Every control one painted frame announced: its label, where it sits, and
/// the disclosure position it publishes.
#[cfg(not(target_arch = "wasm32"))]
fn announced_controls(output: &egui::FullOutput) -> Vec<(String, egui::Rect, Option<bool>)> {
    output
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
        .unwrap_or_default()
}

/// A fresh install's shelf leads with the Passives rows: that one band opens
/// by default, every other band folds — and states its count — until the
/// reader unfolds it. The reader's own position then outlives the shipped
/// default, because the default sits only behind the persisted flag.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_fresh_shelf_opens_passives_and_folds_the_rest_until_the_reader_moves_one() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();

    // Two settling frames: the first builds the font set, the second lays
    // out against it, and only then are the band rects worth clicking.
    let _ = shelf_frame(&ctx, &mut app, Vec::new());
    let (text, controls) = shelf_frame(&ctx, &mut app, Vec::new());

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
    let _ = shelf_frame(&ctx, &mut app, click_events(at));
    let (text, controls) = shelf_frame(&ctx, &mut app, Vec::new());
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

// ------------------------------------------------------------------ RF ports

/// One RF port, as the palette places it: a reference and the parameter string
/// the property sheet writes.
fn rf_port(id: u64, name: &str, params: &str) -> crate::state::Component {
    let mut component = crate::state::Component::new(
        id,
        ComponentType::RfPort,
        crate::state::Point::new(80, 40 + id as i32),
    );
    component.name = name.to_owned();
    component.params = params.to_owned();
    component
}

/// A design whose ports carry `params`, with the navigator active over it.
fn rf_bench(ports: Vec<crate::state::Component>) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.components.extend(ports);
    app.state.sync_active_schematic_to_workspace();
    app
}

/// The ports the rail lists, resolved the way the section resolves them.
fn listed_rf_ports(app: &RSpiceApp) -> Vec<crate::simulation::placed_sources::PlacedRfPort> {
    crate::simulation::placed_sources::placed_rf_ports(
        &app.state.schematic,
        app.state.sim_setup.analysis_plan.as_ref(),
    )
}

/// A plan holding one analysis of `kind`, enabled.
fn plan_with(
    kind: crate::simulation::plan::AnalysisKind,
) -> crate::simulation::plan::SimulationPlan {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (instance, _) = plan.insert(kind).expect("the fixture analysis inserts");
    plan.set_enabled(instance, true)
        .expect("the fixture analysis takes its enabled flag");
    plan
}

/// The rail exists only above a design that places a port.
///
/// Every other band in this navigator answers about something every design
/// has. A permanent empty "RF ports" band would spend a row of a narrow rail —
/// on every sheet that is not an RF testbench, forever — to report that a
/// device the design never reached for is still unused, which is the density
/// bar this product is held to rather than a matter of taste.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rf_port_rail_is_drawn_only_above_a_design_that_places_one() {
    let announces = |panel: &NavigatorPanel| panel.band_if_present("RF ports").is_some();

    // The ordinary design: an interface, conductors, no port.
    assert!(
        !announces(&NavigatorPanel::open(interface_design())),
        "a design that places no RF port carries no RF band"
    );

    // One port is enough, and the band states what it holds.
    let panel = NavigatorPanel::open(rf_bench(vec![rf_port(601, "P1", "port=1 z0=50")]));
    assert!(announces(&panel));
    assert_eq!(panel.stated_count("RF ports").as_deref(), Some("1"));
    assert!(
        panel.rows_under("RF ports").contains(&"P1".to_owned()),
        "the band paints its row: {:?}",
        panel.rows_under("RF ports")
    );
}

/// The rail lists ports in the order an S-parameter matrix indexes them, and
/// every row states the number it is indexed by, the impedance it presents and
/// what it does behind that impedance.
///
/// Document order is the order the ports happened to be drawn in, which is no
/// order at all: an `.sp` run addresses a port by its `port=` number, so a rail
/// in drawing order would put the row a run calls port 1 second.
#[test]
fn the_rf_port_rail_indexes_its_rows_the_way_an_s_parameter_run_does() {
    let app = rf_bench(vec![
        rf_port(602, "PLOAD", "port=3 z0=75"),
        rf_port(603, "PIN", "port=1 z0=50 pwr=-10"),
        rf_port(604, "POUT", "port=2 z0=5e1"),
    ]);

    let rows = listed_rf_ports(&app);

    assert_eq!(
        rows.iter()
            .map(|port| (port.reference.as_str(), rf_port_meta(port)))
            .collect::<Vec<_>>(),
        [
            (
                "PIN",
                "#1 \u{00b7} Z0 50 \u{00b7} drive \u{00b7} no S-parameter run".to_owned()
            ),
            // `5e1` and `50` are one impedance, and the rail prints them as one.
            (
                "POUT",
                "#2 \u{00b7} Z0 50 \u{00b7} term \u{00b7} no S-parameter run".to_owned()
            ),
            (
                "PLOAD",
                "#3 \u{00b7} Z0 75 \u{00b7} term \u{00b7} no S-parameter run".to_owned()
            ),
        ]
    );
}

/// The meta column names what reads the port, and says the plan reads none of
/// them in the words the studio's Excitations page uses.
///
/// The distinction is the point: a source nothing reads is a finding, and a
/// port nothing reads is a termination still loading the design. Borrowing the
/// excitation rail's `no reader` here would call every port of a time-domain
/// testbench a defect.
#[test]
fn an_rf_port_row_names_the_s_parameter_run_that_reads_it() {
    let mut app = rf_bench(vec![rf_port(605, "P1", "port=1 z0=50")]);

    let unread = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        unread.ends_with("no S-parameter run"),
        "a plan with no `.sp` run states what is missing: {unread}"
    );

    // A transient reaches the port as the two-terminal element the netlist
    // already carries, which is not readership: it would make every plan read
    // every port.
    app.state.sim_setup.analysis_plan =
        Some(plan_with(crate::simulation::plan::AnalysisKind::Transient));
    let loaded = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        loaded.ends_with("no S-parameter run"),
        "only an `.sp` run indexes a port: {loaded}"
    );

    app.state.sim_setup.analysis_plan =
        Some(plan_with(crate::simulation::plan::AnalysisKind::SParameter));
    let read = rf_port_meta(&listed_rf_ports(&app)[0]);
    assert!(
        read.ends_with("S-parameter port"),
        "the row names the part the run reads it as: {read}"
    );
    let tooltip = rf_port_tooltip(&listed_rf_ports(&app)[0], false, None);
    assert!(
        tooltip.contains("S-parameter port") && !tooltip.contains("disabled"),
        "{tooltip}"
    );
}

/// Two ports claiming one number are marked on both rows, in the tone a hazard
/// is stated in rather than the tone a count is, and the tooltip names the
/// number they are fighting over.
///
/// An S-parameter run addresses a port by its number, so the ports sharing one
/// cannot both be the port that run measures. The meta column has room to paint
/// the row as a hazard but not to say what the hazard is, which is why the
/// sentence is in the tooltip.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn two_rf_ports_claiming_one_number_are_marked_on_both_rows() {
    let app = rf_bench(vec![
        rf_port(606, "PA", "port=1 z0=50"),
        rf_port(607, "PB", "port=1 z0=50"),
        rf_port(608, "PC", "port=2 z0=50"),
    ]);

    let rows = listed_rf_ports(&app);
    let collisions = crate::simulation::placed_sources::duplicate_port_numbers(&rows);
    assert_eq!(collisions, vec![1]);

    let tooltip = rf_port_tooltip(&rows[0], true, None);
    assert!(
        tooltip.contains("Port number 1 is claimed by more than one placed port"),
        "the collision names its own number: {tooltip}"
    );
    assert!(
        !rf_port_tooltip(&rows[2], false, None).contains("claimed by more than one"),
        "the port claiming a number alone states no hazard"
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
    assert_eq!(tone(&rf_port_meta(&rows[0])), Some(warn));
    assert_eq!(tone(&rf_port_meta(&rows[1])), Some(warn));
    assert_ne!(
        tone(&rf_port_meta(&rows[2])),
        Some(warn),
        "the port claiming its number alone states no hazard"
    );
}

/// The tooltip carries what the row has no room for: the terminals the port
/// sits across, and the sentence that a plan holding no `.sp` run is not a
/// plan that broke the port.
#[test]
fn an_rf_port_tooltip_reads_the_port_the_row_could_only_summarise() {
    let app = rf_bench(vec![rf_port(609, "P1", "port=1 z0=50")]);
    let rows = listed_rf_ports(&app);

    let tooltip = rf_port_tooltip(&rows[0], false, None);
    assert!(
        tooltip.starts_with("P1 \u{00b7} term \u{00b7} Z0 50"),
        "{tooltip}"
    );
    assert!(
        tooltip.contains("No S-parameter analysis in this plan reads this port"),
        "{tooltip}"
    );
}

// -------------------------------------------- the whole design's excitations

/// One independent source, as the palette places it.
fn source(id: u64, kind: ComponentType, name: &str, params: &str) -> crate::state::Component {
    let mut component =
        crate::state::Component::new(id, kind, crate::state::Point::new(120, 40 + id as i32));
    component.name = name.to_owned();
    component.params = params.to_owned();
    component
}

/// The design this half of the rail exists for: the root places `VDD` and two
/// instances of one master, and that master places a source and an RF port.
///
/// Two instances rather than one, because the multiplicity is the claim — a run
/// flattens the hierarchy, so the master's `V1` is two cards in the deck and the
/// rail owes the reader a row for each.
fn hierarchical_excitations() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .schematic
        .components
        .push(source(701, ComponentType::VoltageSource, "VDD", "dc=5"));
    app.state
        .schematic
        .components
        .push(placed(702, "XA", "work", "afe"));
    app.state
        .schematic
        .components
        .push(placed(703, "XB", "work", "afe"));
    app.state.sync_active_schematic_to_workspace();

    let mut child = SchematicState::default();
    child.components.push(source(
        711,
        ComponentType::VoltageSourceSin,
        "V1",
        "freq=1k",
    ));
    child.components.push(rf_port(712, "P1", "port=1 z0=50"));
    add_master(&mut app.state, "work", "afe", child);
    app
}

/// The whole design, at every occurrence the run reaches a source through.
///
/// The rail read the editor's buffer, so the root of every hierarchical design
/// reported that it places nothing but its own supplies — while the run drove
/// every source drawn below it. Each row of another occurrence states the path
/// in front of the reference, because `V1` names nothing until the path is read
/// and one drawn source becomes two rows here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_excitation_rail_lists_every_occurrence_the_run_reaches_a_source_through() {
    let panel = NavigatorPanel::open(hierarchical_excitations());

    assert_eq!(
        panel.announced_rows(|label| label.ends_with("VDD") || label.ends_with("V1")),
        ["VDD", "/XA/V1", "/XB/V1"],
        "the root's own source leads, and each occurrence of the master states \
         where the run reaches its source"
    );
    assert_eq!(
        panel.stated_count("Excitations").as_deref(),
        Some("3"),
        "the band counts what the run drives, not what the sheet holds"
    );
}

/// The RF rail joins the same two readings, and keeps the one order an
/// S-parameter run has.
///
/// A number claimed by two occurrences is the finding: the run indexes the
/// flattened design, so one port drawn once and reached twice claims one index
/// of one matrix.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rf_rail_marks_the_number_two_occurrences_of_one_master_both_claim() {
    let panel = NavigatorPanel::open(hierarchical_excitations());

    assert_eq!(
        panel.announced_rows(|label| label.ends_with("P1")),
        ["/XA/P1", "/XB/P1"]
    );
    assert_eq!(panel.stated_count("RF ports").as_deref(), Some("2"));

    let warn = Tokens::get(&panel.ctx).color.warn;
    let meta = panel
        .runs
        .iter()
        .filter(|(run, _, _)| run.starts_with("#1 \u{00b7} Z0 50"))
        .map(|(_, _, color)| *color)
        .collect::<Vec<_>>();
    assert_eq!(
        meta,
        vec![warn, warn],
        "both rows carry the hazard, because the collision is a fact about the pair"
    );
}

/// A row of another occurrence answers a click by opening that occurrence.
///
/// The one thing it must not do is select here: a component id is unique inside
/// one buffer and repeats across them, so applying the row's id to the sheet in
/// front of the reader selects whatever that sheet happens to carry under it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_excitation_of_another_occurrence_opens_it_rather_than_selecting_here() {
    let mut panel = NavigatorPanel::open(hierarchical_excitations());
    let occurrence = InstancePath::root()
        .child("XB")
        .expect("the fixture instance is nameable");

    panel.click("/XB/V1");

    assert_eq!(
        panel.app.state.workspace.occurrence_path(),
        occurrence,
        "the click lands the session on the occurrence the row names"
    );
    assert_eq!(
        panel.app.state.workspace.active_view,
        CellViewRef::new("work", "afe", "schematic")
    );
    assert!(
        panel.app.state.schematic.selection.has_component(711),
        "and selects the instance there, once the session is standing on it"
    );
}

/// Sheet scope narrows the active cell view, and a row of another occurrence is
/// on no sheet of it.
///
/// Narrowing those rows away would make a control that says "this sheet"
/// silently answer a question about the hierarchy that it does not ask — and
/// would hide, at the position most readers leave it in, every source the run
/// drives below the root. The narrowing itself is unchanged and still reaches
/// the rows of the sheet on screen; see
/// [`the_sheet_scope_narrows_the_object_rails_to_the_active_sheet`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_leaves_the_rows_of_another_occurrence_alone() {
    let mut app = two_sheet_named_signals();
    let elsewhere = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    app.state
        .schematic
        .components
        .push(placed(304, "XA", "work", "afe"));
    app.state.sync_active_schematic_to_workspace();

    for scope in SheetScope::OPTIONS {
        assert!(
            derived_row_is_in_scope(&app.state, scope, Some(&elsewhere), 302),
            "{scope:?} has no authority over a row drawn in another occurrence"
        );
    }
    // 302 is the source the catalog assigned to the sheet the session is not
    // on, which is exactly the row the scope control does narrow.
    assert!(
        !derived_row_is_in_scope(&app.state, SheetScope::ActiveSheet, None, 302),
        "a row of the sheet on screen is still the scope control's to narrow"
    );
}

/// The occurrence being edited is listed once, from the live buffer.
///
/// Both readings can reach it — the projection binds it and the editor holds
/// it — and the row that has to survive is the editor's, because that is the one
/// carrying uncommitted edits and answering the object menu.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_occurrence_in_front_of_the_reader_is_listed_once_and_from_its_buffer() {
    let mut app = hierarchical_excitations();
    let occurrence = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    hierarchy_tree::open_occurrence(&mut app.state, &occurrence);

    let rows = whole_design_sources(&app);

    assert_eq!(
        rows.iter()
            .map(|source| (source.occurrence_label(), source.reference.as_str()))
            .collect::<Vec<_>>(),
        [
            ("/".to_owned(), "V1"),
            ("/".to_owned(), "VDD"),
            ("/XB".to_owned(), "V1"),
        ],
        "the sheet on screen is read as its own root and the second instance \
         keeps the row that reaches it"
    );
    assert_eq!(
        row_is_elsewhere(&app.state, rows[0].occurrence.as_ref()),
        None,
        "the row of the occurrence being edited carries no path in front of it"
    );

    // The design root is its own separator: read from inside a child master,
    // the root's supply is `/VDD` and not `//VDD`.
    let panel = NavigatorPanel::open(app);
    assert!(
        panel.controls.iter().any(|(label, _, _)| label == "/VDD"),
        "the rail announces: {:#?}",
        panel
            .controls
            .iter()
            .map(|(label, _, _)| label.as_str())
            .collect::<Vec<_>>()
    );
}

/// A code model whose port widths are not fixed cannot be armed until they are
/// chosen, so its click raises the placement dialog and arms nothing. The
/// decision is held in one place, so every row that offers the model asks the
/// same question.
#[test]
fn a_code_model_with_open_port_widths_asks_before_it_arms() {
    let (mut open, mut fixed) = (None, None);
    for descriptor in engine_only_xspice_devices() {
        if builtin_xspice_library_binding(descriptor).is_err() {
            continue;
        }
        let Ok(ports) = builtin_xspice_vector_ports(descriptor) else {
            continue;
        };
        if ports
            .iter()
            .any(|port| port.maximum.is_none_or(|maximum| maximum != port.minimum))
        {
            open.get_or_insert(descriptor);
        } else {
            fixed.get_or_insert(descriptor);
        }
    }
    let open = open.expect("the registry carries a code model with an open port width");
    let fixed = fixed.expect("the registry carries a code model whose ports are all fixed");

    let mut app = RSpiceApp::test_instance();
    assert!(
        place_builtin_xspice(&mut app, fixed.stable_id).is_some(),
        "{} has nothing left to choose, so it arms straight away",
        fixed.stable_id
    );
    assert!(
        !app.state.dialogs.builtin_xspice_placement.open,
        "and asks nothing"
    );

    assert!(
        place_builtin_xspice(&mut app, open.stable_id).is_none(),
        "{} cannot be armed until its widths are chosen",
        open.stable_id
    );
    assert!(
        app.state.dialogs.builtin_xspice_placement.open,
        "so the click raises the placement dialog instead"
    );
    assert_eq!(
        app.state.dialogs.builtin_xspice_placement.stable_id, open.stable_id,
        "for the model the row named"
    );
}

// -------------------------------------------------- shelf pinning and recents

/// One frame of the whole Component shelf, through the same entry point the
/// dock calls — so the placement watch, the bands, and the catalog all see one
/// frame, as they do in the product.
#[cfg(not(target_arch = "wasm32"))]
fn component_shelf_output(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 2400.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    component_shelf(ui, app);
                });
        },
    )
}

/// What one such frame painted: the joined text and the runs it came from.
///
/// Both, because a gesture has to land on a row, and only the runs carry where
/// the rows are.
#[cfg(not(target_arch = "wasm32"))]
fn component_shelf_frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> (String, Vec<(String, egui::Rect, egui::Color32)>) {
    let output = component_shelf_output(ctx, app, events);
    (painted_text(&output), painted_runs(&output))
}

/// A press and a release of the secondary button, which is what egui reads as
/// the gesture that opens a context menu.
#[cfg(not(target_arch = "wasm32"))]
fn secondary_click_events(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// Painted runs are one galley each, so a row label is its own line: an exact
/// line match tells a band's row apart from a catalog row whose name contains
/// it.
#[cfg(not(target_arch = "wasm32"))]
fn paints_line(text: &str, line: &str) -> bool {
    text.lines().any(|painted| painted == line)
}

/// Where one painted run of a frame sits, for a gesture that has to land on
/// it.
#[cfg(not(target_arch = "wasm32"))]
fn run_rect(runs: &[(String, egui::Rect, egui::Color32)], text: &str) -> Option<egui::Rect> {
    runs.iter()
        .find(|(run, _, _)| run == text)
        .map(|(_, rect, _)| *rect)
}

/// The shelf held open across frames, driven to a settled layout.
///
/// Two frames: the first builds the font set, the second lays out against it,
/// and only then are the painted rects worth clicking.
#[cfg(not(target_arch = "wasm32"))]
fn settled_shelf() -> (egui::Context, RSpiceApp) {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    (ctx, app)
}

/// The stored history, as the durable domain holds it.
#[cfg(not(target_arch = "wasm32"))]
fn stored_recent(app: &RSpiceApp) -> Vec<String> {
    app.state.ui.preferences.component_shelf().recent
}

/// Arm one primitive and put it on the sheet, the way a click-to-arm placement
/// reaches the design.
#[cfg(not(target_arch = "wasm32"))]
fn place(app: &mut RSpiceApp, kind: ComponentType) {
    app.state.schematic.arm_tool(Tool::Place(kind));
    app.state
        .schematic
        .add_component(kind, crate::state::Point::new(40, 40));
}

/// Every shelf-entry key survives the round trip through the durable form,
/// including names that carry the characters a naive encoding would split on.
#[test]
fn every_shelf_entry_kind_round_trips_through_its_stored_key() {
    for entry in [
        ShelfEntry::Primitive(ComponentType::Resistor),
        ShelfEntry::Primitive(ComponentType::Ground),
        ShelfEntry::LibraryPart("1N4148".to_owned()),
        ShelfEntry::BuiltinXspice("gain".to_owned()),
        ShelfEntry::GeneratedVerilogA("bsimcmg".to_owned()),
        ShelfEntry::Cell {
            library: "work".to_owned(),
            cell: "opamp".to_owned(),
            view: "schematic".to_owned(),
        },
        // A cell whose names carry the slash a display path joins on and the
        // colon a key tag would.
        ShelfEntry::Cell {
            library: "an/odd lib".to_owned(),
            cell: "cell:with:colons".to_owned(),
            view: "symbol".to_owned(),
        },
    ] {
        let key = entry.storage_key();
        assert_eq!(
            ShelfEntry::from_storage_key(&key).as_ref(),
            Some(&entry),
            "{entry:?} did not survive the key {key:?}"
        );
    }

    // A key this build does not understand is refused rather than guessed at,
    // which is what lets the stored list keep it untouched.
    for unknown in ["", "future-family\u{1f}x", "primitive\u{1f}NoSuchType"] {
        assert_eq!(
            ShelfEntry::from_storage_key(unknown),
            None,
            "{unknown:?} must not resolve"
        );
    }
}

/// A fresh profile takes the shipped pin set, and the first pin or unpin makes
/// the reader's own set authoritative: the shipped list never merges back in,
/// so a default the reader removed stays removed.
#[test]
fn a_fresh_profile_takes_the_shipped_pins_until_the_reader_moves_one() {
    let mut app = RSpiceApp::test_instance();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert_eq!(
        pinned_keys(&app),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
        ],
        "a profile that has never pinned anything sees the shipped set"
    );
    assert!(
        app.state.ui.preferences.component_shelf().pinned.is_none(),
        "and the shipped set is not yet written as the reader's own"
    );

    // A pin lands at the end: the rail the reader builds keeps its positions.
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Diode));
    assert_eq!(
        pinned_keys(&app),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
            key(ComponentType::Diode),
        ]
    );

    // Unpinning a shipped default removes it for good.
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Resistor));
    assert_eq!(
        pinned_keys(&app),
        vec![
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
            key(ComponentType::Diode),
        ],
        "the shipped default must not merge back into the reader's set"
    );

    // Emptied on purpose stays empty rather than reverting to the shipped set.
    for kind in [
        ComponentType::Capacitor,
        ComponentType::Ground,
        ComponentType::Diode,
    ] {
        toggle_pin(&mut app, &ShelfEntry::Primitive(kind));
    }
    assert_eq!(pinned_keys(&app), Vec::<String>::new());
    assert_eq!(
        app.state.ui.preferences.component_shelf().pinned,
        Some(Vec::new()),
        "an emptied set is the reader's answer, not an absent one"
    );
}

/// The Pinned band paints the reader's set, and disappears — heading and all —
/// when that set empties. An empty band under a heading would claim a rail the
/// reader just took apart.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pinned_band_paints_the_set_and_is_absent_once_it_empties() {
    let (ctx, mut app) = settled_shelf();

    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(paints_line(&text, "PINNED"), "the fresh band is painted");
    for shipped in ["Resistor", "Capacitor", "Ground"] {
        assert!(
            paints_line(&text, shipped),
            "the shipped pin {shipped} is missing: {text}"
        );
    }
    // A folded catalog band's row, pinned, still reaches the top of the shelf.
    assert!(
        !paints_line(&text, "NMOS"),
        "NMOS belongs to a band a fresh install folds: {text}"
    );
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Nmos));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "NMOS"),
        "a pinned row is painted even while its catalog band is folded: {text}"
    );

    for kind in [
        ComponentType::Resistor,
        ComponentType::Capacitor,
        ComponentType::Ground,
        ComponentType::Nmos,
    ] {
        toggle_pin(&mut app, &ShelfEntry::Primitive(kind));
    }
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        !paints_line(&text, "PINNED"),
        "an emptied set leaves no band at all: {text}"
    );
    assert!(
        !paints_line(&text, "Ground"),
        "and none of its rows: {text}"
    );
}

/// A placeable row is pinned from its own context menu, where the reader found
/// the part — and unpinned from the same menu, which states the position it is
/// in rather than one fixed verb.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_row_is_pinned_and_unpinned_from_its_context_menu() {
    let (ctx, mut app) = settled_shelf();
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let row = run_rect(&runs, "Transmission Line")
        .expect("the Passives band paints a Transmission Line row");
    let entry = ShelfEntry::Primitive(ComponentType::TransmissionLine);
    assert!(!is_pinned(&app, &entry), "the row starts unpinned");

    let _ = component_shelf_frame(&ctx, &mut app, secondary_click_events(row.center()));
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "the menu offers the position the row is not in: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(pin.center()));
    assert!(
        is_pinned(&app, &entry),
        "the command pinned the row it was opened on"
    );

    // The pinned row now leads the shelf, and its menu offers the other
    // position — the same control, stating where the row stands.
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let pinned_row =
        run_rect(&runs, "Transmission Line").expect("the pinned row is painted at the top");
    let _ = component_shelf_frame(&ctx, &mut app, secondary_click_events(pinned_row.center()));
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Unpin") && !paints_line(&text, "Pin to shelf"),
        "a pinned row's menu offers only the way back out: {text}"
    );

    let unpin = run_rect(&runs, "Unpin").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(unpin.center()));
    assert!(
        !is_pinned(&app, &entry),
        "and the command takes the row back off the rail"
    );
}

/// Placement feeds Recent, newest first, with a re-place moving the part to
/// the front instead of listing it twice.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn placement_feeds_recent_front_ordered_and_deduplicated() {
    let (ctx, mut app) = settled_shelf();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert!(
        stored_recent(&app).is_empty(),
        "a profile that has placed nothing has no history"
    );

    place(&mut app, ComponentType::Diode);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(stored_recent(&app), vec![key(ComponentType::Diode)]);
    assert!(paints_line(&text, "RECENT"), "the band appears: {text}");

    place(&mut app, ComponentType::Nmos);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app),
        vec![key(ComponentType::Nmos), key(ComponentType::Diode)],
        "the newest placement leads"
    );

    place(&mut app, ComponentType::Diode);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app),
        vec![key(ComponentType::Diode), key(ComponentType::Nmos)],
        "a re-place moves the part to the front rather than listing it twice"
    );
}

/// The band is capped: a session that places more parts than it can show keeps
/// the newest, and the ones that scrolled off are gone from the band.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_recent_band_shows_only_its_newest_entries() {
    let (ctx, mut app) = settled_shelf();

    // Eight parts, all from bands a fresh install folds, so each label can
    // only reach the paint list through the Recent band.
    let placed = [
        ComponentType::Diode,
        ComponentType::Nmos,
        ComponentType::Pmos,
        ComponentType::NpnBjt,
        ComponentType::PnpBjt,
        ComponentType::VoltageSource,
        ComponentType::CurrentSource,
        ComponentType::OpAmp,
    ];
    for kind in placed {
        place(&mut app, kind);
        let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(stored_recent(&app).len(), placed.len());

    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let painted = placed
        .iter()
        .filter(|kind| paints_line(&text, kind.display_name()))
        .count();
    assert_eq!(
        painted, RECENT_SHOWN,
        "the band paints its cap and no more: {text}"
    );
    for dropped in &placed[..placed.len() - RECENT_SHOWN] {
        assert!(
            !paints_line(&text, dropped.display_name()),
            "{} fell off the band and must not be painted: {text}",
            dropped.display_name()
        );
    }
}

/// A pinned part is not listed twice. It keeps its place in the stored
/// history, so unpinning restores it to the band rather than losing it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pinned_part_is_filtered_out_of_recent_but_kept_in_the_history() {
    let (ctx, mut app) = settled_shelf();
    let entry = ShelfEntry::Primitive(ComponentType::Diode);

    place(&mut app, ComponentType::Diode);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Diode"),
        "the placement is listed: {text}"
    );

    toggle_pin(&mut app, &entry);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        !paints_line(&text, "RECENT"),
        "the only recent part is pinned, so the band has nothing left: {text}"
    );
    assert!(
        paints_line(&text, "Diode"),
        "and it is painted once, in the Pinned band: {text}"
    );
    assert_eq!(
        stored_recent(&app),
        vec![entry.storage_key()],
        "the history keeps it, so unpinning restores its position"
    );

    toggle_pin(&mut app, &entry);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "RECENT") && paints_line(&text, "Diode"),
        "unpinning returns it to the band: {text}"
    );
}

/// Only what the shelf offered the canvas is credited. A design that grows on
/// its own — a paste, an import, a script — is not this reader's placement
/// history, and the bands must not claim it is.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_design_that_grows_without_the_shelf_writes_no_history() {
    let (ctx, mut app) = settled_shelf();

    app.state.schematic.cancel_tool();
    app.state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert!(stored_recent(&app).is_empty(), "nothing was offered");
    assert!(
        !paints_line(&text, "RECENT"),
        "so there is no band to paint: {text}"
    );
}

/// Opening a different design does not read its existing objects as
/// placements: the watch adopts the new count in silence.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn switching_designs_credits_nothing_to_the_reader() {
    let (ctx, mut app) = settled_shelf();

    app.state
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());

    // A design the reader opened, already holding objects someone else placed.
    app.state.active_schematic_epoch += 1;
    for offset in 0..4 {
        app.state.schematic.add_component(
            ComponentType::Nmos,
            crate::state::Point::new(40 + offset, 40),
        );
    }
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app).is_empty(),
        "the objects already on an opened design were not placed by this reader"
    );

    // From there the watch tracks the new design normally.
    place(&mut app, ComponentType::Diode);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app),
        vec![ShelfEntry::Primitive(ComponentType::Diode).storage_key()]
    );
}

/// The pin set and the placement history are per-user, cross-project state,
/// and they survive an application restart.
///
/// The boundary proved here is the real one: the same `eframe::Storage` RON
/// round trip `RSpiceApp::save` performs — `eframe::set_value(storage,
/// eframe::APP_KEY, &self.state)` — and `RSpiceApp::new` reads back. Nothing
/// about that store is per-project: it is the user's own application-data
/// file, and the browser's local storage on the web build.
#[test]
fn the_shelf_set_survives_the_restart_boundary_the_workbench_actually_crosses() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    let mut app = RSpiceApp::test_instance();
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Diode));
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Resistor));
    record_placement(&mut app, &ShelfEntry::LibraryPart("1N4148".to_owned()));
    record_placement(
        &mut app,
        &ShelfEntry::Cell {
            library: "work".to_owned(),
            cell: "opamp".to_owned(),
            view: "schematic".to_owned(),
        },
    );
    let saved = app.state.ui.preferences.component_shelf();

    let mut storage = MemoryStorage::default();
    eframe::set_value(&mut storage, eframe::APP_KEY, &app.state);
    let restored: crate::workbench::app_state::AppState =
        eframe::get_value(&storage, eframe::APP_KEY)
            .expect("RSpice must be able to restore a session it just saved");

    assert_eq!(
        restored.ui.preferences.component_shelf(),
        saved,
        "the shelf's personal set must cross the restart boundary intact"
    );
    assert_eq!(
        restored.ui.preferences.component_shelf().pinned,
        Some(vec![
            ShelfEntry::Primitive(ComponentType::Capacitor).storage_key(),
            ShelfEntry::Primitive(ComponentType::Ground).storage_key(),
            ShelfEntry::Primitive(ComponentType::Diode).storage_key(),
        ]),
        "including the order the reader pinned in"
    );
}

/// A fresh profile writes nothing: the domain is absent from the wire until
/// the reader has an answer of their own, so an untouched shelf costs a saved
/// session no bytes and no forward-compatibility surface.
#[test]
fn an_untouched_shelf_writes_no_preference_domain() {
    use crate::workbench::preferences::ComponentShelfPreferences;

    let mut preferences = crate::workbench::UserPreferences::default();
    let untouched = serde_json::to_value(&preferences).expect("preferences encode");

    preferences.set_component_shelf(ComponentShelfPreferences {
        pinned: Some(Vec::new()),
        recent: Vec::new(),
    });
    assert_ne!(
        serde_json::to_value(&preferences).expect("preferences encode"),
        untouched,
        "a set the reader emptied on purpose is not the same answer as no set"
    );

    preferences.set_component_shelf(ComponentShelfPreferences::default());
    assert_eq!(
        serde_json::to_value(&preferences).expect("preferences encode"),
        untouched,
        "and returning to the shipped default clears the domain again"
    );
}

/// A band row is a door to the same place its catalog row leads: clicking it
/// arms the part it names, and the row then states that it is the armed one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_band_row_arms_the_part_it_names_and_says_so() {
    /// The tone one painted run was set in.
    fn tone(runs: &[(String, egui::Rect, egui::Color32)], text: &str) -> Option<egui::Color32> {
        runs.iter()
            .find(|(run, _, _)| run == text)
            .map(|(_, _, colour)| *colour)
    }

    let (ctx, mut app) = settled_shelf();
    // NMOS belongs to a band a fresh install folds, so the only NMOS row on
    // screen is the pinned one — the click cannot land on a catalog row by
    // accident.
    toggle_pin(&mut app, &ShelfEntry::Primitive(ComponentType::Nmos));
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let row = run_rect(&runs, "NMOS").expect("the pinned NMOS row is painted");
    let resting = tone(&runs, "NMOS");
    assert_ne!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Nmos),
        "nothing is armed before the click"
    );

    let _ = component_shelf_frame(&ctx, &mut app, click_events(row.center()));
    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Nmos),
        "the click armed the part the row names"
    );

    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_ne!(
        tone(&runs, "NMOS"),
        resting,
        "and the row states that it is the armed one"
    );
    assert_eq!(
        tone(&runs, "Ground"),
        resting,
        "while the rest of the rail is left alone"
    );
}

/// A part dragged onto the sheet is credited, which is the route the armed
/// tool never covers: the drop consumes the payload, so the shelf sees the
/// design grow only on the frame after the offer is gone.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_drag_onto_the_sheet_credits_the_part_the_drop_consumed() {
    let (ctx, mut app) = settled_shelf();
    let payload = SchematicShelfDragPayload::primitive(ComponentType::Nmos)
        .expect("a primitive travels to the canvas as a drag payload");

    // The drag in flight: an identity is on offer and nothing has landed.
    egui::DragAndDrop::set_payload(&ctx, payload);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app).is_empty(),
        "a drag still in flight has placed nothing"
    );

    // The drop: the canvas takes the payload and commits the object.
    egui::DragAndDrop::clear_payload(&ctx);
    app.state
        .schematic
        .add_component(ComponentType::Nmos, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert_eq!(
        stored_recent(&app),
        vec![ShelfEntry::Primitive(ComponentType::Nmos).storage_key()],
        "the dropped part is what this reader most recently placed"
    );
    assert!(paints_line(&text, "NMOS"), "and the band lists it: {text}");
}

/// An offer the reader took back credits nothing later. The shelf holds a
/// spent offer exactly one frame past the act that consumed it — long enough
/// for the drop to land — and no longer, so a paste or an import made
/// afterwards is not read as another placement of the part last armed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tool_the_reader_put_away_credits_no_later_growth() {
    let (ctx, mut app) = settled_shelf();

    app.state
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    app.state.schematic.cancel_tool();
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app).is_empty(),
        "arming a tool and putting it away places nothing"
    );

    // The design grows on its own, well after the offer was withdrawn.
    app.state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert!(
        stored_recent(&app).is_empty(),
        "the shelf had nothing on offer when the design grew: {:?}",
        stored_recent(&app)
    );
    assert!(
        !paints_line(&text, "RECENT"),
        "so there is no band to paint: {text}"
    );
}

/// The stored history outlasts the band it feeds, and stops at its own cap.
///
/// Both halves matter: keeping more than is shown is what lets unpinning
/// restore a full band, and stopping is what keeps a personal preference file
/// from growing without bound.
#[test]
fn the_history_outlasts_the_band_and_stops_at_its_own_cap() {
    assert!(
        RECENT_STORED > RECENT_SHOWN,
        "a history no longer than the band cannot survive a pin"
    );

    let mut app = RSpiceApp::test_instance();
    let part = |index: usize| ShelfEntry::LibraryPart(format!("PART{index}"));
    for index in 0..RECENT_STORED + 4 {
        record_placement(&mut app, &part(index));
    }

    let recent = app.state.ui.preferences.component_shelf().recent;
    assert_eq!(recent.len(), RECENT_STORED, "the history stops at its cap");
    assert_eq!(
        recent.first(),
        Some(&part(RECENT_STORED + 3).storage_key()),
        "and it is the newest end that is kept"
    );
    assert!(
        !recent.contains(&part(0).storage_key()),
        "while the oldest falls off"
    );
}

/// The pin menu is reachable without a pointer: the shelf's rows are in the
/// tab ring, and Shift+F10 opens the focused row's menu — the same key the
/// navigator's object menu answers to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pin_menu_opens_from_the_keyboard_on_the_focused_row() {
    /// The row the keyboard is on, as the accessibility tree reports it.
    fn focused_label(output: &egui::FullOutput) -> Option<String> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    }

    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());

    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    // Walk the keyboard onto a catalog row the way a reader without a pointer
    // reaches it. The ring is finite, so a row that never takes focus is a
    // row no such reader could ever pin.
    let mut landed = false;
    for _ in 0..80 {
        let output = component_shelf_output(
            &ctx,
            &mut app,
            vec![key(egui::Key::Tab, egui::Modifiers::NONE)],
        );
        if focused_label(&output).as_deref() == Some("Transmission Line") {
            landed = true;
            break;
        }
    }
    assert!(landed, "a shelf row must be reachable from the keyboard");

    let _ = component_shelf_frame(
        &ctx,
        &mut app,
        vec![key(egui::Key::F10, egui::Modifiers::SHIFT)],
    );
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "Shift+F10 opens the focused row's pin menu: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(pin.center()));
    assert!(
        is_pinned(
            &app,
            &ShelfEntry::Primitive(ComponentType::TransmissionLine)
        ),
        "and the menu acts on the row the keyboard was on"
    );
}

/// The object menu answers the same key on the navigator's own rails. A
/// keyboard open has no pointer position for the menu to sit at, so the row
/// anchors it — on every frame it stays open, not only the one that opened it,
/// or the menu would paint once and vanish before it could be read.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_object_menu_opens_from_the_keyboard_on_the_focused_row() {
    /// The row the keyboard is on, as the accessibility tree reports it.
    fn focused_label(output: &egui::FullOutput) -> Option<String> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    }

    let mut app = interface_design();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // A closure over the app rather than a helper handed it, so the frame
    // harness costs the whole-application-access ratchet nothing.
    let mut navigator_output = |events: Vec<egui::Event>| {
        ctx.run_ui(
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
                        navigator(ui, &mut app);
                    });
            },
        )
    };
    let _ = navigator_output(Vec::new());
    let _ = navigator_output(Vec::new());

    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    // Walk the keyboard onto a port row the way a reader without a pointer
    // reaches it.
    let mut landed = false;
    for _ in 0..80 {
        let output = navigator_output(vec![key(egui::Key::Tab, egui::Modifiers::NONE)]);
        if focused_label(&output).as_deref() == Some("ALPHA") {
            landed = true;
            break;
        }
    }
    assert!(
        landed,
        "a navigator object row must be reachable from the keyboard"
    );

    let _ = navigator_output(vec![key(egui::Key::F10, egui::Modifiers::SHIFT)]);
    // The frame after the opening one is the load-bearing frame: a menu whose
    // anchor only lives on the opening frame is gone by now.
    let output = navigator_output(Vec::new());
    let runs = painted_runs(&output);
    let find = run_rect(&runs, "Find references and consumers…").unwrap_or_else(|| {
        panic!(
            "Shift+F10 keeps the focused row's object menu open: {}",
            painted_text(&output)
        )
    });

    let _ = navigator_output(click_events(find.center()));
    assert_eq!(
        app.state.workbench.navigator_filter(),
        "ALPHA",
        "and the menu acts on the row the keyboard was on"
    );
}
