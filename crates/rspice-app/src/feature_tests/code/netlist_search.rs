//! The Netlist page's find bar drawn through the application's context.

use crate::workbench::RSpiceApp;
use crate::workbench::surfaces::netlist::search::find_replace_window;

/// What the find dialog names, by accessible label or value, after a few
/// frames over `app`.
fn find_dialog_names(app: &mut RSpiceApp) -> Vec<String> {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(app)),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    nodes
        .values()
        .flat_map(|node| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// The generated deck is read-only, so finding in it offers no Replace
/// field and no Replace actions, and the dialog is called Find; an owned
/// deck is replaced in. The dialog used to show a disabled Replace field
/// and disabled Replace buttons over the generated deck, and three status
/// lines: a footer hint, a body line and the error.
#[test]
fn a_read_only_deck_is_found_in_but_offers_no_replace() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    app.state.code.netlist.find.open = true;
    let names = find_dialog_names(&mut app);
    let says = |text: &str| names.iter().any(|name| name == text);
    assert!(says("Find in source"), "{names:?}");
    assert!(says("Find"), "the Find field is missing");
    assert!(!says("Replace"), "a read-only deck offers Replace");
    assert!(!says("Replace all"), "a read-only deck offers Replace all");
    assert!(
        !says("Enter text to search the current document."),
        "a second status line in the body"
    );

    crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action_in(
        &app.state.model,
        &mut app.state.code.netlist,
        crate::workbench::documents::code_workspace::CodeSourceFileAction::New,
    )
    .unwrap();
    crate::workbench::documents::netlist_document::commit_netlist_lifecycle_action(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.code.netlist_open_documents,
    )
    .unwrap();
    let names = find_dialog_names(&mut app);
    let says = |text: &str| names.iter().any(|name| name == text);
    assert!(says("Find and replace in source"), "{names:?}");
    assert!(says("Replace"), "an owned deck offers no Replace");
}

/// The Find field is a control's height like the Scope select beside it,
/// a finger's target in a touch shell. It was egui's default single line,
/// shorter than the select at the desktop size and half its height on a
/// tablet.
#[test]
fn the_find_field_is_as_tall_as_the_scope_select() {
    use egui::accesskit::Role;

    for coarse in [false, true] {
        let mut app = RSpiceApp::test_instance();
        app.state.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        app.state.code.netlist.find.open = true;
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::Theme::default().apply_responsive_metrics_with_target(
            &ctx,
            coarse.then_some(crate::ui::tokens::TOUCH_TARGET),
        );
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1024.0, 768.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ui| find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(&mut app)),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let height = |role: Role| {
            nodes
                .values()
                .find(|node| node.role() == role)
                .and_then(|node| node.bounds())
                .map(|bounds| bounds.y1 - bounds.y0)
                .unwrap_or_else(|| panic!("no {role:?}"))
        };
        let (field, select) = (height(Role::TextInput), height(Role::ComboBox));
        assert!(
            (field - select).abs() <= 1.0,
            "touch {coarse}: the field is {field} tall and the select {select}"
        );
    }
}

/// The options start at the fields' left edge, under the Find field, not
/// under the labels beside it.
#[test]
fn the_options_start_where_the_fields_do() {
    use egui::accesskit::Role;

    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    app.state.code.netlist.find.open = true;
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| find_replace_window(ui.ctx(), &mut crate::workbench::features::cx(&mut app)),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let lefts = |role: Role| {
        nodes
            .values()
            .filter(|node| node.role() == role)
            .filter_map(|node| node.bounds())
            .map(|bounds| bounds.x0)
            .collect::<Vec<_>>()
    };
    let field = lefts(Role::TextInput)[0];
    let checks = lefts(Role::CheckBox);
    assert_eq!(checks.len(), 5, "{checks:?}");
    let first = checks.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(
        (first - field).abs() < 1.0,
        "the options start at {first}, the field at {field}"
    );
}
