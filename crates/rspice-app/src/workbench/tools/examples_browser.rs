//! Examples: the demonstration circuits, each opened as a project of its own.
//!
//! The circuits are the ones `workbench::examples` draws through the
//! schematic's own API, which until this dialog only the tests reached.
//! Opening one replaces the session with a new project named after it, draws
//! the circuit as its top cell, and saves it where the reader says: a file
//! they choose on the desktop, this browser's storage on the web. While the
//! open project has unsaved changes, Open asks to save them first, as New
//! project does.

use egui::{Context, Sense, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};
use crate::workbench::app_state::{AppState, Model, PlatformState};
use crate::workbench::framework::feature::{Cx, ModelsPort};
use crate::workbench::project::ProjectFeature;
use crate::workbench::slots::ProjectState;
use crate::workbench::workflows::project_workflow::NewProjectParams;
use rspice_app_state::workbench::examples::{EXAMPLES, Example, load_example};

/// The name column every row shares.
const NAME_COLUMN: f32 = 220.0;
/// How far a row's highlight reaches past its words on either side: the
/// words keep the dialog's left edge, and the highlight runs out to the
/// frame, as a table row's fill does.
const HIGHLIGHT_BLEED: f32 = crate::ui::widgets::DIALOG_BODY_INSET;

/// Replace the session with a new project holding `example`, named after it.
///
/// The project is not saved: [`show`] asks where once this succeeds.
pub(crate) fn create_example_project(state: &mut AppState, example: Example) -> Result<(), String> {
    let params = NewProjectParams {
        name: example.name.to_owned(),
        ..NewProjectParams::default()
    };
    state.create_new_project_with(&params)?;
    load_example(example.name, &mut state.model.schematic);
    state.sync_active_schematic_to_workspace();
    Ok(())
}

/// Open `name` as a new project, then ask where to save it.
fn open_example(state: &mut AppState, name: &str) {
    if let Some(example) = EXAMPLES.into_iter().find(|example| example.name == name)
        && create_example_project(state, example).is_ok()
    {
        state.save_project_as();
    }
}

/// Open the example New project's prompt let through, once the project it
/// replaces was saved or let go.
pub(crate) fn open_confirmed(state: &mut AppState) {
    if let Some(name) = state.project.examples_confirmed.take() {
        open_example(state, &name);
    }
}

/// Open `example` in place of the open project, asking first about any
/// changes it has, as New project does.
pub(crate) fn request_example(state: &mut AppState, example: Example) {
    let unsaved = state.platform.project_lifecycle.is_open()
        && (crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
            || ModelsPort::model_editor_has_unsaved_changes(&*state.models));
    if unsaved {
        state
            .workbench
            .confirmation_dialog
            .show_example(example.name);
    } else {
        open_example(state, example.name);
    }
}

/// Why Open is refused, if it is: configuration edits not applied, which
/// no save prompt resolves.
pub(crate) fn open_refusal(
    project: &ProjectState,
    model: &Model,
    platform: &PlatformState,
) -> Option<&'static str> {
    if !platform.project_lifecycle.is_open() {
        return None;
    }
    project
        .configuration
        .is_dirty(&model.workspace.content.configuration_sets)
        .then_some("Apply or revert the configuration edits first: an example replaces the project")
}

/// Where a row of `rect` paints: its highlight, and the x its name starts
/// at, which is the dialog's left edge.
fn row_geometry(rect: egui::Rect) -> (egui::Rect, f32) {
    (rect.expand2(vec2(HIGHLIGHT_BLEED, 0.0)), rect.left())
}

/// One example in the list: its name, then what it is.
fn example_row(ui: &mut egui::Ui, example: Example, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), t.metrics.ctl_h + tokens::SP_4),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            true,
            selected,
            format!("{}: {}", example.name, example.summary),
        )
    });
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let (highlight, name_left) = row_geometry(rect);
        if selected {
            painter.rect_filled(highlight, 0.0, t.color.bg_active);
        } else if response.hovered() {
            painter.rect_filled(highlight, 0.0, t.color.bg_hover);
        }
        painter.text(
            egui::pos2(name_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            example.name,
            theme::sans(tokens::FS_1, FontWeight::Medium),
            t.color.text,
        );
        painter.text(
            egui::pos2(name_left + NAME_COLUMN, rect.center().y),
            egui::Align2::LEFT_CENTER,
            example.summary,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    // The selection fill already shows which row a click chose; the ring is
    // for a reader who tabbed here.
    if theme::focus_is_visible(ui.ctx(), &response) {
        theme::paint_focus_ring(ui, &response, row_geometry(rect).0);
    }
    response
}

/// The Examples dialog, while it is open. It returns the example the reader
/// asked to open, for the shell to request in place of the open project.
pub(crate) fn show(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) -> Option<Example> {
    let mut selected = cx.state.examples_selection?;
    selected = selected.min(EXAMPLES.len() - 1);
    let refusal = open_refusal(cx.state, cx.model, cx.platform);
    let (hint, tone) = match refusal {
        Some(refusal) => (refusal, DialogHintTone::Warn),
        None if cfg!(target_arch = "wasm32") => (
            "Opens as a new project saved in this browser",
            DialogHintTone::Neutral,
        ),
        None => (
            "Opens as a new project you save where you choose",
            DialogHintTone::Neutral,
        ),
    };
    let mut open = false;
    let choice = Dialog::new("Examples", "Open")
        .description("Open a demonstration circuit as a new project of its own.")
        .size(DialogSize::Transaction)
        // Six rows never scroll: no gutter cutting the rows short of the
        // right edge.
        .overlay_scrollbar()
        .ghost("Cancel")
        .primary_enabled(refusal.is_none())
        .hint(hint)
        .hint_tone(tone)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = tokens::SP_2;
            for (index, example) in EXAMPLES.into_iter().enumerate() {
                let response = example_row(ui, example, index == selected);
                if response.clicked() {
                    selected = index;
                }
                if response.double_clicked() && refusal.is_none() {
                    selected = index;
                    open = true;
                }
            }
            if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
                selected = (selected + 1).min(EXAMPLES.len() - 1);
            }
            if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
                selected = selected.saturating_sub(1);
            }
        });
    cx.state.examples_selection = Some(selected);
    match choice {
        DialogChoice::Primary => open = true,
        DialogChoice::None => {}
        _ => {
            cx.state.examples_selection = None;
            return None;
        }
    }
    if open {
        cx.state.examples_selection = None;
        return Some(EXAMPLES[selected]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An example's name starts on the dialog's left edge, the title's, and
    /// its highlight stands proud of the row on both sides rather than
    /// pushing the words in.
    #[test]
    fn an_example_row_keeps_the_dialog_left_edge() {
        let rect = egui::Rect::from_min_size(egui::pos2(353.0, 0.0), vec2(734.0, 36.0));
        let (highlight, name_left) = row_geometry(rect);
        assert_eq!(name_left, rect.left());
        assert!(highlight.left() < rect.left() && highlight.right() > rect.right());
        assert_eq!(highlight.y_range(), rect.y_range());
    }
}
