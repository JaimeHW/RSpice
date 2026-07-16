//! Mockup-defined shortcut Preferences page and transactional binding editor.

use std::time::Duration;

use egui::{Context, Key, Modifiers, Response, Stroke, Ui, WidgetInfo, WidgetType};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, select};
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext};
use crate::workbench::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;
use crate::workbench::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ProtectedShortcutPolicy, ShortcutBindingSlot,
    ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};

use super::preferences_shell::{
    page_heading, right_aligned, scope_strip, section_label, setting_row,
};
use super::{AppState, ConsoleMessage, PreferencePageActions};
use crate::common::app::app_dialog_state::{
    ShortcutCaptureTarget, ShortcutEditorContext, ShortcutEditorState,
};

const RESPONSIVE_TABLE_BREAKPOINT: f32 = 760.0;
const EDITOR_TABLE_MIN_WIDTH: f32 = 920.0;
const KEY_CAPTURE_MIN_WIDTH: f32 = 92.0;

const REGISTRY_ROWS: &[(Command, &str)] = &[
    (Command::CommandPalette, "Command palette"),
    (Command::ToggleFocusMode, "Focus workspace"),
    (Command::RunSimulation, "Run active plan"),
    (Command::StopSimulation, "Stop active run"),
    (Command::OpenProject, "Open project"),
    (Command::NewProject, "New project"),
    (Command::Save, "Save project"),
    (Command::CloseActiveDocument, "Close active document"),
    (Command::ToggleFullScreen, "Enter full screen"),
    (Command::GenerateNetlist, "Open generated netlist"),
    (Command::ToggleConsole, "Toggle console"),
];

pub(super) fn render_page(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    scope_strip(
        ui,
        "User profile",
        "portable bindings with platform and organization exceptions",
    );
    page_heading(
        ui,
        "Keyboard and command shortcuts",
        "Bindings are editable, context-aware, platform-validated and portable. Protected browser commands always expose a conflict-free alternate.",
    );

    render_registry_table(ui, state.ui.preferences.shortcuts());

    setting_row(
        ui,
        "Binding editor",
        "Record keys, resolve conflicts, set alternates and reset individual commands.",
        |ui| {
            right_aligned(ui, |ui| {
                if Button::new("Edit shortcuts\u{2026}")
                    .accent()
                    .show(ui)
                    .clicked()
                {
                    actions.open_shortcut_editor = true;
                }
            });
        },
    );

    // Import/export intentionally remain absent until their production file,
    // schema, review, rollback and platform workflows are integrated.
    section_label(ui, "Binding policy");
    // Render against an isolated value so a read-only frame never advances
    // the shortcut-library revision and invalidates an open import review.
    let mut policy_draft = state.ui.preferences.shortcuts().clone();
    render_policy_rows(ui, &mut policy_draft);
    if policy_draft != *state.ui.preferences.shortcuts()
        && let Err(error) = state
            .ui
            .preferences
            .shortcut_profiles_mut()
            .replace_active(policy_draft)
    {
        state.push_console_message(ConsoleMessage::error(format!(
            "Shortcut policy was not saved: {error}"
        )));
    }
}

fn render_policy_rows(ui: &mut Ui, profile: &mut ShortcutPreferences) {
    let current = profile.policies().single_key_canvas();
    if let Some(index) = policy_select(
        ui,
        "Single-key canvas commands",
        "Disable unmodified placement keys for voice input or assistive typing workflows.",
        "preferences.shortcuts.single-key-canvas",
        current.label(),
        SingleKeyCanvasPolicy::ALL
            .iter()
            .map(|value| value.label().to_owned())
            .collect(),
    ) {
        profile
            .policies_mut()
            .set_single_key_canvas(SingleKeyCanvasPolicy::ALL[index]);
    }

    let current = profile.policies().chord_timeout();
    if let Some(index) = policy_select(
        ui,
        "Chord timeout",
        "Applies only to multi-key command chords and never to normal text entry.",
        "preferences.shortcuts.chord-timeout",
        current.label(),
        ChordTimeoutPolicy::ALL
            .iter()
            .map(|value| value.label().to_owned())
            .collect(),
    ) {
        profile
            .policies_mut()
            .set_chord_timeout(ChordTimeoutPolicy::ALL[index]);
    }

    let current = profile.policies().protected_shortcuts();
    if let Some(index) = policy_select(
        ui,
        "Protected platform shortcuts",
        "Browser, operating-system and accessibility bindings cannot be shadowed silently.",
        "preferences.shortcuts.protected-platform",
        current.label(),
        ProtectedShortcutPolicy::ALL
            .iter()
            .map(|value| value.label().to_owned())
            .collect(),
    ) {
        profile
            .policies_mut()
            .set_protected_shortcuts(ProtectedShortcutPolicy::ALL[index]);
    }

    let current = profile.policies().context_precedence();
    if let Some(index) = policy_select(
        ui,
        "Context precedence",
        "Resolve global, workspace, editor and modal bindings deterministically.",
        "preferences.shortcuts.context-precedence",
        current.label(),
        ContextPrecedencePolicy::ALL
            .iter()
            .map(|value| value.label().to_owned())
            .collect(),
    ) {
        profile
            .policies_mut()
            .set_context_precedence(ContextPrecedencePolicy::ALL[index]);
    }
}

fn policy_select(
    ui: &mut Ui,
    title: &str,
    help: &str,
    id: &str,
    current: &str,
    options: Vec<String>,
) -> Option<usize> {
    let mut picked = None;
    setting_row(ui, title, help, |ui| {
        right_aligned(ui, |ui| {
            picked = select(
                ui,
                id,
                title,
                current,
                &options,
                ui.available_width().min(390.0),
            );
        });
    });
    picked
}

fn render_registry_table(ui: &mut Ui, profile: &ShortcutPreferences) {
    let audit = profile.audit();
    let operating_system = ui.ctx().os();
    let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
    let horizontal = if ui.ctx().content_rect().width() <= 560.0 {
        16
    } else {
        24
    };
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(horizontal, 0))
        .show(ui, |ui| {
            if narrow {
                for (command, label) in REGISTRY_ROWS.iter().copied() {
                    registry_card(ui, profile, &audit, operating_system, command, label);
                }
            } else {
                let grid = egui::Grid::new("preferences.shortcuts.registry-table")
                    .num_columns(5)
                    .striped(true)
                    .spacing([10.0, 0.0])
                    .show(ui, |ui| {
                        for heading in [
                            "Command",
                            "Desktop primary",
                            "Browser / touch alternate",
                            "Context",
                            "Status",
                        ] {
                            table_text(ui, heading, true, None);
                        }
                        ui.end_row();
                        for (command, label) in REGISTRY_ROWS.iter().copied() {
                            table_text(ui, label, false, None);
                            table_text(
                                ui,
                                &binding_for_platform(
                                    profile,
                                    command,
                                    ShortcutBindingSlot::Primary,
                                    CommandPlatform::Desktop,
                                    operating_system,
                                ),
                                false,
                                Some(true),
                            );
                            table_text(
                                ui,
                                &binding_for_platform(
                                    profile,
                                    command,
                                    ShortcutBindingSlot::Alternate,
                                    CommandPlatform::Browser,
                                    operating_system,
                                ),
                                false,
                                Some(true),
                            );
                            table_text(ui, command.shortcut_context().label(), false, Some(true));
                            table_text(ui, registry_status(&audit, command), false, Some(false));
                            ui.end_row();
                        }
                    });
                ui.ctx().accesskit_node_builder(grid.response.id, |node| {
                    node.set_role(egui::accesskit::Role::Table);
                    node.set_label("Protected shortcut registry");
                });
            }
            ui.add_space(7.0);
            audit_status(ui, profile, &audit);
            ui.add_space(9.0);
        });
}

fn registry_card(
    ui: &mut Ui,
    profile: &ShortcutPreferences,
    audit: &ShortcutProfileAudit,
    operating_system: egui::os::OperatingSystem,
    command: Command,
    label: &str,
) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            card_value(
                ui,
                "Desktop primary",
                &binding_for_platform(
                    profile,
                    command,
                    ShortcutBindingSlot::Primary,
                    CommandPlatform::Desktop,
                    operating_system,
                ),
            );
            card_value(
                ui,
                "Browser / touch alternate",
                &binding_for_platform(
                    profile,
                    command,
                    ShortcutBindingSlot::Alternate,
                    CommandPlatform::Browser,
                    operating_system,
                ),
            );
            card_value(ui, "Context", command.shortcut_context().label());
            card_value(ui, "Status", registry_status(audit, command));
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(label);
    });
    ui.add_space(6.0);
}

fn card_value(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(format!("{label}:"))
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_faint),
        );
        ui.label(
            egui::RichText::new(value)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

fn table_text(ui: &mut Ui, text: &str, header: bool, mono: Option<bool>) {
    let t = Tokens::get(ui.ctx());
    let font = if mono.unwrap_or(false) {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(
            tokens::FS_0,
            if header {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        )
    };
    let response = ui.add_sized(
        [
            ((text.chars().count() as f32 * 6.25) + 12.0).clamp(86.0, 166.0),
            28.0,
        ],
        egui::Label::new(egui::RichText::new(text).font(font).color(if header {
            t.color.text_faint
        } else {
            t.color.text_dim
        }))
        .truncate(),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if header {
            egui::accesskit::Role::ColumnHeader
        } else {
            egui::accesskit::Role::Cell
        });
        node.set_label(text);
    });
}

fn binding_for_platform(
    profile: &ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    profile
        .effective_bindings(command)
        .into_iter()
        .find(|binding| binding.slot() == slot && binding.platforms().contains(&platform))
        .map_or_else(
            || "\u{2014}".to_owned(),
            |binding| binding.sequence().display_label_for(operating_system),
        )
}

fn registry_status(audit: &ShortcutProfileAudit, command: Command) -> &'static str {
    let issues = audit_issues_for_command(audit, command).collect::<Vec<_>>();
    if issues.iter().any(|issue| {
        issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
            && issue.severity() == ShortcutProfileIssueSeverity::Warning
    }) {
        "protected \u{00b7} exception acknowledged"
    } else if issues
        .iter()
        .any(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
    {
        "conflict requires review"
    } else if !issues.is_empty() {
        "review"
    } else if command.primary_is_reserved_on(CommandPlatform::Browser) {
        "protected \u{00b7} alternate resolved"
    } else {
        "available"
    }
}

fn audit_status(ui: &mut Ui, profile: &ShortcutPreferences, audit: &ShortcutProfileAudit) {
    let collisions = audit
        .issues()
        .iter()
        .filter(|issue| {
            matches!(
                issue.code(),
                ShortcutProfileIssueCode::ExactCollision
                    | ShortcutProfileIssueCode::PrefixCollision
            )
        })
        .count();
    let reserved = audit
        .issues()
        .iter()
        .filter(|issue| {
            issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
                && issue.severity() == ShortcutProfileIssueSeverity::Error
        })
        .count();
    let binding_count = COMMAND_REGISTRY
        .iter()
        .copied()
        .flat_map(|command| profile.effective_bindings(command))
        .map(|binding| binding.platforms().len())
        .sum::<usize>();
    let text = format!(
        "{binding_count} normalized platform bindings \u{00b7} {collisions} unresolved collisions \u{00b7} {reserved} unhandled reserved bindings."
    );
    status_text(ui, &text, !audit.is_valid());
}

pub(super) fn render_editor(ctx: &Context, state: &mut AppState) {
    if !state.dialogs.shortcut_editor.open {
        return;
    }

    process_key_capture(ctx, &mut state.dialogs.shortcut_editor);
    synchronize_editor_validation(&mut state.dialogs.shortcut_editor);

    let (hint, save_enabled) = {
        let editor = &state.dialogs.shortcut_editor;
        let audit = editor
            .draft
            .as_ref()
            .expect("open editor owns a draft")
            .audit();
        let errors = blocking_issue_count(&audit);
        let hint = if errors == 0 {
            format!(
                "{} bindings \u{00b7} conflict-free draft",
                audit.binding_count()
            )
        } else {
            format!("{errors} blocking conflict(s) \u{00b7} review before save")
        };
        (
            hint,
            editor.dirty && editor.recording.is_none() && errors == 0,
        )
    };

    let choice = {
        let editor = &mut state.dialogs.shortcut_editor;
        let mut scroll_offset = editor.body_scroll_offset;
        let choice = Dialog::new(
            "PREFERENCES \u{00b7} COMMAND BINDINGS \u{00b7} CONFLICT SAFE",
            "Keyboard shortcut editor",
            "Save shortcut profile",
        )
        .description("Edit a versioned user shortcut profile. Changes remain isolated until the complete profile validates and Save succeeds.")
        .size(DialogSize::CapabilityReview)
        .primary_enabled(save_enabled)
        .primary_on_enter(false)
        .ghost("Cancel")
        .hint(&hint)
        .body_scroll_offset(&mut scroll_offset)
        .flush_body()
        .show(ctx, |ui| render_editor_body(ui, editor));
        editor.body_scroll_offset = scroll_offset;
        choice
    };

    match choice {
        DialogChoice::Primary => commit_editor(state),
        DialogChoice::Ghost | DialogChoice::Cancelled => request_editor_cancel(state),
        DialogChoice::None | DialogChoice::Secondary => {}
    }

    render_discard_confirmation(ctx, state);
}

fn render_editor_body(ui: &mut Ui, editor: &mut ShortcutEditorState) {
    editor_toolbar(ui, editor);
    if let Some(summary) = editor.error_summary.as_deref() {
        error_banner(ui, summary);
    }
    if let Some(receipt) = editor.repair_receipt.as_deref() {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| status_text(ui, receipt, false));
    }

    let audit = editor.draft.as_ref().expect("editor draft").audit();
    let commands = filtered_commands(editor, ui.ctx().os());
    if commands.is_empty() {
        let t = Tokens::get(ui.ctx());
        egui::Frame::NONE
            .inner_margin(egui::Margin::same(16))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("No commands match this search and context.")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
    } else if ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT {
        for command in commands {
            editor_card(ui, editor, &audit, command);
        }
    } else {
        editor_table(ui, editor, &audit, &commands);
    }

    if !audit.issues().is_empty() {
        conflict_list(ui, editor, &audit);
    }
    editor_notes(ui);
}

fn editor_toolbar(ui: &mut Ui, editor: &mut ShortcutEditorState) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
            if narrow {
                ui.vertical(|ui| toolbar_controls(ui, editor, true));
            } else {
                ui.horizontal(|ui| toolbar_controls(ui, editor, false));
            }
        });
}

fn toolbar_controls(ui: &mut Ui, editor: &mut ShortcutEditorState, narrow: bool) {
    let control_width = if narrow {
        ui.available_width()
    } else {
        ui.available_width().min(300.0)
    };
    let search = ui.add_sized(
        [control_width, Tokens::get(ui.ctx()).metrics.ctl_h.max(28.0)],
        egui::TextEdit::singleline(&mut editor.query)
            .hint_text("Command, context, or key\u{2026}")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
    );
    search.widget_info(|| {
        WidgetInfo::labeled(WidgetType::TextEdit, true, "Filter keyboard commands")
    });

    let labels = ShortcutEditorContext::ALL
        .iter()
        .map(|context| context.label().to_owned())
        .collect::<Vec<_>>();
    let context_width = if narrow {
        ui.available_width()
    } else {
        ui.available_width().min(156.0)
    };
    if let Some(index) = select(
        ui,
        "preferences.shortcuts.editor-context",
        "Shortcut context",
        editor.context.label(),
        &labels,
        context_width,
    ) {
        editor.context = ShortcutEditorContext::ALL[index];
        if editor
            .selected_command
            .is_some_and(|command| !context_matches(command, editor.context))
        {
            editor.selected_command = None;
        }
    }

    ui.label(
        egui::RichText::new(format!(
            "Current platform \u{00b7} {}",
            crate::common::app::runtime_command_platform(ui.ctx()).label()
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(Tokens::get(ui.ctx()).color.text_faint),
    );

    let reset_selected = Button::new("Reset selected")
        .enabled(editor.selected_command.is_some())
        .show(ui);
    if reset_selected.clicked()
        && let Some(command) = editor.selected_command
    {
        editor
            .draft
            .as_mut()
            .expect("editor draft")
            .reset_command(command);
        mark_editor_changed(editor);
    }
    let reset_context = Button::new("Reset context")
        .enabled(editor.context != ShortcutEditorContext::All)
        .show(ui);
    if reset_context.clicked() && editor.context != ShortcutEditorContext::All {
        let context = editor.context;
        let draft = editor.draft.as_mut().expect("editor draft");
        for command in COMMAND_REGISTRY
            .iter()
            .copied()
            .filter(|command| context_matches(*command, context))
        {
            draft.reset_command(command);
        }
        mark_editor_changed(editor);
    }
    if Button::new("Reset all").show(ui).clicked() {
        editor.draft.as_mut().expect("editor draft").reset_all();
        mark_editor_changed(editor);
    }
}

fn editor_table(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    audit: &ShortcutProfileAudit,
    commands: &[Command],
) {
    egui::ScrollArea::horizontal()
        .id_salt("shortcut-editor-table-scroll")
        .show(ui, |ui| {
            ui.set_min_width(EDITOR_TABLE_MIN_WIDTH);
            let grid = egui::Grid::new("shortcut-editor-table")
                .num_columns(6)
                .striped(true)
                .spacing([12.0, 2.0])
                .show(ui, |ui| {
                    for heading in [
                        "Command",
                        "Context",
                        "Primary binding",
                        "Alternate",
                        "Platform",
                        "Status",
                    ] {
                        table_text(ui, heading, true, None);
                    }
                    ui.end_row();
                    for command in commands.iter().copied() {
                        let selected = editor.selected_command == Some(command);
                        let response = ui.selectable_label(selected, command.spec().label);
                        response.widget_info(|| {
                            WidgetInfo::selected(
                                WidgetType::SelectableLabel,
                                true,
                                selected,
                                command.spec().label,
                            )
                        });
                        if response.clicked() {
                            editor.selected_command = Some(command);
                        }
                        table_text(ui, editor_context(command).label(), false, Some(true));
                        capture_button(ui, editor, command, ShortcutBindingSlot::Primary);
                        capture_button(ui, editor, command, ShortcutBindingSlot::Alternate);
                        table_text(
                            ui,
                            &platform_summary(
                                editor.draft.as_ref().expect("editor draft"),
                                command,
                            ),
                            false,
                            Some(true),
                        );
                        table_text(ui, editor_status(audit, command), false, None);
                        ui.end_row();
                    }
                });
            ui.ctx().accesskit_node_builder(grid.response.id, |node| {
                node.set_role(egui::accesskit::Role::Table);
                node.set_label("Editable command bindings");
            });
        });
}

fn editor_card(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    audit: &ShortcutProfileAudit,
    command: Command,
) {
    let t = Tokens::get(ui.ctx());
    let selected = editor.selected_command == Some(command);
    let response = egui::Frame::NONE
        .fill(if selected {
            t.color.bg_active
        } else {
            t.color.bg_panel
        })
        .stroke(Stroke::new(
            if selected { 2.0 } else { 1.0 },
            if selected {
                t.color.accent
            } else {
                t.color.border
            },
        ))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if ui
                .selectable_label(selected, command.spec().label)
                .clicked()
            {
                editor.selected_command = Some(command);
            }
            card_value(ui, "Context", editor_context(command).label());
            card_value(
                ui,
                "Platform",
                &platform_summary(editor.draft.as_ref().expect("editor draft"), command),
            );
            ui.horizontal_wrapped(|ui| {
                capture_button(ui, editor, command, ShortcutBindingSlot::Primary);
                capture_button(ui, editor, command, ShortcutBindingSlot::Alternate);
            });
            card_value(ui, "Status", editor_status(audit, command));
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(command.spec().label);
        node.set_selected(selected);
    });
    ui.add_space(6.0);
}

fn capture_button(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    command: Command,
    slot: ShortcutBindingSlot,
) -> Response {
    let target = ShortcutCaptureTarget { command, slot };
    let is_recording = editor.recording == Some(target);
    let label = if is_recording {
        if editor.capture_strokes.is_empty() {
            "Press keys\u{2026}".to_owned()
        } else {
            format!(
                "{} \u{00b7} waiting\u{2026}",
                ShortcutSequence::new(editor.capture_strokes.clone())
                    .expect("capture stroke count is bounded")
                    .display_label_for(ui.ctx().os())
            )
        }
    } else {
        binding_for_slot(
            editor.draft.as_ref().expect("editor draft"),
            command,
            slot,
            ui.ctx().os(),
        )
        .unwrap_or_else(|| "Record\u{2026}".to_owned())
    };
    let accessible = if is_recording {
        format!(
            "Recording {} binding for {}. Press keys, Enter to finish, plain Backspace to clear, or Escape to cancel.",
            slot.label(),
            command.spec().label
        )
    } else {
        format!(
            "Record {} binding for {}",
            slot.label(),
            command.spec().label
        )
    };
    let response = Button::new(&label)
        .min_width(KEY_CAPTURE_MIN_WIDTH)
        .accessible_label(&accessible)
        .show(ui);
    if response.clicked() {
        editor.selected_command = Some(command);
        editor.recording = Some(target);
        editor.capture_strokes.clear();
        editor.capture_last_input_at = None;
        editor.error_summary = None;
        response.request_focus();
    }
    if editor.focus_error == Some(target) {
        response.request_focus();
        editor.focus_error = None;
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_toggled(is_recording.into());
        if is_recording {
            node.set_description(
                "Key capture active. Enter completes a multi-stroke sequence and Escape cancels.",
            );
        }
    });
    response
}

fn process_key_capture(ctx: &Context, editor: &mut ShortcutEditorState) {
    let Some(target) = editor.recording else {
        return;
    };
    let events = ctx.input(|input| {
        input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    modifiers,
                    ..
                } => Some((*key, *modifiers)),
                _ => None,
            })
            .collect::<Vec<_>>()
    });

    for (key, modifiers) in events {
        if editor.recording != Some(target) {
            break;
        }
        ctx.input_mut(|input| {
            input.consume_key(modifiers, key);
        });
        if key == Key::Escape {
            cancel_capture(editor);
            return;
        }
        let no_modifiers = modifiers_are_empty(modifiers);
        if key == Key::Enter && no_modifiers && !editor.capture_strokes.is_empty() {
            finish_capture(editor, target);
            return;
        }
        if key == Key::Backspace && no_modifiers && editor.capture_strokes.is_empty() {
            clear_capture(editor, target);
            return;
        }
        editor.capture_strokes.push(ShortcutStroke::new(
            key,
            modifiers.command,
            modifiers.alt,
            modifiers.shift,
        ));
        editor.capture_last_input_at = Some(ctx.input(|input| input.time));
        if editor.capture_strokes.len() == MAX_SHORTCUT_SEQUENCE_STROKES {
            finish_capture(editor, target);
            return;
        }
    }

    if editor.recording != Some(target) {
        return;
    }

    let timeout = editor
        .draft
        .as_ref()
        .expect("editor draft")
        .policies()
        .chord_timeout()
        .seconds();
    if let (Some(timeout), Some(last)) = (timeout, editor.capture_last_input_at) {
        let now = ctx.input(|input| input.time);
        if !editor.capture_strokes.is_empty() && now - last >= timeout {
            finish_capture(editor, target);
        } else {
            ctx.request_repaint_after(Duration::from_secs_f64((timeout - (now - last)).max(0.01)));
        }
    }
}

fn finish_capture(editor: &mut ShortcutEditorState, target: ShortcutCaptureTarget) {
    let strokes = std::mem::take(&mut editor.capture_strokes);
    let Ok(sequence) = ShortcutSequence::new(strokes) else {
        cancel_capture(editor);
        return;
    };
    let platforms = binding_platforms(
        editor.draft.as_ref().expect("editor draft"),
        target.command,
        target.slot,
    );
    match editor.draft.as_mut().expect("editor draft").set_binding(
        target.command,
        target.slot,
        platforms,
        Some(sequence),
    ) {
        Ok(()) => mark_editor_changed(editor),
        Err(error) => {
            editor.error_summary = Some(format!(
                "Could not record {} for {}: {error}",
                target.slot.label(),
                target.command.spec().label
            ))
        }
    }
    editor.recording = None;
    editor.capture_last_input_at = None;
}

fn clear_capture(editor: &mut ShortcutEditorState, target: ShortcutCaptureTarget) {
    let platforms = binding_platforms(
        editor.draft.as_ref().expect("editor draft"),
        target.command,
        target.slot,
    );
    match editor.draft.as_mut().expect("editor draft").set_binding(
        target.command,
        target.slot,
        platforms,
        None,
    ) {
        Ok(()) => mark_editor_changed(editor),
        Err(error) => editor.error_summary = Some(error.to_string()),
    }
    cancel_capture(editor);
}

fn cancel_capture(editor: &mut ShortcutEditorState) {
    editor.recording = None;
    editor.capture_strokes.clear();
    editor.capture_last_input_at = None;
}

fn modifiers_are_empty(modifiers: Modifiers) -> bool {
    !modifiers.alt
        && !modifiers.ctrl
        && !modifiers.shift
        && !modifiers.mac_cmd
        && !modifiers.command
}

fn binding_platforms(
    profile: &ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
) -> Vec<CommandPlatform> {
    if let Some(binding) = profile
        .resolved_bindings(command)
        .into_iter()
        .find(|binding| binding.slot() == slot)
    {
        return binding.platforms().to_vec();
    }
    match slot {
        ShortcutBindingSlot::Primary
            if command.primary_is_reserved_on(CommandPlatform::Browser) =>
        {
            vec![CommandPlatform::Desktop]
        }
        ShortcutBindingSlot::Primary => CommandPlatform::ALL.to_vec(),
        ShortcutBindingSlot::Alternate => vec![
            CommandPlatform::Browser,
            CommandPlatform::Tablet,
            CommandPlatform::Phone,
        ],
    }
}

fn mark_editor_changed(editor: &mut ShortcutEditorState) {
    editor.dirty = editor.draft != editor.original;
    editor.repair_receipt = None;
    synchronize_editor_validation(editor);
}

fn synchronize_editor_validation(editor: &mut ShortcutEditorState) {
    let audit = editor.draft.as_ref().expect("editor draft").audit();
    let errors = blocking_issue_count(&audit);
    if errors == 0 {
        editor.error_summary = None;
        editor.focus_error = None;
    } else {
        let summary = format!(
            "Resolve {errors} blocking shortcut conflict(s) before saving. Focus moved to the first affected binding."
        );
        if editor.error_summary.as_deref() != Some(summary.as_str()) {
            editor.error_summary = Some(summary);
            editor.focus_error = first_error_target(&audit);
            if let Some(target) = editor.focus_error {
                editor.query.clear();
                editor.context = editor_context(target.command);
                editor.selected_command = Some(target.command);
            }
        }
    }
}

fn filtered_commands(
    editor: &ShortcutEditorState,
    operating_system: egui::os::OperatingSystem,
) -> Vec<Command> {
    let query = editor.query.trim().to_lowercase();
    COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| !matches!(command, Command::KeyboardShortcuts))
        .filter(|command| context_matches(*command, editor.context))
        .filter(|command| {
            if query.is_empty() {
                return true;
            }
            let bindings = editor
                .draft
                .as_ref()
                .expect("editor draft")
                .resolved_bindings(*command)
                .into_iter()
                .map(|binding| binding.sequence().display_label_for(operating_system))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "{} {} {} {}",
                command.spec().label,
                editor_context(*command).label(),
                command.stable_id(),
                bindings
            )
            .to_lowercase()
            .contains(&query)
        })
        .collect()
}

fn editor_context(command: Command) -> ShortcutEditorContext {
    match command.shortcut_context() {
        ShortcutContext::EditContext
        | ShortcutContext::EngineeringCanvas
        | ShortcutContext::SymbolCanvas
        | ShortcutContext::DesignWorkspace => ShortcutEditorContext::Schematic,
        ShortcutContext::SimulationWorkspace => ShortcutEditorContext::Simulation,
        ShortcutContext::ResultsWorkspace => ShortcutEditorContext::Results,
        ShortcutContext::VerificationWorkspace | ShortcutContext::ViolationNavigation => {
            ShortcutEditorContext::Verification
        }
        ShortcutContext::Global
        | ShortcutContext::ApplicationChrome
        | ShortcutContext::RunnableProject => ShortcutEditorContext::Global,
    }
}

fn context_matches(command: Command, filter: ShortcutEditorContext) -> bool {
    filter == ShortcutEditorContext::All || editor_context(command) == filter
}

fn binding_for_slot(
    profile: &ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
    operating_system: egui::os::OperatingSystem,
) -> Option<String> {
    let bindings = profile
        .resolved_bindings(command)
        .into_iter()
        .filter(|binding| binding.slot() == slot)
        .map(|binding| binding.sequence().display_label_for(operating_system))
        .collect::<Vec<_>>();
    (!bindings.is_empty()).then(|| bindings.join(" / "))
}

fn platform_summary(profile: &ShortcutPreferences, command: Command) -> String {
    let mut platforms = profile
        .resolved_bindings(command)
        .into_iter()
        .flat_map(|binding| binding.platforms().to_vec())
        .collect::<Vec<_>>();
    platforms.sort_unstable();
    platforms.dedup();
    if platforms == CommandPlatform::ALL {
        return "all".to_owned();
    }
    platforms
        .into_iter()
        .map(|platform| platform.label().to_lowercase())
        .collect::<Vec<_>>()
        .join(" / ")
}

fn editor_status(audit: &ShortcutProfileAudit, command: Command) -> &str {
    audit_issues_for_command(audit, command)
        .next()
        .map_or("available", ShortcutProfileIssue::message)
}

fn audit_issues_for_command(
    audit: &ShortcutProfileAudit,
    command: Command,
) -> impl Iterator<Item = &ShortcutProfileIssue> {
    audit.issues().iter().filter(move |issue| {
        issue
            .command_id()
            .is_some_and(|id| id.as_str() == command.stable_id())
    })
}

fn conflict_list(ui: &mut Ui, editor: &mut ShortcutEditorState, audit: &ShortcutProfileAudit) {
    let t = Tokens::get(ui.ctx());
    let has_repairable_invalid_data = audit.issues().iter().any(|issue| {
        issue.severity() == ShortcutProfileIssueSeverity::Error
            && (issue.command_id().is_none()
                || matches!(
                    issue.code(),
                    ShortcutProfileIssueCode::MalformedCommandId
                        | ShortcutProfileIssueCode::MalformedOverride
                        | ShortcutProfileIssueCode::MalformedKey
                        | ShortcutProfileIssueCode::InvalidBinding
                ))
    });
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("Conflict and profile audit")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            for issue in audit.issues() {
                let prefix = match issue.severity() {
                    ShortcutProfileIssueSeverity::Error => "Error",
                    ShortcutProfileIssueSeverity::Warning => "Warning",
                };
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{prefix}: {}", issue.message()))
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(if issue.severity() == ShortcutProfileIssueSeverity::Error {
                                t.color.err
                            } else {
                                t.color.warn
                            }),
                    )
                    .wrap(),
                );
                if issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
                    && editor
                        .draft
                        .as_ref()
                        .expect("editor draft")
                        .policies()
                        .protected_shortcuts()
                        == ProtectedShortcutPolicy::AllowWithConfirmation
                    && let Some(command) = issue
                        .command_id()
                        .and_then(|id| Command::from_stable_id(id.as_str()))
                {
                    if issue.severity() == ShortcutProfileIssueSeverity::Error {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "Allow with confirmation is active. This command will have no browser/touch alternate and may be unavailable where its primary is protected. Confirm this exception for the versioned profile before saving.",
                                )
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.warn),
                            )
                            .wrap(),
                        );
                        let label = format!("Confirm protected override for {}", command.spec().label);
                        if Button::new(&label).show(ui).clicked() {
                            editor
                                .draft
                                .as_mut()
                                .expect("editor draft")
                                .acknowledge_protected_override(command);
                            mark_editor_changed(editor);
                        }
                    } else {
                        ui.label(
                            egui::RichText::new("Protected override acknowledged for this saved profile.")
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.warn),
                        );
                    }
                }
            }
            if has_repairable_invalid_data {
                ui.add_space(8.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "The profile contains incompatible data that cannot be edited as a command row. Removing it preserves valid future-command and extension data and records every removed identifier.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
                if Button::new("Remove incompatible entries")
                    .accent()
                    .destructive(true)
                    .show(ui)
                    .clicked()
                {
                    repair_invalid_entries(editor);
                }
            }
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label("Shortcut profile audit");
        node.set_live(egui::accesskit::Live::Polite);
    });
}

fn repair_invalid_entries(editor: &mut ShortcutEditorState) {
    let removed = editor
        .draft
        .as_mut()
        .expect("editor draft")
        .remove_blocking_invalid_entries();
    mark_editor_changed(editor);
    editor.repair_receipt = Some(if removed.is_empty() {
        "No incompatible shortcut profile entries were removed.".to_owned()
    } else {
        format!(
            "Removed {} incompatible shortcut profile entr{}: {}.",
            removed.len(),
            if removed.len() == 1 { "y" } else { "ies" },
            removed.join(", ")
        )
    });
}

fn editor_notes(ui: &mut Ui) {
    let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
    let t = Tokens::get(ui.ctx());
    let frame = |ui: &mut Ui, title: &str, body: &str| {
        egui::Frame::NONE
            .fill(t.color.bg_panel)
            .stroke(Stroke::new(1.0, t.color.border))
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(body)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
    };
    if narrow {
        ui.vertical(|ui| {
            frame(
                ui,
                "Conflict resolution",
                "Bindings are validated by context and platform. Protected browser and operating-system shortcuts require an explicit alternate.",
            );
            frame(
                ui,
                "Reset scope",
                "Reset one command, one context, or the complete user profile without changing organization-managed bindings.",
            );
        });
    } else {
        ui.columns(2, |columns| {
            frame(
                &mut columns[0],
                "Conflict resolution",
                "Bindings are validated by context and platform. Protected browser and operating-system shortcuts require an explicit alternate.",
            );
            frame(
                &mut columns[1],
                "Reset scope",
                "Reset one command, one context, or the complete user profile without changing organization-managed bindings.",
            );
        });
    }
}

fn error_banner(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.err))
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.err),
                )
                .wrap(),
            );
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Alert);
        node.set_label(message);
        node.set_live(egui::accesskit::Live::Assertive);
    });
}

fn status_text(ui: &mut Ui, text: &str, error: bool) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if error {
                    t.color.err
                } else {
                    t.color.text_faint
                }),
        )
        .wrap(),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(text);
        node.set_live(egui::accesskit::Live::Polite);
    });
}

fn blocking_issue_count(audit: &ShortcutProfileAudit) -> usize {
    audit
        .issues()
        .iter()
        .filter(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
        .count()
}

fn commit_editor(state: &mut AppState) {
    let audit = state
        .dialogs
        .shortcut_editor
        .draft
        .as_ref()
        .expect("editor draft")
        .audit();
    if !audit.is_valid() {
        let count = blocking_issue_count(&audit);
        state.dialogs.shortcut_editor.error_summary = Some(format!(
            "Shortcut profile was not saved. Resolve {count} blocking conflict(s); focus moved to the first affected binding."
        ));
        state.dialogs.shortcut_editor.focus_error = first_error_target(&audit);
        if let Some(target) = state.dialogs.shortcut_editor.focus_error {
            state.dialogs.shortcut_editor.query.clear();
            state.dialogs.shortcut_editor.context = editor_context(target.command);
            state.dialogs.shortcut_editor.selected_command = Some(target.command);
        }
        return;
    }
    let draft = state
        .dialogs
        .shortcut_editor
        .draft
        .take()
        .expect("editor draft");
    if let Err(error) = state
        .ui
        .preferences
        .shortcut_profiles_mut()
        .replace_active(draft.clone())
    {
        state.dialogs.shortcut_editor.draft = Some(draft);
        state.dialogs.shortcut_editor.error_summary =
            Some(format!("Shortcut profile was not saved: {error}"));
        state.push_console_message(ConsoleMessage::error(format!(
            "Shortcut profile was not saved: {error}"
        )));
        return;
    }
    state.dialogs.shortcut_editor.close_and_discard();
    state.push_console_message(ConsoleMessage::info(
        "Keyboard shortcut profile saved after complete collision and reserved-binding validation.",
    ));
}

fn first_error_target(audit: &ShortcutProfileAudit) -> Option<ShortcutCaptureTarget> {
    audit
        .issues()
        .iter()
        .find(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
        .and_then(|issue| {
            let command = Command::from_stable_id(issue.command_id()?.as_str())?;
            Some(ShortcutCaptureTarget {
                command,
                slot: issue.slot().unwrap_or(ShortcutBindingSlot::Primary),
            })
        })
}

fn request_editor_cancel(state: &mut AppState) {
    cancel_capture(&mut state.dialogs.shortcut_editor);
    if state.dialogs.shortcut_editor.dirty {
        state.dialogs.shortcut_editor.discard_confirmation = true;
    } else {
        state.dialogs.shortcut_editor.close_and_discard();
    }
}

fn render_discard_confirmation(ctx: &Context, state: &mut AppState) {
    if !state.dialogs.shortcut_editor.discard_confirmation {
        return;
    }
    let choice = Dialog::new(
        "PREFERENCES \u{00b7} UNSAVED COMMAND BINDINGS",
        "Discard shortcut changes?",
        "Discard changes",
    )
    .description("The unsaved shortcut draft will be removed. The active user profile remains unchanged.")
    .destructive()
    .ghost("Keep editing")
    .show(ctx, |ui| {
        ui.add(
            egui::Label::new(
                "Every binding and reset made in this editor session is still isolated from the active profile.",
            )
            .wrap(),
        );
    });
    match choice {
        DialogChoice::Primary => state.dialogs.shortcut_editor.close_and_discard(),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.dialogs.shortcut_editor.discard_confirmation = false;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(width: f32, height: f32) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, height),
            )),
            ..egui::RawInput::default()
        }
    }

    #[test]
    fn page_registry_is_the_exact_mockup_review_projection() {
        assert_eq!(REGISTRY_ROWS.len(), 11);
        assert_eq!(REGISTRY_ROWS[0].1, "Command palette");
        assert_eq!(REGISTRY_ROWS[10].1, "Toggle console");
    }

    #[test]
    fn typed_context_filters_cover_every_registry_command() {
        for command in COMMAND_REGISTRY.iter().copied() {
            assert!(ShortcutEditorContext::ALL.contains(&editor_context(command)));
        }
    }

    #[test]
    fn capture_platforms_preserve_defaults_and_protected_alternates() {
        let profile = ShortcutPreferences::default();
        assert_eq!(
            binding_platforms(&profile, Command::Save, ShortcutBindingSlot::Alternate),
            vec![
                CommandPlatform::Browser,
                CommandPlatform::Tablet,
                CommandPlatform::Phone
            ]
        );
    }

    #[test]
    fn protected_override_requires_explicit_persisted_draft_acknowledgement() {
        let mut profile = ShortcutPreferences::default();
        profile
            .policies_mut()
            .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
        profile
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Alternate,
                vec![
                    CommandPlatform::Browser,
                    CommandPlatform::Tablet,
                    CommandPlatform::Phone,
                ],
                None,
            )
            .unwrap();
        assert!(!profile.audit().is_valid());
        assert!(!profile.protected_override_acknowledged(Command::Save));

        profile.acknowledge_protected_override(Command::Save);

        assert!(profile.audit().is_valid());
        assert!(profile.protected_override_acknowledged(Command::Save));
        assert!(profile.audit().issues().iter().any(|issue| {
            issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
                && issue.severity() == ShortcutProfileIssueSeverity::Warning
        }));
    }

    #[test]
    fn incompatible_profile_repair_is_draft_only_and_emits_a_receipt() {
        let live: ShortcutPreferences = serde_json::from_str(
            r#"{"commands":{"save-project":{"bindings":[{"slot":"primary","platforms":["desktop"],"sequence":[{"key":"DefinitelyNotAKey"}]}]}}}"#,
        )
        .unwrap();
        assert!(!live.audit().is_valid());
        let mut editor = ShortcutEditorState::default();
        editor.open(&live);

        repair_invalid_entries(&mut editor);

        assert!(editor.draft.as_ref().unwrap().audit().is_valid());
        assert!(!editor.original.as_ref().unwrap().audit().is_valid());
        assert!(editor.dirty);
        assert!(
            editor
                .repair_receipt
                .as_deref()
                .is_some_and(|receipt| receipt.contains("save"))
        );
    }

    #[test]
    fn shortcuts_category_remains_unreachable_until_review_workflows_land() {
        assert!(
            !super::super::preferences_shell::PreferenceCategory::ALL
                .contains(&super::super::preferences_shell::PreferenceCategory::Shortcuts)
        );
    }

    #[test]
    fn search_matches_context_and_binding_text() {
        let mut editor = ShortcutEditorState {
            open: true,
            draft: Some(ShortcutPreferences::default()),
            query: "ctrl+alt+l".to_owned(),
            ..ShortcutEditorState::default()
        };
        let os = egui::os::OperatingSystem::Windows;
        assert!(filtered_commands(&editor, os).contains(&Command::GenerateNetlist));
        editor.query = "verification".to_owned();
        assert!(filtered_commands(&editor, os).iter().all(|command| {
            editor_context(*command) == ShortcutEditorContext::Verification
                || command.spec().label.to_lowercase().contains("verification")
        }));
    }

    #[test]
    fn ordered_same_frame_keydowns_produce_one_multi_stroke_sequence() {
        let ctx = Context::default();
        let mut editor = ShortcutEditorState::default();
        editor.open(&ShortcutPreferences::default());
        editor.recording = Some(ShortcutCaptureTarget {
            command: Command::Save,
            slot: ShortcutBindingSlot::Primary,
        });
        let ctrl = Modifiers {
            ctrl: true,
            command: true,
            ..Modifiers::NONE
        };
        let key_event = |key, modifiers| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        };
        let _ = ctx.run(
            egui::RawInput {
                events: vec![
                    key_event(Key::K, ctrl),
                    key_event(Key::C, ctrl),
                    key_event(Key::Enter, Modifiers::NONE),
                ],
                ..egui::RawInput::default()
            },
            |ctx| process_key_capture(ctx, &mut editor),
        );

        assert!(editor.recording.is_none());
        assert_eq!(
            binding_for_slot(
                editor.draft.as_ref().unwrap(),
                Command::Save,
                ShortcutBindingSlot::Primary,
                egui::os::OperatingSystem::Windows,
            )
            .as_deref(),
            Some("Ctrl+K Ctrl+C")
        );
    }

    #[test]
    fn editor_draft_cancel_never_mutates_live_profile() {
        let live = ShortcutPreferences::default();
        let mut editor = ShortcutEditorState::default();
        editor.open(&live);
        editor
            .draft
            .as_mut()
            .unwrap()
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    Key::F6,
                    false,
                    false,
                    false,
                ))),
            )
            .unwrap();
        assert_ne!(editor.draft.as_ref().unwrap(), &live);
        editor.close_and_discard();
        assert_eq!(live, ShortcutPreferences::default());
        assert!(editor.draft.is_none());
    }

    #[test]
    fn valid_save_atomically_replaces_the_live_profile() {
        let mut state = AppState::default();
        let live = state.ui.preferences.shortcuts().clone();
        state.dialogs.shortcut_editor.open(&live);
        state
            .dialogs
            .shortcut_editor
            .draft
            .as_mut()
            .unwrap()
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    Key::F6,
                    false,
                    false,
                    false,
                ))),
            )
            .unwrap();
        state.dialogs.shortcut_editor.dirty = true;

        commit_editor(&mut state);

        assert!(!state.dialogs.shortcut_editor.open);
        assert_eq!(
            binding_for_slot(
                state.ui.preferences.shortcuts(),
                Command::Save,
                ShortcutBindingSlot::Primary,
                egui::os::OperatingSystem::Windows,
            )
            .as_deref(),
            Some("F6")
        );
    }

    #[test]
    fn invalid_save_keeps_the_draft_open_and_focuses_the_first_issue() {
        let mut state = AppState::default();
        let live = state.ui.preferences.shortcuts().clone();
        state.dialogs.shortcut_editor.open(&live);
        state
            .dialogs
            .shortcut_editor
            .draft
            .as_mut()
            .unwrap()
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    Key::J,
                    true,
                    false,
                    false,
                ))),
            )
            .unwrap();
        state.dialogs.shortcut_editor.dirty = true;
        state.dialogs.shortcut_editor.query = "schematic only".to_owned();
        state.dialogs.shortcut_editor.context = ShortcutEditorContext::Schematic;

        commit_editor(&mut state);

        assert!(state.dialogs.shortcut_editor.open);
        assert!(state.dialogs.shortcut_editor.error_summary.is_some());
        assert!(state.dialogs.shortcut_editor.focus_error.is_some());
        assert!(state.dialogs.shortcut_editor.query.is_empty());
        assert_eq!(
            state.dialogs.shortcut_editor.context,
            ShortcutEditorContext::Global
        );
        assert_eq!(state.ui.preferences.shortcuts(), &live);
    }

    #[test]
    fn desktop_page_publishes_exact_heading_table_and_live_status_semantics() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut actions = PreferencePageActions::default();
        let output = ctx.run(input(1_100.0, 900.0), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_page(ui, &mut state, &mut actions);
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("Keyboard and command shortcuts")
                && node.level() == Some(3)
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Table
                && node.label() == Some("Protected shortcut registry")
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Status
                && node
                    .label()
                    .is_some_and(|label| label.contains("unresolved collisions"))
        }));
    }

    #[test]
    fn phone_page_reflows_registry_to_accessible_rows() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut actions = PreferencePageActions::default();
        let output = ctx.run(input(390.0, 844.0), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_page(ui, &mut state, &mut actions);
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Row && node.label() == Some("Command palette")
        }));
        assert!(!nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Table
                && node.label() == Some("Protected shortcut registry")
        }));
    }

    #[test]
    fn editor_exposes_modal_search_table_and_44_point_phone_targets() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let profile = state.ui.preferences.shortcuts().clone();
        state.dialogs.shortcut_editor.open(&profile);
        let output = ctx.run(input(390.0, 844.0), |ctx| render_editor(ctx, &mut state));
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
}
