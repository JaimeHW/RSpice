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
        choice = Dialog::new(TEST_TITLE, "Accept")
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

/// The dialog's node is where the dialog is: the card, not the screen its
/// scrim covers, so assistive technology and a reader measuring the tree find
/// the card's edges.
#[test]
fn the_dialog_node_bounds_the_card_and_not_the_screen() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut underlying = String::new();
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let (_, output) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, false, |ui| {
            ui.label("Body");
        });
        nodes.extend(output.platform_output.accesskit_update.expect("tree").nodes);
    }
    let bounds = |role: egui::accesskit::Role, label: &str| {
        nodes
            .values()
            .find(|node| node.role() == role && node.label() == Some(label))
            .and_then(egui::accesskit::Node::bounds)
            .unwrap_or_else(|| panic!("{label} is published as {role:?}"))
    };
    let dialog = bounds(egui::accesskit::Role::Dialog, TEST_TITLE);
    let accept = bounds(egui::accesskit::Role::Button, "Accept");
    let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, 800.0));
    let card =
        DialogLayout::resolve(Dialog::new(TEST_TITLE, "Accept").size, screen, None).surface_rect;
    assert!(
        (dialog.x0 - f64::from(card.left())).abs() < 1.0
            && (dialog.x1 - f64::from(card.right())).abs() < 1.0,
        "the dialog node runs {}..{}, the card {}..{}",
        dialog.x0,
        dialog.x1,
        card.left(),
        card.right()
    );
    assert!(
        dialog.y0 > 0.0 && dialog.y1 < 800.0,
        "the dialog node is {dialog:?}, the screen's height"
    );
    assert!(
        dialog.x0 <= accept.x0
            && accept.x1 <= dialog.x1
            && dialog.y0 <= accept.y0
            && accept.y1 <= dialog.y1,
        "the dialog node {dialog:?} holds its own button {accept:?}"
    );
}

/// A fixed height holds a workflow surface still across its states, within
/// the window: a window shorter than it holds it to the window less the
/// size's margins, its body scrolling between the header and the footer,
/// and a taller one leaves it at the height it asked for. It held the whole
/// height in any window, so a 740-point dialog in a 700-point window put its
/// header and its footer past the screen's edges.
#[test]
fn a_fixed_height_is_held_to_a_short_window_less_its_margins() {
    let spec = DialogSize::Transaction.spec();
    for (height, fixed, expected) in [
        (700.0, 740.0, 700.0 - spec.vertical_inset),
        (700.0, 600.0, 600.0),
        (1_000.0, 740.0, 740.0),
        (1_000.0, 900.0, spec.max_height),
    ] {
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, height));
        let surface =
            DialogLayout::resolve(DialogSize::Transaction, screen, Some(fixed)).surface_rect;
        assert_eq!(
            surface.height(),
            expected,
            "a fixed height of {fixed} in a window {height} tall"
        );
        assert!(
            screen.contains_rect(surface),
            "{surface:?} leaves the {height}-point window"
        );
    }
}

#[test]
fn retained_cancel_focus_moves_escape_to_discard_then_confirm_restores_workspace() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
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
        choice = Dialog::new(TEST_TITLE, "Accept")
            .description(TEST_DESCRIPTION)
            .ghost("Discard changes")
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
            let mut response = Dialog::new(TEST_TITLE, "Accept")
                .description(TEST_DESCRIPTION)
                .ghost("Discard changes")
                .show_transaction(ctx, |ui| {
                    ui.label("Dirty dialog");
                    None
                });
            choice = response.choice;
            response.retain_cancel_focus(DialogInitialFocus::Ghost);
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
            choice = Dialog::new(TEST_TITLE, "Accept")
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
fn transaction_state_is_absent_when_idle_and_exposes_an_assertive_strip_on_error() {
    let idle = Dialog::new(TEST_TITLE, "Accept");
    assert!(idle.transaction_state.is_none());

    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new(TEST_TITLE, "Accept")
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
fn dialog_publishes_modal_accessibility_semantics() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
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
    crate::Theme::default().apply(&ctx);
    let mut preferred_id = None;

    let _output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new(TEST_TITLE, "Accept")
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
fn known_body_control_processes_opening_input_and_preserves_later_focus() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let first = Id::new("known-dialog-first-field");
    let second = Id::new("known-dialog-second-field");
    let mut first_text = "default".to_owned();
    let mut second_text = String::new();
    let mut render = |ui: &mut Ui| {
        let ctx = ui.ctx();
        Dialog::new(TEST_TITLE, "Accept")
            .initial_focus(DialogInitialFocus::Control(first))
            .show_with_initial_body_focus(ctx, |ui| {
                let response = ui.add(egui::TextEdit::singleline(&mut first_text).id(first));
                ui.add(egui::TextEdit::singleline(&mut second_text).id(second));
                Some(response.id)
            });
    };
    let _ = ctx.run_ui(
        raw_input(vec![
            key_event(Key::A, Modifiers::CTRL | Modifiers::COMMAND),
            egui::Event::Paste("first".to_owned()),
        ]),
        &mut render,
    );
    ctx.memory_mut(|memory| memory.request_focus(second));
    let _ = ctx.run_ui(
        raw_input(vec![egui::Event::Paste("second".to_owned())]),
        render,
    );
    assert_eq!(first_text, "first");
    assert_eq!(second_text, "second");
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(second));
}

#[test]
fn unavailable_known_body_control_falls_back_without_editing_another_field() {
    for enabled in [false, true] {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let declared = Id::new("unavailable-declared-field");
        let mut text = "unchanged".to_owned();
        let _ = ctx.run_ui(
            raw_input(vec![egui::Event::Paste("wrong owner".to_owned())]),
            |ctx| {
                Dialog::new(TEST_TITLE, "Accept")
                    .interaction_enabled(enabled)
                    .initial_focus(DialogInitialFocus::Control(declared))
                    .show_with_initial_body_focus(ctx, |ui| {
                        let actual = if enabled {
                            declared.with("other")
                        } else {
                            declared
                        };
                        Some(ui.add(egui::TextEdit::singleline(&mut text).id(actual)).id)
                    });
            },
        );
        assert_eq!(text, "unchanged");
        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(dialog_focus_id())
        );
    }
}

#[test]
fn escape_keeps_preceding_edits_and_routes_later_input_once_to_restored_focus() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let first = Id::new("escape-first-field");
    let second = Id::new("escape-second-field");
    let mut first_text = String::new();
    let mut second_text = String::new();
    let mut underlying = String::new();
    focus_underlying_editor(&ctx, &mut underlying);
    let mut open = true;
    let mut render = |ui: &mut Ui| {
        ui.add(egui::TextEdit::singleline(&mut underlying).id(underlying_id()));
        if open {
            let choice = Dialog::new(TEST_TITLE, "Accept")
                .initial_focus(DialogInitialFocus::Control(first))
                .show_with_initial_body_focus(ui.ctx(), |ui| {
                    let response = ui.add(egui::TextEdit::singleline(&mut first_text).id(first));
                    ui.add(egui::TextEdit::singleline(&mut second_text).id(second));
                    Some(response.id)
                });
            open = choice == DialogChoice::None;
        }
    };
    let _ = ctx.run_ui(raw_input(Vec::new()), &mut render);
    ctx.memory_mut(|memory| memory.request_focus(second));
    let _ = ctx.run_ui(raw_input(Vec::new()), &mut render);
    let _ = ctx.run_ui(
        raw_input(vec![
            egui::Event::Paste("before".to_owned()),
            key_event(Key::Escape, Modifiers::NONE),
            egui::Event::Text("after".to_owned()),
        ]),
        &mut render,
    );
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
    let _ = ctx.run_ui(
        raw_input(vec![egui::Event::Text(" new".to_owned())]),
        &mut render,
    );
    let _ = ctx.run_ui(raw_input(Vec::new()), render);
    assert!(!open);
    assert_eq!(first_text, "");
    assert_eq!(second_text, "before");
    assert_eq!(underlying, "after new");
}

#[test]
fn enter_before_escape_submits_before_the_later_cancellation() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut text = String::new();
    let field = Id::new("ordered-submit-field");
    let mut choice = DialogChoice::None;
    let mut render = |ui: &mut Ui| {
        choice = Dialog::new(TEST_TITLE, "Accept")
            .initial_focus(DialogInitialFocus::Control(field))
            .show_with_initial_body_focus(ui.ctx(), |ui| {
                Some(ui.add(egui::TextEdit::singleline(&mut text).id(field)).id)
            });
    };
    let _ = ctx.run_ui(raw_input(Vec::new()), &mut render);
    let escape = key_event(Key::Escape, Modifiers::NONE);
    let after = egui::Event::Paste("later".to_owned());
    let _ = ctx.run_ui(
        raw_input(vec![
            egui::Event::Paste("submitted".to_owned()),
            key_event(Key::Enter, Modifiers::NONE),
            escape.clone(),
            after.clone(),
        ]),
        render,
    );
    assert_eq!(choice, DialogChoice::Primary);
    assert_eq!(text, "submitted");
    let _ = ctx.run_ui(raw_input(Vec::new()), |ui| {
        assert!(ui.input(|input| input.events.is_empty()));
    });
    let _ = ctx.run_ui(raw_input(Vec::new()), |ui| {
        assert_eq!(
            ui.input(|input| input.events.clone()),
            vec![escape.clone(), after.clone()]
        );
    });
    let _ = ctx.run_ui(raw_input(Vec::new()), |ui| {
        assert!(ui.input(|input| input.events.is_empty()));
    });
}

#[test]
fn escape_edits_only_the_top_dialog_and_returns_following_text_to_the_parent() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let parent = Id::new("escape-parent-field");
    let child = Id::new("escape-child-field");
    let mut parent_text = String::new();
    let mut child_text = String::new();
    let mut child_open = false;
    let mut parent_choice = DialogChoice::None;
    let mut child_choice = DialogChoice::None;
    for step in 0..5 {
        if step == 1 {
            child_open = true;
        }
        let events = if step == 2 {
            vec![
                egui::Event::Paste("child".to_owned()),
                key_event(Key::Escape, Modifiers::NONE),
                egui::Event::Paste("parent".to_owned()),
            ]
        } else {
            Vec::new()
        };
        let _ = ctx.run_ui(raw_input(events), |ui| {
            parent_choice = Dialog::new("Parent", "Accept")
                .initial_focus(DialogInitialFocus::BodyControl)
                .show_with_initial_body_focus(ui.ctx(), |ui| {
                    Some(
                        ui.add(egui::TextEdit::singleline(&mut parent_text).id(parent))
                            .id,
                    )
                });
            if child_open {
                child_choice = Dialog::new("Child", "Accept")
                    .initial_focus(DialogInitialFocus::Control(child))
                    .show_with_initial_body_focus(ui.ctx(), |ui| {
                        Some(
                            ui.add(egui::TextEdit::singleline(&mut child_text).id(child))
                                .id,
                        )
                    });
                child_open = child_choice == DialogChoice::None;
            }
        });
        assert_eq!(parent_choice, DialogChoice::None);
    }
    assert!(!child_open);
    assert_eq!(child_choice, DialogChoice::Cancelled);
    assert_eq!(child_text, "child");
    assert_eq!(parent_text, "parent");
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(parent));
}

#[test]
fn escape_continuation_does_not_revive_removed_or_disabled_focus() {
    for removed in [false, true] {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut underlying = String::new();
        focus_underlying_editor(&ctx, &mut underlying);
        let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |_| {});
        let (choice, _) = run_dialog(
            &ctx,
            raw_input(vec![
                key_event(Key::Escape, Modifiers::NONE),
                egui::Event::Paste("obsolete target".to_owned()),
            ]),
            &mut underlying,
            true,
            |_| {},
        );
        assert_eq!(choice, DialogChoice::Cancelled);
        for _ in 0..3 {
            let _ = ctx.run_ui(raw_input(Vec::new()), |ui| {
                if !removed {
                    ui.add_enabled(
                        false,
                        egui::TextEdit::singleline(&mut underlying).id(underlying_id()),
                    );
                }
            });
        }
        assert_eq!(underlying, "");
        assert_eq!(ctx.memory(|memory| memory.focused()), None);
    }
}

#[test]
fn escape_dismisses_an_open_popup_before_its_dialog() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let popup = Id::new("dialog-escape-popup");
    let mut underlying = String::new();
    let body = |ui: &mut Ui| {
        let response = ui.button("Options");
        Popup::from_response(&response)
            .id(popup)
            .open_memory(None)
            .show(|ui| {
                ui.button("An option").clicked();
            });
    };
    let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, body);
    Popup::open_id(&ctx, popup);
    let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, body);
    assert!(Popup::is_any_open(&ctx));
    let (choice, _) = run_dialog(
        &ctx,
        raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
        &mut underlying,
        true,
        body,
    );
    assert_eq!(choice, DialogChoice::None);
    assert!(!Popup::is_any_open(&ctx));
    let (choice, _) = run_dialog(
        &ctx,
        raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
        &mut underlying,
        true,
        body,
    );
    assert_eq!(choice, DialogChoice::Cancelled);
}

#[test]
fn unavailable_initial_focus_target_falls_back_to_modal_container() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);

    let _output = ctx.run_ui(raw_input(Vec::new()), |ctx| {
        let _ = Dialog::new(TEST_TITLE, "Accept")
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
    crate::Theme::default().apply(&ctx);
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
    crate::Theme::default().apply(&ctx);
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
    crate::Theme::default().apply(&ctx);
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
    crate::Theme::default().apply(&submit_ctx);
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
    crate::Theme::default().apply(&multiline_ctx);
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

/// Whether any rectangle the frame painted is filled with `fill`.
fn paints_fill(output: &egui::FullOutput, fill: Color32) -> bool {
    fn walk(shape: &egui::Shape, fill: Color32) -> bool {
        match shape {
            egui::Shape::Rect(rect) => rect.fill == fill,
            egui::Shape::Vec(shapes) => shapes.iter().any(|shape| walk(shape, fill)),
            _ => false,
        }
    }
    output
        .shapes
        .iter()
        .any(|clipped| walk(&clipped.shape, fill))
}

/// A footer whose one button is Close commits nothing, so the Close is the
/// neutral button rather than the accent slab a committing primary is; Enter
/// still closes the dialog through it.
#[test]
fn a_lone_close_is_the_neutral_button_and_enter_still_closes() {
    let show = |ctx: &Context, primary: &str, events: Vec<egui::Event>| {
        let mut choice = DialogChoice::None;
        let output = ctx.run_ui(raw_input(events), |ctx| {
            choice = Dialog::new(TEST_TITLE, primary).show(ctx, |ui| {
                let _ = ui.label("Ready");
            });
        });
        (choice, output)
    };
    for (primary, accent) in [("Accept", true), ("Close", false)] {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let mut output = show(&ctx, primary, Vec::new()).1;
        for _ in 0..3 {
            output = show(&ctx, primary, Vec::new()).1;
        }
        let fill = Tokens::get(&ctx).color.accent;
        assert_eq!(paints_fill(&output, fill), accent, "{primary}");

        let (choice, _) = show(&ctx, primary, vec![key_event(Key::Enter, Modifiers::NONE)]);
        assert_eq!(choice, DialogChoice::Primary, "{primary}");
    }
}

/// A table in a dialog's body takes the keyboard when a row is clicked: the
/// arrows then move its selection and Enter opens the row, rather than the
/// dialog taking the focus back or Enter for its own button.
#[test]
fn a_clicked_table_in_a_dialog_takes_the_arrows_and_enter() {
    use crate::widgets::table::{Cell, Column, DataTable, RowKey, TableResponse, TableState};

    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.all_styles_mut(|style| style.animation_time = 0.0);
    let columns = [Column::name("Name", 120.0)];
    let keys: Vec<RowKey> = (0..4).map(|row| RowKey::from(format!("r{row}"))).collect();
    let mut state = TableState::default();
    let frame = |events: Vec<egui::Event>, state: &mut TableState| {
        let mut table = TableResponse::default();
        let mut choice = DialogChoice::None;
        let (_, geometry) = crate::probe::pass(&ctx, raw_input(events), |ui| {
            choice = Dialog::new(TEST_TITLE, "Close").show(ui.ctx(), |ui| {
                table = DataTable::new("dialog-table", &columns, keys.len())
                    .state(state)
                    .keys(&keys)
                    .show(ui, |row| vec![Cell::text(format!("Row {row}"))]);
            });
        });
        (table, choice, geometry)
    };
    let _ = frame(Vec::new(), &mut state);
    let (_, _, geometry) = frame(Vec::new(), &mut state);
    let row = geometry
        .one("table.line.1")
        .expect("the row is laid out")
        .center();
    let press = |pressed| egui::Event::PointerButton {
        pos: row,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    let _ = frame(
        vec![egui::Event::PointerMoved(row), press(true), press(false)],
        &mut state,
    );
    assert_eq!(state.selected_row(), Some(1), "the click selects the row");

    let _ = frame(Vec::new(), &mut state);
    let (table, ..) = frame(vec![key_event(Key::ArrowDown, Modifiers::NONE)], &mut state);
    assert!(table.focused, "the clicked table keeps the keyboard");
    assert_eq!(state.selected_row(), Some(2), "Down moves the selection");
    let (table, choice, _) = frame(vec![key_event(Key::Enter, Modifiers::NONE)], &mut state);
    assert_eq!(table.activate, Some(2), "Enter opens the selected row");
    assert_eq!(
        choice,
        DialogChoice::None,
        "Enter is the table's, not the Close's"
    );
}
