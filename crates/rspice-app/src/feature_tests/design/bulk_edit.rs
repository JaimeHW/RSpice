//! The bulk-edit dialog drawn and committed through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::selection::bulk_edit::tests::component;
use crate::workbench::app::dialogs::selection::bulk_edit::*;
use egui::Context;

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    render_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &mut state.design.schematic_bulk_edit_filters,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

#[test]
fn accessibility_tree_exposes_only_the_mockup_bulk_edit_surface_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", Some("tt"))];
    open_selection_bulk_edit_dialog(
        &mut app.state.design.selection_bulk_edit,
        &app.state.design.schematic_bulk_edit_filters,
        &app.state.model,
    );

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 900.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| render(&mut app, ctx),
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("bulk-edit access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
    }));
    for label in [PRIMARY, "Cancel", "Save filter\u{2026}"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::CheckBox
            && node.label() == Some("Select X1")
            && node.toggled() == Some(egui::accesskit::Toggled::True)
    }));
}

/// The filter's text fields stand as tall as the selects above them and
/// are named by their labels, on desktop and on touch. They were 21 pt
/// beside 28 pt selects (44 on touch), and a screen reader heard nothing.
#[test]
fn the_filter_fields_match_the_selects_and_say_their_names() {
    for coarse in [false, true] {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::Theme::default().apply_responsive_metrics_with_target(
            &ctx,
            coarse.then_some(crate::ui::tokens::TOUCH_TARGET),
        );
        let mut app = RSpiceApp::test_instance();
        app.state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        open_selection_bulk_edit_dialog(
            &mut app.state.design.selection_bulk_edit,
            &app.state.design.schematic_bulk_edit_filters,
            &app.state.model,
        );
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
                |ctx| render(&mut app, ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let height = |label: &str| {
            let bounds = nodes
                .values()
                .find(|node| {
                    node.label() == Some(label) && node.role() != egui::accesskit::Role::Label
                })
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("no control named {label:?}"));
            bounds.y1 - bounds.y0
        };
        let select = height("Object kinds");
        for field in ["Model / cell", "Current property"] {
            assert!(
                (height(field) - select).abs() < 0.5,
                "{field} is {} pt beside a {select} pt select (touch {coarse})",
                height(field)
            );
        }
    }
}

/// Nothing in the body reaches past the footer's right edge. With no
/// section to offer, New value used to state why in a line wider than
/// its column; the body grew about 90 pt past the surface, every row
/// under it took the new width, and the split rule moved with it.
#[test]
fn the_body_stays_inside_the_dialog_when_no_section_is_defined() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", Some("tt"))];
    open_selection_bulk_edit_dialog(
        &mut app.state.design.selection_bulk_edit,
        &app.state.design.schematic_bulk_edit_filters,
        &app.state.model,
    );
    assert!(
        app.state
            .design
            .selection_bulk_edit
            .bound_sections
            .is_empty()
    );

    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| render(&mut app, ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let apply = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some(PRIMARY))
        .and_then(egui::accesskit::Node::bounds)
        .expect("Apply has bounds");
    let footer_right = apply.x1 + 12.0;
    for node in nodes.values() {
        let Some(bounds) = node.bounds() else {
            continue;
        };
        if matches!(
            node.role(),
            egui::accesskit::Role::ComboBox
                | egui::accesskit::Role::TextInput
                | egui::accesskit::Role::CheckBox
                | egui::accesskit::Role::Button
        ) {
            assert!(
                bounds.x1 <= footer_right + 0.5,
                "{:?} {:?} ends at {} past the dialog's {footer_right}",
                node.role(),
                node.label(),
                bounds.x1,
            );
        }
    }
    // Every choice is the kit's select, named for what it picks: the
    // raw combo boxes it replaced had no name to announce.
    for select in [
        "Object kinds",
        "Hierarchy scope",
        "Property",
        "Unset behavior",
        "Model section",
    ] {
        assert!(
            nodes.values().any(|node| {
                node.role() == egui::accesskit::Role::ComboBox && node.label() == Some(select)
            }),
            "no select is named {select:?}"
        );
    }
}
