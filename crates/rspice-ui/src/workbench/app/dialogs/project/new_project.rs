//! New project — the reviewed identity a session's next project is created
//! under.
//!
//! Creating a project writes nothing to disk, so the only decision this
//! transaction carries is identity: the project name, the editable design
//! library, and the top cell. All three are checked against the same contracts
//! the project file itself enforces, so an identity reviewed here can never be
//! refused at save time. The library mirrors a slug of the project name until
//! the reader edits it, after which the two are independent — a mirror that
//! keeps overwriting a deliberate choice is worse than no mirror.

use egui::Context;

use crate::state::workspace::{
    DEFAULT_PROJECT_LIBRARY, DEFAULT_SCHEMATIC_VIEW, DEFAULT_TOP_CELL,
    PROJECT_DESCRIPTOR_SCHEMA_VERSION,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::project_workflow::{NewProjectParams, create_new_project_with};

/// Draft identity for one New project transaction.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NewProjectDialogState {
    pub(crate) open: bool,
    pub(crate) name: String,
    pub(crate) library: String,
    /// Set the moment the reader edits Library, which retires the mirror.
    pub(crate) library_edited: bool,
    pub(crate) top_cell: String,
    /// A refusal from a create attempt, distinct from live field validation.
    pub(crate) error: Option<String>,
}

impl NewProjectDialogState {
    /// The identity the reviewed fields describe, trimmed exactly as creation
    /// will record it.
    pub(crate) fn params(&self) -> NewProjectParams {
        NewProjectParams {
            name: self.name.trim().to_owned(),
            root_library: self.library.trim().to_owned(),
            top_cell: self.top_cell.trim().to_owned(),
        }
    }

    /// Re-derive Library from the project name. A reader who has typed in
    /// Library owns it from then on: a suggestion that keeps overwriting a
    /// deliberate choice is worse than no suggestion.
    fn mirror_library_from_name(&mut self) {
        if !self.library_edited {
            self.library = library_slug(&self.name);
        }
    }
}

/// Open the transaction with the default library and top cell. The project
/// name starts empty because it is the one field with no defensible default:
/// requiring it is the point of the dialog.
pub(crate) fn open_new_project_dialog(state: &mut AppState) {
    state.dialogs.new_project = NewProjectDialogState {
        open: true,
        library: DEFAULT_PROJECT_LIBRARY.to_owned(),
        top_cell: DEFAULT_TOP_CELL.to_owned(),
        ..NewProjectDialogState::default()
    };
}

/// Mirror a project name into a library-name segment: lowercase, every
/// character the persisted key format rejects folded to `_`, runs collapsed,
/// and no leading or trailing separator. A name with nothing usable in it
/// leaves the default library rather than an empty segment.
fn library_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    for character in name.chars() {
        if character.is_alphanumeric() || character == '_' {
            slug.extend(character.to_lowercase());
        } else if !slug.ends_with('_') && !slug.is_empty() {
            slug.push('_');
        }
    }
    let trimmed = slug.trim_matches('_');
    if trimmed.is_empty() {
        DEFAULT_PROJECT_LIBRARY.to_owned()
    } else {
        trimmed.to_owned()
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_new_project_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.new_project.open {
            return;
        }
        // The footer is declared before the body runs, so the primary gate
        // reads the identity as of this frame's start. Creation re-reads and
        // re-validates the post-edit identity, so the one-frame lag can never
        // let an invalid identity through.
        let entry_valid = self.state.dialogs.new_project.params().validate().is_ok();

        let dialog = &mut self.state.dialogs.new_project;
        let choice = Dialog::new(
            "Project lifecycle \u{b7} Transactional \u{b7} Portable",
            "Create RSpice project",
            "Create project",
        )
        .description(
            "Name the new project, its editable design library, and its top cell before it \
             replaces the current session.",
        )
        .size(DialogSize::Transaction)
        .ghost("Cancel")
        .hint("nothing is written to disk until Save")
        .primary_enabled(entry_valid)
        .show(ctx, |ui| {
            ui.columns(2, |columns| {
                render_identity(&mut columns[0], dialog);
                render_contract(&mut columns[1], dialog);
            });
        });

        match choice {
            DialogChoice::Primary => {
                let params = self.state.dialogs.new_project.params();
                match create_new_project_with(&mut self.state, &params) {
                    Ok(()) => self.state.dialogs.new_project = NewProjectDialogState::default(),
                    Err(error) => self.state.dialogs.new_project.error = Some(error),
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.new_project = NewProjectDialogState::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn render_identity(ui: &mut egui::Ui, dialog: &mut NewProjectDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    section_head(ui, "Project identity");
    ui.spacing_mut().item_spacing.y = 2.0;

    let mut edited = input_row(ui, "Project name", &mut dialog.name).changed();
    if edited {
        dialog.mirror_library_from_name();
    }
    if input_row(ui, "Library", &mut dialog.library).changed() {
        dialog.library_edited = true;
        edited = true;
    }
    edited |= input_row(ui, "Top cell", &mut dialog.top_cell).changed();
    if edited {
        // A refusal describes the identity that was submitted; editing one
        // retires it rather than leaving a stale reason beside new fields.
        dialog.error = None;
    }
    kv_row(ui, "View", DEFAULT_SCHEMATIC_VIEW);
    kv_row(
        ui,
        "Format",
        &format!(".rspiceproj \u{b7} schema {PROJECT_DESCRIPTOR_SCHEMA_VERSION}"),
    );

    let message = dialog
        .params()
        .validate()
        .err()
        .or_else(|| dialog.error.clone());
    if let Some(message) = message {
        ui.add_space(tokens::SP_3);
        ui.label(
            egui::RichText::new(message)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }
}

fn render_contract(ui: &mut egui::Ui, dialog: &NewProjectDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let params = dialog.params();
    let identity_valid = params.validate().is_ok();

    property_card(
        ui,
        "Safety contract",
        None,
        &[
            ("Existing files", "untouched until Save"),
            ("Unsaved work", "reviewed first"),
            ("Active run", "blocks creation"),
            ("PDK model catalog", "restored from settings"),
        ],
    );
    ui.add_space(tokens::SP_4);

    let top_view = format!(
        "{}/{}/{DEFAULT_SCHEMATIC_VIEW}",
        params.root_library, params.top_cell
    );
    property_card(
        ui,
        "Validation preview",
        Some(if identity_valid {
            ("ready", c.ok)
        } else {
            ("incomplete", c.err)
        }),
        &[
            ("Design library", params.root_library.as_str()),
            ("Top cell view", top_view.as_str()),
            ("Simulation plan", "personal default"),
            ("Drawing sheet", "personal default"),
        ],
    );
}

/// The mockup's property card: a titled panel with an optional right-aligned
/// status and a key/value list.
fn property_card(
    ui: &mut egui::Ui,
    title: &str,
    status: Option<(&str, egui::Color32)>,
    rows: &[(&str, &str)],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::new()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(c.text),
                );
                if let Some((label, color)) = status {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(label)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(color),
                        );
                    });
                }
            });
            ui.add_space(tokens::SP_2);
            for &(key, value) in rows {
                kv_row(ui, key, value);
            }
        });
}

fn section_head(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.add_space(tokens::SP_2);
}

#[cfg(test)]
mod tests;
