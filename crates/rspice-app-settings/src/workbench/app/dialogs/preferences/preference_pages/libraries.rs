//! Projects and files ▸ Libraries: the folders searched for a netlist's
//! `.include` and `.lib` files once a project's own search paths miss.
//!
//! The desktop's alone. A browser has no folders to search: it resolves a
//! deck's includes only from the bundle it was given.

use std::path::PathBuf;

use egui::{Sense, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark,
};
use crate::workbench::FolderListPreference;

use super::super::preferences_shell::{searchable_row, section_heading, value_column};
use super::PreferencesDialogs;

pub const KEY: FolderListPreference = FolderListPreference::LibraryFolders;

/// Where the selected row is kept between frames. A selection is a pointer's
/// state, not a setting, so it lives with the dialog rather than in the file.
const SELECTION_ID: &str = "preferences.projects.library-folders.selected";

/// What a press on the list asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edit {
    /// Ask the host for a folder, then add it at the end.
    Add,
    Move {
        from: usize,
        to: usize,
    },
    Remove(usize),
}

pub fn section(ui: &mut Ui, state: &mut PreferencesDialogs<'_>) {
    section_heading(ui, "Libraries");
    searchable_row(
        ui,
        "Library folders",
        "Searched top first for .include and .lib files. A project's own search paths win.",
        &["include", "lib", "models", "search path", "spice"],
        |ui| {
            value_column(ui, |ui| {
                let folders = state.model.preferences.folders(KEY).to_vec();
                let Some(edit) = folder_list(ui, &folders) else {
                    return;
                };
                let next = match edit {
                    Edit::Add => {
                        let Some(picked) = rfd::FileDialog::new()
                            .set_title("Add a library folder")
                            .pick_folder()
                        else {
                            return;
                        };
                        if let Some(listed) = folders.iter().position(|folder| *folder == picked) {
                            select(ui, Some(listed));
                            return;
                        }
                        select(ui, Some(folders.len()));
                        let mut next = folders;
                        next.push(picked);
                        next
                    }
                    Edit::Move { from, to } => {
                        let mut next = folders;
                        next.swap(from, to);
                        select(ui, Some(to));
                        next
                    }
                    Edit::Remove(index) => {
                        let mut next = folders;
                        next.remove(index);
                        select(ui, None);
                        next
                    }
                };
                match state.model.preferences.set_folders(KEY, next) {
                    // A validated deck was validated through the folders
                    // that were, the way a change to a project's own search
                    // paths leaves its validation behind.
                    Ok(()) => state.model.invalidate_source_evidence(),
                    Err(refusal) => {
                        state.prompt(format!("The library folders were not changed: {refusal}."));
                    }
                }
            });
        },
    );
}

fn selection_id() -> egui::Id {
    egui::Id::new(SELECTION_ID)
}

fn select(ui: &Ui, index: Option<usize>) {
    ui.data_mut(|data| data.insert_temp(selection_id(), index));
}

/// The folders, one row each in search order, and the commands under them.
/// Returns the edit a press asked for this frame.
fn folder_list(ui: &mut Ui, folders: &[PathBuf]) -> Option<Edit> {
    let t = Tokens::get(ui.ctx());
    let selected = ui
        .data(|data| data.get_temp::<Option<usize>>(selection_id()))
        .flatten()
        .filter(|index| *index < folders.len());
    if !folders.is_empty() {
        let present = presence(ui, folders);
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let width = ui.available_width();
            let top = ui.cursor().top();
            for (index, folder) in folders.iter().enumerate() {
                let row = Row {
                    index,
                    folder,
                    present: present[index],
                    selected: selected == Some(index),
                };
                if folder_row(ui, &row, width) {
                    select(ui, Some(index));
                }
            }
            let well = egui::Rect::from_min_max(
                egui::pos2(ui.min_rect().left(), top),
                egui::pos2(ui.min_rect().left() + width, ui.cursor().top()),
            );
            ui.painter().rect_stroke(
                well,
                0.0,
                egui::Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
        });
        ui.add_space(tokens::SP_3);
    }
    let mut edit = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(tokens::SP_3, tokens::SP_3);
        let height = t.metrics.ctl_h;
        if Button::new("Add folder\u{2026}")
            .min_height(height)
            .show(ui)
            .clicked()
        {
            edit = Some(Edit::Add);
        }
        // With nothing listed there is nothing to move or remove, and three
        // disabled buttons would be the row's whole content.
        if folders.is_empty() {
            return;
        }
        if Button::new("Move up")
            .min_height(height)
            .enabled(selected.is_some_and(|index| index > 0))
            .show(ui)
            .clicked()
            && let Some(index) = selected
        {
            edit = Some(Edit::Move {
                from: index,
                to: index - 1,
            });
        }
        if Button::new("Move down")
            .min_height(height)
            .enabled(selected.is_some_and(|index| index + 1 < folders.len()))
            .show(ui)
            .clicked()
            && let Some(index) = selected
        {
            edit = Some(Edit::Move {
                from: index,
                to: index + 1,
            });
        }
        if Button::new("Remove")
            .min_height(height)
            .enabled(selected.is_some())
            .destructive(true)
            .show(ui)
            .clicked()
            && let Some(index) = selected
        {
            edit = Some(Edit::Remove(index));
        }
    });
    edit
}

/// Whether each folder is one this machine has, looked at no more than once
/// a second. The page is drawn every frame, and a folder on a share that is
/// out of reach can take seconds to answer.
fn presence(ui: &Ui, folders: &[PathBuf]) -> Vec<bool> {
    type Looked = (Vec<PathBuf>, f64, Vec<bool>);
    let id = selection_id().with("presence");
    let now = ui.input(|input| input.time);
    if let Some((looked_at, when, present)) = ui.data(|data| data.get_temp::<Looked>(id))
        && looked_at == folders
        && now - when < 1.0
    {
        return present;
    }
    let present = folders
        .iter()
        .map(|folder| folder.is_absolute() && folder.is_dir())
        .collect::<Vec<_>>();
    ui.data_mut(|data| data.insert_temp::<Looked>(id, (folders.to_vec(), now, present.clone())));
    present
}

/// What one row of the list shows.
struct Row<'a> {
    index: usize,
    folder: &'a std::path::Path,
    present: bool,
    selected: bool,
}

/// One folder: its place in the search order and its path, marked when it
/// is not a folder this machine has. Returns whether it was clicked.
fn folder_row(ui: &mut Ui, row: &Row<'_>, width: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let label = format!("{}. {}", row.index + 1, row.folder.display());
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.row_h), Sense::click());
    // A painted row reaches a screen reader only through what it says here.
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            row.selected,
            &label,
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if row.selected {
            t.color.accent_dim
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        };
        ui.painter().rect_filled(rect, 0.0, fill);
        let inset = tokens::SP_4;
        // Present is every folder's normal state, so only an absent one is
        // marked and named.
        let mut state_width = 0.0;
        if !row.present {
            let font = theme::sans(tokens::FS_0, FontWeight::Regular);
            state_width = ui
                .painter()
                .text(
                    egui::pos2(rect.right() - inset, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    "missing",
                    font,
                    t.color.text_dim,
                )
                .width();
            paint_problem_mark(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::pos2(
                        rect.right() - inset - state_width - PROBLEM_MARK_ADVANCE
                            + PROBLEM_MARK_SIDE * 0.5,
                        rect.center().y,
                    ),
                    egui::Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                Problem::Warning,
            );
            state_width += PROBLEM_MARK_ADVANCE + inset;
        }
        // A path too long for the row ends in an ellipsis; the whole of it
        // is the row's tooltip.
        let path = ui.fonts_mut(|fonts| {
            let mut job = egui::text::LayoutJob::simple_singleline(
                label,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::PLACEHOLDER,
            );
            job.wrap = egui::text::TextWrapping::truncate_at_width(
                (rect.width() - 2.0 * inset - state_width).max(0.0),
            );
            fonts.layout_job(job)
        });
        ui.painter().galley(
            egui::pos2(rect.left() + inset, rect.center().y - path.size().y * 0.5),
            path,
            t.color.text,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_text(row.folder.display().to_string())
        .clicked()
}
