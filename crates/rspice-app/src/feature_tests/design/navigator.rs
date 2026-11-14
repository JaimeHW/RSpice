//! The design navigator drawn through the design context a whole application
//! lends, or through the whole frame: the hierarchy tree, the library rows and
//! what their gestures open.

use crate::schematic::view::sheet_visibility::{self, SheetScope};
use crate::state::{CellViewRef, ComponentType, InstancePath, SchematicState};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::navigator::tests::{
    announced_name, click_events, interface_design, painted_panel, painted_runs, painted_text,
    recursive_design, run_rect, two_sheet_named_signals,
};
use crate::workbench::design::navigator::*;
use crate::workbench::framework::requests::FocusNavigatorSearch;
use crate::workbench::state::{NavigatorTreeNode, Workspace};

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
    let every_sheet = painted_panel(&ctx, |ui| excitation_section(ui, &mut app.design_cx()));
    assert!(
        every_sheet.contains("VIN") && every_sheet.contains("VBIAS"),
        "the default scope lists the whole cell view: {every_sheet}"
    );

    sheet_visibility::set_sheet_scope(&ctx, SheetScope::ActiveSheet);
    let this_sheet = painted_panel(&ctx, |ui| excitation_section(ui, &mut app.design_cx()));
    assert!(
        this_sheet.contains("VIN"),
        "an object on the active sheet is kept: {this_sheet}"
    );
    assert!(
        !this_sheet.contains("VBIAS"),
        "an object owned by another sheet is filtered out: {this_sheet}"
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
    let revision = app.state.model.workspace.content.project.revision().get();
    let catalog = app.state.model.workspace.content.design_management.clone();

    for scope in SheetScope::OPTIONS {
        sheet_visibility::set_sheet_scope(&ctx, scope);
        let _ = painted_panel(&ctx, |ui| navigator(ui, &mut app.design_cx()));
    }

    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        revision
    );
    assert_eq!(app.state.model.workspace.content.design_management, catalog);
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
                    hierarchy_tree::occurrences_section(ui, &mut app.design_cx());
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
    assert!(
        app.state
            .workbench
            .outbox
            .peek::<FocusNavigatorSearch>()
            .is_some()
    );
}

/// Find raised the search flag but left the left dock on the Component shelf
/// that Place instance had opened, so nothing took the focus, and the stale
/// flag stole it later when the reader switched tabs (audit E-10). Find, by
/// menu or by Ctrl+F, now brings the Navigator tab forward.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn find_brings_the_navigator_tab_forward_over_the_component_shelf() {
    for by_key in [false, true] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        app.state.design.show_component_shelf();

        if by_key {
            app.execute_shortcut_command(Command::FindInDesign);
        } else {
            Command::FindInDesign.execute(&mut app);
        }

        assert!(
            !app.state.design.component_shelf_shown(),
            "by key: {by_key}"
        );
        assert!(
            app.state
                .workbench
                .outbox
                .peek::<FocusNavigatorSearch>()
                .is_some()
        );
    }
}

/// Give the workspace an active configuration that cannot resolve: its
/// DUT path names an instance the expanded hierarchy has not got. The
/// design projection refuses such a configuration, so a rail that still
/// lists nets afterwards is listing the editor buffer's.
fn unresolve_configuration(state: &mut crate::workbench::app_state::AppState) {
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

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_nets_section_states_an_unresolved_configuration_instead_of_buffer_nets() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            1,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
    app.state
        .model
        .schematic
        .document_mut_for_test()
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
                net_section(ui, &mut app.design_cx());
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
                net_section(ui, &mut app.design_cx());
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

/// A net row announces its connection count, not only its name.
///
/// The nets rail paints the count in the meta column and the shared indented
/// row published the label alone, so a reader working by ear was told a net
/// exists and never how many pins reach it — which is the one number that
/// separates a wired node from a stub. The Stimulus Library's own case is in
/// `design::navigator::stimulus`; both rails are painted by
/// `nav_row_indented_styled_with_metrics`, and this pair is what holds it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_net_row_announces_the_meta_column_it_paints() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            1,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(crate::state::NetLabel::new(
            2,
            crate::state::Point::new(0, 0),
            "VOUT",
        ));
    app.state.sync_active_schematic_to_workspace();

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                net_section(ui, &mut app.design_cx());
            });
    });
    let announced = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| announced_name(&node))
        .collect::<Vec<_>>();

    assert!(
        !announced.iter().any(|name| name == "VOUT"),
        "the row dropped its meta column on the way to AccessKit: {announced:?}"
    );
    assert!(
        announced.iter().any(|name| name.starts_with("VOUT, ")),
        "the net row must announce the count it paints beside the name: {announced:?}"
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
                        navigator(ui, &mut app.design_cx());
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
    let find = run_rect(&runs, "Filter by ALPHA").unwrap_or_else(|| {
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

/// One frame's input for the navigator alone, in a dock every rail fits in.
#[cfg(not(target_arch = "wasm32"))]
fn navigator_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(260.0, 1600.0),
        )),
        events,
        ..egui::RawInput::default()
    }
}

/// How long egui asks the host to wait before the frame after `output`.
#[cfg(not(target_arch = "wasm32"))]
fn repaint_delay(output: &egui::FullOutput) -> std::time::Duration {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .expect("the root viewport answers")
        .repaint_delay
}

/// Return to parent runs once the navigator has drawn, as the shell answers
/// what a feature raised, so the click's own frame still draws the
/// occurrence it leaves. The click is input, and egui asks for the frame
/// after any input at once; that frame draws the parent.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_parent_a_path_click_ascends_to_draws_on_the_frame_the_click_repaints() {
    let mut app = recursive_design();
    app.state.workbench.workspace = Workspace::Design;
    let instance = InstancePath::root()
        .child("XA")
        .expect("the fixture instance is nameable");
    hierarchy_tree::open_occurrence(
        &mut app.state.design,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.runs,
        &instance,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // As the shell draws it: the navigator in the design feature's context,
    // then what it raised answered.
    let frame = |app: &mut RSpiceApp, events| {
        ctx.run_ui(navigator_input(events), |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    navigator(ui, &mut app.design_cx());
                });
            app.after_contribution();
        })
    };
    let _ = frame(&mut app, Vec::new());
    let settled = frame(&mut app, Vec::new());
    let at = run_rect(&painted_runs(&settled), "/XA")
        .expect("below the root the path row names the occurrence")
        .center();

    let clicked = frame(&mut app, click_events(at));
    assert_eq!(
        app.state.model.workspace.content.occurrence_path(),
        InstancePath::root(),
        "the ascent ran once the navigator had drawn"
    );
    assert!(
        run_rect(&painted_runs(&clicked), "/XA").is_some(),
        "the click's own frame drew before the ascent ran"
    );
    assert_eq!(
        repaint_delay(&clicked),
        std::time::Duration::ZERO,
        "the click's input asks for the next frame at once"
    );

    let next = frame(&mut app, Vec::new());
    assert!(
        run_rect(&painted_runs(&next), "/XA").is_none(),
        "the next frame draws the parent, where no path row names an occurrence"
    );
}

/// Update pins from contract runs once the navigator has drawn, so the
/// click's own frame still lists the port without a pin. The click is
/// input, and egui asks for the frame after any input at once; that frame
/// lists the pin the command placed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pins_a_contract_update_places_are_listed_on_the_frame_the_click_repaints() {
    use crate::state::{Cell, Library, Point, View, ViewType};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let mut schematic = SchematicState::default();
    let port = schematic.add_component(ComponentType::Port, Point::origin());
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("port exists")
        .value = "IN".to_owned();
    app.state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // As the shell draws it: the navigator in the design feature's context,
    // then what it raised answered.
    let frame = |app: &mut RSpiceApp, events| {
        ctx.run_ui(navigator_input(events), |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    symbol::show(ui, &mut app.design_cx());
                });
            app.after_contribution();
        })
    };
    let _ = frame(&mut app, Vec::new());
    let settled = frame(&mut app, Vec::new());
    let runs = painted_runs(&settled);
    assert!(
        run_rect(&runs, "no pin").is_some() && run_rect(&runs, "1. IN").is_none(),
        "the declared port has no pin yet"
    );
    let at = runs
        .iter()
        .find(|(run, _, _)| run.starts_with("Update pins from"))
        .map(|(_, rect, _)| rect.center())
        .expect("a symbol with an interface offers the update beside its pins");

    let clicked = frame(&mut app, click_events(at));
    assert!(
        app.state
            .model
            .load_active_symbol_document()
            .expect("document loads")
            .pin("IN")
            .is_some(),
        "the update ran once the navigator had drawn"
    );
    assert!(
        run_rect(&painted_runs(&clicked), "1. IN").is_none(),
        "the click's own frame drew before the update ran"
    );
    assert_eq!(
        repaint_delay(&clicked),
        std::time::Duration::ZERO,
        "the click's input asks for the next frame at once"
    );

    let next = frame(&mut app, Vec::new());
    assert!(
        run_rect(&painted_runs(&next), "1. IN").is_some(),
        "the next frame lists the pin the update placed"
    );
}
