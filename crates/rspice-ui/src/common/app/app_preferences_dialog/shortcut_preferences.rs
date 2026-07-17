//! Mockup-defined shortcut Preferences page and transactional binding editor.

use std::time::Duration;

use egui::{Context, Key, Modifiers, Response, Stroke, Ui, WidgetInfo, WidgetType};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone, select};
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext};
use crate::workbench::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;
use crate::workbench::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ProtectedShortcutPolicy, ShortcutBindingSlot,
    ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};

use super::preferences_shell::{
    actionable_scope_strip, page_heading, right_aligned, section_label, setting_row,
};
use super::{AppState, ConsoleMessage, PreferencePageActions};
use crate::common::app::app_dialog_state::{
    ShortcutCaptureTarget, ShortcutEditorContext, ShortcutEditorState,
};

const RESPONSIVE_TABLE_BREAKPOINT: f32 = 760.0;
const ENGINEERING_TABLE_BREAKPOINT: f32 = 820.0;
const EDITOR_NARROW_TABLE_MIN_WIDTH: f32 = 660.0;
const REGISTRY_NARROW_TABLE_MIN_WIDTH: f32 = 640.0;
const REGISTRY_COLUMN_COUNT: f32 = 5.0;
const REGISTRY_ROW_HEIGHT: f32 = 28.0;
const REGISTRY_HEADER_HEIGHT: f32 = 27.0;
const KBD_HEIGHT: f32 = 18.0;
const KBD_HORIZONTAL_PADDING: f32 = 4.0;
const KBD_MIN_WIDTH: f32 = 19.0;
const KBD_CORNER_RADIUS: u8 = 3;
const KEY_CAPTURE_MIN_WIDTH: f32 = 74.0;

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
    if actionable_scope_strip(
        ui,
        "User profile",
        "portable bindings with platform and organization exceptions",
        "View resolved policy\u{2026}",
    ) {
        actions.open_resolved_shortcut_policy = true;
    }
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

    setting_row(
        ui,
        "Portable profile",
        "Import or export stable command identifiers and platform mappings.",
        |ui| {
            right_aligned(ui, |ui| {
                let phone = ui.ctx().content_rect().width() <= 560.0;
                let width = ui.available_width();
                let action_height = if phone {
                    44.0
                } else {
                    Tokens::get(ui.ctx()).metrics.ctl_h
                };
                ui.allocate_ui_with_layout(
                    egui::vec2(width, action_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        if !phone {
                            let actions_width = portable_actions_width(ui);
                            ui.add_space((width - actions_width).max(0.0));
                        }
                        if ui
                            .push_id("preferences.shortcuts.import", |ui| {
                                Button::new("Import\u{2026}")
                                    .min_height(action_height)
                                    .show(ui)
                                    .clicked()
                            })
                            .inner
                        {
                            actions.open_shortcut_import = true;
                        }
                        if ui
                            .push_id("preferences.shortcuts.export", |ui| {
                                Button::new("Export\u{2026}")
                                    .min_height(action_height)
                                    .show(ui)
                                    .clicked()
                            })
                            .inner
                        {
                            actions.open_shortcut_export = true;
                        }
                    },
                );
            });
        },
    );

    section_label(ui, "Binding policy");
    // A browser commit is asynchronous. Keep showing the candidate selected
    // by the user while the persist-before-live owner has the controls locked.
    let policy_pending = state.dialogs.shortcut_policy_candidate.is_some();
    let mut policy_draft = state
        .dialogs
        .shortcut_policy_candidate
        .as_ref()
        .unwrap_or_else(|| state.ui.preferences.shortcuts())
        .clone();
    ui.add_enabled_ui(!policy_pending, |ui| {
        render_policy_rows(ui, &mut policy_draft);
    });
    queue_policy_candidate(
        actions,
        state.ui.preferences.shortcuts(),
        policy_draft,
        policy_pending,
    );
}

fn portable_actions_width(ui: &mut Ui) -> f32 {
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let text_width = ui.fonts_mut(|fonts| {
        ["Import\u{2026}", "Export\u{2026}"]
            .iter()
            .map(|label| {
                fonts
                    .layout_no_wrap((*label).to_owned(), font.clone(), egui::Color32::WHITE)
                    .size()
                    .x
            })
            .sum::<f32>()
    });
    text_width + 40.0 + 6.0
}

fn queue_policy_candidate(
    actions: &mut PreferencePageActions,
    live: &ShortcutPreferences,
    candidate: ShortcutPreferences,
    pending: bool,
) {
    if !pending && candidate != *live {
        actions.shortcut_policy_candidate = Some(candidate);
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
                ui.available_width().min(360.0),
            );
        });
    });
    picked
}

fn render_registry_table(ui: &mut Ui, profile: &ShortcutPreferences) {
    let audit = profile.audit();
    let operating_system = ui.ctx().os();
    let viewport_width = ui.available_width().max(1.0);
    let table_width = if viewport_width <= ENGINEERING_TABLE_BREAKPOINT {
        viewport_width.max(REGISTRY_NARROW_TABLE_MIN_WIDTH)
    } else {
        viewport_width
    };
    let column_width = table_width / REGISTRY_COLUMN_COUNT;
    let table_response = ui
        .vertical(|ui| {
            ui.set_width(viewport_width);
            egui::ScrollArea::horizontal()
                .id_salt("shortcut-registry-table-scroll")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_min_width(table_width);
                    ui.vertical(|ui| {
                        ui.set_width(table_width);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        semantic_table_row(ui, "Shortcut registry column headings", |ui| {
                            for heading in [
                                "Command",
                                "Desktop primary",
                                "Browser / touch alternate",
                                "Context",
                                "Status",
                            ] {
                                table_text_cell(
                                    ui,
                                    heading,
                                    true,
                                    false,
                                    column_width,
                                    REGISTRY_HEADER_HEIGHT,
                                    TableCellTone::Normal,
                                );
                            }
                        });
                        for (command, label) in REGISTRY_ROWS.iter().copied() {
                            ui.push_id(command.stable_id(), |ui| {
                                semantic_table_row(ui, label, |ui| {
                                    table_text_cell(
                                        ui,
                                        label,
                                        false,
                                        false,
                                        column_width,
                                        REGISTRY_ROW_HEIGHT,
                                        TableCellTone::Normal,
                                    );
                                    table_kbd(
                                        ui,
                                        &binding_for_platform(
                                            profile,
                                            command,
                                            ShortcutBindingSlot::Primary,
                                            CommandPlatform::Desktop,
                                            operating_system,
                                        ),
                                        column_width,
                                        REGISTRY_ROW_HEIGHT,
                                    );
                                    table_kbd(
                                        ui,
                                        &binding_for_platform(
                                            profile,
                                            command,
                                            ShortcutBindingSlot::Alternate,
                                            CommandPlatform::Browser,
                                            operating_system,
                                        ),
                                        column_width,
                                        REGISTRY_ROW_HEIGHT,
                                    );
                                    table_text_cell(
                                        ui,
                                        command.shortcut_context().label(),
                                        false,
                                        true,
                                        column_width,
                                        REGISTRY_ROW_HEIGHT,
                                        TableCellTone::Normal,
                                    );
                                    table_text_cell(
                                        ui,
                                        registry_status(&audit, command),
                                        false,
                                        false,
                                        column_width,
                                        REGISTRY_ROW_HEIGHT,
                                        registry_status_tone(&audit, command),
                                    );
                                });
                            });
                        }
                    })
                    .response
                });
        })
        .response;
    table_response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            "Protected shortcut registry",
        )
    });
    ui.ctx().accesskit_node_builder(table_response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Protected shortcut registry");
    });
    ui.add_space(7.0);
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(24, 0))
        .show(ui, |ui| audit_status(ui, profile, &audit));
    ui.add_space(9.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableCellTone {
    Normal,
    Ok,
    Warn,
    Error,
}

fn semantic_table_row(ui: &mut Ui, label: &str, add_cells: impl FnOnce(&mut Ui)) {
    let response = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            add_cells(ui);
        })
        .response;
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(label);
    });
}

fn table_text_cell(
    ui: &mut Ui,
    text: &str,
    header: bool,
    mono: bool,
    width: f32,
    height: f32,
    tone: TableCellTone,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(
            tokens::FS_0,
            if header {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            },
        )
    };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), text));
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let color = if header {
        t.color.text_faint
    } else {
        match tone {
            TableCellTone::Normal => t.color.text_dim,
            TableCellTone::Ok => t.color.ok,
            TableCellTone::Warn => t.color.warn,
            TableCellTone::Error => t.color.err,
        }
    };
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = (width - 16.0).max(1.0);
    job.append(
        &if header {
            text.to_uppercase()
        } else {
            text.to_owned()
        },
        0.0,
        egui::TextFormat {
            font_id: font,
            color,
            extra_letter_spacing: if header { 0.04 * tokens::FS_0 } else { 0.0 },
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let clip = rect.shrink2(egui::vec2(8.0, 0.0));
    ui.painter().with_clip_rect(clip).galley(
        egui::pos2(clip.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if header {
            egui::accesskit::Role::ColumnHeader
        } else {
            egui::accesskit::Role::Cell
        });
        node.set_label(text);
    });
    response
}

fn table_kbd(ui: &mut Ui, text: &str, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (cell, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), text));
    ui.painter().hline(
        cell.x_range(),
        cell.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let galley =
        ui.fonts_mut(|fonts| fonts.layout_no_wrap(text.to_owned(), font, t.color.text_faint));
    let capsule_width = (galley.size().x + KBD_HORIZONTAL_PADDING * 2.0)
        .max(KBD_MIN_WIDTH)
        .min((width - 16.0).max(KBD_MIN_WIDTH));
    let capsule = egui::Rect::from_min_size(
        egui::pos2(cell.left() + 8.0, cell.center().y - KBD_HEIGHT * 0.5),
        egui::vec2(capsule_width, KBD_HEIGHT),
    );
    ui.painter().rect_filled(
        capsule,
        egui::CornerRadius::same(KBD_CORNER_RADIUS),
        t.color.bg_panel,
    );
    ui.painter().rect_stroke(
        capsule,
        egui::CornerRadius::same(KBD_CORNER_RADIUS),
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    ui.painter().with_clip_rect(capsule).galley(
        egui::pos2(
            capsule.left() + KBD_HORIZONTAL_PADDING,
            capsule.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text_faint,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Cell);
        node.set_label(text);
        node.set_description("Keyboard binding capsule");
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

fn registry_status_tone(audit: &ShortcutProfileAudit, command: Command) -> TableCellTone {
    let mut tone = TableCellTone::Ok;
    for issue in audit_issues_for_command(audit, command) {
        match issue.severity() {
            ShortcutProfileIssueSeverity::Error => return TableCellTone::Error,
            ShortcutProfileIssueSeverity::Warning => tone = TableCellTone::Warn,
        }
    }
    tone
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

    if !state.dialogs.shortcut_editor.persistence_pending {
        process_key_capture(ctx, &mut state.dialogs.shortcut_editor);
        synchronize_editor_validation(&mut state.dialogs.shortcut_editor);
    }

    let save_enabled = {
        let editor = &state.dialogs.shortcut_editor;
        let audit = editor
            .draft
            .as_ref()
            .expect("open editor owns a draft")
            .audit();
        let errors = blocking_issue_count(&audit);
        editor.dirty && editor.recording.is_none() && errors == 0 && !editor.persistence_pending
    };

    let choice = {
        let editor = &mut state.dialogs.shortcut_editor;
        let mut scroll_offset = editor.body_scroll_offset;
        let cancel_label = if editor.discard_confirmation {
            "Discard changes"
        } else {
            "Cancel"
        };
        let transaction_state = if editor.persistence_pending {
            Some((
                DialogTransactionTone::Progress,
                "Saving shortcut profile",
                "Controls are locked until durable storage acknowledges the new profile."
                    .to_owned(),
            ))
        } else if editor.discard_confirmation {
            Some((
                DialogTransactionTone::Error,
                "Unsaved shortcut changes",
                "Discard changes closes this editor without applying the draft.".to_owned(),
            ))
        } else if let Some(summary) = editor.error_summary.as_deref() {
            Some((
                DialogTransactionTone::Error,
                "Shortcut profile requires attention",
                summary.to_owned(),
            ))
        } else {
            editor.repair_receipt.as_deref().map(|receipt| {
                (
                    DialogTransactionTone::Complete,
                    "Incompatible entries removed",
                    receipt.to_owned(),
                )
            })
        };
        let mut dialog = Dialog::new(
            "PREFERENCES \u{00b7} COMMAND BINDINGS \u{00b7} CONFLICT SAFE",
            "Keyboard shortcut editor",
            "Save shortcut profile",
        )
        .description(
            "Edit context-aware command bindings, resolve conflicts, and save one validated portable shortcut profile.",
        )
        .size(DialogSize::CapabilityReview)
        .primary_enabled(save_enabled)
        .primary_on_enter(false)
        .ghost(cancel_label)
        .body_scroll_offset(&mut scroll_offset)
        .flush_body();
        if let Some((tone, title, detail)) = transaction_state.as_ref() {
            dialog = dialog.transaction_state(*tone, title, detail);
        }
        let choice = dialog.show(ctx, |ui| {
            ui.add_enabled_ui(!editor.persistence_pending, |ui| {
                render_editor_body(ui, editor);
            });
        });
        editor.body_scroll_offset = scroll_offset;
        choice
    };

    match choice {
        DialogChoice::Primary => commit_editor(ctx, state),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if state.dialogs.shortcut_editor.persistence_pending {
                #[cfg(target_arch = "wasm32")]
                if state.cancel_pending_shortcut_library_publication() {
                    state.shortcut_library_publication_continuation = None;
                    state.dialogs.shortcut_editor.close_and_discard();
                } else {
                    state.dialogs.shortcut_editor.error_summary = Some(
                        "The durable commit boundary was already reached. Keep this editor open while the saved profile is installed."
                            .to_owned(),
                    );
                }
            } else {
                request_editor_cancel(state);
            }
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn render_editor_body(ui: &mut Ui, editor: &mut ShortcutEditorState) {
    editor_toolbar(ui, editor);

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
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
            if narrow {
                ui.vertical(|ui| toolbar_controls(ui, editor, true));
            } else {
                ui.horizontal(|ui| toolbar_controls(ui, editor, false));
            }
        })
        .response;
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            "Shortcut editor controls",
        )
    });
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label("Shortcut editor controls");
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
}

fn editor_table(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    audit: &ShortcutProfileAudit,
    commands: &[Command],
) {
    let viewport_width = ui.available_width().max(1.0);
    let narrow = viewport_width <= ENGINEERING_TABLE_BREAKPOINT;
    let table_width = if narrow {
        viewport_width.max(EDITOR_NARROW_TABLE_MIN_WIDTH)
    } else {
        viewport_width
    };
    let column_width = table_width / 6.0;
    let row_height = Tokens::get(ui.ctx()).metrics.ctl_h.max(REGISTRY_ROW_HEIGHT);
    egui::ScrollArea::horizontal()
        .id_salt("shortcut-editor-table-scroll")
        .show(ui, |ui| {
            ui.set_min_width(table_width);
            ui.set_width(table_width);
            ui.spacing_mut().item_spacing.y = 0.0;
            let table = ui
                .vertical(|ui| {
                    semantic_table_row(ui, "Shortcut editor column headings", |ui| {
                        for heading in [
                            "Command",
                            "Context",
                            "Primary binding",
                            "Alternate",
                            "Platform",
                            "Status",
                        ] {
                            table_text_cell(
                                ui,
                                heading,
                                true,
                                false,
                                column_width,
                                REGISTRY_HEADER_HEIGHT,
                                TableCellTone::Normal,
                            );
                        }
                    });
                    for command in commands.iter().copied() {
                        ui.push_id(command.stable_id(), |ui| {
                            semantic_table_row(ui, command.spec().label, |ui| {
                                let selected = editor.selected_command == Some(command);
                                let response = selectable_table_cell(
                                    ui,
                                    command.spec().label,
                                    selected,
                                    column_width,
                                    row_height,
                                );
                                if response.clicked() {
                                    editor.selected_command = Some(command);
                                }
                                table_text_cell(
                                    ui,
                                    editor_context(command).label(),
                                    false,
                                    true,
                                    column_width,
                                    row_height,
                                    TableCellTone::Normal,
                                );
                                capture_table_cell(
                                    ui,
                                    editor,
                                    command,
                                    ShortcutBindingSlot::Primary,
                                    column_width,
                                    row_height,
                                );
                                capture_table_cell(
                                    ui,
                                    editor,
                                    command,
                                    ShortcutBindingSlot::Alternate,
                                    column_width,
                                    row_height,
                                );
                                table_text_cell(
                                    ui,
                                    &platform_summary(
                                        editor.draft.as_ref().expect("editor draft"),
                                        command,
                                    ),
                                    false,
                                    true,
                                    column_width,
                                    row_height,
                                    TableCellTone::Normal,
                                );
                                table_text_cell(
                                    ui,
                                    editor_status(audit, command),
                                    false,
                                    false,
                                    column_width,
                                    row_height,
                                    editor_status_tone(audit, command),
                                );
                            });
                        });
                    }
                })
                .response;
            table.widget_info(|| {
                WidgetInfo::labeled(
                    WidgetType::Label,
                    ui.is_enabled(),
                    "Editable command bindings",
                )
            });
            ui.ctx().accesskit_node_builder(table.id, |node| {
                node.set_role(egui::accesskit::Role::Table);
                node.set_label("Editable command bindings");
            });
        });
}

fn selectable_table_cell(
    ui: &mut Ui,
    text: &str,
    selected: bool,
    width: f32,
    height: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::SelectableLabel, ui.is_enabled(), selected, text)
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
    } else if response.hovered() && ui.is_enabled() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley =
        ui.fonts_mut(|fonts| fonts.layout_no_wrap(text.to_owned(), font, t.color.text_dim));
    let clip = rect.shrink2(egui::vec2(8.0, 0.0));
    ui.painter().with_clip_rect(clip).galley(
        egui::pos2(clip.left(), rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Cell);
        node.set_label(text);
        node.set_selected(selected);
    });
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
}

fn capture_table_cell(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    command: Command,
    slot: ShortcutBindingSlot,
    width: f32,
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            format!("{} binding", slot.label()),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let content = rect.shrink2(egui::vec2(8.0, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.set_clip_rect(rect.intersect(ui.clip_rect()));
    capture_button(&mut child, editor, command, slot, content.width());
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Cell);
        node.set_label(format!("{} binding", slot.label()));
    });
}

fn capture_button(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    command: Command,
    slot: ShortcutBindingSlot,
    max_width: f32,
) -> Response {
    let target = ShortcutCaptureTarget { command, slot };
    let is_recording = editor.recording == Some(target);
    let resolved_binding = binding_for_slot(
        editor.draft.as_ref().expect("editor draft"),
        command,
        slot,
        ui.ctx().os(),
    );
    let empty = !is_recording && resolved_binding.is_none();
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
        resolved_binding.unwrap_or_else(|| "Record\u{2026}".to_owned())
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
    let response = key_capture_control(ui, &label, &accessible, is_recording, empty, max_width);
    if response.clicked() {
        editor.selected_command = Some(command);
        editor.recording = Some(target);
        editor.capture_strokes.clear();
        editor.capture_last_input_at = None;
        editor.error_summary = None;
        response.request_focus();
    }
    if ui.is_enabled() && editor.focus_error == Some(target) {
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

fn key_capture_control(
    ui: &mut Ui,
    label: &str,
    accessible: &str,
    recording: bool,
    empty: bool,
    max_width: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let color = if empty {
        t.color.text_faint
    } else {
        t.color.text
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(label.to_owned(), font, color));
    let width = (galley.size().x + 14.0)
        .max(KEY_CAPTURE_MIN_WIDTH)
        .min(max_width.max(1.0));
    let height = if ui.ctx().content_rect().width() <= 560.0 {
        44.0
    } else {
        t.metrics.ctl_h.max(25.0)
    };
    let (rect, mut response) = ui.allocate_exact_size(
        egui::vec2(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, accessible));
    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        enabled && response.hovered(),
        ui.style().animation_time,
    );
    let fill = if recording {
        mix(t.color.bg_inset, t.color.accent_dim, 0.65)
    } else if response.is_pointer_button_down_on() {
        t.color.bg_active
    } else {
        mix(t.color.bg_inset, t.color.bg_hover, hover)
    };
    let opacity = if enabled { 1.0 } else { 0.4 };
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(2),
        fill.gamma_multiply(opacity),
    );
    let border_color = t.color.border_strong.gamma_multiply(opacity);
    if empty {
        paint_dashed_rect(ui.painter(), rect, border_color);
    } else {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(2),
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );
    }
    ui.painter().with_clip_rect(rect).galley(
        egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        color.gamma_multiply(opacity),
    );
    if enabled {
        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
}

fn paint_dashed_rect(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let stroke = Stroke::new(1.0, color);
    let step = 5.0;
    let dash = 3.0;
    let mut x = rect.left() + 1.0;
    while x < rect.right() - 1.0 {
        let end = (x + dash).min(rect.right() - 1.0);
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(end, rect.top())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(end, rect.bottom())],
            stroke,
        );
        x += step;
    }
    let mut y = rect.top() + 1.0;
    while y < rect.bottom() - 1.0 {
        let end = (y + dash).min(rect.bottom() - 1.0);
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.left(), end)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.right(), y), egui::pos2(rect.right(), end)],
            stroke,
        );
        y += step;
    }
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
    editor.discard_confirmation = false;
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

fn editor_status_tone(audit: &ShortcutProfileAudit, command: Command) -> TableCellTone {
    registry_status_tone(audit, command)
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
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), "Shortcut editor notes")
    });
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
    ui.add_space(10.0);
    let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
    let t = Tokens::get(ui.ctx());
    let outer = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            if narrow {
                let first = editor_note(
                    ui,
                    "Conflict resolution",
                    "Bindings are validated by context and platform. Protected browser and operating-system shortcuts require an explicit alternate.",
                );
                ui.painter().hline(
                    first.rect.x_range(),
                    first.rect.bottom(),
                    Stroke::new(1.0, t.color.border_strong),
                );
                editor_note(
                    ui,
                    "Reset scope",
                    "Reset one command, one context, or the complete user profile without changing organization-managed bindings.",
                );
            } else {
                ui.columns(2, |columns| {
                    editor_note(
                        &mut columns[0],
                        "Conflict resolution",
                        "Bindings are validated by context and platform. Protected browser and operating-system shortcuts require an explicit alternate.",
                    );
                    editor_note(
                        &mut columns[1],
                        "Reset scope",
                        "Reset one command, one context, or the complete user profile without changing organization-managed bindings.",
                    );
                });
            }
        })
        .response;
    if !narrow {
        ui.painter().vline(
            outer.rect.center().x,
            outer.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
    ui.ctx().accesskit_node_builder(outer.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label("Shortcut editor notes");
    });
}

fn editor_note(ui: &mut Ui, title: &str, body: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::NONE
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
        })
        .response;
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label(title);
        node.set_description(body);
    });
    response
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

fn commit_editor(ctx: &Context, state: &mut AppState) {
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
        .as_ref()
        .cloned()
        .expect("editor draft");
    let mut candidate = state.ui.preferences.shortcut_profiles().clone();
    if let Err(error) = candidate.replace_active(draft) {
        state.dialogs.shortcut_editor.error_summary =
            Some(format!("Shortcut profile was not saved: {error}"));
        state.push_console_message(ConsoleMessage::error(format!(
            "Shortcut profile was not saved: {error}"
        )));
        return;
    }
    if state.shortcut_library_publication_continuation.is_some() {
        state.complete_shortcut_library_publication(
            super::ShortcutLibraryPublicationContinuation::Editor,
            Err("another shortcut-library publication is awaiting acknowledgement".to_owned()),
        );
        return;
    }
    match state.publish_shortcut_library_candidate(&candidate, ctx) {
        Ok(super::ShortcutLibraryPublication::Published) => state
            .complete_shortcut_library_publication(
                super::ShortcutLibraryPublicationContinuation::Editor,
                Ok(()),
            ),
        Ok(super::ShortcutLibraryPublication::Pending) => {
            state.dialogs.shortcut_editor.persistence_pending = true;
            state.shortcut_library_publication_continuation =
                Some(super::ShortcutLibraryPublicationContinuation::Editor);
        }
        Err(error) => state.complete_shortcut_library_publication(
            super::ShortcutLibraryPublicationContinuation::Editor,
            Err(error),
        ),
    }
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
    if state.dialogs.shortcut_editor.dirty && !state.dialogs.shortcut_editor.discard_confirmation {
        state.dialogs.shortcut_editor.discard_confirmation = true;
    } else {
        state.dialogs.shortcut_editor.close_and_discard();
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
        let first = ctx.run(input(1_100.0, 900.0), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_page(ui, &mut state, &mut actions);
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
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: input(1_100.0, 900.0).screen_rect,
                    events,
                    ..egui::RawInput::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        render_page(ui, &mut state, &mut actions);
                    });
                },
            );
        }
        actions
    }

    #[test]
    fn page_registry_is_the_exact_mockup_review_projection() {
        assert_eq!(REGISTRY_ROWS.len(), 11);
        assert_eq!(REGISTRY_ROWS[0].1, "Command palette");
        assert_eq!(REGISTRY_ROWS[10].1, "Toggle console");
        assert_eq!(ENGINEERING_TABLE_BREAKPOINT, 820.0);
        assert_eq!(EDITOR_NARROW_TABLE_MIN_WIDTH, 660.0);
        assert_eq!(REGISTRY_NARROW_TABLE_MIN_WIDTH, 640.0);
        assert_eq!(REGISTRY_COLUMN_COUNT, 5.0);
        assert_eq!(REGISTRY_HEADER_HEIGHT, 27.0);
        assert_eq!(REGISTRY_ROW_HEIGHT, 28.0);
        assert_eq!(KBD_HEIGHT, 18.0);
        assert_eq!(KBD_HORIZONTAL_PADDING, 4.0);
        assert_eq!(KBD_MIN_WIDTH, 19.0);
        assert_eq!(KBD_CORNER_RADIUS, 3);
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
    fn shortcuts_category_is_exposed_after_review_workflows_land() {
        assert!(
            super::super::preferences_shell::PreferenceCategory::ALL
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
        state.enable_volatile_test_shortcut_persistence();
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

        commit_editor(&Context::default(), &mut state);

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

        commit_editor(&Context::default(), &mut state);

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
        for label in ["Binding editor", "Portable profile"] {
            assert!(nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Label && node.label() == Some(label)
            }));
        }
        for label in [
            "View resolved policy\u{2026}",
            "Edit shortcuts\u{2026}",
            "Import\u{2026}",
            "Export\u{2026}",
        ] {
            assert!(nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }));
        }
        let portable = node_bounds(&nodes, egui::accesskit::Role::Label, "Portable profile");
        let heading = node_bounds(
            &nodes,
            egui::accesskit::Role::Heading,
            "Keyboard and command shortcuts",
        );
        let registry = node_bounds(
            &nodes,
            egui::accesskit::Role::Table,
            "Protected shortcut registry",
        );
        let resolved = node_bounds(
            &nodes,
            egui::accesskit::Role::Button,
            "View resolved policy\u{2026}",
        );
        let import = node_bounds(&nodes, egui::accesskit::Role::Button, "Import\u{2026}");
        let export = node_bounds(&nodes, egui::accesskit::Role::Button, "Export\u{2026}");
        assert!(registry.x0 < heading.x0);
        assert!(registry.x1 > heading.x1);
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
                .count(),
            5
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| {
                    node.role() == egui::accesskit::Role::Row
                        && (node.label() == Some("Shortcut registry column headings")
                            || REGISTRY_ROWS
                                .iter()
                                .any(|(_, label)| node.label() == Some(*label)))
                })
                .count(),
            REGISTRY_ROWS.len() + 1
        );
        assert!(
            nodes
                .iter()
                .filter(|(_, node)| {
                    node.role() == egui::accesskit::Role::Cell
                        && node.description() == Some("Keyboard binding capsule")
                })
                .count()
                >= REGISTRY_ROWS.len() * 2
        );
        assert!(resolved.x0 > portable.x0);
        assert!(import.x0 > portable.x1);
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

        let resolved = click_page_button("View resolved policy\u{2026}");
        assert!(resolved.open_resolved_shortcut_policy);
        assert!(!resolved.open_shortcut_import);
        assert!(!resolved.open_shortcut_export);
        assert!(!resolved.open_shortcut_editor);
    }

    #[test]
    fn phone_page_keeps_the_registry_fluid_and_portable_actions_in_source_order() {
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
        let registry = node_bounds(
            &nodes,
            egui::accesskit::Role::Table,
            "Protected shortcut registry",
        );
        assert!(registry.x0 >= 0.0);
        assert!(registry.x1 <= 390.0);
        assert!(registry.x1 - registry.x0 >= 360.0);
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Row && node.label() == Some("Command palette")
        }));
        let import = node_bounds(&nodes, egui::accesskit::Role::Button, "Import\u{2026}");
        let export = node_bounds(&nodes, egui::accesskit::Role::Button, "Export\u{2026}");
        assert!(import.x0 < export.x0);
        assert_eq!(import.y0, export.y0);
        assert!(import.y1 - import.y0 >= 44.0);
        let import_index = nodes
            .iter()
            .position(|(_, node)| {
                node.role() == egui::accesskit::Role::Button
                    && node.label() == Some("Import\u{2026}")
            })
            .unwrap();
        let export_index = nodes
            .iter()
            .position(|(_, node)| {
                node.role() == egui::accesskit::Role::Button
                    && node.label() == Some("Export\u{2026}")
            })
            .unwrap();
        assert!(import_index < export_index);
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
            node.role() == egui::accesskit::Role::Group
                && node.label() == Some("Shortcut editor notes")
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

    #[test]
    fn pending_editor_is_immutable_and_exposes_a_disabled_progress_state() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let profile = state.ui.preferences.shortcuts().clone();
        state.dialogs.shortcut_editor.open(&profile);
        state.dialogs.shortcut_editor.query = "save".to_owned();
        state.dialogs.shortcut_editor.selected_command = Some(Command::Save);
        state.dialogs.shortcut_editor.recording = Some(ShortcutCaptureTarget {
            command: Command::Save,
            slot: ShortcutBindingSlot::Primary,
        });
        state
            .dialogs
            .shortcut_editor
            .capture_strokes
            .push(ShortcutStroke::new(Key::K, true, false, false));
        state.dialogs.shortcut_editor.capture_last_input_at = Some(1.0);
        state.dialogs.shortcut_editor.dirty = true;
        state.dialogs.shortcut_editor.persistence_pending = true;
        let before = state.dialogs.shortcut_editor.clone();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: input(1_100.0, 900.0).screen_rect,
                events: vec![egui::Event::Key {
                    key: Key::F6,
                    physical_key: Some(Key::F6),
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                }],
                ..egui::RawInput::default()
            },
            |ctx| render_editor(ctx, &mut state),
        );

        let after = &state.dialogs.shortcut_editor;
        assert_eq!(after.draft, before.draft);
        assert_eq!(after.query, before.query);
        assert_eq!(after.context, before.context);
        assert_eq!(after.selected_command, before.selected_command);
        assert_eq!(after.recording, before.recording);
        assert_eq!(after.capture_strokes, before.capture_strokes);
        assert_eq!(after.capture_last_input_at, before.capture_last_input_at);
        assert_eq!(after.dirty, before.dirty);
        assert_eq!(after.error_summary, before.error_summary);
        assert_eq!(after.focus_error, before.focus_error);
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Status
                && node.label() == Some("Saving shortcut profile")
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button
                && node
                    .label()
                    .is_some_and(|label| label.starts_with("Record Primary binding for"))
                && node.is_disabled()
        }));
    }
}
