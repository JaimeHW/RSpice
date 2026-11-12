//! Explicit hierarchy edit-context transaction owned by the Design menu.
//!
//! Toolbar, inspector, double-click, and Shift+E gestures follow the user's
//! edit-in-place preference. The menu workflow exposes the upgraded mockup's
//! target, explicit edit mode, and working-checkpoint contract.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice, FormRows};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::symbol_in_place::selected_symbol_in_place_target;
use crate::workbench::app::actions::workspace::{
    hierarchical_edit_in_place_enabled, selected_hierarchy_master,
};
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HierarchyDescendEditMode {
    #[default]
    EditInPlace,
    OpenIsolated,
    ReadOnlyReference,
    /// Edit the instance's symbol master on the parent, through the
    /// instance's transform.
    EditSymbolInPlace,
}

/// Isolated task draft for Design ▸ Descend into selected instance.
///
/// Direct toolbar, inspector, double-click, and Shift+E gestures use the
/// user's hierarchy preference. The menu workflow owns the explicit
/// edit-context choices defined by the upgraded mockup.
#[derive(Debug, Clone, Default)]
pub(crate) struct DescendHierarchyDialogState {
    pub(crate) open: bool,
    pub(crate) instance_name: String,
    pub(crate) reference: Option<crate::state::CellViewRef>,
    pub(crate) edit_mode: HierarchyDescendEditMode,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) parent_dirty: bool,
    pub(crate) validation_error: Option<String>,
}

impl DescendHierarchyDialogState {
    pub(crate) fn open(
        &mut self,
        instance_name: String,
        reference: crate::state::CellViewRef,
        authority: SchematicEditAuthority,
        parent_dirty: bool,
        preferred_edit_mode: HierarchyDescendEditMode,
    ) {
        *self = Self {
            open: true,
            instance_name,
            reference: Some(reference),
            edit_mode: preferred_edit_mode,
            authority: Some(authority),
            parent_dirty,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Descend into hierarchy";
const PRIMARY: &str = "Open edit context";
const DESCRIPTION: &str =
    "Review the operation, its inputs, impact, validation state, and available actions.";

pub(crate) fn open_descend_hierarchy_dialog(
    dialog: &mut DescendHierarchyDialogState,
    model: &Model,
    workbench: &mut WorkbenchSession,
) -> bool {
    if dialog.open {
        return false;
    }
    let Some((instance_name, reference)) = selected_hierarchy_master(model) else {
        workbench.prompt("Select one instance with a resolved schematic master first.");
        return false;
    };
    let authority = SchematicEditAuthority::capture(model);
    let parent_dirty = model.schematic.session.is_dirty;
    let preferred_edit_mode = if hierarchical_edit_in_place_enabled(model) {
        HierarchyDescendEditMode::EditInPlace
    } else {
        HierarchyDescendEditMode::OpenIsolated
    };
    dialog.open(
        instance_name,
        reference,
        authority,
        parent_dirty,
        preferred_edit_mode,
    );
    true
}

/// Draw Descend into hierarchy over its draft and the model it reads. A
/// checked choice goes to the shell as [`DesignRequest::Descend`]; a refused
/// one stays on the form with its reason.
pub(in crate::workbench) fn render_descend_hierarchy_dialog(
    dialog: &mut DescendHierarchyDialogState,
    model: &Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }

    let target = dialog
        .reference
        .as_ref()
        .map(|reference| format!("{} \u{00b7} {}", reference.cell, reference.view));
    let instance = dialog.instance_name.clone();
    let occurrence_path = descend_target_path(&model.workspace, &instance);
    let error = dialog.validation_error.clone();
    let draft = &mut *dialog;
    let choice = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        // Wide enough for "Unsaved parent changes" on one line.
        .label_column(136.0)
        .status(error.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            Some(descend_rows(
                rows,
                draft,
                &occurrence_path,
                target.as_deref(),
            ))
        });

    match choice {
        DialogChoice::Primary => match descend_request(dialog, model) {
            Ok(choice) => workbench.outbox.push(DesignRequest::Descend(choice)),
            Err(error) => dialog.validation_error = Some(error),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.close();
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// The occurrence this transaction would open: the session's current
/// occurrence with the selected instance below it, rooted at the implicit
/// design root so the top cell never occupies a segment.
///
/// An instance the path grammar cannot spell has no occurrence to name, so the
/// row reports why instead of a path that addresses a different instance.
fn descend_target_path(workspace: &crate::state::ProjectWorkspace, instance: &str) -> String {
    workspace
        .content
        .occurrence_path()
        .child(instance)
        .map_or_else(|error| error.to_string(), |path| path.to_string())
}

/// Descend into hierarchy's checked choice, which the dialog's step hands the
/// shell as [`DesignRequest::Descend`].
#[derive(Debug, Clone)]
pub(crate) enum DescendHierarchy {
    /// Edit the selected instance's symbol on this sheet.
    EditSymbolInPlace,
    /// Open the selected instance's master as the reader chose.
    Open {
        edit_mode: HierarchyDescendEditMode,
        instance: String,
        reference: crate::state::CellViewRef,
    },
}

/// Check the draft against the model as it stands: the choice to carry out,
/// or why it is refused.
fn descend_request(
    draft: &DescendHierarchyDialogState,
    model: &Model,
) -> Result<DescendHierarchy, String> {
    let authority = draft
        .authority
        .as_ref()
        .ok_or_else(|| "The hierarchy edit authority is missing.".to_owned())?;
    authority.validate(model, "Descend into hierarchy")?;
    // The symbol is edited on this sheet rather than descended into, so the
    // instance it is edited at is the whole target.
    if draft.edit_mode == HierarchyDescendEditMode::EditSymbolInPlace {
        if let Err(refusal) = selected_symbol_in_place_target(model) {
            return Err(format!(
                "Edit symbol in place is unavailable: {}.",
                refusal.reason()
            ));
        }
        return Ok(DescendHierarchy::EditSymbolInPlace);
    }
    let reference = draft
        .reference
        .clone()
        .ok_or_else(|| "The selected hierarchy master no longer resolves.".to_owned())?;
    let current = selected_hierarchy_master(model)
        .ok_or_else(|| "The selected hierarchy master no longer resolves.".to_owned())?;
    if current.0 != draft.instance_name || current.1 != reference {
        return Err(
            "The selected hierarchy target changed. Close and reopen Descend into hierarchy."
                .to_owned(),
        );
    }
    Ok(DescendHierarchy::Open {
        edit_mode: draft.edit_mode,
        instance: draft.instance_name.clone(),
        reference,
    })
}

/// Carry out a checked Descend into hierarchy choice over the whole session,
/// as the dialog once did in place: opening a view, descending and editing a
/// symbol in place still reach the whole session.
pub(crate) fn answer_descend_hierarchy(state: &mut AppState, request: DescendHierarchy) {
    match request {
        DescendHierarchy::EditSymbolInPlace => {
            state.design.descend_hierarchy.close();
            state.edit_selected_symbol_in_place();
        }
        DescendHierarchy::Open {
            edit_mode,
            instance,
            reference,
        } => {
            match edit_mode {
                HierarchyDescendEditMode::EditInPlace => {
                    state.descend_into_instance(Some(instance), reference);
                }
                HierarchyDescendEditMode::OpenIsolated => {
                    state.open_workspace_view(reference);
                }
                HierarchyDescendEditMode::ReadOnlyReference => {
                    state.open_workspace_view(reference);
                    state
                        .model
                        .workspace
                        .content
                        .set_active_read_only_reference(true);
                }
                HierarchyDescendEditMode::EditSymbolInPlace => {}
            }
            state.design.descend_hierarchy.close();
            state.prompt("Descended into the instance; the parent's unsaved changes are kept.");
        }
    }
}

/// The form: what is opened, how, and what becomes of the parent's unsaved
/// work, each a row on the form's one label column with its value in the
/// kit's face. Each row used to carry a bold title over a sentence that
/// restated it, beside a mono select and a small-bold value.
fn descend_rows(
    rows: &mut FormRows<'_>,
    draft: &mut DescendHierarchyDialogState,
    occurrence_path: &str,
    target: Option<&str>,
) -> egui::Id {
    rows.value_mono("Instance", occurrence_path);
    match target {
        Some(target) => rows.value_mono("Cell view", target),
        None => rows.value("Cell view", "The selection no longer resolves"),
    }
    let options = [
        "Edit in place",
        "Open isolated cell view",
        "Read-only reference",
        "Edit symbol in place",
    ]
    .map(str::to_owned);
    let edit_mode = rows.select_with_response(
        "Edit mode",
        "descend-hierarchy-edit-mode",
        edit_mode_label(draft.edit_mode),
        &options,
    );
    if let Some(index) = edit_mode.picked {
        draft.edit_mode = match index {
            1 => HierarchyDescendEditMode::OpenIsolated,
            2 => HierarchyDescendEditMode::ReadOnlyReference,
            3 => HierarchyDescendEditMode::EditSymbolInPlace,
            _ => HierarchyDescendEditMode::EditInPlace,
        };
        draft.validation_error = None;
    }
    // With nothing unsaved there is nothing to say.
    if draft.parent_dirty {
        rows.value("Unsaved parent changes", "Kept as a working checkpoint");
    }
    edit_mode.response.id
}

const fn edit_mode_label(mode: HierarchyDescendEditMode) -> &'static str {
    match mode {
        HierarchyDescendEditMode::EditInPlace => "Edit in place",
        HierarchyDescendEditMode::OpenIsolated => "Open isolated cell view",
        HierarchyDescendEditMode::ReadOnlyReference => "Read-only reference",
        HierarchyDescendEditMode::EditSymbolInPlace => "Edit symbol in place",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point, View,
        ViewType,
    };
    use crate::workbench::TogglePreference;
    use crate::workbench::app::RSpiceApp;

    /// Apply the choice as the dialog's step and the shell's answer do,
    /// straight after one another.
    fn commit_descend_context(state: &mut AppState) -> Result<(), String> {
        let request = descend_request(&state.design.descend_hierarchy, &state.model)?;
        answer_descend_hierarchy(state, request);
        Ok(())
    }

    fn hierarchy_state() -> (AppState, CellViewRef, CellViewRef) {
        let mut state = AppState::default();
        let parent = state.model.workspace.content.active_view.clone();
        let child = CellViewRef::new("hierarchy_test", "afe_core", "schematic");
        let mut cell = Cell::new("afe_core");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut library = Library::new("hierarchy_test");
        library.add_cell(cell);
        state.model.library_manager.add_library(library);

        let mut instance = Component::new(91, ComponentType::CellInstance, Point::new(100, 100))
            .with_library_cell(LibraryCellInstance::new(
                "hierarchy_test",
                "afe_core",
                "schematic",
            ));
        instance.name = "XAFE".to_owned();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(instance);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(91);
        (state, parent, child)
    }

    /// The form states each fact once, in its one label style, with every
    /// value in the kit's 13 point face: mono for the instance and its cell
    /// view, the body face for a choice named in words. Each row was a bold
    /// title over a helper sentence that restated it, beside a mono select
    /// and a small-bold value.
    #[test]
    fn the_form_has_one_label_style_and_values_in_the_kit_face() {
        let (mut state, _, _) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        state.design.descend_hierarchy.parent_dirty = true;
        let mut app = RSpiceApp::test_instance();
        app.state = state;
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1440.0, 900.0),
                        )),
                        ..Default::default()
                    },
                    |ui| {
                        let state = &mut app.state;
                        render_descend_hierarchy_dialog(
                            &mut state.design.descend_hierarchy,
                            &state.model,
                            &mut state.workbench,
                            ui.ctx(),
                        );
                    },
                )
                .shapes;
        }
        let painted = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.clone(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        let font = |wanted: &str| {
            painted
                .iter()
                .find(|(text, _)| text == wanted)
                .map(|(_, font)| font.clone())
                .unwrap_or_else(|| panic!("{wanted:?} is not painted: {painted:?}"))
        };
        let label = font("Instance");
        for other in ["Cell view", "Edit mode", "Unsaved parent changes"] {
            assert_eq!(font(other), label, "{other} is not in the label style");
        }
        assert_eq!(font("/XAFE"), crate::ui::theme::field_mono());
        assert_eq!(font("Edit in place"), crate::ui::theme::field_sans());
        assert_eq!(
            font("Kept as a working checkpoint"),
            crate::ui::theme::field_sans()
        );
        for (text, _) in &painted {
            for gone in [
                "Selected hierarchical",
                "Controls who owns",
                "transactional",
            ] {
                assert!(!text.contains(gone), "{text:?} is painted");
            }
        }
    }

    #[test]
    fn every_mockup_edit_context_has_an_exact_label() {
        assert_eq!(
            edit_mode_label(HierarchyDescendEditMode::EditInPlace),
            "Edit in place"
        );
        assert_eq!(
            edit_mode_label(HierarchyDescendEditMode::OpenIsolated),
            "Open isolated cell view"
        );
        assert_eq!(
            edit_mode_label(HierarchyDescendEditMode::ReadOnlyReference),
            "Read-only reference"
        );
        assert_eq!(
            edit_mode_label(HierarchyDescendEditMode::EditSymbolInPlace),
            "Edit symbol in place"
        );
    }

    #[test]
    fn the_target_row_names_the_occurrence_the_transaction_would_open() {
        let (mut state, _, child) = hierarchy_state();
        assert_eq!(descend_target_path(&state.model.workspace, "XAFE"), "/XAFE");

        state
            .model
            .workspace
            .descend_into("XAFE".to_owned(), child, ViewType::Schematic);
        assert_eq!(
            descend_target_path(&state.model.workspace, "XBIAS"),
            "/XAFE/XBIAS"
        );
        assert!(
            descend_target_path(&state.model.workspace, "X BIAS").contains("X BIAS"),
            "an unspellable instance is reported, not silently truncated"
        );
    }

    #[test]
    fn menu_transaction_opens_without_navigating_and_rejects_selection_drift() {
        let (mut state, parent, _) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        assert!(state.design.descend_hierarchy.open);
        assert_eq!(state.model.workspace.content.active_view, parent);

        state.model.schematic.session.editor.selection.clear();
        let error = commit_descend_context(&mut state).expect_err("selection drift must block");
        assert!(error.contains("selected-object set changed"));
        assert_eq!(state.model.workspace.content.active_view, parent);
        assert!(state.design.descend_hierarchy.open);
    }

    #[test]
    fn direct_gesture_descends_in_place_without_opening_the_menu_dialog() {
        let (mut state, _, child) = hierarchy_state();
        state.open_selected_instance_master();
        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(
            state
                .model
                .workspace
                .content
                .occurrence_labels()
                .last()
                .unwrap(),
            "XAFE"
        );
        assert!(!state.design.descend_hierarchy.open);
    }

    #[test]
    fn disabled_edit_in_place_opens_direct_gestures_as_isolated_views() {
        let (mut state, _, child) = hierarchy_state();
        state
            .model
            .preferences
            .set_toggle(TogglePreference::HierarchicalEditInPlace, false);

        state.open_selected_instance_master();

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(state.model.workspace.content.hierarchy_stack, vec![child]);
        assert_eq!(
            state.model.workspace.content.occurrence_labels(),
            ["afe_core"]
        );
        assert!(!state.design.descend_hierarchy.open);
    }

    #[test]
    fn preference_sets_the_workflow_default_without_removing_explicit_choices() {
        let (mut state, _, child) = hierarchy_state();
        state
            .model
            .preferences
            .set_toggle(TogglePreference::HierarchicalEditInPlace, false);
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.descend_hierarchy.edit_mode,
            HierarchyDescendEditMode::OpenIsolated
        );

        state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditInPlace;
        commit_descend_context(&mut state).expect("explicit edit-in-place choice remains valid");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(
            state
                .model
                .workspace
                .content
                .occurrence_labels()
                .last()
                .unwrap(),
            "XAFE"
        );
    }

    #[test]
    fn the_symbol_mode_edits_the_instance_symbol_on_this_sheet() {
        let (mut state, parent, _) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditSymbolInPlace;
        let error = commit_descend_context(&mut state).expect_err("afe_core has no symbol view");
        assert!(error.contains("the cell has no symbol view"), "{error}");
        assert_eq!(state.model.workspace.content.active_view, parent);

        state
            .model
            .library_manager
            .get_library_mut("hierarchy_test")
            .and_then(|library| library.get_cell_mut("afe_core"))
            .expect("the instance's cell")
            .add_view(View::new("symbol", ViewType::Symbol));
        commit_descend_context(&mut state).expect("the symbol is editable");

        assert!(!state.design.descend_hierarchy.open);
        assert_eq!(
            state.model.workspace.content.active_view,
            CellViewRef::new("hierarchy_test", "afe_core", "symbol")
        );
        assert_eq!(
            state.symbol_in_place().map(|session| &session.parent),
            Some(&parent),
            "the symbol is edited on the sheet, not descended into"
        );
    }

    #[test]
    fn read_only_reference_belongs_to_its_document_and_never_mutates_library_ownership() {
        let (mut state, parent, child) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::ReadOnlyReference;
        commit_descend_context(&mut state).expect("valid reference context");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert!(state.model.workspace.content.active_read_only_reference());
        assert!(state.active_view_read_only());
        assert!(!state.model.schematic.session.read_only);
        assert!(
            !state
                .model
                .library_manager
                .get_library("hierarchy_test")
                .expect("library")
                .read_only
        );

        state.open_workspace_view(parent);
        assert!(
            !state.model.workspace.content.active_read_only_reference(),
            "the parent was never opened as a reference"
        );
        assert!(!state.active_view_read_only());

        state.open_workspace_view(child);
        assert!(
            state.model.workspace.content.active_read_only_reference(),
            "returning to the reference document still refuses writes"
        );
        assert!(
            !state.symbol_editor_copy_available(),
            "the symbol editor's escape hatch follows the same document"
        );
    }
}
