//! Mockup-defined shortcut Preferences page and transactional binding editor.

pub(crate) mod capture;

use capture::*;

use std::time::Duration;

use egui::{Context, Key, Modifiers, Response, Stroke, Ui, WidgetInfo, WidgetType};

use crate::state::command_catalog::{CommandCatalog, CommandKey, CommandPlatform, ShortcutContext};
use crate::state::shortcuts::{
    ChordTimeoutPolicy, MAX_SHORTCUT_SEQUENCE_STROKES, ProtectedShortcutPolicy,
    ShortcutBindingSlot, ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue,
    ShortcutProfileIssueCode, ShortcutProfileIssueSeverity, ShortcutSequence, ShortcutStroke,
    SingleKeyCanvasPolicy,
};
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone, Problem, select,
};
use crate::workbench::app::dialogs::review_primitives::problem_note;
use crate::workbench::framework::command_registry::Registry;

use super::preferences_shell::{
    action_buttons, control_width, preference_row, searchable_row, section_heading, value_column,
};
use super::{PreferencePageActions, PreferencesDialogs};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ShortcutEditorContext {
    #[default]
    All,
    Global,
    Schematic,
    Simulation,
    Results,
    Verification,
    SourceEditor,
}

impl ShortcutEditorContext {
    pub const ALL: [Self; 7] = [
        Self::All,
        Self::Global,
        Self::Schematic,
        Self::Simulation,
        Self::Results,
        Self::Verification,
        Self::SourceEditor,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::All => "All contexts",
            Self::Global => "Global",
            Self::Schematic => "Schematic",
            Self::Simulation => "Simulation",
            Self::Results => "Results",
            Self::Verification => "Verification",
            Self::SourceEditor => "Source editor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutCaptureTarget {
    pub command: CommandKey,
    pub slot: crate::state::shortcuts::ShortcutBindingSlot,
}

/// Retained transaction state for the command-binding editor. Visible
/// controls mutate `draft`; the live profile is replaced only by validated
/// Save.
#[derive(Debug, Clone, Default)]
pub struct ShortcutEditorState {
    pub open: bool,
    pub original: Option<crate::state::shortcuts::ShortcutPreferences>,
    pub draft: Option<crate::state::shortcuts::ShortcutPreferences>,
    pub query: String,
    pub context: ShortcutEditorContext,
    pub selected_command: Option<CommandKey>,
    pub recording: Option<ShortcutCaptureTarget>,
    pub capture_strokes: Vec<crate::state::shortcuts::ShortcutStroke>,
    pub capture_last_input_at: Option<f64>,
    pub dirty: bool,
    /// A browser CAS is in flight. The draft remains isolated and the editor
    /// cannot close or launch a second publication until completion.
    pub persistence_pending: bool,
    pub discard_confirmation: bool,
    pub error_summary: Option<String>,
    pub repair_receipt: Option<String>,
    pub focus_error: Option<ShortcutCaptureTarget>,
    pub body_scroll_offset: f32,
}

impl ShortcutEditorState {
    pub fn open(&mut self, profile: &crate::state::shortcuts::ShortcutPreferences) {
        *self = Self {
            open: true,
            original: Some(profile.clone()),
            draft: Some(profile.clone()),
            ..Self::default()
        };
    }

    pub fn close_and_discard(&mut self) {
        *self = Self::default();
    }
}

const RESPONSIVE_TABLE_BREAKPOINT: f32 = 760.0;
const ENGINEERING_TABLE_BREAKPOINT: f32 = 820.0;
pub const EDITOR_NARROW_TABLE_MIN_WIDTH: f32 = 660.0;
const REGISTRY_ROW_HEIGHT: f32 = 28.0;
const REGISTRY_HEADER_HEIGHT: f32 = 27.0;
const KEY_CAPTURE_MIN_WIDTH: f32 = 74.0;

pub fn render_page(
    ui: &mut Ui,
    state: &mut PreferencesDialogs<'_>,
    actions: &mut PreferencePageActions,
) {
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
        |ui| match action_buttons(
            ui,
            &[
                ("preferences.shortcuts.import", "Import\u{2026}"),
                ("preferences.shortcuts.export", "Export\u{2026}"),
            ],
        ) {
            Some(0) => actions.open_shortcut_import = true,
            Some(_) => actions.open_shortcut_export = true,
            None => {}
        },
    );

    section_heading(ui, "Key handling");
    // A browser commit is asynchronous. Keep showing the candidate selected
    // by the user while the persist-before-live owner has the controls locked.
    let policy_pending = state.shortcut_policy_candidate.is_some();
    let mut policy_draft = state
        .shortcut_policy_candidate
        .as_ref()
        .unwrap_or_else(|| state.model.preferences.shortcuts())
        .clone();
    ui.add_enabled_ui(!policy_pending, |ui| {
        render_policy_rows(ui, &mut policy_draft);
    });
    queue_policy_candidate(
        actions,
        state.model.preferences.shortcuts(),
        policy_draft,
        policy_pending,
    );
}

pub fn queue_policy_candidate(
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

fn registry_status_tone(audit: &ShortcutProfileAudit, command: CommandKey) -> TableCellTone {
    if audit_issues_for_command(audit, command)
        .any(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
    {
        TableCellTone::Error
    } else {
        TableCellTone::Normal
    }
}

pub fn render_editor(ctx: &Context, state: &mut PreferencesDialogs<'_>) {
    if !state.shortcut_editor.open {
        return;
    }

    if !state.shortcut_editor.persistence_pending {
        process_key_capture(ctx, state.shortcut_editor, state.commands);
        synchronize_editor_validation(state.shortcut_editor, state.commands);
    }

    let save_enabled = {
        let editor = &state.shortcut_editor;
        let audit = editor
            .draft
            .as_ref()
            .expect("open editor owns a draft")
            .audit(state.commands);
        let errors = blocking_issue_count(&audit);
        editor.dirty && editor.recording.is_none() && errors == 0 && !editor.persistence_pending
    };

    let choice = {
        let editor = &mut state.shortcut_editor;
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
                render_editor_body(ui, editor, state.commands);
            });
        });
        editor.body_scroll_offset = scroll_offset;
        choice
    };

    match choice {
        DialogChoice::Primary => commit_editor(ctx, state),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            if state.shortcut_editor.persistence_pending {
                #[cfg(target_arch = "wasm32")]
                if state
                    .shortcut_library_parts()
                    .cancel_pending_shortcut_library_publication()
                {
                    *state.shortcut_library_publication_continuation = None;
                    state.shortcut_editor.close_and_discard();
                } else {
                    state.shortcut_editor.error_summary = Some(
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

fn render_editor_body(ui: &mut Ui, editor: &mut ShortcutEditorState, registry: &Registry) {
    editor_toolbar(ui, editor, registry);

    let audit = editor.draft.as_ref().expect("editor draft").audit(registry);
    let commands = filtered_commands(editor, ui.ctx().os(), registry);
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
        editor_table(ui, editor, &audit, &commands, registry);
    }

    if !audit.issues().is_empty() {
        conflict_list(ui, editor, &audit, registry);
    }
    editor_notes(ui);
}

fn editor_toolbar(ui: &mut Ui, editor: &mut ShortcutEditorState, registry: &Registry) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            let narrow = ui.available_width() <= RESPONSIVE_TABLE_BREAKPOINT;
            if narrow {
                ui.vertical(|ui| toolbar_controls(ui, editor, true, registry));
            } else {
                ui.horizontal(|ui| toolbar_controls(ui, editor, false, registry));
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

fn toolbar_controls(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    narrow: bool,
    registry: &Registry,
) {
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
            .is_some_and(|command| !context_matches(command, editor.context, registry))
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
        mark_editor_changed(editor, registry);
    }
}

fn editor_table(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    audit: &ShortcutProfileAudit,
    commands: &[CommandKey],
    registry: &Registry,
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
                        let label = registry.label(command);
                        ui.push_id(command.as_str(), |ui| {
                            semantic_table_row(ui, label, |ui| {
                                let selected = editor.selected_command == Some(command);
                                let response = selectable_table_cell(
                                    ui,
                                    label,
                                    selected,
                                    column_width,
                                    row_height,
                                );
                                if response.clicked() {
                                    editor.selected_command = Some(command);
                                }
                                table_text_cell(
                                    ui,
                                    editor_context(command, registry).label(),
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
                                    registry,
                                );
                                capture_table_cell(
                                    ui,
                                    editor,
                                    command,
                                    ShortcutBindingSlot::Alternate,
                                    column_width,
                                    row_height,
                                    registry,
                                );
                                table_text_cell(
                                    ui,
                                    &platform_summary(
                                        editor.draft.as_ref().expect("editor draft"),
                                        command,
                                        registry,
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
    command: CommandKey,
    slot: ShortcutBindingSlot,
    width: f32,
    height: f32,
    registry: &Registry,
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
    capture_button(&mut child, editor, command, slot, content.width(), registry);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Cell);
        node.set_label(format!("{} binding", slot.label()));
    });
}

fn capture_button(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    command: CommandKey,
    slot: ShortcutBindingSlot,
    max_width: f32,
    registry: &Registry,
) -> Response {
    let target = ShortcutCaptureTarget { command, slot };
    let is_recording = editor.recording == Some(target);
    let resolved_binding = binding_for_slot(
        editor.draft.as_ref().expect("editor draft"),
        command,
        slot,
        ui.ctx().os(),
        registry,
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
            registry.label(command)
        )
    } else {
        format!(
            "Record {} binding for {}",
            slot.label(),
            registry.label(command)
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

fn mark_editor_changed(editor: &mut ShortcutEditorState, registry: &Registry) {
    editor.dirty = editor.draft != editor.original;
    editor.discard_confirmation = false;
    editor.repair_receipt = None;
    synchronize_editor_validation(editor, registry);
}

fn synchronize_editor_validation(editor: &mut ShortcutEditorState, registry: &Registry) {
    let audit = editor.draft.as_ref().expect("editor draft").audit(registry);
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
            editor.focus_error = first_error_target(&audit, registry);
            if let Some(target) = editor.focus_error {
                editor.query.clear();
                editor.context = editor_context(target.command, registry);
                editor.selected_command = Some(target.command);
            }
        }
    }
}

/// The command the editor leaves out: Command reference, which lists the
/// bindings the editor edits.
pub(crate) const COMMAND_REFERENCE: CommandKey = CommandKey::new("command-reference");

pub fn filtered_commands(
    editor: &ShortcutEditorState,
    operating_system: egui::os::OperatingSystem,
    registry: &Registry,
) -> Vec<CommandKey> {
    let query = editor.query.trim().to_lowercase();
    registry
        .commands()
        .iter()
        .copied()
        .filter(|command| *command != COMMAND_REFERENCE)
        .filter(|command| context_matches(*command, editor.context, registry))
        .filter(|command| {
            if query.is_empty() {
                return true;
            }
            let bindings = editor
                .draft
                .as_ref()
                .expect("editor draft")
                .resolved_bindings(registry, *command)
                .into_iter()
                .map(|binding| binding.sequence().display_label_for(operating_system))
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "{} {} {} {}",
                registry.label(*command),
                editor_context(*command, registry).label(),
                command.as_str(),
                bindings
            )
            .to_lowercase()
            .contains(&query)
        })
        .collect()
}

/// The editor's context filter a command lists under; a key no command
/// declares lists with the global ones.
pub fn editor_context(command: CommandKey, registry: &Registry) -> ShortcutEditorContext {
    match registry.context(command) {
        Some(
            ShortcutContext::EditContext
            | ShortcutContext::EngineeringCanvas
            | ShortcutContext::SymbolCanvas
            | ShortcutContext::DesignWorkspace,
        ) => ShortcutEditorContext::Schematic,
        Some(ShortcutContext::SimulationWorkspace) => ShortcutEditorContext::Simulation,
        Some(ShortcutContext::ResultsWorkspace) => ShortcutEditorContext::Results,
        Some(ShortcutContext::VerificationWorkspace | ShortcutContext::ViolationNavigation) => {
            ShortcutEditorContext::Verification
        }
        Some(
            ShortcutContext::Global
            | ShortcutContext::ApplicationChrome
            | ShortcutContext::RunnableProject,
        )
        | None => ShortcutEditorContext::Global,
        Some(ShortcutContext::SourceEditor) => ShortcutEditorContext::SourceEditor,
    }
}

fn context_matches(
    command: CommandKey,
    filter: ShortcutEditorContext,
    registry: &Registry,
) -> bool {
    filter == ShortcutEditorContext::All || editor_context(command, registry) == filter
}

pub fn binding_for_slot(
    profile: &ShortcutPreferences,
    command: CommandKey,
    slot: ShortcutBindingSlot,
    operating_system: egui::os::OperatingSystem,
    registry: &Registry,
) -> Option<String> {
    let bindings = profile
        .resolved_bindings(registry, command)
        .into_iter()
        .filter(|binding| binding.slot() == slot)
        .map(|binding| binding.sequence().display_label_for(operating_system))
        .collect::<Vec<_>>();
    (!bindings.is_empty()).then(|| bindings.join(" / "))
}

fn platform_summary(
    profile: &ShortcutPreferences,
    command: CommandKey,
    registry: &Registry,
) -> String {
    let mut platforms = profile
        .resolved_bindings(registry, command)
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

fn editor_status(audit: &ShortcutProfileAudit, command: CommandKey) -> &str {
    audit_issues_for_command(audit, command)
        .next()
        .map_or("available", ShortcutProfileIssue::message)
}

fn editor_status_tone(audit: &ShortcutProfileAudit, command: CommandKey) -> TableCellTone {
    registry_status_tone(audit, command)
}

fn audit_issues_for_command(
    audit: &ShortcutProfileAudit,
    command: CommandKey,
) -> impl Iterator<Item = &ShortcutProfileIssue> {
    audit.issues().iter().filter(move |issue| {
        issue
            .command_id()
            .is_some_and(|id| id.as_str() == command.as_str())
    })
}

fn conflict_list(
    ui: &mut Ui,
    editor: &mut ShortcutEditorState,
    audit: &ShortcutProfileAudit,
    registry: &Registry,
) {
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
                        .and_then(|id| registry.resolve(id.as_str()))
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
                        let label = format!(
                            "Confirm protected override for {}",
                            registry.label(command)
                        );
                        if Button::new(&label).show(ui).clicked() {
                            editor
                                .draft
                                .as_mut()
                                .expect("editor draft")
                                .acknowledge_protected_override(command);
                            mark_editor_changed(editor, registry);
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
                    repair_invalid_entries(editor, registry);
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

pub fn repair_invalid_entries(editor: &mut ShortcutEditorState, registry: &Registry) {
    let removed = editor
        .draft
        .as_mut()
        .expect("editor draft")
        .remove_blocking_invalid_entries(registry);
    mark_editor_changed(editor, registry);
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

pub fn commit_editor(ctx: &Context, state: &mut PreferencesDialogs<'_>) {
    let audit = state
        .shortcut_editor
        .draft
        .as_ref()
        .expect("editor draft")
        .audit(state.commands);
    if !audit.is_valid() {
        let count = blocking_issue_count(&audit);
        state.shortcut_editor.error_summary = Some(format!(
            "Shortcut profile was not saved. Resolve {count} blocking conflict(s); focus moved to the first affected binding."
        ));
        state.shortcut_editor.focus_error = first_error_target(&audit, state.commands);
        if let Some(target) = state.shortcut_editor.focus_error {
            state.shortcut_editor.query.clear();
            state.shortcut_editor.context = editor_context(target.command, state.commands);
            state.shortcut_editor.selected_command = Some(target.command);
        }
        return;
    }
    let draft = state
        .shortcut_editor
        .draft
        .as_ref()
        .cloned()
        .expect("editor draft");
    let mut candidate = state.model.preferences.shortcut_profiles().clone();
    if let Err(error) = candidate.replace_active(draft) {
        // The editor stays open and states the refusal in place.
        state.shortcut_editor.error_summary =
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
    let commands = state.commands;
    let published = state
        .shortcut_library_parts()
        .publish_shortcut_library_candidate(&candidate, ctx, commands);
    match published {
        Ok(super::ShortcutLibraryPublication::Published) => state
            .complete_shortcut_library_publication(
                super::ShortcutLibraryPublicationContinuation::Editor,
                Ok(()),
            ),
        Ok(super::ShortcutLibraryPublication::Pending) => {
            state.shortcut_editor.persistence_pending = true;
            *state.shortcut_library_publication_continuation =
                Some(super::ShortcutLibraryPublicationContinuation::Editor);
        }
        Err(error) => state.complete_shortcut_library_publication(
            super::ShortcutLibraryPublicationContinuation::Editor,
            Err(error),
        ),
    }
}

fn first_error_target(
    audit: &ShortcutProfileAudit,
    registry: &Registry,
) -> Option<ShortcutCaptureTarget> {
    audit
        .issues()
        .iter()
        .find(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
        .and_then(|issue| {
            let command = registry.resolve(issue.command_id()?.as_str())?;
            Some(ShortcutCaptureTarget {
                command,
                slot: issue.slot().unwrap_or(ShortcutBindingSlot::Primary),
            })
        })
}

fn request_editor_cancel(state: &mut PreferencesDialogs<'_>) {
    cancel_capture(state.shortcut_editor);
    if state.shortcut_editor.dirty && !state.shortcut_editor.discard_confirmation {
        state.shortcut_editor.discard_confirmation = true;
    } else {
        state.shortcut_editor.close_and_discard();
    }
}
