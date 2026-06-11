use super::super::model::PdkSettingsDialogState;
use crate::ui::tokens::Tokens;
use egui::{RichText, Ui};

/// Render the Library Paths tab
pub(in crate::panels::pdk_settings_dialog) fn render_library_paths_tab(
    ui: &mut Ui,
    state: &mut PdkSettingsDialogState,
) {
    let c = Tokens::get(ui.ctx()).color;

    ui.heading("Library Search Paths");
    ui.add_space(4.0);

    ui.label(
        RichText::new("Configure directories where RSpice will search for PDK model files (.lib, .scs, .mod).")
            .color(c.text_dim)
            .size(11.0),
    );

    ui.add_space(8.0);

    let mut action: Option<PathListAction> = None;

    for (idx, entry) in state.config.library_paths().iter().enumerate() {
        ui.horizontal(|ui| {
            let mut enabled = entry.enabled;
            if ui.checkbox(&mut enabled, "").changed() {
                action = Some(PathListAction::ToggleEnabled(idx));
            }

            let mut recursive = entry.recursive;
            if ui
                .checkbox(&mut recursive, "")
                .on_hover_text("Scan subdirectories")
                .changed()
            {
                action = Some(PathListAction::ToggleRecursive(idx));
            }
            ui.label(RichText::new("R").size(10.0).color(if recursive {
                c.ok
            } else {
                c.text_faint
            }));

            if state.editing_path_index == Some(idx) {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut state.edit_path_buffer).desired_width(350.0),
                );
                if response.lost_focus() {
                    action = Some(PathListAction::FinishEdit(idx));
                }
            } else {
                let path_text = if entry.enabled {
                    RichText::new(&entry.path)
                } else {
                    RichText::new(&entry.path).color(c.text_faint)
                };
                if ui.link(path_text).clicked() {
                    state.editing_path_index = Some(idx);
                    state.edit_path_buffer = entry.path.clone();
                }
            }

            if entry.file_count > 0 {
                ui.label(
                    RichText::new(format!("({} files)", entry.file_count))
                        .color(c.ok)
                        .size(10.0),
                );
            }

            if ui.small_button("remove").on_hover_text("Remove path").clicked() {
                action = Some(PathListAction::Remove(idx));
            }

            // Native folder pickers don't exist in the browser build; the
            // editable text path remains available there.
            #[cfg(not(target_arch = "wasm32"))]
            if ui.small_button("browse").on_hover_text("Browse...").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .set_directory(&entry.path)
                    .pick_folder()
            {
                action = Some(PathListAction::Update(
                    idx,
                    path.to_string_lossy().to_string(),
                ));
            }
        });
    }

    if let Some(act) = action {
        match act {
            PathListAction::Remove(idx) => state.remove_library_path(idx),
            PathListAction::ToggleEnabled(idx) => state.toggle_path_enabled(idx),
            PathListAction::ToggleRecursive(idx) => state.toggle_path_recursive(idx),
            PathListAction::FinishEdit(idx) => {
                if !state.edit_path_buffer.is_empty() {
                    let paths = state.config.library_paths_mut();
                    if idx < paths.len() {
                        paths[idx].path = state.edit_path_buffer.clone();
                    }
                }
                state.editing_path_index = None;
                state.edit_path_buffer.clear();
            }
            PathListAction::Update(idx, new_path) => {
                let paths = state.config.library_paths_mut();
                if idx < paths.len() {
                    paths[idx].path = new_path;
                }
            }
        }
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let text_edit = egui::TextEdit::singleline(&mut state.new_path_input)
            .desired_width(400.0)
            .hint_text("Add library path...");
        let response = ui.add(text_edit);

        if (ui.button("Add").clicked()
            || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
            && !state.new_path_input.is_empty()
        {
            state.add_library_path(state.new_path_input.clone());
        }

        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Browse...").clicked()
            && let Some(path) = rfd::FileDialog::new().pick_folder()
        {
            state.add_library_path(path.to_string_lossy().to_string());
        }
    });

    ui.add_space(12.0);
    ui.label(
        RichText::new("Tip: Use environment variables like $PDK_HOME in paths. Configure them in the Environment tab.")
            .color(c.text_dim)
            .size(11.0),
    );
}

enum PathListAction {
    Remove(usize),
    ToggleEnabled(usize),
    ToggleRecursive(usize),
    FinishEdit(usize),
    Update(usize, String),
}
