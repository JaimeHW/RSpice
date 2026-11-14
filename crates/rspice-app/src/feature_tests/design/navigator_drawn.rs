//! The navigator drawn through a whole application: its sheet scope, occurrence rows and
//! bands, references, wireless net selection, section folds and keyboard walk.

use crate::schematic::view::sheet_visibility::SheetScope;
use crate::state::{CellViewRef, ComponentType, InstancePath, SchematicState};
use crate::workbench::RSpiceApp;
use crate::workbench::design::navigator::hierarchy_tree::{DesignTreeRow, OccurrenceState};
use crate::workbench::design::navigator::tests::{
    NavigatorPanel, add_master, click_events, interface_design, painted_panel, placed,
    recursive_design, two_sheet_named_signals,
};
use crate::workbench::design::navigator::*;
use crate::workbench::framework::requests::FocusNavigatorSearch;
use crate::workbench::state::{NavigatorTreeNode, NavigatorTreeState, Workspace};
use rspice_design::connectivity::summary::DesignNet;
use std::collections::HashSet;

/// A drawing with one sheet has nothing to scope, so the control is absent
/// rather than offered with one position that means anything.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sheet_scope_control_is_offered_only_above_one_sheet() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    let mut single = RSpiceApp::test_instance();
    let one_sheet = painted_panel(&ctx, |ui| navigator(ui, &mut single.design_cx()));
    assert!(
        !one_sheet.contains(SheetScope::ActiveSheet.label())
            && !one_sheet.contains(SheetScope::AllSheets.label()),
        "a single-sheet cell view offers no scope control: {one_sheet}"
    );

    let mut multi = two_sheet_named_signals();
    let two_sheets = painted_panel(&ctx, |ui| navigator(ui, &mut multi.design_cx()));
    assert!(
        two_sheets.contains(SheetScope::ActiveSheet.label())
            && two_sheets.contains(SheetScope::AllSheets.label()),
        "both positions are offered above one sheet: {two_sheets}"
    );
}

/// The occurrence rows of one app against one reading position.
fn occurrence_rows(app: &RSpiceApp, tree: &NavigatorTreeState) -> Vec<DesignTreeRow> {
    let projection = app
        .state
        .model
        .workspace
        .design_projection(
            &app.state.model.library_manager,
            &app.state.model.workspace.content.active_view,
            &app.state.model.schematic,
        )
        .expect("the fixture design projects");
    hierarchy_tree::occurrence_rows(
        &hierarchy_tree::TreeSource {
            projection: projection.as_ref(),
            sheets: &app.state.model.workspace.content.design_management,
            libraries: &app.state.model.library_manager,
            query: "",
        },
        tree,
    )
}

/// One app's design projection, which every hierarchy rail reads.
fn projection_of(app: &RSpiceApp) -> std::sync::Arc<crate::state::workspace::DesignProjection> {
    app.state
        .model
        .workspace
        .design_projection(
            &app.state.model.library_manager,
            &app.state.model.workspace.content.active_view,
            &app.state.model.schematic,
        )
        .expect("the fixture design projects")
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

    hierarchy_tree::open_occurrence(
        &mut app.state.design,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.runs,
        &leaf,
    );
    assert_eq!(app.state.model.workspace.content.occurrence_path(), leaf);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "leaf", "schematic")
    );

    hierarchy_tree::open_occurrence(
        &mut app.state.design,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.runs,
        &instance,
    );
    assert_eq!(
        app.state.model.workspace.content.occurrence_path(),
        instance
    );
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "child", "schematic")
    );
}

/// A design whose root places one master that resolves and two occurrences of
/// one that does not: `work/missing` is never given a cell, so the resolver
/// records it unresolved rather than binding it.
fn design_with_a_broken_master() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let components = &mut app.state.model.schematic.document_mut_for_test().components;
    components.push(placed(101, "XA", "work", "child"));
    components.push(placed(102, "XGONE", "work", "missing"));
    components.push(placed(103, "XGONE2", "work", "missing"));
    app.state.sync_active_schematic_to_workspace();
    add_master(&mut app.state, "work", "child", SchematicState::default());
    app
}

/// A master the design cannot find still belongs to the rail named after it.
///
/// Masters used to read the execution plan, which carries only what resolved,
/// so a deleted cell vanished from the rail and from its count while the
/// Occurrences rail went on showing its instances as `unbound`. The broken
/// master is the one worth finding here.
#[test]
fn the_masters_rail_lists_a_master_that_did_not_resolve() {
    let app = design_with_a_broken_master();
    let projection = projection_of(&app);
    let masters = hierarchy_tree::declared_masters(projection.as_ref(), "");

    let cells = masters.get("work").expect("the fixture library is named");
    let views = |cell: &str| {
        cells
            .get(cell)
            .unwrap_or_else(|| panic!("{cell} is a master this design names"))
    };
    let child = &views("child")[0];
    assert_eq!(child.occurrences, 1);
    assert!(
        child.condition.is_none(),
        "a master that resolved states its count, not a condition"
    );

    let missing = &views("missing")[0];
    assert_eq!(
        missing.occurrences, 2,
        "an unresolved master is counted the way a resolved one is: how many \
         places in the design reach for it"
    );
    let condition = missing
        .condition
        .as_ref()
        .expect("the row carries the resolver's own outcome");
    assert_eq!(condition.label, "unresolved");
    assert!(
        condition
            .diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("missing")),
        "the row can say why: {:?}",
        condition.diagnostic
    );

    // The band counts master views, and the broken one is one of them.
    let total = masters
        .values()
        .flat_map(|cells| cells.values())
        .map(Vec::len)
        .sum::<usize>();
    assert_eq!(total, 3, "root, child, and the master that did not resolve");
}

/// The Occurrences band states the design, not the disclosure.
#[test]
fn the_occurrences_band_counts_the_design_and_not_the_rows_on_screen() {
    let app = recursive_design();
    let projection = projection_of(&app);
    let root = InstancePath::root();
    let instance = root.child("XA").expect("the fixture instance is nameable");

    let mut tree = NavigatorTreeState::default();
    let folded = occurrence_rows(&app, &tree);
    assert_eq!(
        folded
            .iter()
            .filter(|row| matches!(row, DesignTreeRow::Occurrence(_)))
            .count(),
        1,
        "a folded root paints one row"
    );
    let folded_count = hierarchy_tree::occurrence_band_count(projection.as_ref(), "", &folded);

    tree.expand(NavigatorTreeNode::Occurrence(root.fold_key()));
    tree.expand(NavigatorTreeNode::Occurrence(instance.fold_key()));
    let unfolded = occurrence_rows(&app, &tree);
    assert_eq!(
        unfolded
            .iter()
            .filter(|row| matches!(row, DesignTreeRow::Occurrence(_)))
            .count(),
        4,
        "the fixture's whole hierarchy is root, XA, XLEAF and the cycle"
    );
    let unfolded_count = hierarchy_tree::occurrence_band_count(projection.as_ref(), "", &unfolded);

    assert_eq!(
        folded_count, unfolded_count,
        "folding a node changes what is on screen, not how big the design is"
    );
    assert_eq!(
        folded_count,
        projection.hierarchy_resolution().total_instances,
        "the band states the figure preflight and the configuration surface state"
    );
    assert_eq!(folded_count, 4);

    // A filter asks about the whole design, so under one the band answers the
    // filter rather than restating the design's size.
    assert_eq!(
        hierarchy_tree::occurrence_band_count(projection.as_ref(), "xleaf", &unfolded),
        unfolded
            .iter()
            .filter(|row| matches!(row, DesignTreeRow::Occurrence(_)))
            .count()
    );
}

/// A design that places one built-in XSPICE code model: the PWM oscillator,
/// bound into the engine's own `rspice` namespace rather than into a library.
fn design_placing_a_builtin_xspice_device() -> (RSpiceApp, InstancePath) {
    use crate::state::{builtin_xspice_library_binding, engine_only_xspice_devices};

    let descriptor = engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.model_type == "d_pwm")
        .expect("the PWM oscillator is an authoritative catalog entry");
    let binding =
        builtin_xspice_library_binding(descriptor).expect("the PWM oscillator places a binding");

    let mut app = RSpiceApp::test_instance();
    let mut component = crate::state::Component::new(
        401,
        ComponentType::CellInstance,
        crate::state::Point::new(40, 40),
    )
    .with_library_cell(binding);
    component.name = "A1".to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    app.state.sync_active_schematic_to_workspace();

    let occurrence = InstancePath::root()
        .child("A1")
        .expect("the fixture instance is nameable");
    (app, occurrence)
}

/// A master the engine implements is not a master either rail can open.
///
/// A built-in XSPICE code model is staged under `rspice`, which is a binding
/// label and not a library, so there is no cell view behind it. Both rails
/// used to offer the gesture anyway, and taking it re-rooted the session onto
/// a blank schematic belonging to no cell — which read as the device having
/// an empty schematic of its own.
#[test]
fn a_built_in_device_master_is_inert_in_both_hierarchy_rails() {
    let (mut app, occurrence) = design_placing_a_builtin_xspice_device();
    let root_view = app.state.model.workspace.content.active_view.clone();
    let device = CellViewRef::new("rspice", "d_pwm", "xspice");

    let mut tree = NavigatorTreeState::default();
    tree.expand(NavigatorTreeNode::Occurrence(
        InstancePath::root().fold_key(),
    ));
    assert_eq!(
        occurrence_states(&occurrence_rows(&app, &tree)),
        vec![
            (InstancePath::root(), OccurrenceState::Expanded),
            (occurrence.clone(), OccurrenceState::Device),
        ],
        "the device occurrence is bound, but not to anything that can be read"
    );
    assert!(
        !OccurrenceState::Device.is_bound(),
        "a device occurrence has nothing to descend into, so its row is inert"
    );

    // The predicate the Masters rail asks before it offers to open a row.
    assert!(
        !hierarchy_tree::authored_view(&app.state.model.library_manager, &device),
        "no library holds the engine's own code model"
    );
    assert!(
        hierarchy_tree::authored_view(&app.state.model.library_manager, &root_view),
        "the project's own master is openable"
    );

    hierarchy_tree::open_occurrence(
        &mut app.state.design,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.runs,
        &occurrence,
    );
    assert_eq!(
        app.state.model.workspace.content.active_view, root_view,
        "descending into a device leaves the session where it stood"
    );
    assert_eq!(
        app.state.model.workspace.content.occurrence_path(),
        InstancePath::root()
    );
    assert!(
        app.state
            .model
            .workspace
            .clone_schematic_editor(&device.key())
            .is_none(),
        "no blank schematic is minted for a cell view the project does not have"
    );
}

/// A cell view with one sheet *is* its sheet, so the tree offers a sheet node
/// only where entering one means something.
#[test]
fn sheet_nodes_appear_only_above_one_sheet() {
    let mut app = RSpiceApp::test_instance();
    let project = &mut app.state.model.workspace.content;
    let reference = project.active_view.clone();
    let key = reference.key();
    let first = project
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("the fixture cell view takes a sheet catalog");
    assert!(hierarchy_tree::sheets_of(&project.design_management, &reference).is_empty());

    let catalog = project
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

    let sheets = hierarchy_tree::sheets_of(&project.design_management, &reference);
    assert_eq!(
        sheets
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>(),
        ["Sheet 1", "Sheet 2"]
    );
}

/// The object menu's find row answers about the object it was raised over:
/// the rail comes back filtering by that object's own name, not by nothing.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn finding_references_seeds_the_navigator_filter_with_the_object_name() {
    let mut app = RSpiceApp::test_instance();
    app.state.design.show_component_shelf();

    find_navigator_object_references(
        &mut app.state.design,
        &mut app.state.workbench,
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
    assert!(!app.state.design.component_shelf_shown());
    assert!(app.state.workbench.navigator_visible);
    assert!(
        app.state
            .workbench
            .outbox
            .peek::<FocusNavigatorSearch>()
            .is_some()
    );
}

#[test]
fn wireless_navigator_net_selection_is_exact_and_self_invalidating() {
    let mut app = RSpiceApp::test_instance();
    let net = DesignNet {
        name: "PORT_OUT".to_owned(),
        authored_name: true,
        class: rspice_design::connectivity::summary::NetClass::Signal,
        terminals: vec![rspice_design::connectivity::summary::NetTerminal {
            component_id: 9,
            reference: "X1".to_owned(),
            pin: "OUT".to_owned(),
        }],
        port: Some(crate::state::PortDirection::Out),
        wire_ids: Vec::new(),
    };
    let session = &mut app.state.model.schematic.session;
    session.editor.selection.select_only_component(9);
    session
        .editor
        .net_highlight
        .highlight_named_wires(&net.name, HashSet::new());
    assert!(navigator_net_selection_matches(&app.state.model, &net));

    let session = &mut app.state.model.schematic.session;
    session.editor.selection.select_only_component(10);
    session.editor.net_highlight.clear();
    assert!(!navigator_net_selection_matches(&app.state.model, &net));
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
        panel.painted().lines().any(|line| line == "Properties…"),
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
