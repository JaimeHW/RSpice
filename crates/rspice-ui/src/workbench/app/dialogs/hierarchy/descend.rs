//! Explicit hierarchy edit-context transaction owned by the Design menu.
//!
//! Toolbar, inspector, double-click, and Shift+E gestures follow the user's
//! edit-in-place preference. The menu workflow exposes the upgraded mockup's
//! target, explicit edit mode, parent context, and working-checkpoint contract.

use egui::{Context, Frame, Margin, Stroke, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::SchematicHierarchyVisibility;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, select_mono_with_response,
};

use crate::workbench::app::{
    AppState, HierarchyDescendEditMode, HierarchyParentContext, RSpiceApp, SchematicEditAuthority,
};

const EYEBROW: &str = "SCHEMATIC \u{00b7} EDIT CONTEXT";
const TITLE: &str = "Descend into hierarchy";
const PRIMARY: &str = "Open edit context";
const DESCRIPTION: &str = "Review the operation, its exact inputs, impact, validation state, and available transaction actions.";

pub(crate) fn open_descend_hierarchy_dialog(state: &mut AppState) -> bool {
    if state.dialogs.descend_hierarchy.open {
        return false;
    }
    let Some((instance_name, reference)) = state.selected_hierarchy_master() else {
        state.push_user_message(ConsoleMessage::warning(
            "Select one instance with a resolved schematic master first.",
        ));
        return false;
    };
    let authority = SchematicEditAuthority::capture(state);
    let parent_dirty = state.schematic.is_dirty;
    let preferred_edit_mode = if state.hierarchical_edit_in_place_enabled() {
        HierarchyDescendEditMode::EditInPlace
    } else {
        HierarchyDescendEditMode::OpenIsolated
    };
    state.dialogs.descend_hierarchy.open(
        instance_name,
        reference,
        authority,
        parent_dirty,
        preferred_edit_mode,
    );
    true
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_descend_hierarchy_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.descend_hierarchy.open {
            return;
        }

        let target = self
            .state
            .dialogs
            .descend_hierarchy
            .reference
            .as_ref()
            .map(|reference| format!("{} \u{00b7} {}", reference.cell, reference.view))
            .unwrap_or_else(|| "Selection no longer resolves".to_owned());
        let instance = self.state.dialogs.descend_hierarchy.instance_name.clone();
        let labels = self.state.workspace.occurrence_labels();
        let occurrence_path = if labels.is_empty() {
            format!("/{}/{}", self.state.workspace.active_view.cell, instance)
        } else {
            format!("/{}/{}", labels.join("/"), instance)
        };
        let choice = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .initial_focus(DialogInitialFocus::BodyControl)
            .show_with_initial_body_focus(ctx, |ui| {
                Some(descend_body(
                    ui,
                    &mut self.state.dialogs.descend_hierarchy,
                    &occurrence_path,
                    &target,
                ))
            });

        match choice {
            DialogChoice::Primary => {
                if let Err(error) = commit_descend_context(&mut self.state) {
                    self.state.dialogs.descend_hierarchy.validation_error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.descend_hierarchy.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn commit_descend_context(state: &mut AppState) -> Result<(), String> {
    let draft = state.dialogs.descend_hierarchy.clone();
    let authority = draft
        .authority
        .as_ref()
        .ok_or_else(|| "The hierarchy edit authority is missing.".to_owned())?;
    authority.validate(state, "Descend into hierarchy")?;
    let reference = draft
        .reference
        .clone()
        .ok_or_else(|| "The selected hierarchy master no longer resolves.".to_owned())?;
    let current = state
        .selected_hierarchy_master()
        .ok_or_else(|| "The selected hierarchy master no longer resolves.".to_owned())?;
    if current.0 != draft.instance_name || current.1 != reference {
        return Err(
            "The selected hierarchy target changed. Close and reopen Descend into hierarchy."
                .to_owned(),
        );
    }

    state.ui.schematic_visibility.hierarchy = match draft.edit_mode {
        HierarchyDescendEditMode::EditInPlace => match draft.parent_context {
            HierarchyParentContext::ShowOneLevel => SchematicHierarchyVisibility::ActiveAndParent,
            HierarchyParentContext::ShowFullHierarchy => {
                SchematicHierarchyVisibility::FullVisibleHierarchy
            }
            HierarchyParentContext::HideParent => SchematicHierarchyVisibility::ActiveOnly,
        },
        HierarchyDescendEditMode::OpenIsolated | HierarchyDescendEditMode::ReadOnlyReference => {
            SchematicHierarchyVisibility::ActiveOnly
        }
    };

    match draft.edit_mode {
        HierarchyDescendEditMode::EditInPlace => {
            state.descend_into_instance(Some(draft.instance_name), reference);
        }
        HierarchyDescendEditMode::OpenIsolated => {
            state.open_workspace_view(reference);
        }
        HierarchyDescendEditMode::ReadOnlyReference => {
            state.open_workspace_view(reference);
            state.workbench.hierarchy_reference_read_only = true;
        }
    }
    state.dialogs.descend_hierarchy.close();
    state.push_user_message(ConsoleMessage::info(
        "Hierarchy edit context opened; the parent working revision was preserved.",
    ));
    Ok(())
}

fn descend_body(
    ui: &mut Ui,
    draft: &mut crate::workbench::app::DescendHierarchyDialogState,
    occurrence_path: &str,
    target: &str,
) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let first = target_row(ui, occurrence_path, target);

    let edit_mode = setting_row(
        ui,
        "Edit mode",
        "Controls ownership and parent-context visibility.",
        "descend-hierarchy-edit-mode",
        edit_mode_label(draft.edit_mode),
        &[
            "Edit in place",
            "Open isolated cell view",
            "Read-only reference",
        ],
        true,
    );
    if let Some(index) = edit_mode.0 {
        draft.edit_mode = match index {
            1 => HierarchyDescendEditMode::OpenIsolated,
            2 => HierarchyDescendEditMode::ReadOnlyReference,
            _ => HierarchyDescendEditMode::EditInPlace,
        };
        draft.validation_error = None;
    }

    let parent_enabled = draft.edit_mode == HierarchyDescendEditMode::EditInPlace;
    let parent_context = setting_row(
        ui,
        "Parent context",
        if parent_enabled {
            "Dimmed parent connectivity remains available for cross-probing."
        } else {
            "Isolated and reference modes intentionally hide occurrence context."
        },
        "descend-hierarchy-parent-context",
        parent_context_label(draft.parent_context),
        &[
            "Show one level",
            "Show full hierarchy",
            "Hide parent context",
        ],
        parent_enabled,
    );
    if let Some(index) = parent_context.0 {
        draft.parent_context = match index {
            1 => HierarchyParentContext::ShowFullHierarchy,
            2 => HierarchyParentContext::HideParent,
            _ => HierarchyParentContext::ShowOneLevel,
        };
        draft.validation_error = None;
    }

    checkpoint_row(ui, draft.parent_dirty);
    if let Some(error) = draft.validation_error.as_deref() {
        validation_error(ui, error);
    }
    first
}

fn target_row(ui: &mut Ui, occurrence_path: &str, target: &str) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let output = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            if ui.available_width() < 520.0 {
                setting_copy(
                    ui,
                    "Target",
                    "Selected hierarchical instance and bound view.",
                );
                ui.add_space(7.0);
                target_value(ui, occurrence_path, target)
            } else {
                ui.horizontal_top(|ui| {
                    let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                    ui.allocate_ui_with_layout(
                        vec2(copy_width, t.metrics.ctl_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            setting_copy(
                                ui,
                                "Target",
                                "Selected hierarchical instance and bound view.",
                            )
                        },
                    );
                    ui.add_space(12.0);
                    target_value(ui, occurrence_path, target)
                })
                .inner
            }
        });
    paint_row_separator(ui, output.response.rect);
    output.inner
}

fn setting_row(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    id: &str,
    selected: &str,
    options: &[&str],
    enabled: bool,
) -> (Option<usize>, egui::Id) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let options = options
        .iter()
        .map(|option| (*option).to_owned())
        .collect::<Vec<_>>();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            if ui.available_width() < 520.0 {
                setting_copy(ui, title, detail);
                ui.add_space(7.0);
                let control_width = ui.available_width();
                ui.add_enabled_ui(enabled, |ui| {
                    select_mono_with_response(ui, id, title, selected, &options, control_width)
                })
                .inner
            } else {
                ui.horizontal_top(|ui| {
                    let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                    ui.allocate_ui_with_layout(
                        vec2(copy_width, t.metrics.ctl_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| setting_copy(ui, title, detail),
                    );
                    ui.add_space(12.0);
                    let control_width = ui.available_width();
                    ui.add_enabled_ui(enabled, |ui| {
                        select_mono_with_response(ui, id, title, selected, &options, control_width)
                    })
                    .inner
                })
                .inner
            }
        });
    paint_row_separator(ui, row.response.rect);
    (row.inner.picked, row.inner.response.id)
}

fn checkpoint_row(ui: &mut Ui, parent_dirty: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            if ui.available_width() < 520.0 {
                setting_copy(
                    ui,
                    "Unsaved parent changes",
                    "Preserved as a transactional working checkpoint.",
                );
                ui.add_space(7.0);
                checkpoint_value(ui, parent_dirty);
            } else {
                ui.horizontal_top(|ui| {
                    let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                    ui.allocate_ui_with_layout(
                        vec2(copy_width, t.metrics.ctl_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            setting_copy(
                                ui,
                                "Unsaved parent changes",
                                "Preserved as a transactional working checkpoint.",
                            )
                        },
                    );
                    ui.add_space(12.0);
                    checkpoint_value(ui, parent_dirty);
                });
            }
        });
}

fn validation_error(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(error)
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.err),
            );
        });
}

fn paint_row_separator(ui: &Ui, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn setting_copy(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(detail)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn target_value(ui: &mut Ui, occurrence_path: &str, target: &str) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new(occurrence_path)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
        );
        ui.label(
            egui::RichText::new(target)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .id
    })
    .inner
}

fn checkpoint_value(ui: &mut Ui, parent_dirty: bool) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(if parent_dirty {
            "working checkpoint preserved"
        } else {
            "safe to descend"
        })
        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
        .color(t.color.ok),
    );
}

const fn edit_mode_label(mode: HierarchyDescendEditMode) -> &'static str {
    match mode {
        HierarchyDescendEditMode::EditInPlace => "Edit in place",
        HierarchyDescendEditMode::OpenIsolated => "Open isolated cell view",
        HierarchyDescendEditMode::ReadOnlyReference => "Read-only reference",
    }
}

const fn parent_context_label(context: HierarchyParentContext) -> &'static str {
    match context {
        HierarchyParentContext::ShowOneLevel => "Show one level",
        HierarchyParentContext::ShowFullHierarchy => "Show full hierarchy",
        HierarchyParentContext::HideParent => "Hide parent context",
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

    fn hierarchy_state() -> (AppState, CellViewRef, CellViewRef) {
        let mut state = AppState::default();
        let parent = state.workspace.active_view.clone();
        let child = CellViewRef::new("hierarchy_test", "afe_core", "schematic");
        let mut cell = Cell::new("afe_core");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut library = Library::new("hierarchy_test");
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let mut instance = Component::new(91, ComponentType::CellInstance, Point::new(100, 100))
            .with_library_cell(LibraryCellInstance::new(
                "hierarchy_test",
                "afe_core",
                "schematic",
            ));
        instance.name = "XAFE".to_owned();
        state.schematic.components.push(instance);
        state.schematic.selection.select_only_component(91);
        (state, parent, child)
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
            parent_context_label(HierarchyParentContext::ShowFullHierarchy),
            "Show full hierarchy"
        );
    }

    #[test]
    fn menu_transaction_opens_without_navigating_and_rejects_selection_drift() {
        let (mut state, parent, _) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(&mut state));
        assert!(state.dialogs.descend_hierarchy.open);
        assert_eq!(state.workspace.active_view, parent);

        state.schematic.selection.clear();
        let error = commit_descend_context(&mut state).expect_err("selection drift must block");
        assert!(error.contains("selected-object set changed"));
        assert_eq!(state.workspace.active_view, parent);
        assert!(state.dialogs.descend_hierarchy.open);
    }

    #[test]
    fn direct_gesture_descends_in_place_without_opening_the_menu_dialog() {
        let (mut state, _, child) = hierarchy_state();
        state.ui.schematic_visibility.hierarchy =
            SchematicHierarchyVisibility::FullVisibleHierarchy;
        state.open_selected_instance_master();
        assert_eq!(state.workspace.active_view, child);
        assert_eq!(state.workspace.occurrence_labels().last().unwrap(), "XAFE");
        assert_eq!(
            state.ui.schematic_visibility.hierarchy,
            SchematicHierarchyVisibility::FullVisibleHierarchy
        );
        assert!(!state.dialogs.descend_hierarchy.open);
    }

    #[test]
    fn disabled_edit_in_place_opens_direct_gestures_as_isolated_views() {
        let (mut state, _, child) = hierarchy_state();
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::HierarchicalEditInPlace, false);

        state.open_selected_instance_master();

        assert_eq!(state.workspace.active_view, child);
        assert_eq!(state.workspace.hierarchy_stack, vec![child]);
        assert_eq!(state.workspace.occurrence_labels(), ["afe_core"]);
        assert!(!state.dialogs.descend_hierarchy.open);
    }

    #[test]
    fn preference_sets_the_workflow_default_without_removing_explicit_choices() {
        let (mut state, _, child) = hierarchy_state();
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::HierarchicalEditInPlace, false);
        assert!(open_descend_hierarchy_dialog(&mut state));
        assert_eq!(
            state.dialogs.descend_hierarchy.edit_mode,
            HierarchyDescendEditMode::OpenIsolated
        );

        state.dialogs.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditInPlace;
        state.dialogs.descend_hierarchy.parent_context = HierarchyParentContext::ShowFullHierarchy;
        commit_descend_context(&mut state).expect("explicit edit-in-place choice remains valid");

        assert_eq!(state.workspace.active_view, child);
        assert_eq!(state.workspace.occurrence_labels().last().unwrap(), "XAFE");
        assert_eq!(
            state.ui.schematic_visibility.hierarchy,
            SchematicHierarchyVisibility::FullVisibleHierarchy
        );
    }

    #[test]
    fn read_only_reference_is_session_scoped_and_never_mutates_library_ownership() {
        let (mut state, parent, child) = hierarchy_state();
        assert!(open_descend_hierarchy_dialog(&mut state));
        state.dialogs.descend_hierarchy.edit_mode = HierarchyDescendEditMode::ReadOnlyReference;
        commit_descend_context(&mut state).expect("valid reference context");

        assert_eq!(state.workspace.active_view, child);
        assert!(state.workbench.hierarchy_reference_read_only);
        assert!(state.active_view_read_only());
        assert!(!state.schematic.read_only);
        assert!(
            !state
                .library_manager
                .get_library("hierarchy_test")
                .expect("library")
                .read_only
        );

        state.open_workspace_view(parent);
        assert!(!state.workbench.hierarchy_reference_read_only);
    }
}
