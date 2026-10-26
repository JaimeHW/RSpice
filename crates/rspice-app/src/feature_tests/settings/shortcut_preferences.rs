//! The Shortcuts page and editor over a whole application.

use crate::feature_tests::settings::preferences_dialogs;
use crate::state::shortcuts::ChordTimeoutPolicy;
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::app::dialogs::preferences::PreferencePageActions;
use crate::workbench::app::dialogs::preferences::shortcut_preferences::*;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::commands::vocabulary::command_registry;
use egui::Context;
use egui::Modifiers;

pub(crate) fn input(width: f32, height: f32) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, height),
        )),
        ..egui::RawInput::default()
    }
}

fn node_bounds(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> egui::accesskit::Rect {
    nodes
        .iter()
        .find(|(_, node)| node.role() == role && node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| panic!("missing {role:?} node {label}"))
}

fn click_page_button(label: &str) -> PreferencePageActions {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let mut actions = PreferencePageActions::default();
    let first = ctx.run_ui(input(1_100.0, 900.0), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_page(ui, &mut preferences_dialogs(&mut state), &mut actions);
        });
    });
    let nodes = first
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    let bounds = node_bounds(&nodes, egui::accesskit::Role::Button, label);
    let position = egui::pos2(
        ((bounds.x0 + bounds.x1) * 0.5) as f32,
        ((bounds.y0 + bounds.y1) * 0.5) as f32,
    );

    for pressed in [true, false] {
        let events = vec![
            egui::Event::PointerMoved(position),
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            },
        ];
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: input(1_100.0, 900.0).screen_rect,
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    render_page(ui, &mut preferences_dialogs(&mut state), &mut actions);
                });
            },
        );
    }
    actions
}

#[test]
fn policy_edits_queue_persist_before_live_candidates() {
    let live = ShortcutPreferences::default();
    let mut candidate = live.clone();
    candidate
        .policies_mut()
        .set_chord_timeout(ChordTimeoutPolicy::ALL[1]);
    let mut actions = PreferencePageActions::default();

    queue_policy_candidate(&mut actions, &live, candidate.clone(), false);

    assert_eq!(actions.shortcut_policy_candidate, Some(candidate));
    assert_eq!(live, ShortcutPreferences::default());
}

#[test]
fn pending_policy_publication_cannot_queue_a_second_candidate() {
    let live = ShortcutPreferences::default();
    let mut candidate = live.clone();
    candidate
        .policies_mut()
        .set_chord_timeout(ChordTimeoutPolicy::ALL[1]);
    let mut actions = PreferencePageActions::default();

    queue_policy_candidate(&mut actions, &live, candidate, true);

    assert!(actions.shortcut_policy_candidate.is_none());
}

#[test]
fn typed_context_filters_cover_every_registry_command() {
    for command in command_registry().order().iter().copied() {
        assert!(ShortcutEditorContext::ALL.contains(&editor_context(command, command_registry())));
    }
}

#[test]
fn incompatible_profile_repair_is_draft_only_and_emits_a_receipt() {
    let live: ShortcutPreferences = serde_json::from_str(
        r#"{"commands":{"save-project":{"bindings":[{"slot":"primary","platforms":["desktop"],"sequence":[{"key":"DefinitelyNotAKey"}]}]}}}"#,
    )
    .unwrap();
    assert!(!live.audit(&CommandRegistry).is_valid());
    let mut editor = ShortcutEditorState::default();
    editor.open(&live);

    repair_invalid_entries(&mut editor, command_registry());

    assert!(
        editor
            .draft
            .as_ref()
            .unwrap()
            .audit(&CommandRegistry)
            .is_valid()
    );
    assert!(
        !editor
            .original
            .as_ref()
            .unwrap()
            .audit(&CommandRegistry)
            .is_valid()
    );
    assert!(editor.dirty);
    assert!(
        editor
            .repair_receipt
            .as_deref()
            .is_some_and(|receipt| receipt.contains("save"))
    );
}

#[test]
fn shortcuts_category_is_exposed_after_review_workflows_land() {
    assert!(
        crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory::ALL
            .contains(&crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory::Shortcuts)
    );
}

#[test]
fn desktop_page_publishes_its_rows_its_actions_and_its_policy_heading() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let mut actions = PreferencePageActions::default();
    let output = ctx.run_ui(input(1_100.0, 900.0), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_page(ui, &mut preferences_dialogs(&mut state), &mut actions);
        });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Key handling")
            && node.level() == Some(4)
    }));
    for label in ["Key bindings", "Shortcut file"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Label && node.label() == Some(label)
            }),
            "missing row label {label}"
        );
    }
    for label in ["Edit shortcuts\u{2026}", "Import\u{2026}", "Export\u{2026}"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
    // The page no longer opens a table of its own, and no longer offers a
    // resolved-policy review: every row of that review read "User
    // profile", "allowed", unlocked.
    assert!(
        !nodes
            .iter()
            .any(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
    );
    assert!(!nodes.iter().any(|(_, node)| {
        node.label() == Some("View resolved policy\u{2026}")
            || node.label() == Some("Protected shortcut registry")
    }));

    let bindings = node_bounds(&nodes, egui::accesskit::Role::Label, "Key bindings");
    let portable = node_bounds(&nodes, egui::accesskit::Role::Label, "Shortcut file");
    let edit = node_bounds(
        &nodes,
        egui::accesskit::Role::Button,
        "Edit shortcuts\u{2026}",
    );
    let import = node_bounds(&nodes, egui::accesskit::Role::Button, "Import\u{2026}");
    let export = node_bounds(&nodes, egui::accesskit::Role::Button, "Export\u{2026}");

    assert_eq!(
        bindings.x0, portable.x0,
        "the two row labels share one left edge"
    );
    assert_eq!(
        edit.x0, import.x0,
        "the action on each row starts at the page's control column"
    );
    assert!(
        export.x0 > import.x1,
        "portable action controls overlap or are reversed: import={import:?}, export={export:?}"
    );
    assert_eq!(import.y0, export.y0);
    assert_eq!(import.y1, export.y1);
}

#[test]
fn portable_profile_buttons_emit_distinct_workflow_actions() {
    let imported = click_page_button("Import\u{2026}");
    assert!(imported.open_shortcut_import);
    assert!(!imported.open_shortcut_export);
    assert!(!imported.open_shortcut_editor);

    let exported = click_page_button("Export\u{2026}");
    assert!(exported.open_shortcut_export);
    assert!(!exported.open_shortcut_import);
    assert!(!exported.open_shortcut_editor);
}

#[test]
fn phone_page_keeps_the_portable_actions_on_one_reachable_row() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let mut actions = PreferencePageActions::default();
    let output = ctx.run_ui(input(390.0, 844.0), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_page(ui, &mut preferences_dialogs(&mut state), &mut actions);
        });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    let import = node_bounds(&nodes, egui::accesskit::Role::Button, "Import\u{2026}");
    let export = node_bounds(&nodes, egui::accesskit::Role::Button, "Export\u{2026}");
    assert!(import.x0 < export.x0);
    assert_eq!(import.y0, export.y0);
    assert!(import.y1 - import.y0 >= 44.0);
    // Order is asserted spatially, above: Import sits left of Export on a
    // shared row. This used to also compare their positions in the flat
    // AccessKit update, but that vector is egui's emission order, not a
    // reading order — egui 0.35 scatters the two buttons across it while
    // rendering them in exactly the same place, and this harness's update
    // carries no parent/child links to read a true reading order from.
}

#[test]
fn editor_exposes_modal_search_table_and_44_point_phone_targets() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let profile = state.model.preferences.shortcuts().clone();
    state.settings.shortcut_editor.open(&profile);
    let output = ctx.run_ui(input(390.0, 844.0), |ctx| {
        render_editor(ctx, &mut preferences_dialogs(&mut state))
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog
            && node.label() == Some("Keyboard shortcut editor")
            && node.is_modal()
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput
            && node.label() == Some("Filter keyboard commands")
    }));
    let table = node_bounds(
        &nodes,
        egui::accesskit::Role::Table,
        "Editable command bindings",
    );
    assert!(table.x1 - table.x0 >= EDITOR_NARROW_TABLE_MIN_WIDTH as f64);
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
            .count(),
        6
    );
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Row
            && node.label() == Some("Shortcut editor column headings")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Group
            && node.label() == Some("Shortcut editor controls")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Group && node.label() == Some("Shortcut editor notes")
    }));
    let button = nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button
                && node
                    .label()
                    .is_some_and(|label| label.starts_with("Record Primary binding for"))
        })
        .and_then(|(_, node)| node.bounds())
        .expect("primary capture button bounds");
    assert!(button.y1 - button.y0 >= 44.0);
}
