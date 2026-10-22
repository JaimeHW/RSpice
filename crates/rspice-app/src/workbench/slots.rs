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
mod platform;
mod project;
mod results;
mod runs;
mod settings;
mod sheets;
mod shell;
mod simulate;

pub(crate) use cloud::CloudState;
pub(crate) use code::CodeState;
pub(crate) use design::DesignState;
pub(crate) use hardcopy::HardcopyState;
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
    pub(crate) fn dialog_modal_open(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        let autosave_restore_open = self.platform.pending_autosave_restore.is_some();
        #[cfg(target_arch = "wasm32")]
        let autosave_restore_open = false;

        self.shell.about
            || self.shell.shortcuts_help
            || self.shell.help_center.open
            || self.workbench.window_session_dialog.open
            || self.project.new_cell_dialog
            || self.project.new_view_dialog
            || self.code.new_veriloga_device.open
            || self.code.veriloga_build_settings.is_some()
            || self.project.copy_cell_dialog
            || self.project.rename_cell_dialog
            || self.project.rename_view_dialog
            || self.project.new_library_dialog
            || self.project.rename_library_dialog
            || self.project.delete_library_dialog
            || self.project.library_deletion_review.target.is_some()
            || self.simulate.plan_removal_review.target.is_some()
            || self.simulate.plan_removal_refusal.analysis.is_some()
            || self.results.dataset_export.is_some()
            || self.settings.preferences_open
            || self.hardcopy.hardcopy.open
            || self.sheets.drawing_sheet_defaults.open
            || self.sheets.drawing_sheet_presets.any_open()
            || self.sheets.drawing_sheet_support.any_open()
            || self.settings.workspace_layout_manager.open
            || self.settings.shortcut_portability.application_modal_open()
            || self.settings.shortcut_editor.open
            || self.cloud.license_dialog.open
            || self.project.new_project.open
            || self.cloud.publish_web.open
            || self.cloud.live_session.open
            || self.cloud.unpublish_web.open
            || self.simulate.run_details.is_some()
            || self.workbench.command_palette.open
            || self.design.bus_tap.open
            || self.design.builtin_xspice_placement.open
            || self.design.net_label_placement.open
            || self.design.schematic_visibility_dialog.open
            || self.sheets.drawing_sheet_overflow_open
            || self.design.grid_snap_routing.open
            || self.design.engineering_table.open
            || self.design.pin_port.open
            || self.design.design_note.open
            || self.design.documentation_shape.open
            || self.design.array_selection.open
            || self.design.replace_instance.open
            || self.design.stimulus_link.open
            || self.design.create_hierarchy.open
            || self.project.check_and_save.open
            || self.design.connectivity_manager.open
            || self.sheets.delete_sheet.is_some()
            || self.project.design_management.open
            || self.design.selection_bulk_edit.open
            || self.design.design_review_comments.open
            || self.project.project_revision_history.open
            || self.design.create_model_bound_symbol.open
            || self.design.symbol_import.open
            || self.design.symbol_parameter_form.open
            || self.simulate.configuration_sets.open
            || self.design.rule_setup.open
            || self.design.object_properties.open
            || self.design.rename_selection.open
            || self.design.find_replace.open
            || self.design.shared_properties.open
            || self.project.technology_attachment.open
            || self.workbench.confirmation_dialog.visible
            || self.workbench.project_review_dialog.request.is_some()
            || autosave_restore_open
    }
}

#[cfg(test)]
mod tests;
