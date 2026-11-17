//! New project's dialog, drawn in the project feature's context. Creating
//! the project replaces the session, so the shell carries the creation out
//! as the dialog returns.

use egui::{Context, Ui, vec2};

use crate::state::workspace::DEFAULT_SCHEMATIC_VIEW;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize, input_row, select};
use crate::workbench::app::dialogs::project::new_project::{NO_TECHNOLOGY, NewProjectDialogState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use rspice_ui_kit::panels::elide_text;

/// The label column of the dialog's own rows, as the input rows set theirs.
const LABEL_WIDTH: f32 = 92.0;

impl ProjectFeature {
    /// Draw New project. Create hands the shell the creation, which replaces
    /// the session; the dialog stays open until the shell writes the outcome
    /// back into it.
    pub(in crate::workbench) fn render_new_project_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_project.open {
            return;
        }
        // The footer is declared before the body runs, so the primary gate
        // reads the identity as of this frame's start. Creation re-reads and
        // re-validates the post-edit identity, so the one-frame lag can never
        // let an invalid identity through.
        let entry_valid = cx.state.new_project.creatable().is_ok();
        // What Create waits for, or why it refuses, stands in the footer
        // beside it, so the body ends at its last row.
        let (hint, tone) = match cx.state.new_project.footer() {
            Ok(fact) => (fact.to_owned(), DialogHintTone::Neutral),
            Err(refusal) => (refusal, DialogHintTone::Error),
        };

        let dialog = &mut cx.state.new_project;
        let choice = Dialog::new("Create RSpice project", "Create project")
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint(hint)
            .hint_tone(tone)
            .primary_enabled(entry_valid)
            .show(ctx, |ui| {
                // One column at every width: the location's path and the
                // project file's want the whole row, and two columns of
                // fields were what a phone could not hold.
                render_identity(ui, dialog);
                ui.add_space(tokens::SP_5);
                render_contract(ui, dialog);
            });

        match choice {
            DialogChoice::Primary => cx.wb.outbox.push(ProjectRequest::CreateProject),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                cx.state.new_project = NewProjectDialogState::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
const DESCRIPTION: &str = "Name the new project, where it lives, its editable design library, its \
     top cell and its technology. Create writes the project file and replaces the current \
     session.";
#[cfg(target_arch = "wasm32")]
const DESCRIPTION: &str = "Name the new project, its editable design library, its top cell and \
     its technology before it replaces the current session.";

fn render_identity(ui: &mut Ui, dialog: &mut NewProjectDialogState) {
    ui.spacing_mut().item_spacing.y = 2.0;

    let mut edited = input_row(ui, "Project name", &mut dialog.name).changed();
    if edited {
        dialog.mirror_library_from_name();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        edited |= location_row(ui, &mut dialog.location);
    }
    if input_row(ui, "Library", &mut dialog.library).changed() {
        dialog.library_edited = true;
        edited = true;
    }
    edited |= input_row(ui, "Top cell", &mut dialog.top_cell).changed();
    edited |= technology_row(ui, dialog);
    if edited {
        // A refusal describes the identity that was submitted; editing one
        // retires it rather than leaving a stale reason beside new fields.
        dialog.error = None;
    }
}

/// A form row: the dimmed label in the input rows' column, then `content`
/// in what is left. The row is the theme's row height.
fn form_row<R>(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    let row_h = t.metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_WIDTH, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                t.color.text_dim,
            );
            content(ui)
        },
    )
    .inner
}

/// The folder the project's own folder is made in: typed, or picked with
/// Browse. Returns whether it changed.
#[cfg(not(target_arch = "wasm32"))]
fn location_row(ui: &mut Ui, location: &mut String) -> bool {
    use crate::ui::widgets::{Button, mono_input};
    form_row(ui, "Location", |ui| {
        let browse = Button::new("Browse\u{2026}");
        let browse_width = browse.measured_width(ui);
        let field_width = (ui.available_width() - browse_width - 8.0).max(48.0);
        let mut changed = mono_input(ui, "Location", location, field_width).changed();
        if browse.show(ui).clicked() {
            let mut picker = rfd::FileDialog::new().set_title("Project location");
            let current = std::path::Path::new(location.trim());
            if current.is_dir() {
                picker = picker.set_directory(current);
            }
            if let Some(picked) = picker.pick_folder() {
                *location = picked.display().to_string();
                changed = true;
            }
        }
        changed
    })
}

/// The technology the project is created with, from what this session can
/// attach, or none. Returns whether the choice changed.
fn technology_row(ui: &mut Ui, dialog: &mut NewProjectDialogState) -> bool {
    let options = std::iter::once(NO_TECHNOLOGY.to_owned())
        .chain(
            dialog
                .technologies
                .iter()
                .map(|technology| technology.label.clone()),
        )
        .collect::<Vec<_>>();
    let selected = dialog.technology_label().to_owned();
    let picked = form_row(ui, "Technology", |ui| {
        let width = ui.available_width();
        select(
            ui,
            "new_project.technology",
            "Technology",
            &selected,
            &options,
            width,
        )
    });
    let Some(index) = picked else {
        return false;
    };
    let technology = index
        .checked_sub(1)
        .and_then(|index| dialog.technologies.get(index))
        .cloned();
    let changed = technology != dialog.technology;
    dialog.technology = technology;
    changed
}

fn render_contract(ui: &mut Ui, dialog: &NewProjectDialogState) {
    let params = dialog.params();
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    // The "Safety contract" card above this said the same four things for
    // every project anyone ever created, so it was not about this one.
    let top_view = format!(
        "{}/{}/{DEFAULT_SCHEMATIC_VIEW}",
        params.root_library, params.top_cell
    );
    egui::Frame::new()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("Creates")
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(c.text),
            );
            ui.add_space(tokens::SP_2);
            #[cfg(not(target_arch = "wasm32"))]
            {
                let file = dialog
                    .project_file()
                    .map_or_else(|_| "\u{2014}".to_owned(), |file| file.display().to_string());
                contract_row(ui, "Project file", &file, rspice_ui_kit::panels::elide_path);
            }
            contract_row(ui, "Top cell view", &top_view, elide_text);
            contract_row(ui, "Simulation plan", "personal default", elide_text);
            contract_row(ui, "Drawing sheet", "personal default", elide_text);
        });
}

/// How a value is shortened to the room beside its key.
type Shorten = fn(&Ui, &str, &egui::FontId, f32) -> String;

/// A key and its value, the value shortened to the room beside the key (a
/// path from its middle, so the file's own name stays in view) and whole on
/// hover.
fn contract_row(ui: &mut Ui, key: &str, value: &str, shorten: Shorten) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 22.0), egui::Sense::hover());
    let key_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let key_width = ui
        .painter()
        .layout_no_wrap(key.to_owned(), key_font.clone(), c.text_dim)
        .size()
        .x;
    let font = theme::field_mono();
    let shown = shorten(ui, value, &font, rect.width() - key_width - 12.0);
    let elided = shown != value;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, true, format!("{key}: {value}"))
    });
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        key,
        key_font,
        c.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.right(), rect.center().y),
        egui::Align2::RIGHT_CENTER,
        shown,
        font,
        c.text,
    );
    if elided {
        response.on_hover_text(value);
    }
}
