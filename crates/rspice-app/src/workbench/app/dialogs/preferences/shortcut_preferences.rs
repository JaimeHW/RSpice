//! Mockup-defined shortcut Preferences page and transactional binding editor.

mod capture;

use capture::*;

use std::time::Duration;

use egui::{Context, Key, Modifiers, Response, Stroke, Ui, WidgetInfo, WidgetType};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone, Problem, select,
};
use crate::workbench::app::dialogs::review_primitives::problem_note;
use crate::workbench::commands::{
    ShortcutContext,
    vocabulary::{COMMAND_REGISTRY, Command, CommandPlatform},
};
use crate::workbench::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;
use crate::workbench::{
    ChordTimeoutPolicy, ProtectedShortcutPolicy, ShortcutBindingSlot, ShortcutPreferences,
    ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};

use super::preferences_shell::{
    control_width, preference_row, searchable_row, section_heading, value_column,
};
use super::{AppState, PreferencePageActions};
use crate::workbench::app::dialogs::state::{
    ShortcutCaptureTarget, ShortcutEditorContext, ShortcutEditorState,
};

const RESPONSIVE_TABLE_BREAKPOINT: f32 = 760.0;
const ENGINEERING_TABLE_BREAKPOINT: f32 = 820.0;
const EDITOR_NARROW_TABLE_MIN_WIDTH: f32 = 660.0;
const REGISTRY_ROW_HEIGHT: f32 = 28.0;
const REGISTRY_HEADER_HEIGHT: f32 = 27.0;
const KEY_CAPTURE_MIN_WIDTH: f32 = 74.0;

pub(super) fn render_page(ui: &mut Ui, state: &mut AppState, actions: &mut PreferencePageActions) {
    // A read-only five-column table of eleven hand-picked commands used to
    // open this page. The editor below it is the searchable list of every
    // command in the registry, so the table was a sample of the real thing
    // presented as the thing itself, and nothing on it could be changed.
    searchable_row(
        ui,
        "Key bindings",
        "Record keys, resolve conflicts, set alternates and reset individual commands.",
        &["keyboard", "shortcuts", "hotkeys", "keymap"],
        |ui| {
            value_column(ui, |ui| {
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

    preference_row(
        ui,
        "Shortcut file",
        "Share key bindings between machines or with your team.",
        |ui| {
            value_column(ui, |ui| {
                let phone = ui.ctx().content_rect().width() <= 560.0;
                let width = ui.available_width();
                let action_height = if phone {
                    44.0
                } else {
                    Tokens::get(ui.ctx()).metrics.ctl_h
                };
                // Both buttons start at the page's control column, like every
                // other value on it. They used to be pushed to the far right
                // of the row, which put the only pair of controls on the page
                // four hundred pixels clear of the column the rest share.
                ui.allocate_ui_with_layout(
                    egui::vec2(width, action_height),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
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

    section_heading(ui, "Key handling");
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
        "How long RSpice waits for the next key of a multi-key shortcut.",
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

    // "Protected platform shortcuts" and "Context precedence" rows used to
    // follow. The first only changed how loudly the editor's audit reported
    // nine commands; the second reordered a resolver nobody needs to tune.
    // Both values stay in the shortcut file at what RSpice ships.
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
    preference_row(ui, title, help, |ui| {
        value_column(ui, |ui| {
            picked = select(ui, id, title, current, &options, control_width(ui));
        });
    });
    picked
}

/// What a cell in the binding editor's table is saying about its row.
///
/// Only a failure is painted; everything else is the same neutral cell,
/// because a binding that simply works is not news.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TableCellTone {
    Normal,
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

fn registry_status_tone(audit: &ShortcutProfileAudit, command: Command) -> TableCellTone {
    if audit_issues_for_command(audit, command)
        .any(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
    {
        TableCellTone::Error
    } else {
        TableCellTone::Normal
    }
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
                "Saving",
                "Controls unlock when the shortcuts are saved.".to_owned(),
            ))
        } else if editor.discard_confirmation {
            Some((
                DialogTransactionTone::Error,
                "Unsaved shortcut changes",
                "Discard changes closes this editor without applying the draft.".to_owned(),
            ))
        } else {
            // A repair reports what it removed, in the audit list beside the
            // button that ran it; the strip is for work still to be done.
            editor.error_summary.as_deref().map(|summary| {
                (
                    DialogTransactionTone::Error,
                    "Shortcut profile requires attention",
                    summary.to_owned(),
                )
            })
        };
        let mut dialog = Dialog::new("Keyboard shortcut editor", "Save shortcut profile")
            .description("Change the keys for any command, resolve conflicts, and save.")
            .size(DialogSize::CapabilityReview)
            .primary_enabled(save_enabled)
            .primary_on_enter(false)
            .ghost(cancel_label)
            .body_scroll_offset(&mut scroll_offset)
            .flush_body();
        if let Some((tone, title, detail)) = transaction_state.as_ref() {
            dialog = dialog.transaction_state(*tone, *title, detail.as_str());
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
                        "The shortcuts are already being saved. Keep this editor open until it finishes."
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
        ShortcutContext::SourceEditor => ShortcutEditorContext::SourceEditor,
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
                problem_note(
                    ui,
                    match issue.severity() {
                        ShortcutProfileIssueSeverity::Error => Problem::Failure,
                        ShortcutProfileIssueSeverity::Warning => Problem::Warning,
                    },
                    issue.message(),
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
                    // An acknowledged override is a settled state and says
                    // nothing; only the one still to confirm has a row.
                    if issue.severity() == ShortcutProfileIssueSeverity::Error {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    "No browser alternate. Confirm before saving.",
                                )
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
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
            // Which identifiers went is the only part of a repair the rows
            // cannot show, so it is the only part that is reported.
            if let Some(removed) = editor.repair_receipt.as_deref() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(removed)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
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
        // The editor stays open and states the refusal in place.
        state.dialogs.shortcut_editor.error_summary =
            Some(format!("Shortcut profile was not saved: {error}"));
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
        let first = ctx.run_ui(input(1_100.0, 900.0), |ctx| {
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
            let _ = ctx.run_ui(
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

    /// The page shows no table of its own.
    ///
    /// It used to open with a read-only five-column projection of eleven
    /// hand-picked commands. The binding editor below it is the searchable
    /// table of every command in the registry, so the page's copy was a
    /// sample of the real list presented as the list, and nothing on it could
    /// be changed.
    #[test]
    fn the_page_does_not_reproduce_the_editors_table() {
        let source = include_str!("shortcut_preferences.rs");
        let page = source
            .split("pub(super) fn render_page(")
            .nth(1)
            .and_then(|source| source.split("\nfn ").next())
            .expect("page source");
        assert!(!page.contains("semantic_table_row"));
        assert!(!page.contains("table_text_cell"));
        assert!(!page.contains("Browser / touch alternate"));
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
        let _ = ctx.run_ui(
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
    fn desktop_page_publishes_its_rows_its_actions_and_its_policy_heading() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut actions = PreferencePageActions::default();
        let output = ctx.run_ui(input(1_100.0, 900.0), |ctx| {
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
                render_page(ui, &mut state, &mut actions);
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
        let profile = state.ui.preferences.shortcuts().clone();
        state.dialogs.shortcut_editor.open(&profile);
        let output = ctx.run_ui(input(390.0, 844.0), |ctx| render_editor(ctx, &mut state));
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
        let output = ctx.run_ui(
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
            node.role() == egui::accesskit::Role::Status && node.label() == Some("Saving")
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
