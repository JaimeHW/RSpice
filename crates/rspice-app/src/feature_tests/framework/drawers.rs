//! The phone's drawers and navigation sheet: the account and notification
//! entries, and keyboard focus inside a side drawer.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::ShellCommand;
use egui::{Area, Context, Id, Order, Sense, UiKind};
use rspice_app_cloud::workbench::cloud::commands::CloudCommand;
use rspice_app_workbench::workbench::docks::drawers::*;

#[derive(Clone, Copy)]
struct ModalFocusFixtureIds {
    background: Id,
    first: Id,
    disabled: Id,
    last: Id,
}

fn focus_test_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        events,
        ..Default::default()
    }
}

fn focus_test_key(shift: bool) -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Tab,
        physical_key: Some(egui::Key::Tab),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            shift,
            ..egui::Modifiers::NONE
        },
    }
}

fn run_modal_focus_fixture(
    ctx: &Context,
    input: egui::RawInput,
    request_first: bool,
) -> ModalFocusFixtureIds {
    let mut rendered = None;
    let _output = ctx.run_ui(input, |ctx| {
        let background = egui::CentralPanel::default()
            .show(ctx, |ui| ui.button("Background action"))
            .inner;

        let floor = Area::new(Id::new("drawer-focus-test.floor"))
            .kind(UiKind::Modal)
            .sense(Sense::hover())
            .order(Order::Foreground)
            .fixed_pos(egui::Pos2::ZERO);
        ctx.memory_mut(|memory| memory.set_modal_layer(floor.layer()));
        floor.show(ctx, |ui| {
            ui.allocate_exact_size(egui::vec2(800.0, 600.0), Sense::hover());
        });

        let controls = Area::new(Id::new("drawer-focus-test.controls"))
            .kind(UiKind::Modal)
            .sense(Sense::hover())
            .order(Order::Tooltip)
            .fixed_pos(egui::pos2(20.0, 20.0))
            .show(ctx, |ui| {
                let first = ui.button("First action");
                let disabled = ui.add_enabled(false, egui::Button::new("Unavailable action"));
                let last = ui.button("Last action");
                if request_first {
                    first.request_focus();
                }
                (first.id, disabled.id, last.id)
            })
            .inner;
        rendered = Some(ModalFocusFixtureIds {
            background: background.id,
            first: controls.0,
            disabled: controls.1,
            last: controls.2,
        });
    });
    rendered.expect("focus fixture must render")
}

#[test]
fn mobile_organization_entry_opens_the_canonical_account_manager() {
    let action = TOOL_ACTIONS
        .iter()
        .find(|action| action.label == "Organization & licensing")
        .expect("the account entry");
    assert_eq!(action.command, CloudCommand::AccountOrganization.key());
}

/// A phone's title bar draws no bell, so the sheet carries it: the panel
/// one tap away, and the unread count beside it in the row's meta.
#[test]
fn the_phone_sheet_reaches_notifications_and_counts_what_is_unread() {
    let action = TOOL_ACTIONS
        .iter()
        .find(|action| action.command == ShellCommand::OpenNotifications.key())
        .expect("the sheet offers the notification panel");
    assert_eq!(action.label, "Notifications");

    let ctx = Context::default();
    let mut app = RSpiceApp::test_instance();
    assert_eq!(
        card_meta(ShellCommand::OpenNotifications.key(), &app),
        None,
        "nothing unread says nothing"
    );
    app.state.workbench.toasts.info(&ctx, "Recovery copy saved");
    app.state.workbench.toasts.info(&ctx, "Library reloaded");
    assert_eq!(
        card_meta(ShellCommand::OpenNotifications.key(), &app).as_deref(),
        Some("2 unread")
    );
    assert_eq!(card_meta(ShellCommand::OpenConsole.key(), &app), None);
}

#[test]
fn side_drawer_modal_focus_skips_unavailable_controls_and_wraps_both_directions() {
    let ctx = Context::default();
    let ids = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), true);
    let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.first));

    let current =
        run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(false)]), false);
    assert_eq!(current.first, ids.first);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.last));
    assert_ne!(ctx.memory(|memory| memory.focused()), Some(ids.disabled));

    let _ = run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(false)]), false);
    let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(ids.first));

    let _ = run_modal_focus_fixture(&ctx, focus_test_input(vec![focus_test_key(true)]), false);
    let _ = run_modal_focus_fixture(&ctx, focus_test_input(Vec::new()), false);
    let focused = ctx.memory(|memory| memory.focused());
    assert_eq!(focused, Some(ids.last));
    assert_ne!(focused, Some(ids.background));
    assert_ne!(focused, Some(ids.disabled));
}
