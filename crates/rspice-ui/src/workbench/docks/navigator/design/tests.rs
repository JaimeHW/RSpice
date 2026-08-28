//! What the design navigator claims about the design it is reading.
//!
//! The hierarchy rows are the load-bearing ones: an unfolded node builds its
//! children and a folded one does not, a master that repeats on its own
//! ancestry is an inert leaf, and an occurrence click lands the session on
//! that occurrence. The rest hold the mockup geometry and the rail contracts
//! the sections were built to.
//!
//! The shelf tab's own cases are in [`shelf`]; the frame-reading helpers they
//! share with these — [`painted_runs`], [`run_rect`], [`glyph_canvas`] — stay
//! here, because a harness that reads one painted frame is not either tab's.

use super::hierarchy_tree::{DesignTreeRow, OccurrenceState};
use super::shelf::*;
use super::*;
use crate::state::{CellViewRef, ComponentType, InstancePath, SchematicState};
use crate::workbench::state::{NavigatorTreeNode, NavigatorTreeState, Workspace};

mod shelf;

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
        &mut app.state,
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
    assert!(navigator_net_selection_matches(&app.state, &net));

    app.state.schematic.selection.select_only_component(10);
    app.state.schematic.net_highlight.clear();
    assert!(!navigator_net_selection_matches(&app.state, &net));
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

/// Where one painted run of a frame sits, for a gesture that has to land on
/// it.
#[cfg(not(target_arch = "wasm32"))]
fn run_rect(runs: &[(String, egui::Rect, egui::Color32)], text: &str) -> Option<egui::Rect> {
    runs.iter()
        .find(|(run, _, _)| run == text)
        .map(|(_, rect, _)| *rect)
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
    /// The dock's height, which is what decides whether the rail scrolls.
    height: f32,
    /// Every control the frame announced: what it says, where it is, and the
    /// disclosure position it publishes.
    controls: Vec<(String, egui::Rect, Option<bool>)>,
    /// Every run of text the frame painted.
    runs: Vec<(String, egui::Rect, egui::Color32)>,
    /// What the keyboard is on, as the accessibility tree reports it.
    focus: Option<String>,
    /// Which traversal keys were still in the input once the panel had run:
    /// the whole of the panel's consumption contract, read from the outside.
    survived: Vec<egui::Key>,
}

/// The keys a rail may claim, and the one it must not.
#[cfg(not(target_arch = "wasm32"))]
const TRAVERSAL_KEYS: [egui::Key; 7] = [
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::Home,
    egui::Key::End,
    egui::Key::Escape,
];

/// One unmodified press.
#[cfg(not(target_arch = "wasm32"))]
fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

/// What one accessibility node says it is called.
///
/// `label` for a control and `value` for a `Label`-role node, because egui
/// files a label's text under the second of those and a reader asking only the
/// first would find the panel's plain rows nameless.
#[cfg(not(target_arch = "wasm32"))]
fn announced_name(node: &egui::accesskit::Node) -> Option<String> {
    node.label().or_else(|| node.value()).map(str::to_owned)
}

#[cfg(not(target_arch = "wasm32"))]
impl NavigatorPanel {
    /// The panel at a height every rail fits in, so a case that is not about
    /// scrolling never has to think about it.
    fn open(app: RSpiceApp) -> Self {
        Self::open_within(app, 1600.0)
    }

    /// The panel in a dock of `height`, for the cases that are.
    fn open_within(app: RSpiceApp, height: f32) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut panel = Self {
            ctx,
            app,
            height,
            controls: Vec::new(),
            runs: Vec::new(),
            focus: None,
            survived: Vec::new(),
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
        let height = self.height;
        let mut survived = Vec::new();
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(260.0, height),
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
                // Read once the panel has had the press: what is still in the
                // input is what the panel left for the canvas behind it.
                survived = TRAVERSAL_KEYS
                    .into_iter()
                    .filter(|key| {
                        ctx.ctx()
                            .input_mut(|input| input.consume_key(egui::Modifiers::NONE, *key))
                    })
                    .collect();
            },
        );
        self.survived = survived;
        self.runs = painted_runs(&output);
        let update = output.platform_output.accesskit_update;
        self.focus = update.as_ref().and_then(|update| {
            update
                .nodes
                .iter()
                .find(|(id, _)| *id == update.focus)
                .and_then(|(_, node)| announced_name(node))
        });
        self.controls = update
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

    /// One press, and the frame that settles what it moved.
    ///
    /// Two passes, because a fold moved from the keyboard is applied once the
    /// rail is painted — exactly as a press on the caret is — so the rows it
    /// discloses belong to the frame after. What the press left behind is
    /// carried across from the pass that carried the press.
    fn press(&mut self, key: egui::Key) {
        self.pass(vec![key_event(key)]);
        let survived = std::mem::take(&mut self.survived);
        self.pass(Vec::new());
        self.survived = survived;
    }

    /// Run the panel until whatever it set in motion has stopped moving.
    ///
    /// egui animates a scroll over the frames after the one that asked for it,
    /// so a case reading where a row ended up has to let those frames run.
    fn settle(&mut self) {
        for _ in 0..60 {
            self.pass(Vec::new());
        }
    }

    /// What the keyboard is on.
    fn focused(&self) -> Option<&str> {
        self.focus.as_deref()
    }

    /// Every run the last pass painted, one to a line.
    fn painted(&self) -> String {
        self.runs
            .iter()
            .map(|(run, _, _)| run.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Whether the panel left `key` for whatever stands behind it.
    fn survived(&self, key: egui::Key) -> bool {
        self.survived.contains(&key)
    }

    /// Put the keyboard in the filter field, through the route the Find
    /// command uses.
    fn focus_filter(&mut self) {
        self.app.state.workbench.focus_navigator_search = true;
        self.pass(Vec::new());
    }

    /// Fold every rail this design draws.
    ///
    /// What is left is the panel's own bands, so the order a traversal has to
    /// produce is the order [`DESIGN_NAVIGATOR_SECTION_ORDER`] declares rather
    /// than whatever the fixture happens to place inside them.
    fn fold_every_section(&mut self) {
        for section in DESIGN_NAVIGATOR_SECTION_ORDER {
            let title = section.title();
            if self.band_if_present(title).is_some() {
                self.click(title);
            }
        }
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

// ------------------------------------------------------ rails from the keys

/// The filter hands the keyboard to the rows it narrowed, and the vertical
/// keys walk them from there.
///
/// The rail was a Tab ring before this: reaching the last band of a long
/// navigator meant pressing Tab past every row and every disclosure caret
/// above it, and Home and End did nothing at all. Ends that hold rather than
/// wrap are the half worth stating — a tree that wraps takes a reader who
/// meant to stop at the bottom back to the top without saying so.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn down_from_the_filter_steps_into_the_rail_and_the_vertical_keys_walk_it() {
    let mut panel = NavigatorPanel::open(interface_design());
    panel.fold_every_section();

    panel.focus_filter();
    assert_eq!(
        panel.focused(),
        Some("Find instance, net or port…"),
        "the filter takes the keyboard it was asked for"
    );

    panel.press(egui::Key::ArrowDown);
    assert_eq!(
        panel.focused(),
        Some("Masters"),
        "and Down steps out of the query onto the rail's first row"
    );

    for title in [
        "Occurrences",
        "Ports",
        "Nets",
        "Excitations",
        "Named signals",
    ] {
        panel.press(egui::Key::ArrowDown);
        assert_eq!(
            panel.focused(),
            Some(title),
            "Down walks the rail in the order it is painted"
        );
    }

    panel.press(egui::Key::ArrowDown);
    assert_eq!(
        panel.focused(),
        Some("Named signals"),
        "the last row is the last row: the rail holds rather than wraps"
    );

    panel.press(egui::Key::ArrowUp);
    assert_eq!(panel.focused(), Some("Excitations"), "and Up walks back");

    panel.press(egui::Key::Home);
    assert_eq!(panel.focused(), Some("Masters"), "Home reaches the top");
    panel.press(egui::Key::ArrowUp);
    assert_eq!(
        panel.focused(),
        Some("Masters"),
        "which is where Up stops too"
    );

    panel.press(egui::Key::End);
    assert_eq!(
        panel.focused(),
        Some("Named signals"),
        "and End reaches the bottom"
    );
}

/// Right and Left carry the caret's own semantics: they disclose the row the
/// keyboard is on, and climb out of one that has nothing left to disclose.
///
/// Both halves of each key matter. Right on an open row steps onto its first
/// child rather than doing nothing, and Left on a leaf climbs to the row that
/// holds it rather than doing nothing — which is what makes a deep hierarchy
/// walkable without a pointer at all.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_horizontal_keys_disclose_a_hierarchy_row_and_climb_out_of_it() {
    let mut panel = NavigatorPanel::open(interface_design());
    // Every rail but the hierarchy folded, so the row below the Masters band
    // is the one master group this design declares.
    for section in DESIGN_NAVIGATOR_SECTION_ORDER
        .into_iter()
        .filter(|section| *section != DesignNavigatorSection::Masters)
    {
        let title = section.title();
        if panel.band_if_present(title).is_some() {
            panel.click(title);
        }
    }

    panel.focus_filter();
    panel.press(egui::Key::ArrowDown);
    panel.press(egui::Key::ArrowDown);
    let group = panel
        .focused()
        .expect("the row under the Masters band")
        .to_owned();
    assert_eq!(
        panel.expanded(&group),
        Some(false),
        "a master group starts folded"
    );

    panel.press(egui::Key::ArrowRight);
    assert_eq!(
        panel.expanded(&group),
        Some(true),
        "Right unfolds the row the keyboard is on"
    );
    assert_eq!(
        panel.focused(),
        Some(group.as_str()),
        "and leaves the keyboard on it, because unfolding a node is reading \
         it rather than moving to it"
    );

    panel.press(egui::Key::ArrowRight);
    let child = panel
        .focused()
        .expect("a row below the unfolded group")
        .to_owned();
    assert_ne!(
        child, group,
        "a second Right steps onto the first of the children it disclosed"
    );

    // A disclosed row carries its caret as a hit target of its own, sitting in
    // the Tab ring beside the row. A step is between rows, so the caret is
    // never a stop of its own — and never the thing a step lands on.
    panel.press(egui::Key::ArrowUp);
    assert_eq!(
        panel.focused(),
        Some(group.as_str()),
        "Up steps to the row above, not to a disclosure control between them"
    );
    panel.press(egui::Key::ArrowDown);
    assert_eq!(
        panel.focused(),
        Some(child.as_str()),
        "and Down steps back to the row below"
    );

    panel.press(egui::Key::ArrowLeft);
    assert_eq!(
        panel.focused(),
        Some(group.as_str()),
        "Left climbs out of a folded row to the row that holds it"
    );
    panel.press(egui::Key::ArrowLeft);
    assert_eq!(
        panel.expanded(&group),
        Some(false),
        "and folds an unfolded one instead of climbing past it"
    );
    panel.press(egui::Key::ArrowLeft);
    assert_eq!(
        panel.focused(),
        Some("Masters"),
        "which leaves the band above it as the next step out"
    );
    panel.press(egui::Key::ArrowLeft);
    assert_eq!(
        panel.expanded("Masters"),
        Some(false),
        "and the band folds like any other row that discloses children"
    );
}

/// Right on an open rail that holds nothing stays where it is.
///
/// An open band with no rows under it is ordinary — a design that places no
/// source has one — and stepping to the band after it would be Down wearing
/// the other key's name, which is how a reader loses their place.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn right_on_an_open_rail_that_holds_nothing_is_not_a_step_down() {
    let mut panel = NavigatorPanel::open(interface_design());
    assert_eq!(
        panel.expanded("Excitations"),
        Some(true),
        "the rail is open"
    );
    assert!(
        panel
            .rows_under("Excitations")
            .iter()
            .all(|run| run.to_lowercase().contains("no sources placed")),
        "and this design placed nothing in it: {:?}",
        panel.rows_under("Excitations")
    );

    panel.focus_filter();
    let mut landed = false;
    for _ in 0..40 {
        panel.press(egui::Key::ArrowDown);
        if panel.focused() == Some("Excitations") {
            landed = true;
            break;
        }
    }
    assert!(landed, "the band is reachable by stepping");

    panel.press(egui::Key::ArrowRight);
    assert_eq!(
        panel.focused(),
        Some("Excitations"),
        "and Right leaves the keyboard on it rather than walking past it"
    );
}

/// A step past the fold brings the row it lands on into view.
///
/// Without this the keys move a focus ring the reader cannot see, which is
/// worse than no traversal at all: the panel would answer, and answer
/// somewhere off screen.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_step_past_the_fold_scrolls_the_row_it_lands_on_into_view() {
    // A dock too short for the rails this design draws, which is the ordinary
    // case: the navigator shares its column with the canvas.
    let height = 220.0;
    let mut panel = NavigatorPanel::open_within(interface_design(), height);
    let last = "Named signals";
    assert!(
        panel.band(last).0.bottom() > height,
        "the fixture must overflow the dock for this case to mean anything"
    );

    panel.focus_filter();
    panel.press(egui::Key::ArrowDown);
    panel.press(egui::Key::End);
    assert_eq!(
        panel.focused(),
        Some(last),
        "End reached the rail's last row"
    );
    // The scroll is animated, so it lands over the frames after the press
    // rather than inside it.
    panel.settle();

    let bounds = panel.band(last).0;
    assert!(
        bounds.top() >= 0.0 && bounds.bottom() <= height,
        "and the rail scrolled it into the dock: {bounds:?} in a {height} px \
         column"
    );
}

/// The rail claims a key only while one of its rows holds the keyboard.
///
/// This is the whole of its scoping contract, and it is load-bearing: the
/// canvas nudges a selection and traverses objects with the same four arrows,
/// and a panel that ate them from the side would take the canvas's own keys
/// away from it. Escape is never the rail's at all.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rail_claims_the_arrows_only_while_it_holds_the_keyboard() {
    let mut panel = NavigatorPanel::open(interface_design());
    panel.fold_every_section();

    assert_eq!(
        panel.focused(),
        None,
        "nothing in the panel holds the keyboard"
    );
    panel.press(egui::Key::ArrowDown);
    assert_eq!(
        panel.focused(),
        None,
        "so the press was never the panel's to answer"
    );
    assert!(
        panel.survived(egui::Key::ArrowDown),
        "and it is still there for the canvas behind it"
    );

    panel.focus_filter();
    panel.press(egui::Key::ArrowDown);
    assert_eq!(panel.focused(), Some("Masters"));
    assert!(
        !panel.survived(egui::Key::ArrowDown),
        "the filter's own Down is spent stepping into the rail"
    );

    panel.press(egui::Key::ArrowDown);
    assert_eq!(panel.focused(), Some("Occurrences"));
    assert!(
        !panel.survived(egui::Key::ArrowDown),
        "a row on the keyboard owns the vertical arrows"
    );
    panel.press(egui::Key::ArrowRight);
    assert!(
        !panel.survived(egui::Key::ArrowRight),
        "and the horizontal ones"
    );

    panel.press(egui::Key::Escape);
    assert!(
        panel.survived(egui::Key::Escape),
        "and nothing else: Escape goes on meaning put the tool away"
    );
}

/// The rail stands down while a row's menu is open.
///
/// The object menu is raised from the same rows this traversal walks, and a
/// keyboard-raised one is anchored to the row it came from — on every frame it
/// stays open, not only the frame that opened it. A rail that went on
/// answering the arrows would scroll that row, and the menu pinned to it, out
/// from under the reader looking at it. So the press is left alone, which is
/// the guard the canvas already keeps over its own object traversal.
///
/// Where the keyboard ends up is then egui's to decide, and this says nothing
/// about it: standing down means standing down.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_open_row_menu_takes_the_arrows_back_from_the_rail() {
    let mut panel = NavigatorPanel::open(interface_design());
    panel.focus_filter();

    // Down to a row that carries an object menu.
    let mut landed = false;
    for _ in 0..40 {
        panel.press(egui::Key::ArrowDown);
        if panel.focused() == Some("ALPHA") {
            landed = true;
            break;
        }
    }
    assert!(landed, "a port row is reachable by stepping");
    assert!(
        !panel.survived(egui::Key::ArrowDown),
        "and the rail was answering the arrows on the way there"
    );

    panel.pass(vec![egui::Event::Key {
        key: egui::Key::F10,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::SHIFT,
    }]);
    panel.pass(Vec::new());
    assert!(
        panel
            .painted()
            .lines()
            .any(|line| line == "NAVIGATOR OBJECT"),
        "Shift+F10 raised the focused row's menu: {}",
        panel.painted()
    );

    panel.press(egui::Key::ArrowDown);
    assert!(
        panel.survived(egui::Key::ArrowDown),
        "and with the menu up it leaves the press alone"
    );
}

// -------------------------------------------------------- the filter's field

/// The filter offers the control that empties it exactly while there is
/// something to empty, and Escape does the same from the keyboard — leaving
/// the field, so one press both clears the query and gives the keyboard back.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_filter_is_emptied_by_its_own_control_and_by_escape() {
    let mut panel = NavigatorPanel::open(interface_design());
    panel.focus_filter();
    assert!(
        panel.band_if_present("Clear the filter").is_none(),
        "an empty filter has nothing to offer to empty"
    );

    panel
        .app
        .state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Design)
        .push_str("ALPHA");
    panel.pass(Vec::new());
    let clear = panel
        .band_if_present("Clear the filter")
        .expect("a filter holding a query offers the control that empties it")
        .0;
    assert!(
        clear.center().x > panel.band("Masters").0.center().x,
        "which sits in the field's own right inset"
    );

    panel.pass(click_events(clear.center()));
    panel.pass(Vec::new());
    assert_eq!(
        panel.app.state.workbench.navigator_filter(),
        "",
        "a press on it empties the query"
    );
    assert_eq!(
        panel.focused(),
        Some("Find instance, net or port…"),
        "and leaves the reader in the field they were typing in"
    );

    panel
        .app
        .state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Design)
        .push_str("ALPHA");
    panel.pass(Vec::new());
    panel.press(egui::Key::Escape);
    assert_eq!(
        panel.app.state.workbench.navigator_filter(),
        "",
        "Escape empties it too"
    );
    assert_eq!(
        panel.focused(),
        None,
        "and hands the keyboard back rather than holding it in an empty field"
    );
    assert!(
        panel.band_if_present("Clear the filter").is_none(),
        "so the control goes with the query"
    );
}

/// The clear control's mark is painted, not set in a face that lacks it.
///
/// The mockup draws a `✕` and the bundled IBM Plex cuts are Latin subsets that
/// do not hold one, so an authored character would have shipped as the
/// layouter's replacement box while every test that reads announced controls
/// went on passing. The first assertion is the trap itself, stated so a future
/// reader cannot mistake the vector mark for decoration.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_filter_clear_mark_is_painted_because_the_bundled_faces_lack_the_glyph() {
    let pixels = |canvas: &crate::ui::raster::Canvas| -> Vec<egui::Color32> {
        canvas
            .pixels_in(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(24.0, 24.0),
            ))
            .collect()
    };
    let tofu = pixels(&glyph_canvas(ShelfGlyph::Text("\u{2603}")));
    assert_eq!(
        pixels(&glyph_canvas(ShelfGlyph::Text("\u{2715}"))),
        tofu,
        "the mockup's ✕ is not in the bundled faces, which is why the control \
         paints its own mark"
    );

    let mark = crate::ui::raster::render(egui::vec2(24.0, 24.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                WorkbenchIcon::Close.paint(
                    ui.painter(),
                    egui::Rect::from_center_size(egui::pos2(12.0, 12.0), egui::vec2(13.0, 13.0)),
                    egui::Color32::WHITE,
                );
            });
    });
    let background = mark.background();
    let painted = pixels(&mark);
    assert!(
        painted.iter().any(|pixel| *pixel != background),
        "the clear control paints no ink at all"
    );
    assert_ne!(
        painted, tofu,
        "and what it paints is not the replacement box"
    );
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
