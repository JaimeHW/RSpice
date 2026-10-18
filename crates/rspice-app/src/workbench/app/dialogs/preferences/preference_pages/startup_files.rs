//! Projects and files ▸ Startup and files: what RSpice opens with, how long
//! the recent list is, and whether a save keeps the file it replaces.

use egui::Ui;

use crate::ui::widgets::select;
use crate::workbench::ScalarPreference;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::{ChoicePreference, TogglePreference};

use super::super::preferences_shell::{
    control_width, searchable_row, section_heading, value_column,
};
use super::AppState;
#[cfg(not(target_arch = "wasm32"))]
use super::{keyword_choice_row, toggle_row};

/// The recent-list lengths offered, shortest first. The shipped one is 8.
const RECENT_FILES_LISTED: [u32; 4] = [5, 8, 12, 20];

pub(super) fn section(ui: &mut Ui, state: &mut AppState) {
    section_heading(ui, "Startup and files");
    // The browser binds its project file after startup, once its storage
    // answers, and a browser save is a download with no earlier file to keep,
    // so these two rows are the desktop's.
    #[cfg(not(target_arch = "wasm32"))]
    keyword_choice_row(
        ui,
        state,
        ChoicePreference::StartupProject,
        "On startup",
        "A project with unsaved work, or never saved to a file, reopens either way.",
        &["Reopen last project", "Start with no project"],
        &["launch", "open", "session", "restore"],
    );
    recent_files_row(ui, state);
    #[cfg(not(target_arch = "wasm32"))]
    toggle_row(
        ui,
        state,
        TogglePreference::KeepSaveBackup,
        "Keep a backup when saving",
        "A standalone schematic save first copies the old file to .rsch.bak; project saves do not.",
        "Keep a backup when saving a schematic file",
    );
}

/// How many recently opened projects and schematics the recent list keeps.
fn recent_files_row(ui: &mut Ui, state: &mut AppState) {
    let labels = RECENT_FILES_LISTED
        .map(|count| format!("{count} files"))
        .to_vec();
    searchable_row(
        ui,
        "Recent files listed",
        "How many recently opened projects and schematics RSpice remembers.",
        &["recent", "history", "mru"],
        |ui| {
            value_column(ui, |ui| {
                // Shown as the count it is, so a stored count between two
                // of the offered ones is not misreported as its neighbour.
                let current = format!(
                    "{} files",
                    state
                        .ui
                        .preferences
                        .scalar(ScalarPreference::RecentFilesListed)
                );
                if let Some(selected) = select(
                    ui,
                    "preferences.projects.recent-files-listed",
                    "Recent files listed",
                    &current,
                    &labels,
                    control_width(ui),
                ) {
                    state
                        .ui
                        .preferences
                        .set_scalar(
                            ScalarPreference::RecentFilesListed,
                            RECENT_FILES_LISTED[selected],
                        )
                        .expect("every offered count is inside the preference domain");
                    state.trim_recent_files();
                }
            });
        },
    );
}
