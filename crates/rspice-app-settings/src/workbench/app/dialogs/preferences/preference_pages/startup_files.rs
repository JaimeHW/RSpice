//! Projects and files ▸ Startup and files: what RSpice opens with, how long
//! the recent list is, whether a save keeps the file it replaces, and how
//! much space automatic checkpoints are kept in.

use egui::Ui;

use crate::ui::widgets::select;
use crate::workbench::ScalarPreference;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::{ChoicePreference, TogglePreference};

use super::super::preferences_shell::{
    control_width, searchable_row, section_heading, value_column,
};
use super::PreferencesDialogs;
#[cfg(not(target_arch = "wasm32"))]
use super::{keyword_choice_row, toggle_row};

/// The recent-list lengths offered, shortest first. The shipped one is 8.
const RECENT_FILES_LISTED: [u32; 4] = [5, 8, 12, 20];

/// The spaces offered for automatic checkpoints, in MiB, smallest first. The
/// shipped one is 512 MiB.
const CHECKPOINT_BUDGETS_MIB: [u32; 7] = [128, 256, 512, 1024, 2048, 4096, 8192];

pub(super) fn section(ui: &mut Ui, state: &mut PreferencesDialogs<'_>) {
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
        "Saving over a schematic file first copies it to .rsch.bak. Project saves do not.",
        "Keep a backup when saving a schematic file",
    );
    checkpoint_budget_row(ui, state);
}

/// A space in the whole units it is offered in: "512 MiB", "2 GiB".
fn space_label(mib: u32) -> String {
    if mib >= 1024 && mib.is_multiple_of(1024) {
        format!("{} GiB", mib / 1024)
    } else {
        format!("{mib} MiB")
    }
}

/// How much space each project's automatic checkpoints are kept in.
fn checkpoint_budget_row(ui: &mut Ui, state: &mut PreferencesDialogs<'_>) {
    let labels = CHECKPOINT_BUDGETS_MIB.map(space_label).to_vec();
    searchable_row(
        ui,
        "Automatic checkpoint space",
        "Per project. Checkpoints taken before saves and risky changes are kept in it, \
         oldest dropped first; manual and pinned ones stay until you delete them.",
        &["checkpoint", "recovery", "disk", "budget", "retention"],
        |ui| {
            value_column(ui, |ui| {
                // Shown as the space it is, so a stored space between two of
                // the offered ones is not misreported as its neighbour.
                let current = space_label(
                    state
                        .model
                        .preferences
                        .scalar(ScalarPreference::CheckpointBudgetMib),
                );
                if let Some(selected) = select(
                    ui,
                    "preferences.projects.checkpoint-budget",
                    "Automatic checkpoint space",
                    &current,
                    &labels,
                    control_width(ui),
                ) {
                    state
                        .model
                        .preferences
                        .set_scalar(
                            ScalarPreference::CheckpointBudgetMib,
                            CHECKPOINT_BUDGETS_MIB[selected],
                        )
                        .expect("every offered space is inside the preference domain");
                }
            });
        },
    );
}

/// How many recent projects the start page lists.
fn recent_files_row(ui: &mut Ui, state: &mut PreferencesDialogs<'_>) {
    let labels = RECENT_FILES_LISTED
        .map(|count| format!("{count} projects"))
        .to_vec();
    searchable_row(
        ui,
        "Recent projects listed",
        "How many recently opened projects the start page lists.",
        &["recent", "history", "mru"],
        |ui| {
            value_column(ui, |ui| {
                // Shown as the count it is, so a stored count between two
                // of the offered ones is not misreported as its neighbour.
                let current = format!(
                    "{} projects",
                    state
                        .model
                        .preferences
                        .scalar(ScalarPreference::RecentFilesListed)
                );
                if let Some(selected) = select(
                    ui,
                    "preferences.projects.recent-files-listed",
                    "Recent projects listed",
                    &current,
                    &labels,
                    control_width(ui),
                ) {
                    state
                        .model
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
