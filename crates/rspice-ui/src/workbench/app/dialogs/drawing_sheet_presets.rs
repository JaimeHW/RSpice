//! Custom drawing-sheet size library, editor, and portable transfer.
//!
//! Project presets publish through the revisioned design-management catalog.
//! Personal presets publish through `UserPreferences`. Dialog drafts never
//! mutate either authority until their explicit primary action succeeds.

mod model;
mod render;

use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read as _;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(target_arch = "wasm32")]
use std::path::PathBuf;

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AuthoredDrawingSheetSize, DesignManagementCatalog, DrawingSheetPreset,
    DrawingSheetPresetImportConflict, DrawingSheetPresetImportMapping,
    DrawingSheetPresetImportMappingKind, DrawingSheetPresetImportReceipt,
    DrawingSheetPresetImportReference, DrawingSheetPresetImportResolution,
    DrawingSheetPresetImportSkip, DrawingSheetPresetImportSkipReason, DrawingSheetPresetScope,
};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::DrawingSheetPersonalPreferences;
use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

pub(crate) use model::capture_personal_preset_into_project;
pub use model::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPackageInspection,
    DrawingSheetPackageVerification, PublishedDrawingSheetPackage,
    drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
    publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
};

use model::{
    ImportResolution, PresetEditorDraft, PresetEditorMode, PresetEditorUnit, PresetPackageFormat,
    PresetTransferState, StartingFrame, TransferMode, all_visible_presets,
    build_package_with_options, encode_package_with_format, imported_project_preset, parse_package,
    prepare_import_candidates, preset_from_editor, unavailable, unique_copy_name,
    unsigned_exportable, validate_preset_name,
};
use render::{EditorBodyAction, LibraryBodyAction, PresetKey, TransferBodyAction};

const LIBRARY_EYEBROW: &str = "DRAWING SHEET \u{00b7} PROJECT AND PERSONAL PRESETS";
const LIBRARY_TITLE: &str = "Custom sheet sizes";
const LIBRARY_DESCRIPTION: &str = "Named custom sheet sizes available to this project, where they came from, and which sheets depend on them.";

#[derive(Default, Debug, Clone)]
pub(crate) struct DrawingSheetPresetDialogsState {
    library_open: bool,
    search: String,
    edit: Option<SchematicEditAuthority>,
    catalog_revision: u64,
    baseline_personal: DrawingSheetPersonalPreferences,
    editor: Option<PresetEditorDraft>,
    transfer: PresetTransferState,
    delete: Option<PresetKey>,
    error: Option<String>,
    #[cfg(target_arch = "wasm32")]
    import_mailbox: Option<ImportMailbox>,
}

#[cfg(target_arch = "wasm32")]
type ImportMailbox = std::sync::Arc<std::sync::Mutex<Option<Result<(String, String), String>>>>;

impl DrawingSheetPresetDialogsState {
    pub(crate) fn any_open(&self) -> bool {
        self.library_open || self.editor.is_some() || self.transfer.open || self.delete.is_some()
    }

    fn close(&mut self) {
        *self = Self::default();
    }

    fn return_to_library(&mut self) {
        self.editor = None;
        self.transfer = PresetTransferState::default();
        self.delete = None;
        self.library_open = true;
        #[cfg(target_arch = "wasm32")]
        {
            self.import_mailbox = None;
        }
    }
}

pub(crate) fn open_custom_sheet_size_library(state: &mut AppState) -> bool {
    if state.dialogs.drawing_sheet_presets.any_open() {
        return false;
    }
    let personal = state.ui.preferences.drawing_sheet_personal_preferences();
    state.dialogs.drawing_sheet_presets = DrawingSheetPresetDialogsState {
        library_open: true,
        search: String::new(),
        edit: Some(SchematicEditAuthority::capture(state)),
        catalog_revision: state.workspace.design_management.revision(),
        baseline_personal: personal,
        editor: None,
        transfer: PresetTransferState::default(),
        delete: None,
        error: None,
        #[cfg(target_arch = "wasm32")]
        import_mailbox: None,
    };
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_preset_dialogs(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        self.poll_drawing_sheet_preset_import();

        if self.state.dialogs.drawing_sheet_presets.library_open {
            self.render_drawing_sheet_preset_library(ctx);
        }
        if self.state.dialogs.drawing_sheet_presets.editor.is_some() {
            self.render_drawing_sheet_preset_editor(ctx);
        }
        if self.state.dialogs.drawing_sheet_presets.transfer.open {
            self.render_drawing_sheet_preset_transfer(ctx);
        }
        if self.state.dialogs.drawing_sheet_presets.delete.is_some() {
            self.render_drawing_sheet_preset_delete(ctx);
        }
    }

    fn render_drawing_sheet_preset_library(&mut self, ctx: &Context) {
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let presets = sorted_visible_presets(&self.state.workspace.design_management, &personal);
        let usage = usage_counts(&self.state.workspace.design_management, &personal);
        let authority_error =
            validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)
                .err();
        let personal_stale = personal != self.state.dialogs.drawing_sheet_presets.baseline_personal;
        let hint = authority_error.as_deref().or(personal_stale.then_some(
            "Personal drawing-sheet preferences changed. Close and reopen this library.",
        ));

        let mut action = None;
        let choice = Dialog::new(LIBRARY_EYEBROW, LIBRARY_TITLE, "Close")
            .description(LIBRARY_DESCRIPTION)
            .size(DialogSize::DrawingSheetWorkflow)
            // The body owns its section surfaces and they butt on their own
            // edges; the dialog's default inset framed the whole page.
            .flush_body()
            .primary_on_enter(false)
            .show(ctx, |ui| {
                action = render::library_body(
                    ui,
                    &mut self.state.dialogs.drawing_sheet_presets.search,
                    &presets,
                    &usage,
                    authority_error.is_none(),
                    !personal_stale,
                    hint,
                );
                if let Some(error) = self.state.dialogs.drawing_sheet_presets.error.as_deref() {
                    render::error_notice(ui, error);
                }
            });

        if let Some(action) = action {
            self.handle_preset_library_action(action);
        }
        if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
            self.state.dialogs.drawing_sheet_presets.close();
        }
    }

    fn handle_preset_library_action(&mut self, action: LibraryBodyAction) {
        self.state.dialogs.drawing_sheet_presets.error = None;
        let result = match action {
            LibraryBodyAction::New => {
                self.state.dialogs.drawing_sheet_presets.library_open = false;
                self.state.dialogs.drawing_sheet_presets.editor =
                    Some(PresetEditorDraft::default());
                return;
            }
            LibraryBodyAction::Import => {
                self.open_preset_transfer(TransferMode::Import);
                return;
            }
            LibraryBodyAction::Export => {
                self.open_preset_transfer(TransferMode::Export);
                return;
            }
            LibraryBodyAction::Use(key) => self.use_preset(&key),
            LibraryBodyAction::Duplicate(key) => self.duplicate_preset(&key),
            LibraryBodyAction::Rename(key) => self.open_preset_rename(&key),
            LibraryBodyAction::Delete(key) => {
                self.state.dialogs.drawing_sheet_presets.library_open = false;
                self.state.dialogs.drawing_sheet_presets.delete = Some(key);
                return;
            }
        };
        if let Err(error) = result {
            self.state.dialogs.drawing_sheet_presets.error = Some(error);
        }
    }

    fn open_preset_rename(&mut self, key: &PresetKey) -> Result<(), String> {
        let preset = self
            .find_visible_preset(key)
            .ok_or_else(|| "That custom size is no longer available.".to_owned())?;
        let (width_um, height_um) = custom_dimensions(&preset)?;
        let unit = PresetEditorUnit::Millimetres;
        let draft = PresetEditorDraft {
            mode: PresetEditorMode::Edit,
            source_id: Some(preset.id.clone()),
            name: preset.name.clone(),
            scope: preset.scope,
            width: model::format_dimension_um(width_um, unit),
            height: model::format_dimension_um(height_um, unit),
            unit,
            frame: infer_starting_frame(&preset),
            unavailable: unavailable(&preset),
            baseline: Some(preset.clone()),
            last_valid_preview: preset.format.clone(),
            error: None,
        };
        self.state.dialogs.drawing_sheet_presets.library_open = false;
        self.state.dialogs.drawing_sheet_presets.editor = Some(draft);
        Ok(())
    }

    fn duplicate_preset(&mut self, key: &PresetKey) -> Result<(), String> {
        validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)?;
        let source = self
            .find_visible_preset(key)
            .ok_or_else(|| "That custom size is no longer available.".to_owned())?;
        if unavailable(&source) {
            return Err("An unavailable custom size cannot be duplicated.".to_owned());
        }
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = all_visible_presets(&self.state.workspace.design_management, &personal);
        let id = format!("custom-{}", uuid::Uuid::new_v4().simple());
        let name = unique_copy_name(&format!("{} copy", source.name), &visible);
        let format = source
            .format
            .try_update(|draft| {
                draft.inheritance = crate::state::DrawingSheetInheritance::Explicit;
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = Some(id.clone());
                    snapshot.name.clone_from(&name);
                    snapshot.source_preset_unavailable = false;
                }
            })
            .map_err(|error| error.to_string())?;
        let copy = DrawingSheetPreset {
            id,
            name: name.clone(),
            scope: DrawingSheetPresetScope::Project,
            format: format.as_reusable_drawing_sheet_preset(),
        };
        self.publish_project_preset(copy, "Duplicate custom sheet size")?;
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Created project custom size '{name}' from an exact snapshot."
        )));
        Ok(())
    }

    fn use_preset(&mut self, key: &PresetKey) -> Result<(), String> {
        let preset = self
            .find_visible_preset(key)
            .ok_or_else(|| "That custom size is no longer available.".to_owned())?;
        if unavailable(&preset) {
            return Err("This custom size has an unavailable managed dependency.".to_owned());
        }
        crate::workbench::app::open_drawing_sheet_setup_with_preset(self, preset)?;
        self.state.dialogs.drawing_sheet_presets.close();
        Ok(())
    }

    fn render_drawing_sheet_preset_editor(&mut self, ctx: &Context) {
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = all_visible_presets(&self.state.workspace.design_management, &personal);
        let project_error =
            validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)
                .err();
        let personal_stale = personal != self.state.dialogs.drawing_sheet_presets.baseline_personal;
        let draft = self
            .state
            .dialogs
            .drawing_sheet_presets
            .editor
            .as_ref()
            .expect("editor presence was checked");
        let validation = editor_candidate(draft, &visible);
        let authority_allows_scope = match draft.scope {
            DrawingSheetPresetScope::Project => project_error.is_none(),
            DrawingSheetPresetScope::User => !personal_stale,
            DrawingSheetPresetScope::Organization => false,
        };
        let primary_enabled = validation.is_ok() && authority_allows_scope && !draft.unavailable;
        let title = if draft.mode == PresetEditorMode::Edit {
            "Rename custom sheet size"
        } else {
            "New custom sheet size"
        };
        let primary = if draft.mode == PresetEditorMode::Edit {
            "Save name"
        } else {
            "Create custom size"
        };
        let mut action = None;
        let choice = Dialog::new(
            "DRAWING SHEET \u{00b7} NAMED PHYSICAL FORMAT",
            title,
            primary,
        )
            .description(if draft.mode == PresetEditorMode::Edit {
                "Rename the reusable custom size without changing its stable identity or any using sheet."
            } else {
                "Create a reusable custom size without changing the current sheet. A sheet uses the preset only after it is selected in Page Setup."
            })
            .size(DialogSize::DrawingSheetWorkflow)
            .ghost("Cancel")
            .primary_enabled(primary_enabled)
            .show(ctx, |ui| {
                let draft = self
                    .state
                    .dialogs
                    .drawing_sheet_presets
                    .editor
                    .as_mut()
                    .expect("editor presence was checked");
                action = render::editor_body(
                    ui,
                    draft,
                    validation.as_ref().err().map(String::as_str),
                    project_error.as_deref(),
                    personal_stale,
                );
            });
        if matches!(action, Some(EditorBodyAction::RefreshPreview))
            && let Some(draft) = self.state.dialogs.drawing_sheet_presets.editor.as_mut()
            && let Ok(candidate) = editor_candidate(draft, &visible)
        {
            draft.last_valid_preview = candidate.format;
            draft.error = None;
        }
        match choice {
            DialogChoice::Primary => match self.apply_preset_editor() {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_presets.return_to_library();
                }
                Err(error) => {
                    if let Some(draft) = self.state.dialogs.drawing_sheet_presets.editor.as_mut() {
                        draft.error = Some(error);
                    }
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_presets.return_to_library();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn apply_preset_editor(&mut self) -> Result<String, String> {
        let draft = self
            .state
            .dialogs
            .drawing_sheet_presets
            .editor
            .clone()
            .ok_or_else(|| "The custom-size editor is not open.".to_owned())?;
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = all_visible_presets(&self.state.workspace.design_management, &personal);
        let preset = if draft.mode == PresetEditorMode::Edit {
            let name = validate_editor_name(&draft, &visible)?;
            let baseline = draft
                .baseline
                .clone()
                .ok_or_else(|| "The preset rename lost its baseline.".to_owned())?;
            let format = baseline
                .format
                .try_update(|format| {
                    if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut format.authored_size
                    {
                        snapshot.name.clone_from(&name);
                    }
                })
                .map_err(|error| error.to_string())?;
            DrawingSheetPreset {
                name,
                format,
                ..baseline
            }
        } else {
            preset_from_editor(&draft, &visible)?
        };
        let message = match preset.scope {
            DrawingSheetPresetScope::Project => {
                let name = preset.name.clone();
                if draft.mode == PresetEditorMode::Edit {
                    self.rename_project_preset(&preset.id, name.clone())?;
                } else {
                    self.publish_project_preset(preset, "Save custom sheet size")?;
                }
                format!("Saved project custom size '{name}'.")
            }
            DrawingSheetPresetScope::User => {
                let name = preset.name.clone();
                self.publish_personal_preset(preset)?;
                format!("Saved personal custom size '{name}' in Preferences.")
            }
            DrawingSheetPresetScope::Organization => {
                return Err("Organization custom sizes are managed and read-only.".to_owned());
            }
        };
        Ok(message)
    }

    fn render_drawing_sheet_preset_delete(&mut self, ctx: &Context) {
        let key = self
            .state
            .dialogs
            .drawing_sheet_presets
            .delete
            .as_ref()
            .expect("delete presence was checked")
            .clone();
        let preset = self.find_visible_preset(&key);
        let name = preset
            .as_ref()
            .map_or("Unavailable preset", |preset| preset.name.as_str());
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let used_by = usage_count(
            &self.state.workspace.design_management,
            &personal,
            key.scope,
            &key.id,
        );
        let title = format!("Delete '{name}'?");
        let detail = if used_by == 0 {
            "This removes the reusable definition. Existing authored sheets retain their exact embedded snapshots."
                .to_owned()
        } else {
            format!(
                "{used_by} drawing-sheet reference{} still depend on this preset. Change the using sheets or defaults before deleting it.",
                if used_by == 1 { "" } else { "s" }
            )
        };
        let choice = Dialog::new(
            "DRAWING SHEET \u{00b7} DESTRUCTIVE CHANGE",
            &title,
            "Delete preset",
        )
        .description(&detail)
        .size(DialogSize::Transaction)
        .destructive()
        .ghost("Cancel")
        .primary_enabled(preset.is_some() && used_by == 0)
        .show(ctx, |ui| {
            render::delete_body(ui, preset.as_ref(), used_by);
            if let Some(error) = self.state.dialogs.drawing_sheet_presets.error.as_deref() {
                render::error_notice(ui, error);
            }
        });
        match choice {
            DialogChoice::Primary => match self.delete_preset(&key) {
                Ok(message) => {
                    self.state.push_user_message(ConsoleMessage::info(message));
                    self.state.dialogs.drawing_sheet_presets.return_to_library();
                }
                Err(error) => self.state.dialogs.drawing_sheet_presets.error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_presets.return_to_library();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn delete_preset(&mut self, key: &PresetKey) -> Result<String, String> {
        let preset = self
            .find_visible_preset(key)
            .ok_or_else(|| "That custom size is no longer available.".to_owned())?;
        match key.scope {
            DrawingSheetPresetScope::Project => {
                validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)?;
                let before = self.state.workspace.design_management.clone();
                let mut candidate = before.clone();
                candidate
                    .remove_drawing_sheet_preset(candidate.revision(), &key.id)
                    .map_err(|error| error.to_string())?;
                super::drawing_sheet_defaults::commit_project_candidate(
                    self,
                    "Delete custom sheet size",
                    before,
                    candidate,
                )?;
            }
            DrawingSheetPresetScope::User => {
                self.ensure_personal_fresh()?;
                let mut preferences = self.state.ui.preferences.clone();
                let mut personal = preferences.drawing_sheet_personal_preferences();
                if personal_default_references_preset(&personal, &key.id) {
                    return Err(
                        "The personal drawing-sheet default still references this custom size. \
                         Choose another personal default before deleting it."
                            .to_owned(),
                    );
                }
                let before = personal.presets.len();
                personal
                    .presets
                    .retain(|candidate| !candidate.id.eq_ignore_ascii_case(&key.id));
                if personal.presets.len() == before {
                    return Err("That personal custom size is no longer available.".to_owned());
                }
                preferences.set_drawing_sheet_personal_preferences(personal)?;
                self.state.ui.preferences = preferences;
            }
            DrawingSheetPresetScope::Organization => {
                return Err(
                    "Organization custom sizes are managed and cannot be deleted.".to_owned(),
                );
            }
        }
        self.refresh_preset_authorities();
        Ok(format!("Deleted custom size '{}'.", preset.name))
    }

    fn open_preset_transfer(&mut self, mode: TransferMode) {
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = all_visible_presets(&self.state.workspace.design_management, &personal);
        let export_ids = visible
            .iter()
            .filter(|preset| unsigned_exportable(preset))
            .map(transfer_identity)
            .collect();
        let package_name = if mode == TransferMode::Import {
            String::new()
        } else {
            "rspice-sheet-formats.json".to_owned()
        };
        self.state.dialogs.drawing_sheet_presets.library_open = false;
        self.state.dialogs.drawing_sheet_presets.transfer = PresetTransferState {
            open: true,
            mode,
            package_name,
            json: String::new(),
            export_ids,
            import_candidates: Vec::new(),
            reviewed_digest: None,
            package_format: PresetPackageFormat::CanonicalSchema1,
            include_builtin_frame_references: true,
            include_source_metadata: true,
            error: None,
        };
    }

    fn render_drawing_sheet_preset_transfer(&mut self, ctx: &Context) {
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = sorted_visible_presets(&self.state.workspace.design_management, &personal);
        let transfer = &self.state.dialogs.drawing_sheet_presets.transfer;
        let is_import = transfer.mode == TransferMode::Import;
        let title = if is_import {
            "Import sheet-format presets"
        } else {
            "Export sheet-format presets"
        };
        let primary = if is_import {
            "Import reviewed presets"
        } else {
            "Export preset package"
        };
        let primary_enabled = if is_import {
            transfer.reviewed_digest.is_some()
                && transfer
                    .import_candidates
                    .iter()
                    .any(|candidate| candidate.selected)
                && validate_project_authority(
                    &self.state,
                    &self.state.dialogs.drawing_sheet_presets,
                )
                .is_ok()
        } else {
            !transfer.export_ids.is_empty()
        };
        let mut action = None;
        let choice = Dialog::new(
            if is_import {
                "DRAWING SHEET \u{00b7} REVIEWED IMPORT"
            } else {
                "DRAWING SHEET \u{00b7} PORTABLE PRESET PACKAGE"
            },
            title,
            primary,
        )
        .description(if is_import {
            "Review identities, physical dimensions, template dependencies, and conflicts before adding presets to this project."
        } else {
            "Package selected custom sizes and their frame dependencies without exporting project schematic data."
        })
        .size(DialogSize::DrawingSheetWorkflow)
        .fixed_height(690.0)
        .ghost("Cancel")
        .primary_enabled(primary_enabled)
        .primary_on_enter(false)
        .show(ctx, |ui| {
            action = render::transfer_body(
                ui,
                &mut self.state.dialogs.drawing_sheet_presets.transfer,
                &visible,
            );
        });
        if let Some(action) = action {
            self.handle_transfer_body_action(ctx, action, &visible);
        }
        match choice {
            DialogChoice::Primary => {
                let result = if is_import {
                    self.apply_preset_import()
                } else {
                    self.apply_preset_export(&visible)
                };
                match result {
                    Ok(Some(message)) => {
                        self.state.push_user_message(ConsoleMessage::info(message));
                        self.state.dialogs.drawing_sheet_presets.return_to_library();
                    }
                    Ok(None) => {}
                    Err(error) => {
                        self.state.dialogs.drawing_sheet_presets.transfer.error = Some(error);
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_presets.return_to_library();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn handle_transfer_body_action(
        &mut self,
        ctx: &Context,
        action: TransferBodyAction,
        visible: &[DrawingSheetPreset],
    ) {
        match action {
            TransferBodyAction::ChooseImportFile => {
                if let Err(error) = self.choose_preset_import_file(ctx) {
                    self.state.dialogs.drawing_sheet_presets.transfer.error = Some(error);
                }
            }
            TransferBodyAction::SelectAllExport(selected) => {
                let transfer = &mut self.state.dialogs.drawing_sheet_presets.transfer;
                transfer.export_ids.clear();
                if selected {
                    transfer.export_ids.extend(
                        visible
                            .iter()
                            .filter(|preset| unsigned_exportable(preset))
                            .map(transfer_identity),
                    );
                }
            }
        }
    }

    fn review_import_source(
        &mut self,
        source: &str,
        visible: &[DrawingSheetPreset],
    ) -> Result<(), String> {
        let package = parse_package(source, &self.state.pdk_config.publisher_trust_store)?;
        let candidates = prepare_import_candidates(&package, visible)?;
        let transfer = &mut self.state.dialogs.drawing_sheet_presets.transfer;
        transfer.reviewed_digest = Some(package.source_digest_sha256);
        transfer.import_candidates = candidates;
        transfer.error = None;
        Ok(())
    }

    fn apply_preset_import(&mut self) -> Result<Option<String>, String> {
        validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)?;
        self.ensure_personal_fresh()?;
        let transfer = self.state.dialogs.drawing_sheet_presets.transfer.clone();
        let package = parse_package(&transfer.json, &self.state.pdk_config.publisher_trust_store)?;
        if transfer.reviewed_digest.as_deref() != Some(package.source_digest_sha256.as_str()) {
            return Err(
                "The package changed after review. Review the current JSON before importing."
                    .to_owned(),
            );
        }
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let mut visible = all_visible_presets(&self.state.workspace.design_management, &personal);
        let before = self.state.workspace.design_management.clone();
        let mut candidate = before.clone();
        let mut created = 0_usize;
        let mut mapped = 0_usize;
        let mut skipped = 0_usize;
        let selected_candidates = transfer
            .import_candidates
            .iter()
            .filter(|import| import.selected)
            .map(import_source_reference)
            .collect::<Vec<_>>();
        let skipped_candidates = transfer
            .import_candidates
            .iter()
            .filter_map(|import| {
                if !import.selected {
                    Some(DrawingSheetPresetImportSkip {
                        source: import_source_reference(import),
                        reason: DrawingSheetPresetImportSkipReason::NotSelected,
                    })
                } else if import.resolution == ImportResolution::Skip {
                    Some(DrawingSheetPresetImportSkip {
                        source: import_source_reference(import),
                        reason: DrawingSheetPresetImportSkipReason::ExplicitlySkipped,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let conflicts = transfer
            .import_candidates
            .iter()
            .filter(|import| import.existing_id.is_some() || import.missing_managed_dependency)
            .map(|import| {
                let existing = import
                    .existing_id
                    .as_deref()
                    .map(|id| {
                        visible
                            .iter()
                            .find(|preset| preset.id.eq_ignore_ascii_case(id))
                            .map(preset_import_reference)
                            .ok_or_else(|| {
                                format!(
                                    "Reviewed preset mapping target '{id}' is no longer available."
                                )
                            })
                    })
                    .transpose()?;
                Ok(DrawingSheetPresetImportConflict {
                    source: import_source_reference(import),
                    existing,
                    missing_managed_dependency: import.missing_managed_dependency,
                    resolution: durable_import_resolution(import.resolution),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut mappings = Vec::new();
        for import in &transfer.import_candidates {
            if !import.selected || import.resolution == ImportResolution::Skip {
                skipped += 1;
                continue;
            }
            if matches!(
                import.resolution,
                ImportResolution::MatchesByDigest | ImportResolution::MapExisting
            ) {
                let existing_id = import.existing_id.as_deref().ok_or_else(|| {
                    format!(
                        "Reviewed preset '{}' has no mapping target.",
                        import.portable.name
                    )
                })?;
                let target = visible
                    .iter()
                    .find(|preset| preset.id.eq_ignore_ascii_case(existing_id))
                    .map(preset_import_reference)
                    .ok_or_else(|| {
                        format!(
                            "Reviewed preset mapping target '{existing_id}' is no longer available."
                        )
                    })?;
                mappings.push(DrawingSheetPresetImportMapping {
                    source: import_source_reference(import),
                    target,
                    kind: DrawingSheetPresetImportMappingKind::ExistingPreset,
                });
                mapped += 1;
                continue;
            }
            if let Some(preset) = imported_project_preset(import, &visible)? {
                mappings.push(DrawingSheetPresetImportMapping {
                    source: import_source_reference(import),
                    target: preset_import_reference(&preset),
                    kind: DrawingSheetPresetImportMappingKind::CreatedProjectPreset,
                });
                candidate
                    .publish_drawing_sheet_preset(candidate.revision(), preset.clone())
                    .map_err(|error| error.to_string())?;
                visible.push(preset);
                created += 1;
            }
        }
        let receipt = DrawingSheetPresetImportReceipt {
            source_digest_sha256: package.source_digest_sha256.clone(),
            source_schema: package.schema.clone(),
            source_schema_version: package.version,
            reviewed_candidate_count: transfer.import_candidates.len(),
            selected_candidates,
            mappings,
            conflicts,
            skipped_candidates,
        };
        receipt.validate().map_err(|error| error.to_string())?;
        let mut settings = candidate.drawing_sheet_settings().clone();
        settings.preset_import_receipts.push(receipt);
        candidate
            .update_drawing_sheet_settings(candidate.revision(), settings)
            .map_err(|error| error.to_string())?;
        super::drawing_sheet_defaults::commit_project_candidate(
            self,
            "Import custom sheet sizes",
            before,
            candidate,
        )?;
        self.refresh_preset_authorities();
        Ok(Some(format!(
            "Reviewed package {}: imported {created}, mapped {mapped}, skipped {skipped}; durable receipt retained.",
            package.source_digest_sha256
        )))
    }

    fn apply_preset_export(
        &mut self,
        visible: &[DrawingSheetPreset],
    ) -> Result<Option<String>, String> {
        let transfer = &self.state.dialogs.drawing_sheet_presets.transfer;
        let selected = visible
            .iter()
            .filter(|preset| {
                unsigned_exportable(preset)
                    && transfer.export_ids.contains(&transfer_identity(preset))
            })
            .cloned()
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Err("Select at least one available custom size to export.".to_owned());
        }
        let package = build_package_with_options(
            selected,
            transfer.include_builtin_frame_references,
            transfer.include_source_metadata,
        )?;
        let source = encode_package_with_format(&package, transfer.package_format)?;
        let name = normalized_package_name(&transfer.package_name);
        export_package_file(&name, &source).map(|saved| {
            saved.then(|| {
                format!(
                    "Exported {} custom size{} with digest {}.",
                    package.presets.len(),
                    if package.presets.len() == 1 { "" } else { "s" },
                    package.source_digest_sha256
                )
            })
        })
    }

    fn choose_preset_import_file(&mut self, _ctx: &Context) -> Result<(), String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(path) = rfd::FileDialog::new()
                .add_filter("RSpice sheet formats", &["json"])
                .pick_file()
            else {
                return Ok(());
            };
            let source = read_bounded_utf8(&path)?;
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("sheet-formats.json")
                .to_owned();
            let personal = self
                .state
                .ui
                .preferences
                .drawing_sheet_personal_preferences();
            let visible = all_visible_presets(&self.state.workspace.design_management, &personal);
            {
                let transfer = &mut self.state.dialogs.drawing_sheet_presets.transfer;
                transfer.package_name = name;
                transfer.json.clone_from(&source);
            }
            self.review_import_source(&source, &visible)
        }
        #[cfg(target_arch = "wasm32")]
        {
            if self
                .state
                .dialogs
                .drawing_sheet_presets
                .import_mailbox
                .is_some()
            {
                return Err("A sheet-format file picker is already open.".to_owned());
            }
            let mailbox = std::sync::Arc::new(std::sync::Mutex::new(None));
            self.state.dialogs.drawing_sheet_presets.import_mailbox = Some(mailbox.clone());
            let repaint = _ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = if let Some(file) = rfd::AsyncFileDialog::new()
                    .add_filter("RSpice sheet formats", &["json"])
                    .pick_file()
                    .await
                {
                    let name = file.file_name();
                    let bytes = file.read().await;
                    if bytes.len() > model::MAX_PACKAGE_BYTES {
                        Err(format!(
                            "The preset package exceeds the {} byte limit.",
                            model::MAX_PACKAGE_BYTES
                        ))
                    } else {
                        String::from_utf8(bytes)
                            .map(|source| (name, source))
                            .map_err(|_| "The preset package must be UTF-8 JSON.".to_owned())
                    }
                } else {
                    Err("Sheet-format import was cancelled.".to_owned())
                };
                if let Ok(mut slot) = mailbox.lock() {
                    *slot = Some(result);
                }
                repaint.request_repaint();
            });
            Ok(())
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn poll_drawing_sheet_preset_import(&mut self) {
        let result = self
            .state
            .dialogs
            .drawing_sheet_presets
            .import_mailbox
            .as_ref()
            .and_then(|mailbox| mailbox.lock().ok()?.take());
        let Some(result) = result else {
            return;
        };
        self.state.dialogs.drawing_sheet_presets.import_mailbox = None;
        match result {
            Ok((name, source)) => {
                let personal = self
                    .state
                    .ui
                    .preferences
                    .drawing_sheet_personal_preferences();
                let visible =
                    all_visible_presets(&self.state.workspace.design_management, &personal);
                {
                    let transfer = &mut self.state.dialogs.drawing_sheet_presets.transfer;
                    transfer.package_name = name;
                    transfer.json.clone_from(&source);
                }
                if let Err(error) = self.review_import_source(&source, &visible) {
                    self.state.dialogs.drawing_sheet_presets.transfer.error = Some(error);
                }
            }
            Err(error) if error != "Sheet-format import was cancelled." => {
                self.state.dialogs.drawing_sheet_presets.transfer.error = Some(error);
            }
            Err(_) => {}
        }
    }

    fn find_visible_preset(&self, key: &PresetKey) -> Option<DrawingSheetPreset> {
        let personal = self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        all_visible_presets(&self.state.workspace.design_management, &personal)
            .into_iter()
            .find(|preset| preset.scope == key.scope && preset.id.eq_ignore_ascii_case(&key.id))
    }

    fn publish_project_preset(
        &mut self,
        mut preset: DrawingSheetPreset,
        description: &str,
    ) -> Result<(), String> {
        validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)?;
        preset.format = preset.format.as_reusable_drawing_sheet_preset();
        let before = self.state.workspace.design_management.clone();
        let mut candidate = before.clone();
        candidate
            .publish_drawing_sheet_preset(candidate.revision(), preset)
            .map_err(|error| error.to_string())?;
        super::drawing_sheet_defaults::commit_project_candidate(
            self,
            description,
            before,
            candidate,
        )?;
        self.refresh_preset_authorities();
        Ok(())
    }

    fn rename_project_preset(&mut self, preset_id: &str, name: String) -> Result<(), String> {
        validate_project_authority(&self.state, &self.state.dialogs.drawing_sheet_presets)?;
        let before = self.state.workspace.design_management.clone();
        let mut candidate = before.clone();
        candidate
            .rename_drawing_sheet_preset(candidate.revision(), preset_id, name)
            .map_err(|error| error.to_string())?;
        super::drawing_sheet_defaults::commit_project_candidate(
            self,
            "Rename custom sheet size",
            before,
            candidate,
        )?;
        self.refresh_preset_authorities();
        Ok(())
    }

    fn publish_personal_preset(&mut self, mut preset: DrawingSheetPreset) -> Result<(), String> {
        self.ensure_personal_fresh()?;
        preset.format = preset.format.as_reusable_drawing_sheet_preset();
        let mut preferences = self.state.ui.preferences.clone();
        let mut personal = preferences.drawing_sheet_personal_preferences();
        if let Some(existing) = personal
            .presets
            .iter_mut()
            .find(|candidate| candidate.id.eq_ignore_ascii_case(&preset.id))
        {
            *existing = preset;
        } else {
            personal.presets.push(preset);
        }
        preferences.set_drawing_sheet_personal_preferences(personal)?;
        self.state.ui.preferences = preferences;
        self.refresh_preset_authorities();
        Ok(())
    }

    fn ensure_personal_fresh(&self) -> Result<(), String> {
        if self
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences()
            != self.state.dialogs.drawing_sheet_presets.baseline_personal
        {
            return Err(
                "Personal drawing-sheet preferences changed. Close and reopen this library."
                    .to_owned(),
            );
        }
        Ok(())
    }

    fn refresh_preset_authorities(&mut self) {
        let state = &self.state;
        let refreshed = DrawingSheetPresetDialogsState {
            edit: Some(SchematicEditAuthority::capture(state)),
            catalog_revision: state.workspace.design_management.revision(),
            baseline_personal: state.ui.preferences.drawing_sheet_personal_preferences(),
            ..self.state.dialogs.drawing_sheet_presets.clone()
        };
        self.state.dialogs.drawing_sheet_presets = refreshed;
    }
}

fn validate_project_authority(
    app: &AppState,
    state: &DrawingSheetPresetDialogsState,
) -> Result<(), String> {
    state
        .edit
        .as_ref()
        .ok_or_else(|| "Custom sheet sizes has no project edit authority.".to_owned())?
        .validate(app, "Custom sheet sizes")?;
    if app.workspace.design_management.revision() != state.catalog_revision {
        return Err(
            "Project drawing-sheet presets changed. Close and reopen Custom sheet sizes."
                .to_owned(),
        );
    }
    Ok(())
}

fn editor_candidate(
    draft: &PresetEditorDraft,
    visible: &[DrawingSheetPreset],
) -> Result<DrawingSheetPreset, String> {
    if draft.mode == PresetEditorMode::Edit {
        let name = validate_editor_name(draft, visible)?;
        let mut baseline = draft
            .baseline
            .clone()
            .ok_or_else(|| "The preset rename lost its baseline.".to_owned())?;
        baseline.name = name.clone();
        baseline.format = baseline
            .format
            .try_update(|format| {
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut format.authored_size {
                    snapshot.name.clone_from(&name);
                }
            })
            .map_err(|error| error.to_string())?;
        Ok(baseline)
    } else {
        preset_from_editor(draft, visible)
    }
}

fn validate_editor_name(
    draft: &PresetEditorDraft,
    visible: &[DrawingSheetPreset],
) -> Result<String, String> {
    let without_source = visible
        .iter()
        .filter(|preset| {
            !(preset.scope == draft.scope
                && draft
                    .source_id
                    .as_deref()
                    .is_some_and(|id| preset.id.eq_ignore_ascii_case(id)))
        })
        .cloned()
        .collect::<Vec<_>>();
    validate_preset_name(&draft.name, &without_source, None)
}

fn custom_dimensions(preset: &DrawingSheetPreset) -> Result<(u64, u64), String> {
    let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
        return Err("The selected definition is not a custom physical sheet size.".to_owned());
    };
    Ok((snapshot.portrait_width_um, snapshot.portrait_height_um))
}

fn infer_starting_frame(preset: &DrawingSheetPreset) -> StartingFrame {
    use crate::state::{DrawingSheetBorderTemplate, DrawingSheetTitleBlockTemplate};
    match (preset.format.border, preset.format.title_block.template) {
        (DrawingSheetBorderTemplate::None, DrawingSheetTitleBlockTemplate::None) => {
            StartingFrame::None
        }
        (DrawingSheetBorderTemplate::Plain, _) => StartingFrame::Plain,
        _ if preset.format.margins.top_um == 12_700 && preset.format.margins.left_um == 19_050 => {
            StartingFrame::AnsiA
        }
        _ => StartingFrame::IsoA,
    }
}

fn sorted_visible_presets(
    project: &DesignManagementCatalog,
    personal: &DrawingSheetPersonalPreferences,
) -> Vec<DrawingSheetPreset> {
    let mut presets = all_visible_presets(project, personal);
    presets.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| scope_rank(left.scope).cmp(&scope_rank(right.scope)))
            .then_with(|| left.id.cmp(&right.id))
    });
    presets
}

fn scope_rank(scope: DrawingSheetPresetScope) -> u8 {
    match scope {
        DrawingSheetPresetScope::Project => 0,
        DrawingSheetPresetScope::User => 1,
        DrawingSheetPresetScope::Organization => 2,
    }
}

fn import_source_reference(
    candidate: &model::ImportCandidate,
) -> DrawingSheetPresetImportReference {
    DrawingSheetPresetImportReference {
        preset_id: candidate.portable.stable_id.clone(),
        scope: candidate.portable.source_scope,
    }
}

fn preset_import_reference(preset: &DrawingSheetPreset) -> DrawingSheetPresetImportReference {
    DrawingSheetPresetImportReference {
        preset_id: preset.id.clone(),
        scope: preset.scope,
    }
}

const fn durable_import_resolution(
    resolution: ImportResolution,
) -> DrawingSheetPresetImportResolution {
    match resolution {
        ImportResolution::NewIdentity => DrawingSheetPresetImportResolution::NewIdentity,
        ImportResolution::MatchesByDigest => DrawingSheetPresetImportResolution::MatchesByDigest,
        ImportResolution::KeepBothRename => DrawingSheetPresetImportResolution::KeepBothRename,
        ImportResolution::MapExisting => DrawingSheetPresetImportResolution::MapExisting,
        ImportResolution::ReplaceManagedDependencies => {
            DrawingSheetPresetImportResolution::ReplaceManagedDependencies
        }
        ImportResolution::RetainUnavailableDependency => {
            DrawingSheetPresetImportResolution::RetainUnavailableDependency
        }
        ImportResolution::Skip => DrawingSheetPresetImportResolution::Skip,
    }
}

fn usage_counts(
    project: &DesignManagementCatalog,
    personal: &DrawingSheetPersonalPreferences,
) -> BTreeMap<String, usize> {
    let mut usage = BTreeMap::<String, usize>::new();
    for sheet in project
        .sheet_catalogs()
        .values()
        .flat_map(|catalog| catalog.sheets())
    {
        if let AuthoredDrawingSheetSize::Custom { snapshot } = &sheet.page_format().authored_size
            && let Some(id) = snapshot.preset_id.as_ref()
        {
            *usage
                .entry(render::usage_key(DrawingSheetPresetScope::Project, id))
                .or_default() += 1;
        }
    }
    let settings = project.drawing_sheet_settings();
    for format in
        std::iter::once(&settings.default_format).chain(settings.last_explicit_format.as_ref())
    {
        if let AuthoredDrawingSheetSize::Custom { snapshot } = &format.authored_size
            && let Some(id) = snapshot.preset_id.as_ref()
        {
            *usage
                .entry(render::usage_key(DrawingSheetPresetScope::Project, id))
                .or_default() += 1;
        }
    }
    if let AuthoredDrawingSheetSize::Custom { snapshot } = &personal.default_format.authored_size
        && let Some(id) = snapshot.preset_id.as_ref()
    {
        *usage
            .entry(render::usage_key(DrawingSheetPresetScope::User, id))
            .or_default() += 1;
    }
    usage
}

fn usage_count(
    project: &DesignManagementCatalog,
    personal: &DrawingSheetPersonalPreferences,
    scope: DrawingSheetPresetScope,
    id: &str,
) -> usize {
    usage_counts(project, personal)
        .get(&render::usage_key(scope, id))
        .copied()
        .unwrap_or(0)
}

fn personal_default_references_preset(
    personal: &DrawingSheetPersonalPreferences,
    id: &str,
) -> bool {
    matches!(
        &personal.default_format.authored_size,
        AuthoredDrawingSheetSize::Custom { snapshot }
            if snapshot
                .preset_id
                .as_deref()
                .is_some_and(|preset_id| preset_id.eq_ignore_ascii_case(id))
    )
}

fn transfer_identity(preset: &DrawingSheetPreset) -> String {
    format!("{}:{}", scope_rank(preset.scope), preset.id.to_lowercase())
}

fn normalized_package_name(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        return "rspice-sheet-formats.json".to_owned();
    }
    if name.to_ascii_lowercase().ends_with(".json") {
        name.to_owned()
    } else {
        format!("{name}.json")
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_bounded_utf8(path: &Path) -> Result<String, String> {
    let file = std::fs::File::open(path)
        .map_err(|error| format!("Could not open the preset package: {error}"))?;
    let mut bytes = Vec::new();
    file.take((model::MAX_PACKAGE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Could not read the preset package: {error}"))?;
    if bytes.len() > model::MAX_PACKAGE_BYTES {
        return Err(format!(
            "The preset package exceeds the {} byte limit.",
            model::MAX_PACKAGE_BYTES
        ));
    }
    String::from_utf8(bytes).map_err(|_| "The preset package must be UTF-8 JSON.".to_owned())
}

fn export_package_file(name: &str, source: &str) -> Result<bool, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("RSpice sheet formats", &["json"])
            .set_file_name(name)
            .save_file()
        else {
            return Ok(false);
        };
        let expected =
            crate::io::durable_file::observe_expected_content(&path).map_err(|error| {
                format!("Could not authorize the sheet-format destination: {error}")
            })?;
        crate::io::durable_file::compare_exchange_bytes(&path, expected, source.as_bytes())
            .map_err(|error| format!("Could not publish the sheet-format package: {error}"))?;
        Ok(true)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let path = PathBuf::from(name);
        crate::workbench::browser::download::download_text_file(&path, source)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_identity_keeps_equal_ids_in_distinct_authorities() {
        let format =
            model::custom_format("Review strip", 210_000, 594_000, StartingFrame::Plain).unwrap();
        let project = DrawingSheetPreset {
            id: "review-strip".to_owned(),
            name: "Review strip".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: format.clone(),
        };
        let personal = DrawingSheetPreset {
            scope: DrawingSheetPresetScope::User,
            ..project.clone()
        };
        assert_ne!(transfer_identity(&project), transfer_identity(&personal));
    }

    #[test]
    fn package_names_always_receive_json_extension() {
        assert_eq!(normalized_package_name("lab-sheets"), "lab-sheets.json");
        assert_eq!(
            normalized_package_name("LAB-SHEETS.JSON"),
            "LAB-SHEETS.JSON"
        );
    }

    #[test]
    fn personal_default_counts_as_a_preset_reference() {
        let id = "personal-default-panel";
        let mut format = model::custom_format(
            "Personal default panel",
            250_000,
            400_000,
            StartingFrame::IsoA,
        )
        .unwrap();
        format = format
            .try_update(|draft| {
                if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                    snapshot.preset_id = Some(id.to_owned());
                }
            })
            .unwrap();
        let preset = DrawingSheetPreset {
            id: id.to_owned(),
            name: "Personal default panel".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: format.clone(),
        };
        let mut personal = DrawingSheetPersonalPreferences {
            default_format: format
                .try_update(|draft| {
                    draft.inheritance = crate::state::DrawingSheetInheritance::UserDefault;
                })
                .unwrap(),
            presets: vec![preset],
        };
        personal.validate().unwrap();

        assert!(personal_default_references_preset(&personal, id));
        assert_eq!(
            usage_count(
                &DesignManagementCatalog::default(),
                &personal,
                DrawingSheetPresetScope::User,
                id,
            ),
            1
        );

        personal.presets.clear();
        assert!(personal.validate().is_err());
    }

    #[test]
    fn drawing_sheet_preset_import_receipt_retains_reviewed_outcomes() {
        let mut app = RSpiceApp::test_instance();
        let existing = DrawingSheetPreset {
            id: "existing-review-strip".to_owned(),
            name: "Existing review strip".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: model::custom_format(
                "Existing review strip",
                210_001,
                594_002,
                StartingFrame::Plain,
            )
            .unwrap(),
        };
        let mut seeded = app.state.workspace.design_management.clone();
        seeded
            .publish_drawing_sheet_preset(seeded.revision(), existing.clone())
            .unwrap();
        app.state
            .workspace
            .replace_design_management(seeded)
            .unwrap();

        let imported = DrawingSheetPreset {
            id: "new-review-panel".to_owned(),
            name: "New review panel".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: model::custom_format("New review panel", 220_001, 610_002, StartingFrame::IsoA)
                .unwrap(),
        };
        let not_selected = DrawingSheetPreset {
            id: "not-selected-panel".to_owned(),
            name: "Not selected panel".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: model::custom_format(
                "Not selected panel",
                230_001,
                620_002,
                StartingFrame::Plain,
            )
            .unwrap(),
        };
        let explicitly_skipped = DrawingSheetPreset {
            id: "explicitly-skipped-panel".to_owned(),
            name: "Explicitly skipped panel".to_owned(),
            scope: DrawingSheetPresetScope::Organization,
            format: model::custom_format(
                "Explicitly skipped panel",
                240_001,
                630_002,
                StartingFrame::Plain,
            )
            .unwrap(),
        };
        let package = model::build_package([
            existing.clone(),
            imported.clone(),
            not_selected.clone(),
            explicitly_skipped.clone(),
        ])
        .unwrap();
        let (package, trust) = model::authenticate_package_for_test(package);
        app.state.pdk_config.publisher_trust_store = trust;
        let source = model::encode_package(&package).unwrap();

        assert!(open_custom_sheet_size_library(&mut app.state));
        let personal = app
            .state
            .ui
            .preferences
            .drawing_sheet_personal_preferences();
        let visible = all_visible_presets(&app.state.workspace.design_management, &personal);
        app.review_import_source(&source, &visible).unwrap();
        {
            let candidates = &mut app
                .state
                .dialogs
                .drawing_sheet_presets
                .transfer
                .import_candidates;
            candidates
                .iter_mut()
                .find(|candidate| candidate.portable.stable_id == not_selected.id)
                .unwrap()
                .selected = false;
            let explicit = candidates
                .iter_mut()
                .find(|candidate| candidate.portable.stable_id == explicitly_skipped.id)
                .unwrap();
            explicit.selected = true;
            explicit.resolution = ImportResolution::Skip;
        }
        app.state.dialogs.drawing_sheet_presets.transfer.json = source;

        let message = app.apply_preset_import().unwrap().unwrap();
        assert!(message.contains("durable receipt retained"));
        let settings = app
            .state
            .workspace
            .design_management
            .drawing_sheet_settings();
        assert!(settings.find_preset(&existing.id).is_some());
        assert!(settings.find_preset(&imported.id).is_some());
        assert!(settings.find_preset(&not_selected.id).is_none());
        assert!(settings.find_preset(&explicitly_skipped.id).is_none());

        let receipt = settings.preset_import_receipts.last().unwrap();
        assert_eq!(receipt.source_digest_sha256, package.source_digest_sha256);
        assert_eq!(receipt.source_schema, package.schema);
        assert_eq!(receipt.source_schema_version, package.version);
        assert_eq!(receipt.reviewed_candidate_count, 4);
        assert_eq!(receipt.selected_candidates.len(), 3);
        assert_eq!(receipt.mappings.len(), 2);
        assert_eq!(receipt.conflicts.len(), 1);
        assert_eq!(receipt.skipped_candidates.len(), 2);
        assert!(receipt.mappings.iter().any(|mapping| {
            mapping.source.preset_id == existing.id
                && mapping.target.preset_id == existing.id
                && mapping.kind == DrawingSheetPresetImportMappingKind::ExistingPreset
        }));
        assert!(receipt.mappings.iter().any(|mapping| {
            mapping.source.preset_id == imported.id
                && mapping.target.preset_id == imported.id
                && mapping.kind == DrawingSheetPresetImportMappingKind::CreatedProjectPreset
        }));
        assert!(receipt.skipped_candidates.iter().any(|skip| {
            skip.source.preset_id == not_selected.id
                && skip.reason == DrawingSheetPresetImportSkipReason::NotSelected
        }));
        assert!(receipt.skipped_candidates.iter().any(|skip| {
            skip.source.preset_id == explicitly_skipped.id
                && skip.reason == DrawingSheetPresetImportSkipReason::ExplicitlySkipped
        }));
    }
}
