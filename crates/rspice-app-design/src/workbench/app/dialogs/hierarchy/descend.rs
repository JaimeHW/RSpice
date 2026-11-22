//! Explicit hierarchy edit-context transaction owned by the Design menu.
//!
//! Toolbar, inspector, double-click, and Shift+E gestures follow the user's
//! edit-in-place preference. The menu workflow exposes the upgraded mockup's
//! target, explicit edit mode, and working-checkpoint contract.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice, FormRows};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::symbol_in_place::{
    SymbolInPlaceRefusal, selected_symbol_in_place_target,
};
use crate::workbench::app::actions::workspace::{
    NavigationParts, hierarchical_edit_in_place_enabled, selected_hierarchy_master,
    selected_instance_veriloga_source,
};
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HierarchyDescendEditMode {
    #[default]
    EditInPlace,
    OpenIsolated,
    ReadOnlyReference,
    /// Edit the instance's symbol master on the parent, through the
    /// instance's transform.
    EditSymbolInPlace,
    /// Open a Verilog-A instance's module, a view of its cell with no
    /// occurrence below it.
    OpenVerilogA,
}

/// Isolated task draft for Design ▸ Descend into selected instance.
///
/// Direct toolbar, inspector, double-click, and Shift+E gestures use the
/// user's hierarchy preference. The menu workflow owns the explicit
/// edit-context choices defined by the upgraded mockup.
#[derive(Debug, Clone, Default)]
pub struct DescendHierarchyDialogState {
    pub open: bool,
    pub(crate) instance_name: String,
    pub(crate) reference: Option<crate::state::CellViewRef>,
    pub edit_mode: HierarchyDescendEditMode,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub parent_dirty: bool,
    pub(crate) validation_error: Option<String>,
    /// The instance's cell has a symbol and no schematic master: its symbol
    /// edited in place is the one way in.
    pub(crate) symbol_only: bool,
    /// The instance is a Verilog-A instance: its module opens, or its
    /// symbol is edited in place.
    pub(crate) veriloga: bool,
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
            symbol_only: false,
            veriloga: false,
        };
    }

    /// Open over a Verilog-A instance, whose module view `module` opens as
    /// the direct gesture opens it.
    pub(crate) fn open_veriloga(
        &mut self,
        instance_name: String,
        module: crate::state::CellViewRef,
        authority: SchematicEditAuthority,
        parent_dirty: bool,
    ) {
        *self = Self {
            open: true,
            instance_name,
            reference: Some(module),
            edit_mode: HierarchyDescendEditMode::OpenVerilogA,
            authority: Some(authority),
            parent_dirty,
            validation_error: None,
            symbol_only: false,
            veriloga: true,
        };
    }

    /// Open over an instance whose cell has only a symbol, which the form
    /// offers to edit in place and nothing else.
    pub(crate) fn open_symbol_only(
        &mut self,
        instance_name: String,
        authority: SchematicEditAuthority,
        parent_dirty: bool,
    ) {
        *self = Self {
            open: true,
            instance_name,
            reference: None,
            edit_mode: HierarchyDescendEditMode::EditSymbolInPlace,
            authority: Some(authority),
            parent_dirty,
            validation_error: None,
            symbol_only: true,
            veriloga: false,
        };
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Descend into hierarchy";
const PRIMARY: &str = "Open edit context";
const DESCRIPTION: &str =
    "Review the operation, its inputs, impact, validation state, and available actions.";

pub fn open_descend_hierarchy_dialog(
    dialog: &mut DescendHierarchyDialogState,
    model: &Model,
    workbench: &mut WorkbenchSession,
) -> bool {
    if dialog.open {
        return false;
    }
    let authority = SchematicEditAuthority::capture(model);
    let parent_dirty = model.schematic.session.is_dirty;
    let Some((instance_name, reference)) = selected_hierarchy_master(model) else {
        // A Verilog-A instance opens its module, as the direct gesture does.
        if let Some((instance_name, module)) = selected_veriloga_instance(model) {
            dialog.open_veriloga(instance_name, module, authority, parent_dirty);
            return true;
        }
        // A cell with a symbol and no schematic master has no level below to
        // open, but its symbol can still be edited where it stands.
        if let Some(instance_name) = selected_symbol_only_instance(model) {
            dialog.open_symbol_only(instance_name, authority, parent_dirty);
            return true;
        }
        workbench.prompt("Select one instance with a resolved schematic master first.");
        return false;
    };
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

/// The selected Verilog-A instance's name and the module view it opens.
fn selected_veriloga_instance(model: &Model) -> Option<(String, crate::state::CellViewRef)> {
    let module = selected_instance_veriloga_source(model)?;
    let id = model
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| (component.name.clone(), module))
}

/// The name of the selected instance when its cell has no schematic master
/// to descend into but has a symbol that can be edited in place. A cell
/// that draws a schematic is not such a cell, whichever view the instance
/// is bound to.
pub(crate) fn selected_symbol_only_instance(model: &Model) -> Option<String> {
    if selected_hierarchy_master(model).is_some() {
        return None;
    }
    let (id, symbol) = selected_symbol_in_place_target(model).ok()?;
    let draws_schematic = model
        .library_manager
        .get_library(&symbol.library)
        .and_then(|library| library.get_cell(&symbol.cell))
        .is_some_and(|cell| {
            cell.views_sorted().into_iter().any(|view| {
                matches!(
                    view.view_type,
                    crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                )
            })
        });
    if draws_schematic {
        return None;
    }
    model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.name.clone())
}

/// Draw Descend into hierarchy over its draft and the model it reads. A
/// checked choice goes to the shell as [`DesignRequest::Descend`]; a refused
/// one stays on the form with its reason.
pub fn render_descend_hierarchy_dialog(
    dialog: &mut DescendHierarchyDialogState,
    model: &Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }

    // The view the chosen mode opens: the master descended into, or, edited
    // in place, the instance's symbol, or why it cannot be.
    let target = if dialog.edit_mode == HierarchyDescendEditMode::EditSymbolInPlace {
        selected_symbol_in_place_target(model)
            .map(|(_, symbol)| format!("{} \u{00b7} {}", symbol.cell, symbol.view))
            .map_err(SymbolInPlaceRefusal::sentence)
    } else {
        dialog
            .reference
            .as_ref()
            .map(|reference| format!("{} \u{00b7} {}", reference.cell, reference.view))
            .ok_or("The selection no longer resolves")
    };
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
            descend_rows(
                rows,
                draft,
                &occurrence_path,
                target.as_deref().map_err(|reason| *reason),
            )
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
pub enum DescendHierarchy {
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
pub fn descend_request(
    draft: &DescendHierarchyDialogState,
    model: &Model,
) -> Result<DescendHierarchy, String> {
    let authority = draft
        .authority
        .as_ref()
        .ok_or_else(|| "The hierarchy edit authority is missing.".to_owned())?;
    // Descending is navigation, so a read-only drawing descends as the
    // direct gesture does; each way in that writes checks its own target.
    authority.validate_presentation(model, "Descend into hierarchy")?;
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
    if draft.edit_mode == HierarchyDescendEditMode::OpenVerilogA {
        if selected_veriloga_instance(model)
            != Some((draft.instance_name.clone(), reference.clone()))
        {
            return Err(
                "The selected Verilog-A instance changed. Close and reopen Descend into hierarchy."
                    .to_owned(),
            );
        }
        return Ok(DescendHierarchy::Open {
            edit_mode: draft.edit_mode,
            instance: draft.instance_name.clone(),
            reference,
        });
    }
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

/// Carry out a checked Descend into hierarchy choice over the navigation
/// parts: open the view, descend into the instance or edit the symbol in
/// place. The dialog is closed before.
pub fn answer_descend_hierarchy(mut navigation: NavigationParts<'_>, request: DescendHierarchy) {
    match request {
        DescendHierarchy::EditSymbolInPlace => {
            navigation.edit_selected_symbol_in_place();
        }
        DescendHierarchy::Open {
            edit_mode,
            instance,
            reference,
        } => match edit_mode {
            HierarchyDescendEditMode::EditInPlace => {
                navigation.descend_into_instance(Some(instance), reference);
            }
            HierarchyDescendEditMode::OpenIsolated => navigation.open_isolated(reference),
            HierarchyDescendEditMode::ReadOnlyReference => {
                navigation.descend_read_only(instance, reference);
            }
            // A module has no occurrences below it, so opening it is not a
            // descent and records no step.
            HierarchyDescendEditMode::OpenVerilogA => navigation.open_workspace_view(reference),
            HierarchyDescendEditMode::EditSymbolInPlace => {}
        },
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
    target: Result<&str, &str>,
) -> Option<egui::Id> {
    // An isolated view is reached through no instance, so it carries no
    // occurrence path to show.
    if draft.edit_mode == HierarchyDescendEditMode::OpenIsolated {
        rows.value("Instance", "Opens as its own root");
    } else if draft.veriloga {
        // A module is opened, not descended into: there is no occurrence.
        rows.value_mono("Instance", &draft.instance_name);
    } else {
        rows.value_mono("Instance", occurrence_path);
    }
    match target {
        Ok(target) => rows.value_mono("Cell view", target),
        Err(reason) => rows.value("Cell view", reason),
    }
    if draft.symbol_only {
        // One way in is a fact, not a choice.
        rows.value("Edit mode", edit_mode_label(draft.edit_mode));
        if draft.parent_dirty {
            rows.value("Unsaved parent changes", "Kept as a working checkpoint");
        }
        return None;
    }
    // A Verilog-A instance's choices are its cell's views: the module and
    // the symbol.
    let modes: &[HierarchyDescendEditMode] = if draft.veriloga {
        &[
            HierarchyDescendEditMode::OpenVerilogA,
            HierarchyDescendEditMode::EditSymbolInPlace,
        ]
    } else {
        &[
            HierarchyDescendEditMode::EditInPlace,
            HierarchyDescendEditMode::OpenIsolated,
            HierarchyDescendEditMode::ReadOnlyReference,
            HierarchyDescendEditMode::EditSymbolInPlace,
        ]
    };
    let options: Vec<String> = modes
        .iter()
        .map(|mode| edit_mode_label(*mode).to_owned())
        .collect();
    let edit_mode = rows.select_with_response(
        "Edit mode",
        "descend-hierarchy-edit-mode",
        edit_mode_label(draft.edit_mode),
        &options,
    );
    if let Some(mode) = edit_mode.picked.and_then(|index| modes.get(index)) {
        draft.edit_mode = *mode;
        draft.validation_error = None;
    }
    // With nothing unsaved there is nothing to say.
    if draft.parent_dirty {
        rows.value("Unsaved parent changes", "Kept as a working checkpoint");
    }
    Some(edit_mode.response.id)
}

const fn edit_mode_label(mode: HierarchyDescendEditMode) -> &'static str {
    match mode {
        HierarchyDescendEditMode::EditInPlace => "Edit in place",
        HierarchyDescendEditMode::OpenIsolated => "Open isolated cell view",
        HierarchyDescendEditMode::ReadOnlyReference => "Read-only reference",
        HierarchyDescendEditMode::EditSymbolInPlace => "Edit symbol in place",
        HierarchyDescendEditMode::OpenVerilogA => "Open Verilog-A view",
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point, View,
        ViewType,
    };
    use crate::workbench::TogglePreference;
    use crate::workbench::design::test_host::DesignTestHost;

    pub(crate) fn hierarchy_state() -> (DesignTestHost, CellViewRef, CellViewRef) {
        let mut state = DesignTestHost::default();
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

    /// Open the form over the selected instance, pick `mode` and answer the
    /// checked choice as the shell does.
    fn descend_with(
        state: &mut DesignTestHost,
        mode: HierarchyDescendEditMode,
    ) -> Result<(), String> {
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        state.design.descend_hierarchy.edit_mode = mode;
        let request = descend_request(&state.design.descend_hierarchy, &state.model);
        state.design.descend_hierarchy.close();
        answer_descend_hierarchy(state.navigation(), request?);
        Ok(())
    }

    fn back_to_parent_with_the_instance_selected(state: &mut DesignTestHost, parent: CellViewRef) {
        state.open_workspace_view(parent);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(91);
    }

    #[test]
    fn a_read_only_reference_descends_and_return_goes_back_to_the_parent() {
        let (mut state, parent, child) = hierarchy_state();
        descend_with(&mut state, HierarchyDescendEditMode::ReadOnlyReference)
            .expect("a read-only look is navigation");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(
            state.model.workspace.content.occurrence_labels().last(),
            Some(&"XAFE".to_owned()),
            "the block is shown at the instance it was reached through"
        );
        assert_eq!(state.model.workspace.content.hierarchy_stack.len(), 2);
        assert!(state.model.active_view_read_only());
        assert!(
            state
                .prompt_text()
                .is_some_and(|text| text.contains("read-only")),
            "{:?}",
            state.prompt_text()
        );

        state.navigation().ascend_workspace_level();
        assert_eq!(state.model.workspace.content.active_view, parent);
        assert!(!state.model.active_view_read_only());
    }

    #[test]
    fn a_read_only_reference_leaves_an_editable_tab_of_the_block_editable() {
        let (mut state, parent, child) = hierarchy_state();
        state.open_workspace_view(child.clone());
        assert!(!state.model.active_view_read_only());
        back_to_parent_with_the_instance_selected(&mut state, parent);

        descend_with(&mut state, HierarchyDescendEditMode::ReadOnlyReference)
            .expect("a read-only look is navigation");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(state.model.workspace.content.hierarchy_stack.len(), 2);
        assert!(
            !state.model.active_view_read_only(),
            "the reader's own editing tab is not turned read-only"
        );
        assert!(
            state
                .prompt_text()
                .is_some_and(|text| text.contains("stays editable")),
            "{:?}",
            state.prompt_text()
        );
    }

    #[test]
    fn open_isolated_opens_the_block_editable_as_its_own_root() {
        let (mut state, parent, child) = hierarchy_state();
        descend_with(&mut state, HierarchyDescendEditMode::ReadOnlyReference)
            .expect("a read-only look is navigation");
        back_to_parent_with_the_instance_selected(&mut state, parent);

        descend_with(&mut state, HierarchyDescendEditMode::OpenIsolated)
            .expect("an isolated view opens");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(
            state.model.workspace.content.occurrence_labels(),
            ["afe_core"],
            "the occurrence the document was opened at before is dropped"
        );
        assert!(
            !state.model.active_view_read_only(),
            "the earlier read-only look no longer holds"
        );
        assert!(
            state
                .prompt_text()
                .is_some_and(|text| text.contains("its own root")),
            "{:?}",
            state.prompt_text()
        );
    }

    #[test]
    fn a_read_only_drawing_descends_with_options_as_it_does_directly() {
        let (mut state, _, child) = hierarchy_state();
        state
            .model
            .workspace
            .content
            .set_active_read_only_reference(true);
        assert!(state.model.active_view_read_only());

        descend_with(&mut state, HierarchyDescendEditMode::ReadOnlyReference)
            .expect("descending is navigation, not an edit of the parent");

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(state.model.workspace.content.hierarchy_stack.len(), 2);
    }

    #[test]
    fn a_verilog_a_instance_opens_its_module_from_the_form() {
        let mut state = DesignTestHost::default();
        let parent = state.model.workspace.content.active_view.clone();
        let (_, module) =
            crate::schematic::view::context_menu::tests::select_a_veriloga_instance(&mut state);
        assert!(open_descend_hierarchy_dialog(
            &mut state.design.descend_hierarchy,
            &state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.descend_hierarchy.edit_mode,
            HierarchyDescendEditMode::OpenVerilogA,
            "the module is the form's first choice, as it is the direct gesture's"
        );
        let request = descend_request(&state.design.descend_hierarchy, &state.model)
            .expect("the module resolves");
        state.design.descend_hierarchy.close();
        answer_descend_hierarchy(state.navigation(), request);

        assert_eq!(state.model.workspace.content.active_view, module);
        assert_eq!(
            state.model.workspace.content.occurrence_labels(),
            ["gain"],
            "a module has no occurrence below its parent to record"
        );
        assert_ne!(module, parent);
    }

    #[test]
    fn direct_gesture_descends_in_place_without_opening_the_menu_dialog() {
        let (mut state, _, child) = hierarchy_state();
        state.navigation().open_selected_instance_master();
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

        state.navigation().open_selected_instance_master();

        assert_eq!(state.model.workspace.content.active_view, child);
        assert_eq!(state.model.workspace.content.hierarchy_stack, vec![child]);
        assert_eq!(
            state.model.workspace.content.occurrence_labels(),
            ["afe_core"]
        );
        assert!(!state.design.descend_hierarchy.open);
    }
}
