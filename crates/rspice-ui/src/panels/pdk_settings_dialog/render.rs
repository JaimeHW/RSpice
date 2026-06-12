//! Model library paths — the egui implementation of
//! `design/app/volta-dialogs-v2.html` §2.
//!
//! One master–detail surface replaces the four ad-hoc tabs: sources on the
//! left rail (enable checkbox, inline path edit, file count at a glance)
//! with the environment variables underneath — they exist to be used in
//! paths, so they belong next to them, not on a far tab — and the selected
//! source's discovered files on the right with type badges and a live
//! filter. Everything edits the working copy immediately; Rescan is the
//! only deferred action and Close commits.

use std::path::PathBuf;

use egui::{Context, Rect, Sense, Stroke, Ui, vec2};

use super::model::{PdkSettingsDialogResult, PdkSettingsDialogState};
use crate::state::pdk_config::{DiscoveredFile, LibraryPathEntry};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogSize, IconButton, PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W,
    PaneSide, chip, pane_footer, pane_header, pane_section_label, two_pane,
};

/// Height of the two-pane region.
const PANE_HEIGHT: f32 = 340.0;
/// Height of source rows.
const ROW_H: f32 = 26.0;
/// Height of environment rows.
const ENV_ROW_H: f32 = 24.0;
/// Height of discovered-file rows and the table header.
const FILE_ROW_H: f32 = 24.0;
/// Width of the SECTIONS column, measured from the right edge.
const SECTIONS_W: f32 = 132.0;
/// X offset of the FILE column.
const FILE_COL_X: f32 = 64.0;

/// What a sources-rail interaction asked for this frame.
enum SourceAction {
    /// Select (or deselect, when already selected) a source.
    Select(usize),
    /// Toggle the enable checkbox.
    ToggleEnabled(usize),
    /// Begin the inline path edit with the current path text.
    StartEdit(usize, String),
    /// Commit (or cancel, when empty) the inline path edit.
    FinishEdit(usize),
    /// Remove a source.
    Remove(usize),
    /// Add a new source from the inline input.
    Add(String),
    /// Hide the inline new-source input.
    DismissAdd,
}

/// What an environment-list interaction asked for this frame.
enum EnvAction {
    /// Remove an override by name.
    Remove(String),
    /// Add an override from the inline inputs.
    Add(String, String),
    /// Hide the inline new-variable inputs.
    DismissAdd,
}

/// Render the Model library paths dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_pdk_settings_dialog(
    ctx: &Context,
    state: &mut PdkSettingsDialogState,
) -> PdkSettingsDialogResult {
    if !state.open {
        return PdkSettingsDialogResult::None;
    }

    let mut result = PdkSettingsDialogResult::None;
    let mut should_close = false;
    let mut should_rescan = false;

    let choice = Dialog::new("Models", "Model library paths", "Close")
        .size(DialogSize::Lg)
        .secondary("Rescan")
        .hint("edits preview live · Close saves, Esc discards")
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut load: Option<PathBuf> = None;
            two_pane(ui, PANE_RAIL_W, PANE_HEIGHT, |ui, side| match side {
                PaneSide::Rail => sources_rail(ui, state),
                PaneSide::Detail => load = detail_pane(ui, state),
            });
            if let Some(path) = load {
                result = PdkSettingsDialogResult::LoadFile(path);
            }
        });

    match choice {
        DialogChoice::Primary => {
            if state.has_changes() {
                let config = state.apply();
                result = PdkSettingsDialogResult::Applied(config);
            }
            should_close = true;
        }
        DialogChoice::Secondary => {
            should_rescan = true;
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            should_close = true;
            result = PdkSettingsDialogResult::Cancelled;
        }
        DialogChoice::None => {}
    }

    if should_rescan {
        state.rescan();
    }

    if should_close {
        state.close();
    }

    result
}

// ---------------------------------------------------------------------------
// left rail — sources + environment
// ---------------------------------------------------------------------------

fn sources_rail(ui: &mut Ui, state: &mut PdkSettingsDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    pane_header(ui, |ui| {
        header_caption(ui, "Sources");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            // Native folder pickers don't exist in the browser build; the
            // inline "+ Add" input remains the entry point there.
            #[cfg(not(target_arch = "wasm32"))]
            if chip(ui, "Browse…", false).clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                state.add_library_path(path.to_string_lossy().to_string());
            }
            if chip(ui, "+ Add", false).clicked() {
                state.adding_path = true;
                ui.ctx().memory_mut(|m| m.request_focus(add_path_id()));
            }
        });
    });

    // The environment block is pinned below the sources; size it first so
    // the sources list takes exactly the remaining pane height.
    let mut env: Vec<(String, String)> = state
        .config
        .env_overrides()
        .iter()
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    env.sort_by(|a, b| a.0.cmp(&b.0));
    let trailing_h = if state.adding_env {
        ENV_ROW_H + 4.0
    } else {
        ENV_ROW_H
    };
    let env_list_h = (env.len() as f32 * ENV_ROW_H + trailing_h).min(96.0);
    let sources_h = PANE_HEIGHT - PANE_HEADER_H - 23.0 - env_list_h;

    let mut action: Option<SourceAction> = None;
    egui::ScrollArea::vertical()
        .id_salt("volta.pdk.sources")
        .max_height(sources_h)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let count = state.config.library_paths().len();
            for idx in 0..count {
                if state.editing_path_index == Some(idx) {
                    if let Some(act) = source_edit_row(ui, idx, &mut state.edit_path_buffer) {
                        action = Some(act);
                    }
                } else {
                    let entry = state.config.library_paths()[idx].clone();
                    let expanded = state.config.expand_path(&entry.path);
                    let selected = state.selected_source == Some(idx);
                    if let Some(act) = source_row(ui, idx, &entry, selected, &expanded) {
                        action = Some(act);
                    }
                }
            }
            if state.adding_path {
                if let Some(act) = source_add_row(ui, &mut state.new_path_input) {
                    action = Some(act);
                }
            } else if count == 0 {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("No sources — Add a model directory")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                });
            }
        });

    // Environment under a thin separator.
    let sep_y = ui.cursor().top() + 0.5;
    ui.painter()
        .hline(ui.max_rect().x_range(), sep_y, Stroke::new(1.0, c.border));
    ui.add_space(1.0);
    pane_section_label(ui, "Environment");

    let mut env_action: Option<EnvAction> = None;
    egui::ScrollArea::vertical()
        .id_salt("volta.pdk.env")
        .max_height(env_list_h)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            for (index, (name, value)) in env.iter().enumerate() {
                let resolved = state.config.expand_path(value);
                if let Some(act) = env_row(ui, index, name, value, &resolved) {
                    env_action = Some(act);
                }
            }
            if state.adding_env {
                if let Some(act) =
                    env_add_row(ui, &mut state.new_env_name, &mut state.new_env_value)
                {
                    env_action = Some(act);
                }
            } else if env_add_prompt_row(ui) {
                state.adding_env = true;
                ui.ctx().memory_mut(|m| m.request_focus(env_name_id()));
            }
        });

    if let Some(act) = action {
        match act {
            SourceAction::Select(idx) => {
                state.selected_source = if state.selected_source == Some(idx) {
                    None
                } else {
                    Some(idx)
                };
            }
            SourceAction::ToggleEnabled(idx) => state.toggle_path_enabled(idx),
            SourceAction::StartEdit(idx, path) => {
                state.selected_source = Some(idx);
                state.editing_path_index = Some(idx);
                state.edit_path_buffer = path;
                ui.ctx().memory_mut(|m| m.request_focus(path_edit_id(idx)));
            }
            SourceAction::FinishEdit(idx) => {
                let buffer = state.edit_path_buffer.trim().to_owned();
                if !buffer.is_empty() {
                    let paths = state.config.library_paths_mut();
                    if idx < paths.len() {
                        paths[idx].path = buffer;
                    }
                }
                state.editing_path_index = None;
                state.edit_path_buffer.clear();
            }
            SourceAction::Remove(idx) => state.remove_library_path(idx),
            SourceAction::Add(path) => {
                state.adding_path = false;
                state.add_library_path(path);
            }
            SourceAction::DismissAdd => state.adding_path = false,
        }
    }
    if let Some(act) = env_action {
        match act {
            EnvAction::Remove(name) => state.remove_env_override(&name),
            EnvAction::Add(name, value) => {
                state.adding_env = false;
                state.add_env_override(name, value);
            }
            EnvAction::DismissAdd => {
                state.adding_env = false;
                state.new_env_name.clear();
                state.new_env_value.clear();
            }
        }
    }
}

/// One source row: enable checkbox, elided mono path (click to edit),
/// file-count meta, and a remove button on the hovered/selected row.
fn source_row(
    ui: &mut Ui,
    idx: usize,
    entry: &LibraryPathEntry,
    selected: bool,
    expanded: &str,
) -> Option<SourceAction> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let mut action = None;

    ui.allocate_ui_with_layout(
        vec2(width, ROW_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            // The whole-row interaction registers first; the checkbox, path
            // and remove zones added after it sit on top and win clicks.
            let row_id = ui.id().with(("volta.pdk.source", idx));
            let row_response = ui.interact(rect, row_id, Sense::click());
            let hovered = ui.rect_contains_pointer(rect);
            let hover = ui
                .ctx()
                .animate_bool_with_time(row_id.with("hover"), hovered, 0.12);

            {
                let painter = ui.painter();
                if selected {
                    painter.rect_filled(rect, 0.0, c.bg_hover);
                    painter.vline(rect.left() + 1.0, rect.y_range(), Stroke::new(2.0, c.accent));
                } else if hover > 0.0 {
                    painter.rect_filled(
                        rect,
                        0.0,
                        mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
                    );
                }
            }

            // Disabled sources render dimmed, like the design's 55 % row.
            let alpha = if entry.enabled { 1.0 } else { 0.55 };

            ui.add_space(8.0);
            let (box_rect, box_response) = ui.allocate_exact_size(vec2(13.0, 13.0), Sense::click());
            {
                let painter = ui.painter();
                if entry.enabled {
                    painter.rect_filled(box_rect, t.radius.min(2.0), c.accent);
                    let s = box_rect.width();
                    painter.add(egui::Shape::line(
                        vec![
                            box_rect.left_top() + vec2(0.25 * s, 0.55 * s),
                            box_rect.left_top() + vec2(0.42 * s, 0.72 * s),
                            box_rect.left_top() + vec2(0.78 * s, 0.30 * s),
                        ],
                        Stroke::new(1.6, c.accent_ink),
                    ));
                } else {
                    painter.rect(
                        box_rect,
                        t.radius.min(2.0),
                        c.bg_inset,
                        Stroke::new(1.0, c.border_strong),
                    );
                }
            }
            if box_response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(if entry.enabled {
                    "Exclude from scans"
                } else {
                    "Include in scans"
                })
                .clicked()
            {
                action = Some(SourceAction::ToggleEnabled(idx));
            }
            ui.add_space(6.0);

            let meta = if entry.enabled {
                entry.file_count.to_string()
            } else {
                "—".to_owned()
            };
            let meta_galley = ui.fonts(|f| {
                f.layout_no_wrap(
                    meta,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                )
            });
            let meta_w = meta_galley.size().x;
            let name_w = (ui.available_width() - meta_w - 18.0 - 14.0).max(24.0);

            let (name_rect, name_response) =
                ui.allocate_exact_size(vec2(name_w, ROW_H), Sense::click());
            let name_galley = ui.fonts(|f| {
                let mut job = egui::text::LayoutJob::simple_singleline(
                    entry.path.clone(),
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                );
                job.wrap = egui::text::TextWrapping::truncate_at_width(name_w);
                f.layout_job(job)
            });
            ui.painter().galley(
                egui::pos2(
                    name_rect.left(),
                    name_rect.center().y - name_galley.size().y * 0.5,
                ),
                name_galley,
                mix(c.text_dim, c.text, hover).gamma_multiply(alpha),
            );
            let tooltip = if expanded != entry.path {
                format!("{}\n→ {}\nclick to edit", entry.path, expanded)
            } else {
                format!("{}\nclick to edit", entry.path)
            };
            if name_response
                .on_hover_cursor(egui::CursorIcon::Text)
                .on_hover_text(tooltip)
                .clicked()
            {
                action = Some(SourceAction::StartEdit(idx, entry.path.clone()));
            }

            let (meta_rect, _) = ui.allocate_exact_size(vec2(meta_w, ROW_H), Sense::hover());
            ui.painter().galley(
                egui::pos2(
                    meta_rect.left(),
                    meta_rect.center().y - meta_galley.size().y * 0.5,
                ),
                meta_galley,
                c.text_faint.gamma_multiply(alpha),
            );
            ui.add_space(2.0);

            if hovered || selected {
                if IconButton::new(Icon::Close)
                    .side(18.0)
                    .tooltip("Remove source")
                    .show(ui)
                    .clicked()
                {
                    action = Some(SourceAction::Remove(idx));
                }
            } else {
                ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
            }

            if row_response.clicked() {
                action = Some(SourceAction::Select(idx));
            }
        },
    );

    action
}

/// The inline path editor replacing a source row while it is being edited.
fn source_edit_row(ui: &mut Ui, idx: usize, buffer: &mut String) -> Option<SourceAction> {
    let mut action = None;
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, ROW_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(8.0);
            let response = ui.add(
                egui::TextEdit::singleline(buffer)
                    .id(path_edit_id(idx))
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .margin(egui::Margin::symmetric(5.0, 2.0))
                    .desired_width((ui.available_width() - 8.0).max(40.0)),
            );
            if response.lost_focus() {
                action = Some(SourceAction::FinishEdit(idx));
            }
        },
    );
    action
}

/// The inline input appended below the sources by "+ Add". Enter commits;
/// clicking elsewhere dismisses (the typed text is kept).
fn source_add_row(ui: &mut Ui, buffer: &mut String) -> Option<SourceAction> {
    let mut action = None;
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, ROW_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(8.0);
            let response = ui.add(
                egui::TextEdit::singleline(buffer)
                    .id(add_path_id())
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .hint_text("Add library path…")
                    .margin(egui::Margin::symmetric(5.0, 2.0))
                    .desired_width((ui.available_width() - 8.0).max(40.0)),
            );
            if response.lost_focus() {
                let entered = ui.input(|i| i.key_pressed(egui::Key::Enter));
                if entered && !buffer.trim().is_empty() {
                    action = Some(SourceAction::Add(buffer.trim().to_owned()));
                } else {
                    action = Some(SourceAction::DismissAdd);
                }
            }
        },
    );
    action
}

/// One environment row: `$NAME` left, value right (tooltip shows the
/// resolved expansion), remove button on hover.
fn env_row(
    ui: &mut Ui,
    index: usize,
    name: &str,
    value: &str,
    resolved: &str,
) -> Option<EnvAction> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let mut action = None;

    ui.allocate_ui_with_layout(
        vec2(width, ENV_ROW_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            let row_id = ui.id().with(("volta.pdk.env", index));
            let hovered = ui.rect_contains_pointer(rect);
            let hover = ui
                .ctx()
                .animate_bool_with_time(row_id.with("hover"), hovered, 0.12);
            if hover > 0.0 {
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
                );
            }

            ui.add_space(10.0);
            let name_w = (rect.width() * 0.45).max(40.0);
            let name_galley = ui.fonts(|f| {
                let mut job = egui::text::LayoutJob::simple_singleline(
                    format!("${name}"),
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                );
                job.wrap = egui::text::TextWrapping::truncate_at_width(name_w);
                f.layout_job(job)
            });
            let name_used = name_galley.size().x.min(name_w);
            let (name_rect, _) = ui.allocate_exact_size(vec2(name_used, ENV_ROW_H), Sense::hover());
            ui.painter().galley(
                egui::pos2(
                    name_rect.left(),
                    name_rect.center().y - name_galley.size().y * 0.5,
                ),
                name_galley,
                mix(c.text_dim, c.text, hover),
            );

            let value_w = (ui.available_width() - 22.0).max(10.0);
            let (value_rect, value_response) =
                ui.allocate_exact_size(vec2(value_w, ENV_ROW_H), Sense::hover());
            let value_galley = ui.fonts(|f| {
                let mut job = egui::text::LayoutJob::simple_singleline(
                    value.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                );
                job.wrap = egui::text::TextWrapping::truncate_at_width(value_w);
                f.layout_job(job)
            });
            ui.painter().galley(
                egui::pos2(
                    value_rect.right() - value_galley.size().x.min(value_w),
                    value_rect.center().y - value_galley.size().y * 0.5,
                ),
                value_galley,
                c.text_faint,
            );
            if resolved != value {
                value_response.on_hover_text(format!("→ {resolved}"));
            }

            if hovered {
                if IconButton::new(Icon::Close)
                    .side(16.0)
                    .tooltip("Remove variable")
                    .show(ui)
                    .clicked()
                {
                    action = Some(EnvAction::Remove(name.to_owned()));
                }
            } else {
                ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
            }
        },
    );

    action
}

/// The faint "+ variable…" row; returns `true` when clicked.
fn env_add_prompt_row(ui: &mut Ui) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), ENV_ROW_H), Sense::click());
    if !ui.is_rect_visible(rect) {
        return false;
    }
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.12);
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(rect, 0.0, mix(egui::Color32::TRANSPARENT, c.bg_hover, hover));
    }
    painter.text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "+ variable…",
        theme::mono(tokens::FS_1, FontWeight::Regular),
        mix(c.text_faint, c.text_dim, hover),
    );
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

/// The inline name/value inputs opened by "+ variable…". Enter commits
/// when the name is non-empty; ✕ dismisses.
fn env_add_row(ui: &mut Ui, name: &mut String, value: &mut String) -> Option<EnvAction> {
    let mut action = None;
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, ENV_ROW_H + 4.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.add_space(8.0);
            let name_response = ui.add(
                egui::TextEdit::singleline(name)
                    .id(env_name_id())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .hint_text("NAME")
                    .margin(egui::Margin::symmetric(5.0, 2.0))
                    .desired_width(72.0),
            );
            let value_response = ui.add(
                egui::TextEdit::singleline(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .hint_text("value or path")
                    .margin(egui::Margin::symmetric(5.0, 2.0))
                    .desired_width((ui.available_width() - 24.0).max(40.0)),
            );
            if IconButton::new(Icon::Close)
                .side(16.0)
                .tooltip("Dismiss")
                .show(ui)
                .clicked()
            {
                action = Some(EnvAction::DismissAdd);
            }
            let entered = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if entered && (name_response.lost_focus() || value_response.lost_focus()) {
                let trimmed = name.trim().trim_start_matches('$').to_owned();
                if !trimmed.is_empty() {
                    action = Some(EnvAction::Add(trimmed, value.trim().to_owned()));
                }
            }
        },
    );
    action
}

// ---------------------------------------------------------------------------
// right detail — discovered files of the selected source
// ---------------------------------------------------------------------------

fn detail_pane(ui: &mut Ui, state: &mut PdkSettingsDialogState) -> Option<PathBuf> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let path_count = state.config.library_paths().len();
    let selected = state.selected_source.filter(|idx| *idx < path_count);
    let selected_entry: Option<LibraryPathEntry> =
        selected.map(|idx| state.config.library_paths()[idx].clone());
    // Discovery records each file's source as the expanded path.
    let selected_root: Option<PathBuf> = selected_entry
        .as_ref()
        .map(|entry| PathBuf::from(state.config.expand_path(&entry.path)));
    let in_scope = |file: &DiscoveredFile| {
        selected_root
            .as_ref()
            .is_none_or(|root| &file.source_path == root)
    };
    let scope_total = state
        .config
        .discovered_files()
        .iter()
        .filter(|file| in_scope(file))
        .count();

    let mut toggle_recursive = false;
    pane_header(ui, |ui| {
        let chip_reserve = if selected_entry.is_some() { 84.0 } else { 0.0 };
        let path_w = (ui.available_width() - chip_reserve - 196.0).max(40.0);
        let (text, color) = match &selected_entry {
            Some(entry) => (entry.path.clone(), c.text_dim),
            None => ("all sources".to_owned(), c.text_faint),
        };
        let (path_rect, path_response) =
            ui.allocate_exact_size(vec2(path_w, 24.0), Sense::hover());
        let galley = ui.fonts(|f| {
            let mut job = egui::text::LayoutJob::simple_singleline(
                text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::PLACEHOLDER,
            );
            job.wrap = egui::text::TextWrapping::truncate_at_width(path_w);
            f.layout_job(job)
        });
        ui.painter().galley(
            egui::pos2(
                path_rect.left(),
                path_rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            color,
        );
        if let Some(root) = &selected_root {
            path_response.on_hover_text(root.to_string_lossy());
        }
        if let Some(entry) = &selected_entry
            && chip(ui, "recursive", entry.recursive)
                .on_hover_text("Scan subdirectories")
                .clicked()
        {
            toggle_recursive = true;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut state.file_filter)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .hint_text(format!(
                        "Filter {scope_total} file{}…",
                        if scope_total == 1 { "" } else { "s" }
                    ))
                    .desired_width(170.0),
            );
        });
    });
    if toggle_recursive && let Some(idx) = selected {
        state.toggle_path_recursive(idx);
    }

    table_header(ui);

    let empty_message = if state.config.discovered_files().is_empty() {
        "No model files discovered — add a source and Rescan"
    } else if !state.file_filter.is_empty() {
        "No file matches the filter"
    } else {
        "No files under this source"
    };
    let files: Vec<&DiscoveredFile> = state
        .filtered_files()
        .into_iter()
        .filter(|file| in_scope(file))
        .collect();

    let mut load: Option<PathBuf> = None;
    let scroll_h = PANE_HEIGHT - PANE_HEADER_H - FILE_ROW_H - PANE_FOOTER_H;
    egui::ScrollArea::vertical()
        .id_salt("volta.pdk.files")
        .max_height(scroll_h)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            if files.is_empty() {
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(empty_message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                });
            }
            for file in &files {
                if file_row(ui, file).clicked() {
                    load = Some(file.path.clone());
                }
            }
        });

    pane_footer(ui, &scan_summary(state));
    load
}

/// The TYPE / FILE / SECTIONS header strip of the files table.
fn table_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), FILE_ROW_H), Sense::hover());
    for (label, x) in [
        ("Type", rect.left() + 10.0),
        ("File", rect.left() + FILE_COL_X),
        ("Sections", rect.right() - SECTIONS_W),
    ] {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &label.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: theme::mono(9.5, FontWeight::Medium),
                color: c.text_faint,
                extra_letter_spacing: 0.1 * 9.5,
                ..Default::default()
            },
        );
        let galley = ui.fonts(|f| f.layout_job(job));
        let y = rect.center().y - galley.size().y * 0.5;
        ui.painter().galley(egui::pos2(x, y), galley, c.text_faint);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, c.border),
    );
}

/// One discovered-file row: type badge, file name, sections. Clicking
/// loads the file into the model library.
fn file_row(ui: &mut Ui, file: &DiscoveredFile) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), FILE_ROW_H), Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.12);
    if hover > 0.0 {
        ui.painter().rect_filled(
            rect,
            0.0,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }

    // File-type badges draw from the categorical trace palette —
    // theme-aware in both modes, unlike hand-picked RGB.
    let (badge_color, badge_text) = match file.file_type() {
        "lib" => (c.traces[2], "LIB"),
        "scs" => (c.traces[1], "SCS"),
        "mod" => (c.traces[3], "MOD"),
        "sp" | "cir" => (c.text_dim, "SP"),
        _ => (c.text_faint, "???"),
    };
    paint_badge(ui, rect.left() + 10.0, rect.center().y, badge_text, badge_color);

    let sections_x = rect.right() - SECTIONS_W;
    let name_w = (sections_x - rect.left() - FILE_COL_X - 8.0).max(24.0);
    let name_galley = ui.fonts(|f| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            file.file_name().to_owned(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            egui::Color32::PLACEHOLDER,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(name_w);
        f.layout_job(job)
    });
    ui.painter().galley(
        egui::pos2(
            rect.left() + FILE_COL_X,
            rect.center().y - name_galley.size().y * 0.5,
        ),
        name_galley,
        mix(c.text_dim, c.text, hover),
    );

    let sections = if file.sections.is_empty() {
        "—".to_owned()
    } else {
        file.sections.join(" ")
    };
    let sections_w = SECTIONS_W - 10.0;
    let sections_galley = ui.fonts(|f| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            sections,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            egui::Color32::PLACEHOLDER,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(sections_w);
        f.layout_job(job)
    });
    ui.painter().galley(
        egui::pos2(
            sections_x,
            rect.center().y - sections_galley.size().y * 0.5,
        ),
        sections_galley,
        c.text_faint,
    );

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(format!("{}\nclick to load", file.path_str()))
}

/// A bordered type badge (the design's `.badge`), left-anchored at `left`.
fn paint_badge(ui: &Ui, left: f32, center_y: f32, label: &str, color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        label,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(9.5, FontWeight::Medium),
            color,
            extra_letter_spacing: 0.06 * 9.5,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));
    let rect = Rect::from_min_size(
        egui::pos2(left, center_y - 7.0),
        vec2(galley.size().x + 12.0, 14.0),
    );
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        egui::Color32::TRANSPARENT,
        Stroke::new(1.0, color.gamma_multiply(0.5)),
    );
    painter.galley(
        egui::pos2(rect.left() + 6.0, center_y - galley.size().y * 0.5),
        galley,
        color,
    );
}

/// Footer strip text: last scan age plus live totals.
fn scan_summary(state: &PdkSettingsDialogState) -> String {
    let total = state.config.discovered_files().len();
    let enabled = state
        .config
        .library_paths()
        .iter()
        .filter(|entry| entry.enabled)
        .count();
    let mut line = String::new();
    if let Some(scanned) = state
        .config
        .library_paths()
        .iter()
        .filter_map(|entry| entry.last_scanned)
        .max()
    {
        let now = crate::common::time_compat::unix_epoch().as_secs();
        line.push_str(&format!("scanned {} · ", fmt_ago(now.saturating_sub(scanned))));
    }
    line.push_str(&format!(
        "{total} file{} across {enabled} enabled source{}",
        if total == 1 { "" } else { "s" },
        if enabled == 1 { "" } else { "s" },
    ));
    line
}

/// Compact "time ago" for the footer strip.
fn fmt_ago(secs: u64) -> String {
    if secs < 5 {
        "just now".to_owned()
    } else if secs < 60 {
        format!("{secs} s ago")
    } else if secs < 3600 {
        format!("{} min ago", secs / 60)
    } else if secs < 86_400 {
        format!("{} h ago", secs / 3600)
    } else {
        format!("{} d ago", secs / 86_400)
    }
}

// ---------------------------------------------------------------------------
// pane captions (pane chrome lives in ui/widgets/pane.rs)
// ---------------------------------------------------------------------------

/// Mono uppercase caption used inside pane headers.
fn header_caption(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.12 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
}

// ---------------------------------------------------------------------------
// stable ids for the inline editors (caret/focus state lives in egui memory)
// ---------------------------------------------------------------------------

fn path_edit_id(idx: usize) -> egui::Id {
    egui::Id::new(("volta.pdk.path_edit", idx))
}

fn add_path_id() -> egui::Id {
    egui::Id::new("volta.pdk.add_path")
}

fn env_name_id() -> egui::Id {
    egui::Id::new("volta.pdk.env_name")
}
