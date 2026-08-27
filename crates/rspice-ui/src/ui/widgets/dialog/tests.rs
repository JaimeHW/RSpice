//! Behavioral and responsive-layout tests for the shared dialog shell.
//!
//! These cases pin focus, dismissal, footer, sizing, and accessibility
//! contracts that individual workflow dialogs rely on.

use super::*;

const TEST_TITLE: &str = "Modal behavior";
const TEST_DESCRIPTION: &str = "Review this modal operation and its available actions.";

fn dialog_id() -> Id {
    Id::new(("rspice.dialog", TEST_TITLE))
}

fn dialog_focus_id() -> Id {
    dialog_id().with("move")
}

fn underlying_id() -> Id {
    Id::new("dialog-test-underlying-editor")
}

fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, 800.0))),
        events,
        ..Default::default()
    }
}

fn key_event(key: Key, modifiers: Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn focus_underlying_editor(ctx: &Context, underlying: &mut String) {
    let _output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                egui::TextEdit::singleline(underlying)
                    .id(underlying_id())
                    .desired_width(240.0),
            )
            .request_focus();
        });
    });
}

fn run_dialog(
    ctx: &Context,
    input: egui::RawInput,
    underlying: &mut String,
    primary_on_enter: bool,
    mut body: impl FnMut(&mut Ui),
) -> (DialogChoice, egui::FullOutput) {
    let mut choice = DialogChoice::None;
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.add(
                egui::TextEdit::singleline(underlying)
                    .id(underlying_id())
                    .desired_width(240.0),
            );
        });
        choice = Dialog::new("TEST", TEST_TITLE, "Accept")
            .description(TEST_DESCRIPTION)
            .primary_on_enter(primary_on_enter)
            .show(ctx, |ui| body(ui));
    });
    (choice, output)
}

fn assert_focus_is_on_dialog_layer(ctx: &Context) {
    let focused = ctx
        .memory(|memory| memory.focused())
        .expect("dialog should own keyboard focus");
    assert_ne!(focused, underlying_id());
    let response = ctx
        .read_response(focused)
        .expect("focused dialog widget should still exist");
    assert_eq!(
        response.layer_id,
        egui::LayerId::new(Order::Foreground, dialog_id())
    );
}

#[test]
fn capability_review_phone_layout_is_full_viewport() {
    let screen = Rect::from_min_size(egui::pos2(7.0, 11.0), vec2(390.0, 844.0));
    let layout = DialogLayout::resolve(DialogSize::CapabilityReview, screen, None);

    assert_eq!(layout.surface_rect, screen);
    assert_eq!(layout.radius, 0.0);
    assert!(layout.app_background);
}

#[test]
fn authored_initial_height_avoids_the_content_dialog_first_frame_jump() {
    let dialog = Dialog::new("Test", TEST_TITLE, "Accept").initial_height(370.0);
    assert_eq!(dialog.initial_height, Some(370.0));

    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 720.0));
    let layout = DialogLayout::resolve(DialogSize::Transaction, screen, dialog.initial_height);
    assert_eq!(layout.surface_rect.size(), vec2(760.0, 370.0));
    assert_eq!(layout.surface_rect.center(), screen.center());
}

#[test]
fn authored_fixed_height_is_stable_and_rejects_invalid_values() {
    let fixed = Dialog::new("Test", TEST_TITLE, "Accept").fixed_height(612.0);
    assert_eq!(fixed.fixed_height, Some(612.0));

    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::WideWorkflow, screen, fixed.fixed_height);
    assert_eq!(layout.surface_rect.size(), vec2(980.0, 612.0));
    assert_eq!(layout.surface_rect.center(), screen.center());
    assert!(layout.fills_surface_height(fixed.fixed_height));

    let clamped = Dialog::new("Test", TEST_TITLE, "Accept").fixed_height(0.0);
    assert_eq!(clamped.fixed_height, Some(1.0));
    let ignored = Dialog::new("Test", TEST_TITLE, "Accept").fixed_height(f32::NAN);
    assert_eq!(ignored.fixed_height, None);
}

#[test]
fn capability_review_desktop_layout_uses_mockup_caps_and_is_centered() {
    let screen = Rect::from_min_size(egui::pos2(20.0, 30.0), vec2(1_440.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::CapabilityReview, screen, None);

    assert_eq!(layout.surface_rect.width(), 1_040.0);
    assert_eq!(layout.surface_rect.height(), 760.0);
    assert_eq!(layout.surface_rect.center(), screen.center());
    assert_eq!(layout.surface_rect.left(), 220.0);
    assert_eq!(layout.surface_rect.top(), 100.0);
}

#[test]
fn transaction_short_narrow_viewport_is_full_viewport() {
    let screen = Rect::from_min_size(egui::pos2(3.0, 5.0), vec2(390.0, 300.0));
    let layout = DialogLayout::resolve(DialogSize::Transaction, screen, None);

    assert_eq!(layout.surface_rect, screen);
}

#[test]
fn transaction_tablet_retains_mockup_gutters_until_the_phone_breakpoint() {
    let screen = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(800.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::Transaction, screen, Some(500.0));

    assert_eq!(layout.surface_rect.size(), vec2(760.0, 500.0));
    assert_eq!(layout.surface_rect.center(), screen.center());
    assert_eq!(layout.surface_rect.left(), 30.0);
    assert_eq!(layout.radius, 4.0);
    assert!(!layout.narrow);

    let phone_breakpoint = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(560.0, 900.0));
    let phone = DialogLayout::resolve(DialogSize::Transaction, phone_breakpoint, Some(500.0));
    assert_eq!(phone.surface_rect, phone_breakpoint);
    assert_eq!(phone.radius, 0.0);
    assert!(phone.narrow);
}

#[test]
fn manager_phone_layout_uses_the_mockup_four_point_gutter() {
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0));
    let layout = DialogLayout::resolve(DialogSize::Manager, screen, None);

    assert_eq!(layout.surface_rect.left(), 4.0);
    assert_eq!(layout.surface_rect.width(), 382.0);
    assert_eq!(layout.surface_rect.top(), 4.0);
    assert_eq!(layout.surface_rect.height(), 836.0);
    assert!(layout.fill_height);
    assert!(!layout.app_background);
}

#[test]
fn manager_and_wide_workflow_desktop_geometry_match_the_mockup() {
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
    let manager = DialogLayout::resolve(DialogSize::Manager, screen, None);
    let workflow = DialogLayout::resolve(DialogSize::WideWorkflow, screen, Some(612.0));

    assert_eq!(manager.surface_rect.size(), vec2(760.0, 530.0));
    assert_eq!(manager.surface_rect.center(), screen.center());
    assert_eq!(manager.radius, 8.0);
    assert_eq!(workflow.surface_rect.size(), vec2(980.0, 612.0));
    assert_eq!(workflow.surface_rect.center(), screen.center());
    assert_eq!(workflow.radius, 4.0);
    assert!(!workflow.fill_height);
}

#[test]
fn drawing_sheet_workflow_uses_the_approved_supporting_surface_width() {
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::DrawingSheetWorkflow, screen, Some(650.0));

    assert_eq!(layout.surface_rect.size(), vec2(1_160.0, 650.0));
    assert_eq!(layout.surface_rect.center(), screen.center());
    assert_eq!(layout.radius, 4.0);
}

#[test]
fn account_manager_geometry_matches_its_mockup_width_and_breakpoint() {
    let tall_desktop = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 1_080.0));
    let tall = DialogLayout::resolve(DialogSize::AccountManager, tall_desktop, None);
    assert_eq!(tall.surface_rect.size(), vec2(920.0, 820.0));
    assert_eq!(tall.surface_rect.center(), tall_desktop.center());

    let desktop_screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, 800.0));
    let desktop = DialogLayout::resolve(DialogSize::AccountManager, desktop_screen, Some(640.0));
    assert_eq!(desktop.surface_rect.size(), vec2(920.0, 640.0));
    assert_eq!(desktop.surface_rect.left(), 40.0);
    assert_eq!(desktop.surface_rect.center(), desktop_screen.center());

    let breakpoint = Rect::from_min_size(egui::pos2(5.0, 7.0), vec2(820.0, 900.0));
    let narrow = DialogLayout::resolve(DialogSize::AccountManager, breakpoint, Some(500.0));
    assert_eq!(narrow.surface_rect, breakpoint);
    assert_eq!(narrow.radius, 0.0);
    assert!(narrow.narrow);
}

#[test]
fn jobs_manager_geometry_matches_the_mockup_on_desktop_and_phone() {
    let desktop_screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
    let desktop = DialogLayout::resolve(DialogSize::JobsManager, desktop_screen, None);
    assert_eq!(desktop.surface_rect.size(), vec2(1_120.0, 680.0));
    assert_eq!(desktop.surface_rect.center(), desktop_screen.center());
    assert_eq!(desktop.radius, 4.0);
    assert!(desktop.fill_height);
    assert!(desktop.app_background);

    let phone_screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0));
    let phone = DialogLayout::resolve(DialogSize::JobsManager, phone_screen, None);
    assert_eq!(phone.surface_rect.min, egui::pos2(4.0, 4.0));
    assert_eq!(phone.surface_rect.size(), vec2(382.0, 836.0));
    assert_eq!(phone.radius, 4.0);
    assert!(phone.narrow);
}

#[test]
fn component_editor_preserves_mockup_gutters_and_height_cap_at_breakpoint() {
    let desktop_screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
    let desktop = DialogLayout::resolve(DialogSize::ComponentEditor, desktop_screen, None);
    assert_eq!(desktop.surface_rect.size(), vec2(880.0, 680.0));
    assert_eq!(desktop.surface_rect.center(), desktop_screen.center());
    assert!(!desktop.narrow);
    assert!(!desktop.app_background);

    let narrow_screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(740.0, 800.0));
    let narrow = DialogLayout::resolve(DialogSize::ComponentEditor, narrow_screen, None);
    assert_eq!(narrow.surface_rect.size(), vec2(692.0, 680.0));
    assert_eq!(narrow.surface_rect.center(), narrow_screen.center());
    assert!(narrow.narrow);
    assert_eq!(narrow.radius, 8.0);
}

#[test]
fn analysis_catalog_uses_mockup_top_anchor_caps_and_phone_gutter() {
    let desktop_screen = Rect::from_min_size(egui::pos2(20.0, 30.0), vec2(1_440.0, 900.0));
    let desktop = DialogLayout::resolve(DialogSize::AnalysisCatalog, desktop_screen, None);
    assert_eq!(desktop.surface_rect.size(), vec2(1_180.0, 780.0));
    assert_eq!(desktop.surface_rect.left(), 150.0);
    assert_eq!(desktop.surface_rect.top(), 42.0);
    assert_eq!(desktop.radius, 8.0);
    assert!(desktop.fill_height);
    assert!(!desktop.app_background);

    let phone_screen = Rect::from_min_size(egui::pos2(7.0, 11.0), vec2(390.0, 844.0));
    let phone = DialogLayout::resolve(DialogSize::AnalysisCatalog, phone_screen, None);
    assert_eq!(phone.surface_rect.min, egui::pos2(11.0, 23.0));
    assert_eq!(phone.surface_rect.size(), vec2(382.0, 780.0));
    assert_eq!(phone.radius, 8.0);
}

#[test]
fn wide_workflow_becomes_edge_to_edge_at_the_mockup_breakpoint() {
    let screen = Rect::from_min_size(egui::pos2(5.0, 7.0), vec2(820.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::WideWorkflow, screen, Some(500.0));

    assert_eq!(layout.surface_rect, screen);
    assert_eq!(layout.radius, 0.0);
}

#[test]
fn simulation_workflow_becomes_edge_to_edge_at_the_mockup_breakpoint() {
    let screen = Rect::from_min_size(egui::pos2(5.0, 7.0), vec2(820.0, 900.0));
    let layout = DialogLayout::resolve(DialogSize::SimulationWorkflow, screen, Some(500.0));

    assert_eq!(layout.surface_rect, screen);
    assert_eq!(layout.radius, 0.0);
    assert!(layout.narrow);
    assert!(layout.fill_height);
}

#[test]
fn specialist_browser_uses_desktop_cap_and_full_tablet_phone_viewport() {
    let desktop_screen = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(1_440.0, 900.0));
    let desktop = DialogLayout::resolve(DialogSize::SpecialistToolBrowser, desktop_screen, None);
    assert_eq!(desktop.surface_rect.size(), vec2(760.0, 760.0));
    assert_eq!(desktop.surface_rect.center(), desktop_screen.center());
    assert_eq!(desktop.radius, 4.0);
    assert!(!desktop.fill_height);

    for size in [vec2(820.0, 900.0), vec2(390.0, 844.0)] {
        let screen = Rect::from_min_size(egui::pos2(7.0, 11.0), size);
        let compact = DialogLayout::resolve(DialogSize::SpecialistToolBrowser, screen, None);
        assert_eq!(compact.surface_rect, screen);
        assert_eq!(compact.radius, 0.0);
        assert!(compact.narrow);
        assert!(compact.fill_height);
    }
}

#[test]
fn flush_body_is_opt_in() {
    let padded = Dialog::new("Test", TEST_TITLE, "Accept");
    assert!(!padded.flush_body);
    let flush = Dialog::new("Test", TEST_TITLE, "Accept").flush_body();
    assert!(flush.flush_body);
}

#[test]
fn catalog_body_and_note_footer_are_explicit_opt_ins() {
    let dialog = Dialog::new("Test", TEST_TITLE, "Close")
        .manual_body_scroll()
        .note_only_footer()
        .hint("Catalog classification note");
    assert!(dialog.manual_body_scroll);
    assert!(dialog.note_only_footer);
    assert_eq!(dialog.footer_height(false, false, 760.0), 48.0);
}

#[test]
fn retained_cancel_focus_is_an_explicit_one_pass_contract() {
    let retained = Dialog::new("Test", TEST_TITLE, "Accept")
        .ghost("Discard changes")
        .retain_on_cancel_focus(DialogInitialFocus::Ghost);
    assert_eq!(
        retained.retained_cancel_focus,
        Some(DialogInitialFocus::Ghost)
    );
    assert!(
        Dialog::new("Test", TEST_TITLE, "Accept")
            .retained_cancel_focus
            .is_none()
    );
}

#[test]
fn retained_cancel_focus_moves_escape_to_discard_then_confirm_restores_workspace() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut underlying = String::new();
    focus_underlying_editor(&ctx, &mut underlying);

    let mut choice = DialogChoice::None;
    let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.add(
                egui::TextEdit::singleline(&mut underlying)
                    .id(underlying_id())
                    .desired_width(240.0),
            );
        });
        choice = Dialog::new("TEST", TEST_TITLE, "Accept")
            .description(TEST_DESCRIPTION)
            .ghost("Discard changes")
            .retain_on_cancel_focus(DialogInitialFocus::Ghost)
            .show(ctx, |ui| {
                ui.label("Dirty dialog");
            });
    });

    let output = ctx.run_ui(
        raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ui.add(
                    egui::TextEdit::singleline(&mut underlying)
                        .id(underlying_id())
                        .desired_width(240.0),
                );
            });
            choice = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .ghost("Discard changes")
                .retain_on_cancel_focus(DialogInitialFocus::Ghost)
                .show(ctx, |ui| {
                    ui.label("Dirty dialog");
                });
        },
    );
    assert_eq!(choice, DialogChoice::Cancelled);
    let focused = ctx
        .memory(|memory| memory.focused())
        .expect("retained dialog keeps focus");
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    let focused_node = nodes
        .iter()
        .find(|(id, _)| *id == focused.accesskit_id())
        .map(|(_, node)| node)
        .expect("focused discard node is published");
    assert_eq!(focused_node.role(), egui::accesskit::Role::Button);
    assert_eq!(focused_node.label(), Some("Discard changes"));

    let _ = ctx.run_ui(
        raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ui.add(
                    egui::TextEdit::singleline(&mut underlying)
                        .id(underlying_id())
                        .desired_width(240.0),
                );
            });
            choice = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .ghost("Discard changes")
                .show(ctx, |ui| {
                    ui.label("Dirty dialog");
                });
        },
    );
    assert_eq!(choice, DialogChoice::Cancelled);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
}

#[test]
fn long_footer_actions_stack_inside_a_phone_surface() {
    let dialog = Dialog::new("Test", TEST_TITLE, "Apply governed configuration changes")
        .secondary("Review compatibility report")
        .ghost("Discard pending changes")
        .hint("3 validated records");

    assert!(dialog.footer_stacks(390.0));
    assert!(dialog.footer_height(false, true, 390.0) > 48.0);
    assert!(!dialog.footer_stacks(1_440.0));
    assert_eq!(dialog.footer_height(false, false, 1_440.0), 48.0);
}

#[test]
fn catalog_note_footer_uses_the_complete_1280_viewport_surface_width() {
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 720.0));
    let layout = DialogLayout::resolve(DialogSize::AnalysisCatalog, screen, None);
    let footer = dialog_note_footer_rect(
        layout.surface_rect.left(),
        layout.surface_rect.bottom() - 48.0,
        layout.surface_rect.width(),
    );

    assert_eq!(
        layout.surface_rect,
        Rect::from_min_max(egui::pos2(50.0, 12.0), egui::pos2(1_230.0, 708.0),)
    );
    assert_eq!(footer.left(), layout.surface_rect.left());
    assert_eq!(footer.right(), layout.surface_rect.right());
    assert_eq!(footer.height(), 48.0);
}

#[test]
fn action_footer_is_clamped_to_the_phone_surface() {
    let footer = dialog_footer_rect(0.0, 796.0, 390.0, 48.0);

    assert_eq!(footer.left(), 0.0);
    assert_eq!(footer.right(), 390.0);
    assert_eq!(footer.height(), 48.0);
}

#[test]
fn transaction_state_is_absent_when_idle_and_exposes_an_assertive_strip_on_error() {
    let idle = Dialog::new("Test", TEST_TITLE, "Accept");
    assert!(idle.transaction_state.is_none());

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new("Test", TEST_TITLE, "Accept")
            .transaction_state(
                DialogTransactionTone::Error,
                "Unsaved dialog changes",
                "Choose Discard changes again to close.",
            )
            .show(ctx, |ui| {
                ui.label("Dialog body");
            });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Alert
            && node.label() == Some("Unsaved dialog changes")
            && node.description() == Some("Choose Discard changes again to close.")
    }));
}

#[test]
fn close_only_workflow_hides_redundant_footer() {
    let dialog = Dialog::new("Test", TEST_TITLE, "Close").size(DialogSize::CapabilityReview);

    assert_eq!(dialog.footer_height(true, false, 1_040.0), 0.0);
    assert!(dialog.hides_close_only_footer(true));
}

#[test]
fn capability_review_phone_footer_stays_horizontal_and_compact() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(390.0, 844.0))),
        ..Default::default()
    };

    let output = ctx.run_ui(input, |ctx| {
        let _ = Dialog::new(
            "A deliberately long governed workflow",
            TEST_TITLE,
            "Accept",
        )
        .size(DialogSize::CapabilityReview)
        .ghost("Cancel")
        .secondary("Later")
        .show(ctx, |ui| {
            ui.label("Dialog body");
        });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    let bounds = ["Cancel", "Later", "Accept"].map(|label| {
        nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| panic!("missing bounds for {label}"))
    });
    for bounds in &bounds {
        assert_eq!(bounds.y1 - bounds.y0, TOUCH_TARGET_SIDE as f64);
    }
    assert_eq!(bounds[0].y0, bounds[1].y0);
    assert_eq!(bounds[1].y0, bounds[2].y0);
    assert!(bounds[0].x0 < bounds[1].x0);
    assert!(bounds[1].x0 < bounds[2].x0);
    assert!(bounds[2].x1 >= 377.0, "actions must be trailing-grouped");

    let close_bounds = nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Close (Esc)")
        })
        .and_then(|(_, node)| node.bounds())
        .expect("missing workflow close control bounds");
    assert_eq!(close_bounds.x1 - close_bounds.x0, TOUCH_TARGET_SIDE as f64);
    assert_eq!(close_bounds.y1 - close_bounds.y0, TOUCH_TARGET_SIDE as f64);
}

#[test]
fn capability_review_desktop_close_control_uses_mockup_icon_button_target() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0))),
        ..Default::default()
    };

    let output = ctx.run_ui(input, |ctx| {
        let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
            .size(DialogSize::CapabilityReview)
            .show(ctx, |ui| {
                ui.label("Dialog body");
            });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    let bounds = nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Close (Esc)")
        })
        .and_then(|(_, node)| node.bounds())
        .expect("missing workflow close control bounds");

    assert_eq!(bounds.x1 - bounds.x0, DIALOG_CLOSE_TARGET_WIDTH as f64);
    assert_eq!(bounds.y1 - bounds.y0, DIALOG_CLOSE_TARGET_HEIGHT as f64);
}

#[test]
fn dialog_publishes_modal_accessibility_semantics() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut underlying = String::new();

    let (_, output) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
        let _ = ui.label("Dialog body");
    });

    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog
            && node.label() == Some(TEST_TITLE)
            && node.description() == Some(TEST_DESCRIPTION)
            && node.is_modal()
    }));
}

#[test]
fn requested_body_control_receives_initial_focus_once() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut preferred_id = None;

    let _output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
            .initial_focus(DialogInitialFocus::BodyControl)
            .show_with_initial_body_focus(ctx, |ui| {
                let response = ui.button("Preferred body control");
                preferred_id = Some(response.id);
                Some(response.id)
            });
    });

    assert_eq!(ctx.memory(|memory| memory.focused()), preferred_id);
}

#[test]
fn unavailable_initial_focus_target_falls_back_to_modal_container() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);

    let _output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
            .primary_enabled(false)
            .initial_focus(DialogInitialFocus::Primary)
            .show(ctx, |ui| {
                ui.label("Dialog body");
            });
    });

    assert_eq!(
        ctx.memory(|memory| memory.focused()),
        Some(dialog_focus_id())
    );
}

#[test]
fn modal_focus_is_trapped_and_restored_without_leaking_text() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut underlying = "baseline".to_owned();
    focus_underlying_editor(&ctx, &mut underlying);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));

    let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
        let _ = ui.button("Body action");
    });
    assert_eq!(
        ctx.memory(|memory| memory.focused()),
        Some(dialog_focus_id())
    );

    let _ = run_dialog(
        &ctx,
        raw_input(vec![egui::Event::Text("blocked".to_owned())]),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.button("Body action");
        },
    );
    assert_eq!(underlying, "baseline");

    let _ = run_dialog(
        &ctx,
        raw_input(vec![key_event(Key::Tab, Modifiers::NONE)]),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.button("Body action");
        },
    );
    assert_focus_is_on_dialog_layer(&ctx);

    let _ = run_dialog(
        &ctx,
        raw_input(vec![key_event(
            Key::Tab,
            Modifiers {
                shift: true,
                ..Modifiers::NONE
            },
        )]),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.button("Body action");
        },
    );
    assert_focus_is_on_dialog_layer(&ctx);

    let (choice, _) = run_dialog(
        &ctx,
        raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.button("Body action");
        },
    );
    assert_eq!(choice, DialogChoice::Cancelled);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
}

#[test]
fn focused_body_button_owns_enter_instead_of_primary() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut underlying = String::new();
    let mut activated = false;

    let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
        ui.button("Body action").request_focus();
    });
    let (choice, _) = run_dialog(
        &ctx,
        raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
        &mut underlying,
        true,
        |ui| {
            let response = ui.button("Body action");
            if response.clicked() {
                activated = true;
            }
            response.request_focus();
        },
    );

    assert!(activated);
    assert_eq!(choice, DialogChoice::None);
}

#[test]
fn body_close_request_cancels_and_restores_prior_focus() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut underlying = String::new();
    focus_underlying_editor(&ctx, &mut underlying);

    let (choice, _) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
        ui.close_kind(UiKind::Modal)
    });

    assert_eq!(choice, DialogChoice::Cancelled);
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
}

#[test]
fn enter_submits_by_default_but_multiline_opt_out_keeps_newlines() {
    let submit_ctx = Context::default();
    crate::ui::Theme::default().apply(&submit_ctx);
    let mut underlying = String::new();
    let _ = run_dialog(
        &submit_ctx,
        raw_input(Vec::new()),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.label("Ready");
        },
    );
    let (choice, _) = run_dialog(
        &submit_ctx,
        raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
        &mut underlying,
        true,
        |ui| {
            let _ = ui.label("Ready");
        },
    );
    assert_eq!(choice, DialogChoice::Primary);
    assert_eq!(submit_ctx.memory(|memory| memory.focused()), None);

    let multiline_ctx = Context::default();
    crate::ui::Theme::default().apply(&multiline_ctx);
    let mut underlying = String::new();
    let mut body_text = String::new();
    let _ = run_dialog(
        &multiline_ctx,
        raw_input(Vec::new()),
        &mut underlying,
        false,
        |ui| {
            ui.add(egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")))
                .request_focus();
        },
    );
    let (choice, _) = run_dialog(
        &multiline_ctx,
        raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
        &mut underlying,
        false,
        |ui| {
            ui.add(egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")))
                .request_focus();
        },
    );

    assert_eq!(choice, DialogChoice::None);
    assert_eq!(body_text, "\n");
}

fn collect_rust_sources(directory: &std::path::Path, sources: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read source directory") {
        let path = entry.expect("read source entry").path();
        if path.is_dir() {
            collect_rust_sources(&path, sources);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            sources.push(path);
        }
    }
}

#[test]
fn every_production_dialog_callsite_supplies_a_description() {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources = Vec::new();
    collect_rust_sources(&source_root, &mut sources);
    sources.sort();

    let mut audited = 0;
    let mut missing = Vec::new();
    for path in sources {
        let relative = path
            .strip_prefix(&source_root)
            .expect("source beneath crate root")
            .to_string_lossy()
            .replace('\\', "/");
        // The primitive contains deliberately partial construction tests;
        // feature-availability descriptions have their own route-specific
        // source contract because their purpose text depends on the route.
        // Extracted test modules are also non-production sources and may
        // intentionally exercise incomplete builder chains.
        if relative.ends_with("/tests.rs")
            || relative.contains("/tests/")
            || matches!(
                relative.as_str(),
                "ui/widgets/dialog.rs" | "workbench/feature_availability.rs"
            )
        {
            continue;
        }

        let source = std::fs::read_to_string(&path).expect("read Rust source");
        let production_source = source.split("\n#[cfg(test)]").next().unwrap_or(&source);
        for (offset, _) in production_source.match_indices("Dialog::new(") {
            if offset > 0
                && (production_source.as_bytes()[offset - 1].is_ascii_alphanumeric()
                    || production_source.as_bytes()[offset - 1] == b'_')
            {
                // Exclude other types whose names end in `Dialog`, such
                // as native `FileDialog` and persisted dialog-state data.
                continue;
            }
            audited += 1;
            let tail = &production_source[offset..];
            let chain_end = [
                tail.find(".show("),
                tail.find(".show_with_initial_body_focus("),
                tail.find(';'),
            ]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(tail.len());
            if !tail[..chain_end].contains(".description(") {
                let line = production_source[..offset]
                    .bytes()
                    .filter(|byte| *byte == b'\n')
                    .count()
                    + 1;
                missing.push(format!("{relative}:{line}"));
            }
        }
    }

    assert!(audited > 0, "source audit did not find a production dialog");
    assert!(
        missing.is_empty(),
        "production dialogs must publish explicit purpose text:\n{}",
        missing.join("\n")
    );
}

/// Render one `flush_body` dialog until its surface height settles, and return
/// the region the body laid its content out in, the surface's content box —
/// the resolved surface rect less its border — and the footer's height.
fn settled_flush_body_geometry(size: DialogSize, rows: usize) -> (Rect, Rect, f32) {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_440.0, 900.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let dialog = || {
        Dialog::new("TEST", TEST_TITLE, "Accept")
            .description(TEST_DESCRIPTION)
            .size(size)
            .flush_body()
    };
    let mut viewport = Rect::ZERO;
    // A content-height surface is laid out against the height its previous
    // pass measured, so the settled geometry is the second pass's.
    for _ in 0..2 {
        let _ = ctx.run_ui(input.clone(), |ctx| {
            let _ = dialog().show(ctx, |ui| {
                viewport = ui.max_rect();
                for row in 0..rows {
                    ui.label(format!("row {row}"));
                }
            });
        });
    }
    let measured =
        ctx.data(|data| data.get_temp::<f32>(dialog_id().with(("measured-surface-height", false))));
    let content = DialogLayout::resolve(size, screen, measured)
        .surface_rect
        .shrink(SURFACE_BORDER_WIDTH);
    (
        viewport,
        content,
        dialog().footer_height(false, false, content.width()),
    )
}

/// The body is given exactly what the header and the footer leave of the
/// surface's content box, on both a content-height and a filled surface.
///
/// The resolved surface rect measures the outer edge, border included, so a
/// body solved against that rect was handed exactly the border more room than
/// the surface has. The overrun is the same for a body of three rows as for one
/// of forty — it is the surface's arithmetic, not any one row's height — and
/// the footer, painted after the body, then covered the last points of the
/// body's final row.
#[test]
fn a_flush_body_is_given_exactly_the_room_its_surface_leaves_it() {
    for size in [DialogSize::WideWorkflow, DialogSize::Manager] {
        // Forty rows overflow either surface, so the viewport rather than the
        // content decides where the body ends and the footer begins.
        let (viewport, content, footer_height) = settled_flush_body_geometry(size, 40);
        assert_eq!(
            viewport.bottom() + footer_height,
            content.bottom(),
            "{size:?}: the body viewport must end where the footer begins, and \
             the footer on the surface's inner bottom edge"
        );
    }

    // Across the content box less the scrollbar's gutter, which the body
    // spends whether or not a bar is showing. A body short enough to need no
    // bar is laid out in exactly the width one long enough to need one gets,
    // so no body ever re-wraps because a bar arrived.
    let (short, content, _) = settled_flush_body_geometry(DialogSize::WideWorkflow, 3);
    let (overflowing, _, _) = settled_flush_body_geometry(DialogSize::WideWorkflow, 40);
    assert_eq!(
        short.x_range(),
        overflowing.x_range(),
        "the body's width must not answer to whether its content overflows"
    );
    assert_eq!(
        short.left(),
        content.left(),
        "the body starts on the surface's content box"
    );
    assert_eq!(
        short.right(),
        content.right() - body_scrollbar_gutter(),
        "the body ends a scrollbar gutter short of the surface's content box"
    );
}

/// The width the body withholds for the scrollbar's track, under the theme
/// every dialog is rendered with.
fn body_scrollbar_gutter() -> f32 {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let gutter = ctx.global_style().spacing.scroll.allocated_width();
    assert!(
        gutter > 0.0,
        "the solid scrollbar style this theme selects allocates a track"
    );
    gutter
}

/// A content-height dialog reaches its content in one re-measure, and then
/// nothing on it moves again.
///
/// The body's height is a function of its width, and its width was a function
/// of whether the scroll area was showing a bar — which it does when the body
/// is too tall for the surface. The two were solving each other a point at a
/// time. A body that outgrew the surface its dialog had already settled on took
/// a bar, the bar narrowed the body, the narrower body wrapped taller still,
/// and the surface measured from it crept upward for a dozen frames while the
/// bar's own reveal animated the width out from under it.
///
/// Withholding the bar's gutter from the body whether or not a bar shows takes
/// the width out of that loop, so the height the surface solves for is the
/// height its next pass lays out.
#[test]
fn a_content_height_dialog_settles_in_one_pass_and_stops_moving() {
    /// Render one pass and report where the body was laid out and what height
    /// the surface resolved for the pass after it.
    fn pass(ctx: &Context, rows: usize) -> (Rect, f32) {
        let mut body = Rect::ZERO;
        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .size(DialogSize::WideWorkflow)
                .show(ctx, |ui| {
                    body = ui.max_rect();
                    for row in 0..rows {
                        ui.label(format!("row {row}"));
                    }
                });
        });
        let height = ctx
            .data(|data| data.get_temp::<f32>(dialog_id().with(("measured-surface-height", false))))
            .expect("a content-height dialog measures the surface it resolved");
        (body, height)
    }

    const SETTLED_ROWS: usize = 3;
    // Enough rows to overflow a surface settled on three, and few enough that
    // the surface they ask for still clears its ceiling — a surface pinned
    // against the clamp cannot show whether the arithmetic stopped it.
    const GROWN_ROWS: usize = 18;

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);

    // The crawl needs a surface that has already come down onto its content,
    // and content that then outgrows it.
    for _ in 0..3 {
        let _ = pass(&ctx, SETTLED_ROWS);
    }
    let (settled_body, settled_height) = pass(&ctx, SETTLED_ROWS);

    let grown: Vec<(Rect, f32)> = (0..10).map(|_| pass(&ctx, GROWN_ROWS)).collect();
    let ceiling = DialogSize::WideWorkflow.spec().max_height;
    assert!(
        settled_height < grown[0].1 && grown[0].1 < ceiling,
        "the grown body must ask for more room than the settled surface had \
         ({settled_height}) and less than the {ceiling}-point ceiling, not {}",
        grown[0].1
    );

    // The first pass is laid out against the height the short body settled on,
    // so its body rect is the stale one. Every pass from the second on is laid
    // out against the height this one resolved, and is identical to it.
    let (expected_body, expected_height) = grown[1];
    for (index, (body, height)) in grown.iter().enumerate().skip(1) {
        assert_eq!(
            (*body, *height),
            (expected_body, expected_height),
            "pass {} moved the dialog after it had settled",
            index + 1
        );
    }

    // And the body's width never answered to any of it — not to the short
    // content, not to the grown content, and not to the bar's reveal.
    for (index, body) in std::iter::once(settled_body)
        .chain(grown.iter().map(|(body, _)| *body))
        .enumerate()
    {
        assert_eq!(
            body.x_range(),
            settled_body.x_range(),
            "pass {index} laid the body out at a width that answered to its \
             content's height"
        );
    }
}

/// A dialog that arrives at a surface one row short of its content paints its
/// final row — rendered and read back as pixels, the way the defect was
/// originally seen on a design-review render.
///
/// A body that overflows its surface spends exactly the room the surface
/// offers, so the measured stack always equalled the height that was offered:
/// every too-short height was a fixed point of the re-measure loop. A
/// content-height dialog that arrived at one — an authored seed a row short of
/// the content, or content grown a row after the surface had settled — kept
/// its final row below the fold for good, and the footer, painted after the
/// body, covered what still showed of it. The measurement now adds back what
/// the body could not show, so a too-short surface resolves to its content's
/// height instead of resting wherever it happened to be.
///
/// The bad fixed point is entered deliberately here: the dialog is seeded one
/// row short through `initial_height` and rendered through the offscreen
/// rasterizer at its default pass count. The pixels then assert both halves —
/// the loop leaves the seeded height, and it has done so by the pass a review
/// render reads.
///
/// The modal fades in, and the rasterizer's few passes leave that fade
/// unfinished, so the authored colour is not what lands on the canvas. The
/// ink is therefore asserted as identity: the final row must read back
/// pixel-for-pixel as the first row does, and both must be the red the rows
/// were painted in — dominant in its red channel however far the fade has
/// come — rather than the grey of the surface or footer fill that covers a
/// clipped row.
#[test]
fn a_dialog_seeded_a_row_short_of_its_content_paints_its_final_row() {
    const ROWS: usize = 8;
    const ROW_HEIGHT: f32 = 24.0;
    const ROW_INK: egui::Color32 = egui::Color32::from_rgb(220, 40, 40);

    // Every row is a filled rect rather than text: the rasterizer's own
    // header warns that glyph ink is sampled, while a solid rect reads back
    // uniformly. Returns the first and final row rects.
    let paint_rows = |ui: &mut Ui| -> (Rect, Rect) {
        let mut first = Rect::NOTHING;
        let mut last = Rect::NOTHING;
        for row in 0..ROWS {
            let (rect, _) = ui.allocate_exact_size(vec2(200.0, ROW_HEIGHT), Sense::hover());
            ui.painter().rect_filled(rect, 0.0, ROW_INK);
            if row == 0 {
                first = rect;
            }
            last = rect;
        }
        (first, last)
    };

    // Settle the same dialog once without a seed to learn the height its
    // content actually asks for; the regression is rendered from one row
    // less than that.
    let settled = {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        for _ in 0..3 {
            let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
                let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                    .description(TEST_DESCRIPTION)
                    .show(ctx, |ui| {
                        paint_rows(ui);
                    });
            });
        }
        ctx.data(|data| data.get_temp::<f32>(dialog_id().with(("measured-surface-height", false))))
            .expect("a content-height dialog measures the surface it resolved")
    };
    assert!(
        settled > ROW_HEIGHT + 1.0,
        "the settled surface ({settled}) leaves no room to seed a row short"
    );

    let mut first_row = Rect::NOTHING;
    let mut final_row = Rect::NOTHING;
    let canvas = crate::ui::raster::render(vec2(1_000.0, 800.0), |ui, _| {
        let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
            .description(TEST_DESCRIPTION)
            .initial_height(settled - ROW_HEIGHT)
            .show(ui, |ui| {
                (first_row, final_row) = paint_rows(ui);
            });
    });

    // Inset past the tessellator's one-pixel feathering, and past the half
    // pixel a centred surface can sit off the pixel grid by.
    let first_pixels: Vec<egui::Color32> = canvas.pixels_in(first_row.shrink(2.0)).collect();
    let ink = *first_pixels
        .first()
        .expect("the first row lies off the canvas");
    assert!(
        ink.r() > ink.g() + 40 && ink.r() > ink.b() + 40,
        "the first row did not read back as the red it was painted in: {ink:?}"
    );
    assert!(
        first_pixels.iter().all(|pixel| *pixel == ink),
        "the first row is not uniform ink"
    );

    let final_region = final_row.shrink(2.0);
    let final_pixels: Vec<egui::Color32> = canvas.pixels_in(final_region).collect();
    assert!(
        !final_pixels.is_empty(),
        "the final row lies off the canvas"
    );
    assert!(
        final_pixels.iter().all(|pixel| *pixel == ink),
        "a dialog seeded a row short of its content clipped its final row: \
         the surface settled on the seed instead of the content"
    );
}

/// The pass that measures a dialog for the first time measures the dialog the
/// reader is about to be shown.
///
/// An `egui::Area` lays its opening frame out as a *sizing pass*: painted
/// invisibly, and with every centre cross-alignment rewritten to `Align::Min`
/// so each widget reports the least room it can live in. That is the pass a
/// dialog is first measured on, and anything on the surface that reaches its
/// authored height by filling a track — the header row does — reports something
/// smaller. The surface measured from it is short by the same amount, and it is
/// that measurement the first *visible* frame is laid out against, so the whole
/// dialog stepped once more after the reader could already see it.
///
/// The two passes are compared as heights rather than by asserting the header's
/// own rectangle: what has to agree is the number the next pass is laid out
/// against, and the header is only the part of the surface that disagreed.
#[test]
fn the_opening_sizing_pass_measures_the_surface_the_first_visible_pass_draws() {
    fn pass(ctx: &Context) -> f32 {
        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .size(DialogSize::WideWorkflow)
                .show(ctx, |ui| {
                    for row in 0..3 {
                        ui.label(format!("row {row}"));
                    }
                });
        });
        ctx.data(|data| data.get_temp::<f32>(dialog_id().with(("measured-surface-height", false))))
            .expect("a content-height dialog measures the surface it resolved")
    }

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let sizing = pass(&ctx);
    let first_visible = pass(&ctx);
    let settled = pass(&ctx);
    assert_eq!(
        first_visible, settled,
        "the first visible pass did not settle the surface"
    );
    assert_eq!(
        sizing, settled,
        "the opening sizing pass measured a {sizing}-point surface for a dialog \
         that draws {settled}, so the first frame the reader sees is laid out \
         against a height it then has to correct"
    );
}
