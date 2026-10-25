//! The phone's drawers and navigation sheet: the scrim's tokens, the grid's
//! height against its real rows, the workspace entry and the tool groups, the
//! account and notification entries, and the floor on very narrow viewports.

use crate::ui::tokens::{Mode, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::docks::drawers::*;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::state::{Drawer, Workspace};
use egui::{Area, Context, Id, Order, Sense, UiKind};

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
fn drawer_scrims_match_the_mockup_dark_and_light_tokens() {
    let mut dark = Tokens::default();
    dark.mode = Mode::Dark;
    assert_eq!(
        drawer_scrim_color(&dark),
        egui::Color32::from_rgba_unmultiplied(2, 6, 8, 158)
    );

    let mut light = dark.clone();
    light.mode = Mode::Light;
    assert_eq!(
        drawer_scrim_color(&light),
        egui::Color32::from_rgba_unmultiplied(41, 46, 50, 97)
    );
}

/// Fourteen one-line rows under two heads, not taglined cards under four:
/// the one-column sheet was 1,237 px of scrolling.
#[test]
fn mobile_navigation_grid_height_tracks_its_real_rows() {
    assert_eq!(mobile_navigation_natural_height(2), 486.0);
    assert_eq!(mobile_navigation_natural_height(1), 850.0);
}

/// A workspace is called what the rest of the shell calls it, and the
/// sheet has two heads: the destinations, and everything else.
#[test]
fn the_phone_sheet_names_a_workspace_once_and_groups_the_rest_as_tools() {
    for workspace in Workspace::ALL {
        assert!(
            WORKSPACE_ACTIONS.iter().any(|action| {
                action.command == Command::OpenWorkspace(workspace).key()
                    && action.label == workspace.label()
            }),
            "{workspace:?} is offered under the name the shell gives it"
        );
    }
    assert_eq!(
        MOBILE_NAV_GROUPS.map(|(label, _)| label),
        ["Workspaces", "Tools"]
    );
}

#[test]
fn mobile_organization_entry_opens_the_canonical_account_manager() {
    let action = TOOL_ACTIONS
        .iter()
        .find(|action| action.label == "Organization & licensing")
        .expect("the account entry");
    assert_eq!(action.command, Command::AccountOrganization.key());
}

/// A phone's title bar draws no bell, so the sheet carries it: the panel
/// one tap away, and the unread count beside it in the row's meta.
#[test]
fn the_phone_sheet_reaches_notifications_and_counts_what_is_unread() {
    let action = TOOL_ACTIONS
        .iter()
        .find(|action| action.command == Command::OpenNotifications.key())
        .expect("the sheet offers the notification panel");
    assert_eq!(action.label, "Notifications");

    let ctx = Context::default();
    let mut app = RSpiceApp::test_instance();
    assert_eq!(
        card_meta(Command::OpenNotifications.key(), &app),
        None,
        "nothing unread says nothing"
    );
    app.state.workbench.toasts.info(&ctx, "Recovery copy saved");
    app.state.workbench.toasts.info(&ctx, "Library reloaded");
    assert_eq!(
        card_meta(Command::OpenNotifications.key(), &app).as_deref(),
        Some("2 unread")
    );
    assert_eq!(card_meta(Command::OpenConsole.key(), &app), None);
}

#[test]
fn mobile_navigation_never_overflows_an_exceptionally_narrow_viewport() {
    assert_eq!(mobile_navigation_width(298.0), 280.0);
    assert_eq!(mobile_navigation_width(280.0), 262.0);
    assert_eq!(mobile_navigation_width(12.0), 0.0);
}

#[test]
fn drawer_close_targets_follow_pointer_and_breakpoint_cascade() {
    let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchSession::default());
    assert_eq!(drawer_close_geometry(compact), (44.0, 5.0));

    let intermediate = LayoutSpec::resolve(834.0, 1_112.0, &WorkbenchSession::default());
    assert_eq!(drawer_close_geometry(intermediate), (34.0, 5.0));

    let coarse =
        LayoutSpec::resolve_with_pointer(834.0, 1_112.0, true, &WorkbenchSession::default());
    assert_eq!(drawer_close_geometry(coarse), (44.0, 0.0));
}

#[test]
fn intermediate_inspector_drawer_uses_only_its_inner_edge() {
    let state = WorkbenchSession::default();
    let compact = LayoutSpec::resolve(390.0, 844.0, &state);
    let intermediate = LayoutSpec::resolve(834.0, 1_112.0, &state);

    assert_eq!(drawer_border(compact, Drawer::Inspector), DrawerBorder::Box);
    assert_eq!(
        drawer_border(intermediate, Drawer::Inspector),
        DrawerBorder::InnerLeft
    );
    assert_eq!(
        drawer_border(intermediate, Drawer::Navigator),
        DrawerBorder::Box
    );
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
