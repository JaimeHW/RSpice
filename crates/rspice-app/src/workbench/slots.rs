//! The state each feature owns, one slot per feature.
//!
//! [`AppState`](crate::workbench::app_state::AppState) holds a slot for every
//! feature that keeps state between frames: its open dialogs, the drafts they
//! edit, and its interaction session. A feature's slot is its own; another
//! feature reads it through the owner and changes it by asking the owner. A
//! slot carries no serde: what a session keeps of it is named, field for
//! field, by the session wire. Each is `Default + Clone`, because a project
//! transaction builds its result in a clone of the whole state.

mod cloud;
mod code;
mod design;
mod hardcopy;
mod models;
pub(in crate::workbench) use rspice_app_services::workbench::slots::platform;
mod project;
mod results;
pub(crate) use rspice_app_runs::workbench::slots::runs;
mod settings;
mod sheets;
mod shell;
mod simulate;

pub(crate) use cloud::CloudState;
pub(crate) use code::CodeState;
pub(crate) use design::DesignState;
pub(crate) use hardcopy::HardcopyState;
pub(crate) use models::ModelsState;
pub(crate) use platform::PlatformState;
pub(crate) use project::ProjectState;
pub(crate) use results::ResultsState;
pub(crate) use runs::RunsState;
pub(crate) use settings::SettingsState;
pub(crate) use sheets::SheetsState;
pub(crate) use shell::ShellState;
pub(crate) use simulate::SimulateState;

use crate::workbench::app_state::AppState;

impl AppState {
    /// Whether an app-owned modal has exclusive keyboard intent.
    ///
    /// This is deliberately derived from the authoritative workflow states,
    /// rather than from egui's previous-frame modal layer. Shortcut dispatch
    /// runs before dialogs are painted, so retained state is the only value
    /// that is both current and deterministic at that point in the frame.
    /// The shell lists what the dialog gate counts.
    pub(crate) fn dialog_modal_open(&self) -> bool {
        crate::workbench::framework::dialogs::dialog_modal_open(self)
    }
}

#[cfg(test)]
mod defaults_tests;
#[cfg(test)]
mod tests;
