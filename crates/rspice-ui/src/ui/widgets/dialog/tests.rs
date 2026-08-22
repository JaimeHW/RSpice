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

    // A body short enough to need no scrollbar spans the content box exactly:
    // one measure decides the room the whole structural stack shares.
    let (viewport, content, _) = settled_flush_body_geometry(DialogSize::WideWorkflow, 3);
    assert_eq!(
        viewport.x_range(),
        content.x_range(),
        "the body spans the surface's content box"
    );
}

/// A body that outgrows its surface lifts it to the content in one further
/// pass, rather than crawling toward it two points at a time.
///
/// The resolved surface height was the frame's outer rect plus a border
/// allowance that rect already carried. Two points of surplus mean nothing to
/// a body that fits, but an overflowing body spends every point the surface
/// offers, so each pass measured two points more than the last and handed them
/// to the next: the dialog grew two points a frame for as many frames as its
/// ceiling allowed. Nothing on it ever stopped moving, and an offscreen render
/// caught it mid-crawl.
///
/// A clamped body reports the height it was allowed, never the height it
/// wants, so the surface is told what the scroll area could not show. Measured
/// against a dialog opened on the same body from the start, which has no
/// smaller height to climb out of.
#[test]
fn a_body_that_outgrows_its_surface_settles_on_it_within_one_pass() {
    const SHORT_ROWS: usize = 3;
    // Enough rows to overflow a surface settled on three, and few enough to
    // stay clear of the ceiling — a body pinned against the ceiling cannot
    // show whether the surface climbed to it or merely stopped there.
    const GROWN_ROWS: usize = 20;

    fn pass(ctx: &Context, rows: usize) -> f32 {
        let _ = ctx.run_ui(raw_input(Vec::new()), |ctx| {
            let _ = Dialog::new("TEST", TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .size(DialogSize::WideWorkflow)
                .show(ctx, |ui| {
                    for row in 0..rows {
                        ui.label(format!("row {row}"));
                    }
                });
        });
        ctx.data(|data| data.get_temp::<f32>(dialog_id().with(("measured-surface-height", false))))
            .expect("a content-height dialog measures the surface it resolved")
    }

    fn settled(rows: usize) -> f32 {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = pass(&ctx, rows);
        pass(&ctx, rows)
    }

    let content_height = settled(GROWN_ROWS);
    let short_height = settled(SHORT_ROWS);
    assert!(
        content_height > short_height,
        "the two bodies must want different surfaces for this to measure anything"
    );

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = pass(&ctx, SHORT_ROWS);
    assert!(
        (pass(&ctx, SHORT_ROWS) - short_height).abs() <= 0.5,
        "the dialog must settle on the short body before its body grows"
    );

    let reached = pass(&ctx, GROWN_ROWS);
    assert!(
        (reached - content_height).abs() <= 0.5,
        "a body that grew to want {content_height} points was given {reached}"
    );
    let held = pass(&ctx, GROWN_ROWS);
    assert!(
        (held - reached).abs() <= 0.5,
        "the settled surface moved again, from {reached} to {held}"
    );
}
