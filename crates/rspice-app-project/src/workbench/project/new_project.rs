//! New project's dialog, drawn in the project feature's context. Creating
//! the project replaces the session, so the shell carries the creation out
//! as the dialog returns.
//!
//! Two bands, Project and Design, each a column of fields that share one
//! label column and one right edge. A field holding what Create cannot use
//! wears the error border and says why under itself; the footer says it
//! again beside the Create it holds back, and is silent otherwise.

use egui::{Context, Id, Response, RichText, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    DIALOG_BODY_INSET, Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize,
    band_header, name_control, select_with_details, text_field,
};
use crate::workbench::app::dialogs::project::new_project::{
    NO_TECHNOLOGY, NewProjectDialogState, NewProjectField,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

/// The dialog's width: a project's path, in a folder under the reader's
/// documents, whole on the line that says where it is saved.
const DIALOG_WIDTH: f32 = 560.0;
/// The least a field keeps beside the label column. With less room, each
/// label stands above its field, which then takes the whole row.
const FIELD_MIN_WIDTH: f32 = 300.0;
/// Air between the label column and the fields.
const LABEL_GAP: f32 = tokens::SP_5;
/// Every label the form has; the label column is as wide as the widest.
const LABELS: [&str; 5] = ["Name", "Location", "Library", "Top cell", "Technology"];

impl ProjectFeature {
    /// Draw New project. Create hands the shell the creation, which replaces
    /// the session; the dialog stays open until the shell writes the outcome
    /// back into it.
    pub(in crate::workbench) fn render_new_project_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_project.open {
            return;
        }
        let dialog = &mut cx.state.new_project;
        dialog.refresh_folder_checks();
        // Declared before the body runs, so the gate reads the fields as of
        // this frame's start. Creation validates them again, so the lag of a
        // frame can never let an invalid identity through.
        let creatable = dialog.creatable().is_ok();
        let mut shell = Dialog::new("New project", "Create")
            .size(DialogSize::Transaction)
            .width_from_content(DIALOG_WIDTH)
            .flush_body()
            .overlay_scrollbar()
            .ghost("Cancel")
            .primary_enabled(creatable)
            .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(reason) = dialog.footer() {
            shell = shell.hint(reason).hint_tone(DialogHintTone::Error);
        }
        let choice = shell.show_with_initial_body_focus(ctx, |ui| Some(render_form(ui, dialog)));

        match choice {
            DialogChoice::Primary => cx.wb.outbox.push(ProjectRequest::CreateProject),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                cx.state.new_project = NewProjectDialogState::default();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

/// The two bands. Returns the Name field's id, which the dialog opens on.
fn render_form(ui: &mut Ui, dialog: &mut NewProjectDialogState) -> Id {
    ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
    let form = Form::new(ui);

    band_header(ui, "Project", None);
    ui.add_space(tokens::SP_5);
    let refusal = dialog.refusal(NewProjectField::Name);
    let name = form.field(ui, "Name", refusal.as_deref(), |ui, width| {
        let edit = egui::TextEdit::singleline(&mut dialog.name)
            .id_salt("new_project.name")
            .font(theme::field_sans());
        text_field(ui, edit, width)
    });
    // Opened on, the default name is selected, so typing replaces it.
    select_all_until_focused(ui, &name, &dialog.name);
    let mut edited = name.changed();
    if edited {
        dialog.mirror_library_from_name();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        ui.add_space(tokens::SP_4);
        edited |= location_field(ui, &form, dialog);
    }
    ui.add_space(tokens::SP_5);

    band_header(ui, "Design", None);
    ui.add_space(tokens::SP_5);
    let refusal = dialog.refusal(NewProjectField::Library);
    let library = form.field(ui, "Library", refusal.as_deref(), |ui, width| {
        let edit = egui::TextEdit::singleline(&mut dialog.library)
            .id_salt("new_project.library")
            .font(theme::field_sans());
        text_field(ui, edit, width)
    });
    if library.changed() {
        dialog.library_edited = true;
        edited = true;
    }
    ui.add_space(tokens::SP_4);
    let refusal = dialog.refusal(NewProjectField::TopCell);
    edited |= form
        .field(ui, "Top cell", refusal.as_deref(), |ui, width| {
            let edit = egui::TextEdit::singleline(&mut dialog.top_cell)
                .id_salt("new_project.top_cell")
                .font(theme::field_sans());
            text_field(ui, edit, width)
        })
        .changed();
    ui.add_space(tokens::SP_4);
    edited |= technology_field(ui, &form, dialog);
    ui.add_space(tokens::SP_5);

    if edited {
        // A refusal describes the identity that was submitted; editing one
        // retires it rather than leaving a stale reason beside new fields.
        dialog.error = None;
    }
    name.id
}

/// The folder the project's own folder is made in, typed or picked with
/// Browse, and under it the file Create writes. Returns whether it changed.
#[cfg(not(target_arch = "wasm32"))]
fn location_field(ui: &mut Ui, form: &Form, dialog: &mut NewProjectDialogState) -> bool {
    use crate::ui::widgets::Button;
    let refusal = dialog.refusal(NewProjectField::Location);
    let mut picked = false;
    let location = &mut dialog.location;
    let field = form.field(ui, "Location", refusal.as_deref(), |ui, width| {
        let browse = Button::new("Browse\u{2026}");
        let browse_width = browse.measured_width(ui);
        let edit = egui::TextEdit::singleline(location)
            .id_salt("new_project.location")
            .font(theme::field_sans());
        let field = text_field(ui, edit, (width - browse_width - tokens::SP_4).max(0.0));
        ui.add_space(tokens::SP_4);
        if browse.show(ui).clicked() {
            let mut picker = rfd::FileDialog::new().set_title("Project location");
            let current = std::path::Path::new(location.trim());
            if current.is_dir() {
                picker = picker.set_directory(current);
            }
            if let Some(folder) = picker.pick_folder() {
                *location = folder.display().to_string();
                picked = true;
            }
        }
        field
    });
    // A name the project file refuses names no file: the line would show a
    // path nothing will be written to.
    if refusal.is_none()
        && dialog.refusal(NewProjectField::Name).is_none()
        && let Ok(file) = dialog.project_file()
    {
        form.saved_as(ui, &file.display().to_string());
    }
    field.changed() || picked
}

/// The technology the project is created with, from what this session can
/// attach, or none. Returns whether the choice changed.
fn technology_field(ui: &mut Ui, form: &Form, dialog: &mut NewProjectDialogState) -> bool {
    let options = std::iter::once(NO_TECHNOLOGY.to_owned())
        .chain(
            dialog
                .technologies
                .iter()
                .map(|technology| technology.name.clone()),
        )
        .collect::<Vec<_>>();
    let versions = std::iter::once(String::new())
        .chain(
            dialog
                .technologies
                .iter()
                .map(|technology| technology.version.clone()),
        )
        .collect::<Vec<_>>();
    let selected = dialog.technology_label().to_owned();
    let mut picked = None;
    form.field(ui, "Technology", None, |ui, width| {
        let output = select_with_details(
            ui,
            "new_project.technology",
            "Technology",
            &selected,
            &options,
            &versions,
            width,
        );
        picked = output.picked;
        output.response
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

/// The form's one label column, or labels above their fields when the
/// dialog is too narrow to set them side by side.
struct Form {
    label_width: f32,
    stacked: bool,
}

impl Form {
    fn new(ui: &Ui) -> Self {
        let label_width = LABELS
            .iter()
            .map(|label| text_width(ui, label))
            .fold(0.0, f32::max);
        let room = ui.available_width() - 2.0 * DIALOG_BODY_INSET - label_width - LABEL_GAP;
        Self {
            label_width,
            stacked: room < FIELD_MIN_WIDTH,
        }
    }

    /// Where the fields start, from the body's left edge.
    fn field_left(&self) -> f32 {
        if self.stacked {
            DIALOG_BODY_INSET
        } else {
            DIALOG_BODY_INSET + self.label_width + LABEL_GAP
        }
    }

    /// One labelled field: `field` drawn at the field column's width, and
    /// named for its label. A `refusal` marks it and is said under it.
    fn field(
        &self,
        ui: &mut Ui,
        label: &str,
        refusal: Option<&str>,
        field: impl FnOnce(&mut Ui, f32) -> Response,
    ) -> Response {
        let t = Tokens::get(ui.ctx());
        let width = ui.available_width() - self.field_left() - DIALOG_BODY_INSET;
        let text = RichText::new(label)
            .font(form_font())
            .color(t.color.text_dim);
        if self.stacked {
            // Top-aligned, so the row is the label's height and the label
            // sits on its field rather than centred in a control's height.
            ui.horizontal_top(|ui| {
                ui.add_space(DIALOG_BODY_INSET);
                ui.add(egui::Label::new(text.clone()).truncate());
            });
            ui.add_space(tokens::SP_2);
        }
        let response = ui
            .horizontal(|ui| {
                ui.add_space(DIALOG_BODY_INSET);
                if !self.stacked {
                    ui.allocate_ui_with_layout(
                        vec2(self.label_width + LABEL_GAP, t.metrics.ctl_h),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.set_min_size(vec2(self.label_width + LABEL_GAP, t.metrics.ctl_h));
                            ui.add(egui::Label::new(text).truncate());
                        },
                    );
                }
                field(ui, width)
            })
            .inner;
        name_control(ui, &response, label);
        if let Some(refusal) = refusal {
            mark_refused(ui, &response, refusal);
            self.under(ui, |ui| {
                ui.add(
                    egui::Label::new(RichText::new(refusal).font(form_font()).color(t.color.err))
                        .wrap(),
                );
            });
        }
        response
    }

    /// The file Create writes, dim under the Location field: shortened from
    /// the middle at a separator when the column cannot hold it, and whole on
    /// hover.
    #[cfg(not(target_arch = "wasm32"))]
    fn saved_as(&self, ui: &mut Ui, path: &str) {
        const PREFIX: &str = "Saved as ";
        let t = Tokens::get(ui.ctx());
        let room = ui.available_width() - self.field_left() - DIALOG_BODY_INSET;
        let shown =
            crate::ui::panels::elide_path(ui, path, &form_font(), room - text_width(ui, PREFIX));
        self.under(ui, |ui| {
            let line = ui.add(
                egui::Label::new(
                    RichText::new(format!("{PREFIX}{shown}"))
                        .font(form_font())
                        .color(t.color.text_dim),
                )
                .truncate(),
            );
            if shown != path {
                line.on_hover_text(path);
            }
        });
    }

    /// A line under a field, in the field column.
    fn under(&self, ui: &mut Ui, line: impl FnOnce(&mut Ui)) {
        let width = ui.available_width() - self.field_left() - DIALOG_BODY_INSET;
        ui.add_space(tokens::SP_2);
        ui.horizontal_top(|ui| {
            ui.add_space(self.field_left());
            ui.allocate_ui_with_layout(
                vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(width);
                    line(ui);
                },
            );
        });
    }
}

/// The error border on a field that refuses what it holds, and the refusal
/// told to a screen reader with it.
fn mark_refused(ui: &Ui, response: &Response, refusal: &str) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_stroke(
        response.rect,
        t.radius,
        egui::Stroke::new(1.0, t.color.err),
        egui::StrokeKind::Inside,
    );
    let refusal = refusal.to_owned();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(refusal);
        node.set_invalid(egui::accesskit::Invalid::True);
    });
}

/// Hold the whole of `value` selected until the field takes the keyboard,
/// so the first keystroke after the dialog opens on it replaces the value.
fn select_all_until_focused(ui: &Ui, response: &Response, value: &str) {
    if response.has_focus() || value.is_empty() {
        return;
    }
    let mut state = egui::TextEdit::load_state(ui.ctx(), response.id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            egui::text::CCursor::new(value.chars().count()),
        )));
    egui::TextEdit::store_state(ui.ctx(), response.id, state);
}

/// The face of the labels and of the lines under the fields.
fn form_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Regular)
}

fn text_width(ui: &Ui, text: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), form_font(), egui::Color32::WHITE)
        .size()
        .x
}
