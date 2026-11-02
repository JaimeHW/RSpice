//! New project's dialog, drawn in the project feature's context. Creating
//! the project replaces the session, so the shell carries the creation out
//! as the dialog returns.

use egui::Context;

use crate::state::workspace::{DEFAULT_SCHEMATIC_VIEW, PROJECT_DESCRIPTOR_SCHEMA_VERSION};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize, input_row, kv_row};
use crate::workbench::app::dialogs::project::new_project::NewProjectDialogState;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

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
        let entry_valid = cx.state.new_project.params().validate().is_ok();
        // What Create waits for, or why it refuses, stands in the footer
        // beside it, so the body ends at its last row.
        let (hint, tone) = match cx.state.new_project.footer() {
            Ok(fact) => (fact.to_owned(), DialogHintTone::Neutral),
            Err(refusal) => (refusal, DialogHintTone::Error),
        };

        let dialog = &mut cx.state.new_project;
        let choice = Dialog::new("Create RSpice project", "Create project")
            .description(
                "Name the new project, its editable design library, and its top cell before it \
             replaces the current session.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint(hint)
            .hint_tone(tone)
            .primary_enabled(entry_valid)
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    render_identity(&mut columns[0], dialog);
                    render_contract(&mut columns[1], dialog);
                });
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

fn render_identity(ui: &mut egui::Ui, dialog: &mut NewProjectDialogState) {
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
}

fn render_contract(ui: &mut egui::Ui, dialog: &NewProjectDialogState) {
    let params = dialog.params();

    // The "Safety contract" card above this said the same four things for
    // every project anyone ever created, so it was not about this one.
    let top_view = format!(
        "{}/{}/{DEFAULT_SCHEMATIC_VIEW}",
        params.root_library, params.top_cell
    );
    // The enabled Create button says the inputs are complete, and the field
    // errors say which one is not.
    property_card(
        ui,
        "Creates",
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
fn property_card(ui: &mut egui::Ui, title: &str, rows: &[(&str, &str)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::new()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(c.text),
            );
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
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;
    use crate::workbench::app::dialogs::project::new_project::open_new_project_dialog;
    use crate::workbench::project::tests::with_cx;

    /// New project open on `name`, over the session it would replace.
    fn naming(name: &str) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        open_new_project_dialog(&mut app.state);
        app.state.project.new_project.name = name.to_owned();
        app
    }

    /// What a creation leaves showing: the dialog, the status bar, the
    /// Console, and the project's root library, top cell and open view.
    fn left(
        app: &RSpiceApp,
    ) -> (
        NewProjectDialogState,
        Option<String>,
        Vec<String>,
        (String, String, String),
    ) {
        let content = &app.state.model.workspace.content;
        (
            app.state.project.new_project.clone(),
            app.state.prompt_text().map(str::to_owned),
            app.state
                .model
                .log_buffer
                .entries()
                .map(|entry| entry.message.clone())
                .collect(),
            (
                content.project.root_library.clone(),
                content.project.top_cell.clone(),
                content.active_view.display_path(),
            ),
        )
    }

    /// A creation handed to the shell ends where the creation carried out in
    /// place did, and the dialog held open for it takes the outcome: closed
    /// once the project is created, the refusal stated when it is not.
    #[test]
    fn a_project_created_by_the_shell_ends_where_the_creation_in_place_did() {
        for name in ["Bandgap reference", "  "] {
            let mut in_place = naming(name);
            let params = in_place.state.project.new_project.params();
            match in_place.state.create_new_project_with(&params) {
                Ok(()) => in_place.state.project.new_project = NewProjectDialogState::default(),
                Err(error) => in_place.state.project.new_project.error = Some(error),
            }

            let mut handed = naming(name);
            with_cx(&mut handed, |cx| {
                cx.wb.outbox.push(ProjectRequest::CreateProject);
            });
            handed.after_contribution();

            assert_eq!(left(&handed), left(&in_place), "{name:?}");
            assert_eq!(
                handed.state.project.new_project.open,
                name.trim().is_empty(),
                "{name:?}"
            );
        }
    }
}
