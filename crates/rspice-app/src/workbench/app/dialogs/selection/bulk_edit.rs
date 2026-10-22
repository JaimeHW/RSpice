//! Filtered selection and atomic schematic bulk editing.
//!
//! Queries may inspect the active hierarchy or complete project, but edits are
//! deliberately limited to the active schematic owner. A commit preflights
//! every selected target, validates its frozen document authority, and
//! publishes all component changes through one schematic undo boundary.

use std::collections::{BTreeSet, HashSet};

use egui::{ComboBox, Context, Grid, RichText, ScrollArea, TextEdit, Ui, Vec2};

pub(crate) use crate::state::bulk_edit::{SelectionBulkProperty, SelectionBulkUnsetBehavior};
use crate::state::bulk_edit::{
    apply_bulk_edit, mutate_component, parameter_key_from_filter, validate_bulk_value,
};
use crate::state::{
    Component, ComponentDisplayMode, ComponentType, SchematicSnapshot, SchematicState,
    parse_replacement_parameters_strict,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::dialogs::review_primitives::{DIALOG_BODY_INSET, band_header};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::{SelectionBulkFilter, SelectionBulkHierarchyScope, SelectionBulkObjectKind};

use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const TITLE: &str = "Bulk edit";
const PRIMARY: &str = "Apply";

const BULK_PROPERTIES: [SelectionBulkProperty; 5] = [
    SelectionBulkProperty::ModelSection,
    SelectionBulkProperty::Temperature,
    SelectionBulkProperty::Tolerance,
    SelectionBulkProperty::Display,
    SelectionBulkProperty::ParameterOverride,
];

const fn bulk_property_label(value: SelectionBulkProperty) -> &'static str {
    match value {
        SelectionBulkProperty::ModelSection => "Model section",
        SelectionBulkProperty::Temperature => "Temperature",
        SelectionBulkProperty::Tolerance => "Tolerance",
        SelectionBulkProperty::Display => "Display",
        SelectionBulkProperty::ParameterOverride => "Parameter override",
    }
}

const BULK_UNSET_BEHAVIORS: [SelectionBulkUnsetBehavior; 3] = [
    SelectionBulkUnsetBehavior::LeaveUnchanged,
    SelectionBulkUnsetBehavior::SetExplicitValue,
    SelectionBulkUnsetBehavior::RestoreInheritedValue,
];

const fn bulk_unset_label(value: SelectionBulkUnsetBehavior) -> &'static str {
    match value {
        SelectionBulkUnsetBehavior::LeaveUnchanged => "Leave unchanged",
        SelectionBulkUnsetBehavior::SetExplicitValue => "Set explicit value",
        SelectionBulkUnsetBehavior::RestoreInheritedValue => "Restore inherited value",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SelectionBulkRowKind {
    Instance,
    Port,
    Pin,
    Wire,
    Bus,
    BusTap,
    Junction,
    NetLabel,
    DesignNote,
    DocumentationShape,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SelectionBulkRowKey {
    view_key: String,
    kind: SelectionBulkRowKind,
    object_id: u64,
    member: u32,
}

#[derive(Debug, Clone)]
struct SelectionBulkRow {
    key: SelectionBulkRowKey,
    object: String,
    path: String,
    value: String,
    search_text: String,
    lock: Option<String>,
}

impl SelectionBulkRow {
    fn editable(&self) -> bool {
        self.lock.is_none()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SelectionBulkEditAuthority {
    active: SchematicEditAuthority,
    active_key: String,
    design_management_revision: u64,
    project_buffers: Vec<(String, SchematicSnapshot)>,
}

impl SelectionBulkEditAuthority {
    fn capture(state: &AppState) -> Self {
        let active_key = state
            .model
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let mut project_buffers = state
            .model
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active_key))
            .map(|(key, schematic)| {
                (
                    key.clone(),
                    SchematicSnapshot::capture(schematic.document()),
                )
            })
            .collect::<Vec<_>>();
        project_buffers.sort_by(|(left, _), (right, _)| left.cmp(right));
        Self {
            active: SchematicEditAuthority::capture(state),
            active_key,
            design_management_revision: state.model.workspace.content.design_management.revision(),
            project_buffers,
        }
    }

    fn stale_reason(&self, state: &AppState) -> Option<String> {
        let reopen = |reason: &str| format!("{reason}. Close Bulk edit and open it again.");
        let current = crate::workbench::app::schematic_editor_request_source(state);
        if self.active.source.design_epoch != current.design_epoch {
            return Some(reopen("The design document changed"));
        }
        if self.active.source.document_epoch != current.document_epoch {
            return Some(reopen("The active schematic buffer changed"));
        }
        if self.active.source.topology_version != current.topology_version
            || !self
                .active
                .snapshot
                .is_equal_document(state.model.schematic.document())
        {
            return Some(reopen("The active schematic changed"));
        }
        if self.active.view_path != state.model.workspace.content.active_view.display_path()
            || self.active_key
                != state
                    .model
                    .workspace
                    .content
                    .active_schematic_reference()
                    .key()
        {
            return Some(reopen("The active cell/view changed"));
        }
        if !self.active.matches_source(&current) {
            return Some(reopen("The schematic source or editing scope changed"));
        }
        if self.active.grid_size != state.model.schematic.document().grid_size
            || self.active.document_policy != state.model.schematic.document().document_policy
        {
            return Some(reopen("The schematic grid or editing policy changed"));
        }
        if self.active.selection != state.model.schematic.session.editor.selection {
            return Some(reopen("The selected-object set changed"));
        }
        if self.design_management_revision
            != state.model.workspace.content.design_management.revision()
        {
            return Some(reopen("The sheet ownership catalog changed"));
        }
        let live_external = state
            .model
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&self.active_key))
            .collect::<Vec<_>>();
        if live_external.len() != self.project_buffers.len() {
            return Some(reopen("The project schematic set changed"));
        }
        for (key, expected) in &self.project_buffers {
            let Some(live) = state.model.workspace.content.schematic_buffers.get(key) else {
                return Some(reopen("A project schematic was closed"));
            };
            if !expected.is_equal_document(live.document()) {
                return Some(reopen("A project schematic changed"));
            }
        }
        None
    }

    fn validate_commit(&self, state: &AppState) -> Result<(), String> {
        self.active.validate(state, TITLE)?;
        if state.workbench.safe_mode.project_read_only() {
            return Err("Safe mode has opened the project read-only.".to_owned());
        }
        if let Some(reason) = self.stale_reason(state) {
            return Err(reason);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SelectionBulkEditDialogState {
    pub(crate) open: bool,
    pub(crate) filter: SelectionBulkFilter,
    selected: BTreeSet<SelectionBulkRowKey>,
    pub(crate) property: SelectionBulkProperty,
    pub(crate) new_value: String,
    pub(crate) unset_behavior: SelectionBulkUnsetBehavior,
    /// The sections a model section can be set to, from the libraries the
    /// matched instances are bound to. Resolved when the filter changes.
    bound_sections: Vec<String>,
    authority: Option<SelectionBulkEditAuthority>,
    pub(crate) error: Option<String>,
    pub(crate) body_scroll_offset: f32,
    save_filter_open: bool,
    save_filter_name: String,
    selected_saved_filter: Option<String>,
}

impl Default for SelectionBulkEditDialogState {
    fn default() -> Self {
        Self {
            open: false,
            filter: SelectionBulkFilter::default(),
            selected: BTreeSet::new(),
            property: SelectionBulkProperty::default(),
            new_value: String::new(),
            unset_behavior: SelectionBulkUnsetBehavior::SetExplicitValue,
            bound_sections: Vec::new(),
            authority: None,
            error: None,
            body_scroll_offset: 0.0,
            save_filter_open: false,
            save_filter_name: String::new(),
            selected_saved_filter: None,
        }
    }
}

impl SelectionBulkEditDialogState {
    fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionBulkBodyAction {
    None,
    FilterChanged,
    SaveFilter,
    LoadFilter,
    DeleteFilter,
}

pub(crate) fn open_selection_bulk_edit_dialog(state: &mut AppState) {
    let filter = state.ui.schematic_bulk_edit_filters.active.clone();
    let authority = SelectionBulkEditAuthority::capture(state);
    let property = SelectionBulkProperty::default();
    let rows = build_rows(state, &filter, property);
    let selected = rows
        .iter()
        .filter(|row| row.editable())
        .map(|row| row.key.clone())
        .collect();
    let bound_sections = bound_library_sections(state, &rows);
    // Nothing is proposed: the value is the user's to choose, and Apply
    // waits for it.
    state.dialogs.selection_bulk_edit = SelectionBulkEditDialogState {
        open: true,
        filter,
        selected,
        property,
        new_value: String::new(),
        unset_behavior: SelectionBulkUnsetBehavior::SetExplicitValue,
        bound_sections,
        authority: Some(authority),
        error: None,
        body_scroll_offset: 0.0,
        save_filter_open: false,
        save_filter_name: String::new(),
        selected_saved_filter: None,
    };
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_selection_bulk_edit_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.selection_bulk_edit.open {
            return;
        }
        let filter = self.state.dialogs.selection_bulk_edit.filter.clone();
        let property = self.state.dialogs.selection_bulk_edit.property;
        let rows = build_rows(&self.state, &filter, property);
        let impact = preview_impact(
            &self.state.model.schematic,
            &rows,
            &self.state.dialogs.selection_bulk_edit.selected,
            property,
            &self.state.dialogs.selection_bulk_edit.new_value,
            self.state.dialogs.selection_bulk_edit.unset_behavior,
            &self
                .state
                .dialogs
                .selection_bulk_edit
                .filter
                .current_property,
        );
        let stale = self
            .state
            .dialogs
            .selection_bulk_edit
            .authority
            .as_ref()
            .and_then(|authority| authority.stale_reason(&self.state));
        let write_allowed = !self.state.schematic_edit_read_only()
            && !self.state.workbench.safe_mode.project_read_only();
        let primary_enabled =
            write_allowed && stale.is_none() && impact.error.is_none() && impact.changed > 0;
        let mut body_scroll_offset = self.state.dialogs.selection_bulk_edit.body_scroll_offset;
        // A value not yet chosen is not a failed edit: the footer says what
        // Apply is waiting for, and no "nothing was changed" banner opens
        // over a dialog nobody has touched.
        let missing_value = missing_value_reason(&self.state.dialogs.selection_bulk_edit);
        let error = self
            .state
            .dialogs
            .selection_bulk_edit
            .error
            .as_deref()
            .or(stale.as_deref())
            .or(impact.error.as_deref().filter(|_| missing_value.is_none()))
            .map(str::to_owned);
        let mut dialog = Dialog::new(TITLE, PRIMARY)
            .description("Change one property on every object the filter matches.")
            .size(DialogSize::SimulationWorkflow)
            .initial_height(640.0)
            .flush_body()
            // The split's bands and rule reach the frame's right edge; a
            // withheld scroll gutter left bare surface beside them.
            .overlay_scrollbar()
            .ghost("Cancel")
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if let Some(reason) = missing_value {
            dialog = dialog.hint(reason);
        }
        if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Read only",
                "This schematic cannot be edited. The filter still works.",
            );
        } else if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Nothing was changed",
                error,
            );
        }

        let saved = self
            .state
            .ui
            .schematic_bulk_edit_filters
            .saved()
            .iter()
            .map(|entry| entry.name.clone())
            .collect::<Vec<_>>();
        let analyses = self.state.model.sim_setup.enabled_analysis_instance_count();
        let hierarchy_path = active_hierarchy_path(&self.state);
        let mut action = SelectionBulkBodyAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = selection_bulk_body(
                ui,
                &mut self.state.dialogs.selection_bulk_edit,
                &rows,
                &impact,
                analyses,
                &saved,
                &hierarchy_path,
            );
            None
        });
        self.state.dialogs.selection_bulk_edit.body_scroll_offset = body_scroll_offset;
        self.handle_selection_bulk_body_action(action);

        match choice {
            DialogChoice::Primary => {
                if let Err(error) = self.commit_selection_bulk_edit() {
                    self.state.dialogs.selection_bulk_edit.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.selection_bulk_edit.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_selection_bulk_body_action(&mut self, action: SelectionBulkBodyAction) {
        match action {
            SelectionBulkBodyAction::None => {}
            SelectionBulkBodyAction::FilterChanged => {
                let filter = self.state.dialogs.selection_bulk_edit.filter.clone();
                self.state.ui.schematic_bulk_edit_filters.active = filter.clone();
                let property = self.state.dialogs.selection_bulk_edit.property;
                let rows = build_rows(&self.state, &filter, property);
                self.state.dialogs.selection_bulk_edit.bound_sections =
                    bound_library_sections(&self.state, &rows);
                self.state.dialogs.selection_bulk_edit.selected = rows
                    .into_iter()
                    .filter(SelectionBulkRow::editable)
                    .map(|row| row.key)
                    .collect();
                self.state.dialogs.selection_bulk_edit.error = None;
                self.state.dialogs.selection_bulk_edit.authority =
                    Some(SelectionBulkEditAuthority::capture(&self.state));
            }
            SelectionBulkBodyAction::SaveFilter => {
                let name = self
                    .state
                    .dialogs
                    .selection_bulk_edit
                    .save_filter_name
                    .clone();
                self.state.ui.schematic_bulk_edit_filters.active =
                    self.state.dialogs.selection_bulk_edit.filter.clone();
                match self.state.ui.schematic_bulk_edit_filters.save_active(&name) {
                    Ok(()) => {
                        self.state.dialogs.selection_bulk_edit.selected_saved_filter =
                            Some(name.trim().to_owned());
                        self.state
                            .dialogs
                            .selection_bulk_edit
                            .save_filter_name
                            .clear();
                        self.state.dialogs.selection_bulk_edit.save_filter_open = false;
                        self.state.dialogs.selection_bulk_edit.error = None;
                    }
                    Err(error) => self.state.dialogs.selection_bulk_edit.error = Some(error),
                }
            }
            SelectionBulkBodyAction::LoadFilter => {
                let name = self
                    .state
                    .dialogs
                    .selection_bulk_edit
                    .selected_saved_filter
                    .clone();
                if let Some(name) = name
                    && self.state.ui.schematic_bulk_edit_filters.load(&name)
                {
                    let filter = self.state.ui.schematic_bulk_edit_filters.active.clone();
                    self.state.dialogs.selection_bulk_edit.filter = filter.clone();
                    let property = self.state.dialogs.selection_bulk_edit.property;
                    self.state.dialogs.selection_bulk_edit.selected =
                        build_rows(&self.state, &filter, property)
                            .into_iter()
                            .filter(SelectionBulkRow::editable)
                            .map(|row| row.key)
                            .collect();
                    self.state.dialogs.selection_bulk_edit.authority =
                        Some(SelectionBulkEditAuthority::capture(&self.state));
                    self.state.dialogs.selection_bulk_edit.error = None;
                }
            }
            SelectionBulkBodyAction::DeleteFilter => {
                let name = self
                    .state
                    .dialogs
                    .selection_bulk_edit
                    .selected_saved_filter
                    .clone();
                if let Some(name) = name
                    && self.state.ui.schematic_bulk_edit_filters.remove(&name)
                {
                    self.state.dialogs.selection_bulk_edit.selected_saved_filter = None;
                    self.state.dialogs.selection_bulk_edit.error = None;
                }
            }
        }
    }

    fn commit_selection_bulk_edit(&mut self) -> Result<(), String> {
        let dialog = &self.state.dialogs.selection_bulk_edit;
        let authority = dialog
            .authority
            .as_ref()
            .ok_or_else(|| "Bulk-edit document authority is missing.".to_owned())?;
        authority.validate_commit(&self.state)?;
        let rows = build_rows(&self.state, &dialog.filter, dialog.property);
        let selected_active_ids = rows
            .iter()
            .filter(|row| row.editable() && dialog.selected.contains(&row.key))
            .filter(|row| row.key.view_key.eq_ignore_ascii_case(&authority.active_key))
            .filter_map(|row| {
                matches!(
                    row.key.kind,
                    SelectionBulkRowKind::Instance | SelectionBulkRowKind::Port
                )
                .then_some(row.key.object_id)
            })
            .collect::<BTreeSet<_>>();
        let receipt = apply_bulk_edit(
            &mut self.state.model.schematic,
            &selected_active_ids,
            dialog.property,
            &dialog.new_value,
            dialog.unset_behavior,
            Some(&dialog.filter.current_property),
        )?;
        if receipt.changed == 0 {
            return Err("The selected objects already satisfy the requested value.".to_owned());
        }
        self.state.sync_active_schematic_to_workspace();
        let analyses = self.state.model.sim_setup.enabled_analysis_instance_count();
        self.state.prompt(format!(
            "Bulk edit changed {} object{} as one undo step; {} enabled analys{} affected.",
            receipt.changed,
            if receipt.changed == 1 { "" } else { "s" },
            analyses,
            if analyses == 1 { "is" } else { "es" },
        ));
        self.state.dialogs.selection_bulk_edit.close();
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct SelectionBulkImpact {
    changed: usize,
    locked_or_excluded: usize,
    error: Option<String>,
}

fn preview_impact(
    schematic: &SchematicState,
    rows: &[SelectionBulkRow],
    selected: &BTreeSet<SelectionBulkRowKey>,
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: &str,
) -> SelectionBulkImpact {
    let editable_ids = rows
        .iter()
        .filter(|row| row.editable() && selected.contains(&row.key))
        .filter_map(|row| {
            matches!(
                row.key.kind,
                SelectionBulkRowKind::Instance | SelectionBulkRowKind::Port
            )
            .then_some(row.key.object_id)
        })
        .collect::<BTreeSet<_>>();
    let locked_or_excluded = rows.iter().filter(|row| !row.editable()).count()
        + rows
            .iter()
            .filter(|row| row.editable() && !selected.contains(&row.key))
            .count();
    let preview = || -> Result<usize, String> {
        let normalized = validate_bulk_value(property, value, unset, Some(current_property))?;
        let parameter_key = if property == SelectionBulkProperty::ParameterOverride {
            parameter_key_from_filter(value, None).ok()
        } else {
            None
        };
        let mut found = HashSet::new();
        let mut changed = 0;
        for component in &schematic.document().components {
            if !editable_ids.contains(&component.id) {
                continue;
            }
            found.insert(component.id);
            let mut candidate = component.clone();
            changed += usize::from(mutate_component(
                &mut candidate,
                property,
                &normalized,
                unset,
                parameter_key.as_deref(),
            )?);
        }
        if found.len() != editable_ids.len() {
            return Err("At least one selected component is no longer present.".to_owned());
        }
        Ok(changed)
    };
    match preview() {
        Ok(changed) => SelectionBulkImpact {
            changed,
            locked_or_excluded,
            error: None,
        },
        Err(error) => SelectionBulkImpact {
            changed: 0,
            locked_or_excluded,
            error: Some(error),
        },
    }
}

fn selection_bulk_body(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    rows: &[SelectionBulkRow],
    impact: &SelectionBulkImpact,
    affected_analyses: usize,
    saved_filters: &[String],
    hierarchy_path: &str,
) -> SelectionBulkBodyAction {
    let mut action = SelectionBulkBodyAction::None;
    let tokens = Tokens::get(ui.ctx());
    // Nothing in this body may widen it. A value longer than its control is
    // elided there: laid out whole, one such line grew the body past the
    // surface's right edge, and every row under it took the new width.
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
    egui::Frame::NONE
        .fill(tokens.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(BODY_INSET_I8, 8))
        .show(ui, |ui| {
            let band_w = ui.available_width();
            ui.set_width(band_w);
            // The button keeps its own width at the band's end; the field
            // takes the rest.
            let row = Vec2::new(band_w, tokens.metrics.ctl_h);
            ui.allocate_ui_with_layout(
                row,
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if Button::new("Save filter\u{2026}").show(ui).clicked() {
                        dialog.save_filter_open = !dialog.save_filter_open;
                    }
                    let width = ui.available_width();
                    let response = ui.add_sized(
                        [width, tokens.metrics.ctl_h],
                        TextEdit::singleline(&mut dialog.filter.query)
                            .hint_text(
                                "Find instance, property, model, net, expression or path\u{2026}",
                            )
                            .margin(egui::Margin {
                                left: 29,
                                right: 8,
                                top: 5,
                                bottom: 5,
                            }),
                    );
                    WorkbenchIcon::Search.paint(
                        ui.painter(),
                        egui::Rect::from_center_size(
                            egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                            Vec2::splat(16.0),
                        ),
                        tokens.color.text_faint,
                    );
                    if response.changed() {
                        action = SelectionBulkBodyAction::FilterChanged;
                    }
                },
            );
            if dialog.save_filter_open {
                saved_filter_controls(ui, dialog, saved_filters, &tokens, &mut action);
            }
        });
    ui.separator();

    let filter_w = (ui.available_width() * FILTER_PANE_SHARE).round();
    let split = egui::Frame::NONE
        .fill(tokens.color.bg_inset)
        .show(ui, |ui| {
            let top = ui.cursor().min;
            let full_w = ui.available_width();
            let filter = egui::Rect::from_min_size(top, Vec2::new(filter_w, f32::INFINITY));
            let edit = egui::Rect::from_min_size(
                top + Vec2::new(filter_w, 0.0),
                Vec2::new(full_w - filter_w, f32::INFINITY),
            );
            let filter_bottom = pane(ui, "selection-bulk-filter-pane", filter, |ui| {
                selection_filter_panel(ui, dialog, rows, hierarchy_path, &mut action);
            });
            let edit_bottom = pane(ui, "selection-bulk-edit-pane", edit, |ui| {
                bulk_edit_panel(ui, dialog, impact, affected_analyses, &mut action);
            });
            ui.allocate_rect(
                egui::Rect::from_min_max(
                    top,
                    egui::pos2(top.x + full_w, filter_bottom.max(edit_bottom)),
                ),
                egui::Sense::hover(),
            );
        });
    let split_rect = split.response.rect;
    ui.painter().vline(
        split_rect.left() + filter_w,
        split_rect.y_range(),
        egui::Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.painter().hline(
        split_rect.x_range(),
        split_rect.bottom(),
        egui::Stroke::new(1.0, tokens.color.border),
    );
    action
}

/// [`DIALOG_BODY_INSET`] as a frame margin.
const BODY_INSET_I8: i8 = DIALOG_BODY_INSET as i8;
/// What the filter and its results take of the body's width. The table has
/// four columns to fit; the edit pane beside it has two controls a row.
const FILTER_PANE_SHARE: f32 = 0.58;

/// One pane of the split, laid top down in `rect` and clipped to it, so the
/// pane beside it starts where the split says whatever this one holds.
/// Returns the pane's bottom edge.
fn pane(ui: &mut Ui, id_salt: &str, rect: egui::Rect, add: impl FnOnce(&mut Ui)) -> f32 {
    let mut pane = ui.new_child(
        egui::UiBuilder::new()
            .id_salt(id_salt)
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    pane.set_clip_rect(rect.intersect(ui.clip_rect()));
    pane.set_width(rect.width());
    pane.spacing_mut().item_spacing.x = 8.0;
    add(&mut pane);
    pane.min_rect().bottom()
}

/// A pane's content, on the body's left edge and as far in from its right.
fn pane_content(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(BODY_INSET_I8, 10))
        .show(ui, add);
}

fn saved_filter_controls(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    saved_filters: &[String],
    tokens: &Tokens,
    action: &mut SelectionBulkBodyAction,
) {
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label("Filter name");
        ui.add_sized(
            [220.0, tokens.metrics.ctl_h],
            TextEdit::singleline(&mut dialog.save_filter_name),
        );
        if Button::new("Save").accent().show(ui).clicked() {
            *action = SelectionBulkBodyAction::SaveFilter;
        }
        if !saved_filters.is_empty() {
            ComboBox::from_id_salt("selection-bulk-saved-filter")
                .selected_text(
                    dialog
                        .selected_saved_filter
                        .as_deref()
                        .unwrap_or("Saved filters"),
                )
                .show_ui(ui, |ui| {
                    for name in saved_filters {
                        ui.selectable_value(
                            &mut dialog.selected_saved_filter,
                            Some(name.clone()),
                            name,
                        );
                    }
                });
            let has_saved = dialog.selected_saved_filter.is_some();
            if ui
                .add_enabled(has_saved, egui::Button::new("Load"))
                .clicked()
            {
                *action = SelectionBulkBodyAction::LoadFilter;
            }
            if ui
                .add_enabled(has_saved, egui::Button::new("Delete"))
                .clicked()
            {
                *action = SelectionBulkBodyAction::DeleteFilter;
            }
        }
    });
}

fn selection_filter_fields(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    hierarchy_path: &str,
    action: &mut SelectionBulkBodyAction,
) {
    ui.spacing_mut().item_spacing.x = 12.0;
    ui.columns(2, |columns| {
        columns[0].label("Object kinds");
        ComboBox::from_id_salt("selection-bulk-object-kind")
            .width(columns[0].available_width())
            .selected_text(dialog.filter.object_kind.label())
            .show_ui(&mut columns[0], |ui| {
                for kind in SelectionBulkObjectKind::ALL {
                    if ui
                        .selectable_value(&mut dialog.filter.object_kind, kind, kind.label())
                        .changed()
                    {
                        *action = SelectionBulkBodyAction::FilterChanged;
                    }
                }
            });
        columns[1].label("Hierarchy scope");
        ComboBox::from_id_salt("selection-bulk-hierarchy-scope")
            .width(columns[1].available_width())
            .selected_text(match dialog.filter.hierarchy_scope {
                SelectionBulkHierarchyScope::ActiveHierarchyPath => hierarchy_path,
                scope => scope.label(),
            })
            .show_ui(&mut columns[1], |ui| {
                for scope in SelectionBulkHierarchyScope::ALL {
                    let label = if scope == SelectionBulkHierarchyScope::ActiveHierarchyPath {
                        hierarchy_path
                    } else {
                        scope.label()
                    };
                    if ui
                        .selectable_value(&mut dialog.filter.hierarchy_scope, scope, label)
                        .changed()
                    {
                        *action = SelectionBulkBodyAction::FilterChanged;
                    }
                }
            });
    });
    ui.add_space(9.0);
    ui.columns(2, |columns| {
        columns[0].label("Model / cell");
        if columns[0]
            .add(
                TextEdit::singleline(&mut dialog.filter.model_cell)
                    .desired_width(columns[0].available_width())
                    .hint_text("Any"),
            )
            .changed()
        {
            *action = SelectionBulkBodyAction::FilterChanged;
        }
        columns[1].label("Current property");
        if columns[1]
            .add(
                TextEdit::singleline(&mut dialog.filter.current_property)
                    .desired_width(columns[1].available_width())
                    .hint_text("name = value"),
            )
            .changed()
        {
            *action = SelectionBulkBodyAction::FilterChanged;
        }
    });
}

fn selection_filter_panel(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    rows: &[SelectionBulkRow],
    hierarchy_path: &str,
    action: &mut SelectionBulkBodyAction,
) {
    band_header(
        ui,
        "Selection filter",
        Some(&format!("{} matched", rows.len())),
    );
    pane_content(ui, |ui| {
        selection_filter_fields(ui, dialog, hierarchy_path, action);
        ui.add_space(12.0);
        selection_results(ui, dialog, rows);
    });
}

fn selection_results(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    rows: &[SelectionBulkRow],
) {
    // A check box is never narrower than a control is tall, so its column
    // is as wide as the box the rows actually lay out.
    let check_w = RESULT_CHECK_W.max(ui.spacing().interact_size.y);
    let [object_w, path_w, value_w, lock_w] = result_columns(
        ui.available_width(),
        check_w + ui.spacing().scroll.allocated_width(),
    );
    Grid::new("selection-bulk-result-header")
        .num_columns(5)
        .striped(false)
        .min_col_width(0.0)
        .spacing(Vec2::new(RESULT_COLUMN_GAP, 4.0))
        .show(ui, |ui| {
            ui.allocate_space(Vec2::new(check_w, 16.0));
            for (label, width) in [
                ("Object", object_w),
                ("Path", path_w),
                ("Value", value_w),
                ("Locked", lock_w),
            ] {
                result_cell(ui, width, RichText::new(label).strong());
            }
            ui.end_row();
        });
    // A short result is as tall as its rows; a long one scrolls in place.
    let rows_h = (rows.len().max(1) as f32 * 24.0).min(RESULTS_MAX_H);
    ScrollArea::vertical()
        .id_salt("selection-bulk-result-scroll")
        .max_height(RESULTS_MAX_H)
        .min_scrolled_height(rows_h)
        .auto_shrink([false, true])
        .show_rows(ui, 24.0, rows.len(), |ui, range| {
            Grid::new("selection-bulk-result-grid")
                .num_columns(5)
                .striped(true)
                .min_col_width(0.0)
                .spacing(Vec2::new(RESULT_COLUMN_GAP, 4.0))
                .show(ui, |ui| {
                    for row in &rows[range] {
                        let mut selected = dialog.selected.contains(&row.key);
                        // In a cell as wide as the header's, so the columns
                        // after it start where their headings do.
                        let response = ui
                            .allocate_ui_with_layout(
                                Vec2::new(check_w, RESULT_ROW_H),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.set_width(check_w);
                                    ui.add_enabled(
                                        row.editable(),
                                        egui::Checkbox::without_text(&mut selected),
                                    )
                                },
                            )
                            .inner
                            .on_disabled_hover_text(row.lock.as_deref().unwrap_or("Not editable"));
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Checkbox,
                                row.editable(),
                                selected,
                                format!("Select {}", row.object),
                            )
                        });
                        if response.changed() {
                            if selected {
                                dialog.selected.insert(row.key.clone());
                            } else {
                                dialog.selected.remove(&row.key);
                            }
                        }
                        result_cell(ui, object_w, row.object.as_str()).on_hover_text(&row.object);
                        result_cell(ui, path_w, RichText::new(&row.path).monospace())
                            .on_hover_text(&row.path);
                        result_cell(ui, value_w, row.value.as_str()).on_hover_text(&row.value);
                        // A cell only when something is in the way; an
                        // editable row says nothing, which is the whole
                        // column's news.
                        let lock = row.lock.as_deref().unwrap_or("");
                        let cell = result_cell(ui, lock_w, lock);
                        if !lock.is_empty() {
                            cell.on_hover_text(lock);
                        }
                        ui.end_row();
                    }
                });
        });
}

/// The narrowest a row's select box is, and the air between two result
/// columns.
const RESULT_CHECK_W: f32 = 20.0;
const RESULT_COLUMN_GAP: f32 = 6.0;
const RESULT_ROW_H: f32 = 20.0;
/// The tallest the result list grows before it scrolls.
const RESULTS_MAX_H: f32 = 300.0;

/// Object, Path, Value and Locked across whatever width this pane has.
///
/// These were four fixed widths summing to more than half the dialog, so the
/// Locked column painted over the impact panel beside it at every viewport
/// the dialog actually opens at. A path is the long one; a lock is a short
/// reason, which a column narrower than it would elide on every locked row.
/// `fixed` is what the select box and the list's scroll bar take out of the
/// pane: counted short, the rows were wider than the list, which grew to
/// hold them.
fn result_columns(pane: f32, fixed: f32) -> [f32; 4] {
    let text = (pane - fixed - RESULT_COLUMN_GAP * 4.0).max(140.0);
    let [object, path, value] = [text * 0.22, text * 0.30, text * 0.18];
    [object, path, value, text - object - path - value]
}

/// One result cell: its text from the cell's left edge, elided at its right.
fn result_cell(ui: &mut Ui, width: f32, text: impl Into<egui::WidgetText>) -> egui::Response {
    ui.allocate_ui_with_layout(
        Vec2::new(width, RESULT_ROW_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_width(width);
            ui.add(egui::Label::new(text).truncate())
        },
    )
    .inner
}

fn bulk_edit_panel(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    impact: &SelectionBulkImpact,
    affected_analyses: usize,
    action: &mut SelectionBulkBodyAction,
) {
    band_header(ui, "Bulk edit and impact", None);
    pane_content(ui, |ui| {
        bulk_edit_fields(ui, dialog, impact, affected_analyses, action);
    });
}

fn bulk_edit_fields(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    impact: &SelectionBulkImpact,
    affected_analyses: usize,
    action: &mut SelectionBulkBodyAction,
) {
    ui.spacing_mut().item_spacing.x = 12.0;
    ui.columns(2, |columns| {
        columns[0].label("Property");
        let before = dialog.property;
        ComboBox::from_id_salt("selection-bulk-property")
            .width(columns[0].available_width())
            .selected_text(bulk_property_label(dialog.property))
            .show_ui(&mut columns[0], |ui| {
                for property in BULK_PROPERTIES {
                    ui.selectable_value(
                        &mut dialog.property,
                        property,
                        bulk_property_label(property),
                    );
                }
            });
        if before != dialog.property {
            dialog.new_value.clear();
            *action = SelectionBulkBodyAction::FilterChanged;
        }
        columns[1].label("New value");
        if new_value_control(&mut columns[1], dialog) {
            dialog.error = None;
        }
    });
    ui.add_space(9.0);
    ui.label("Unset behavior");
    ComboBox::from_id_salt("selection-bulk-unset")
        .width(ui.available_width())
        .selected_text(bulk_unset_label(dialog.unset_behavior))
        .show_ui(ui, |ui| {
            for behavior in BULK_UNSET_BEHAVIORS {
                ui.selectable_value(
                    &mut dialog.unset_behavior,
                    behavior,
                    bulk_unset_label(behavior),
                );
            }
        });
    ui.add_space(12.0);
    impact_row(ui, "Objects changed", &impact.changed.to_string());
    // Only a non-zero count is news. A row reading "0" on every clean filter
    // is a row the reader learns to skip.
    if impact.locked_or_excluded > 0 {
        impact_row(
            ui,
            "Locked or excluded",
            &impact.locked_or_excluded.to_string(),
        );
    }
    impact_row(
        ui,
        "Analyses affected",
        if impact.changed == 0 {
            "0".to_owned()
        } else {
            affected_analyses.to_string()
        }
        .as_str(),
    );
}

/// One count of the edit's impact: its words on the pane's left edge, the
/// count at its right, where a column of counts lines up.
fn impact_row(ui: &mut Ui, label: &str, value: &str) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), IMPACT_ROW_H),
        egui::Sense::hover(),
    );
    let spoken = format!("{label}: {value}");
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &spoken));
    ui.painter().text(
        rect.left_center(),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(crate::ui::tokens::FS_0, FontWeight::Regular),
        tokens.color.text_dim,
    );
    ui.painter().text(
        rect.right_center(),
        egui::Align2::RIGHT_CENTER,
        value,
        theme::mono(crate::ui::tokens::FS_0, FontWeight::Regular),
        tokens.color.text,
    );
}

const IMPACT_ROW_H: f32 = 22.0;

fn new_value_control(ui: &mut Ui, dialog: &mut SelectionBulkEditDialogState) -> bool {
    match dialog.property {
        SelectionBulkProperty::ModelSection => {
            let before = dialog.new_value.clone();
            // The footer says why the list is empty; the control says only
            // that it is.
            let shown = if dialog.bound_sections.is_empty() {
                RichText::new("None defined")
            } else if dialog.new_value.is_empty() {
                RichText::new("Choose a section").color(Tokens::get(ui.ctx()).color.text_faint)
            } else {
                RichText::new(dialog.new_value.as_str())
            };
            ui.add_enabled_ui(!dialog.bound_sections.is_empty(), |ui| {
                ComboBox::from_id_salt("selection-bulk-model-section")
                    .width(ui.available_width())
                    .selected_text(shown)
                    .show_ui(ui, |ui| {
                        for section in &dialog.bound_sections {
                            ui.selectable_value(&mut dialog.new_value, section.clone(), section);
                        }
                    });
            });
            before != dialog.new_value
        }
        SelectionBulkProperty::Display => {
            let before = dialog.new_value.clone();
            ComboBox::from_id_salt("selection-bulk-display")
                .width(ui.available_width())
                .selected_text(&dialog.new_value)
                .show_ui(ui, |ui| {
                    for mode in [
                        ComponentDisplayMode::NameAndValue,
                        ComponentDisplayMode::NameOnly,
                        ComponentDisplayMode::ValueOnly,
                        ComponentDisplayMode::Hidden,
                        ComponentDisplayMode::Inherit,
                    ] {
                        ui.selectable_value(
                            &mut dialog.new_value,
                            mode.label().to_owned(),
                            mode.label(),
                        );
                    }
                });
            before != dialog.new_value
        }
        SelectionBulkProperty::Temperature
        | SelectionBulkProperty::Tolerance
        | SelectionBulkProperty::ParameterOverride => ui
            .add(TextEdit::singleline(&mut dialog.new_value).desired_width(ui.available_width()))
            .changed(),
    }
}

const NO_BOUND_SECTIONS: &str = "The bound library defines no model sections";

/// Why Apply is waiting on the value, or `None` once there is one to apply.
///
/// Restoring the inherited value needs no new value, so it never waits.
fn missing_value_reason(dialog: &SelectionBulkEditDialogState) -> Option<&'static str> {
    if dialog.unset_behavior == SelectionBulkUnsetBehavior::RestoreInheritedValue
        || !dialog.new_value.trim().is_empty()
    {
        return None;
    }
    Some(match dialog.property {
        SelectionBulkProperty::ModelSection if dialog.bound_sections.is_empty() => {
            NO_BOUND_SECTIONS
        }
        SelectionBulkProperty::ModelSection => "Choose a section to apply",
        _ => "Enter a new value to apply",
    })
}

/// The sections the matched instances' model libraries define, once each.
///
/// Read from the same library records Models ▸ Corners & sections lists: the
/// sections each library's corners bind, and the sections its model
/// metadata declares. A library this project does not hold contributes
/// nothing, and nothing is offered that no bound library defines.
fn bound_library_sections(state: &AppState, rows: &[SelectionBulkRow]) -> Vec<String> {
    let instances = rows
        .iter()
        .filter(|row| row.key.kind == SelectionBulkRowKind::Instance)
        .map(|row| row.key.object_id)
        .collect::<BTreeSet<_>>();
    let libraries = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| instances.contains(&component.id))
        .filter_map(|component| component.library_cell.as_ref())
        .map(|binding| binding.library.as_str())
        .collect::<BTreeSet<_>>();
    let mut sections = Vec::new();
    for name in libraries {
        let Some(library) = state.model.model_library_manager.get_library(name) else {
            continue;
        };
        sections.extend(
            library
                .corners
                .values()
                .flat_map(|corner| corner.effective_section_bindings())
                .map(|binding| binding.section),
        );
        sections.extend(
            library
                .model_definition_metadata
                .values()
                .flat_map(|metadata| metadata.sections.iter())
                .map(|section| section.name.clone()),
        );
    }
    sections.retain(|section: &String| !section.trim().is_empty());
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    sections
}

/// The occurrence the active schematic is open at — the dialog's scope line and
/// the display path of every row it owns. The design root is implicit, so the
/// top sheet is `/` rather than the name of the cell that happens to be top.
fn active_hierarchy_path(state: &AppState) -> String {
    state.model.workspace.content.occurrence_path().to_string()
}

/// The library cell a buffer outside the active occurrence belongs to.
///
/// It carries no leading `/`: an occurrence path starts at the design root and
/// this does not name an occurrence at all, so the two never read alike in the
/// same column.
fn schematic_display_path(cell_view_key: &str) -> String {
    let mut segments = cell_view_key.split('/').collect::<Vec<_>>();
    if segments.len() > 1 {
        segments.pop();
    }
    segments.join("/")
}

fn build_rows(
    state: &AppState,
    filter: &SelectionBulkFilter,
    property: SelectionBulkProperty,
) -> Vec<SelectionBulkRow> {
    let active_key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let mut buffers = Vec::new();
    buffers.push((
        active_key.clone(),
        active_hierarchy_path(state),
        state.model.schematic.document(),
        true,
    ));
    match filter.hierarchy_scope {
        SelectionBulkHierarchyScope::ActiveHierarchyPath => {
            let hierarchy_keys = state
                .model
                .workspace
                .content
                .hierarchy_stack
                .iter()
                .map(|reference| reference.key())
                .collect::<HashSet<_>>();
            for (key, schematic) in &state.model.workspace.content.schematic_buffers {
                if !key.eq_ignore_ascii_case(&active_key) && hierarchy_keys.contains(key) {
                    buffers.push((
                        key.clone(),
                        schematic_display_path(key),
                        schematic.document(),
                        false,
                    ));
                }
            }
        }
        SelectionBulkHierarchyScope::CurrentSheet => {}
        SelectionBulkHierarchyScope::CompleteProject => {
            for (key, schematic) in &state.model.workspace.content.schematic_buffers {
                if !key.eq_ignore_ascii_case(&active_key) {
                    buffers.push((
                        key.clone(),
                        schematic_display_path(key),
                        schematic.document(),
                        false,
                    ));
                }
            }
        }
    }
    buffers.sort_by(|(left, _, _, _), (right, _, _, _)| left.cmp(right));
    let active_sheet = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&active_key)
        .and_then(|catalog| catalog.active_sheet_id());
    let mut rows = Vec::new();
    for (view_key, display_path, schematic, active_owner) in buffers {
        append_buffer_rows(
            &mut rows,
            &view_key,
            &display_path,
            schematic,
            active_owner,
            filter,
            property,
            if filter.hierarchy_scope == SelectionBulkHierarchyScope::CurrentSheet {
                active_sheet
            } else {
                None
            },
            &state.model.workspace.content.design_management,
        );
    }
    rows.retain(|row| row_matches_query(row, filter));
    rows.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.object.cmp(&right.object))
            .then_with(|| left.key.cmp(&right.key))
    });
    rows
}

fn append_buffer_rows(
    rows: &mut Vec<SelectionBulkRow>,
    view_key: &str,
    display_path: &str,
    schematic: &rspice_design::schematic::document::SchematicDocument,
    active_owner: bool,
    filter: &SelectionBulkFilter,
    property: SelectionBulkProperty,
    active_sheet: Option<crate::state::SheetId>,
    design_management: &crate::state::DesignManagementCatalog,
) {
    let on_current_sheet = |id| {
        active_sheet.is_none_or(|sheet| {
            design_management.sheet_for_object_or_active(view_key, id) == Some(sheet)
        })
    };
    match filter.object_kind {
        SelectionBulkObjectKind::InstancesAndParameters | SelectionBulkObjectKind::PortsAndPins => {
            for component in &schematic.components {
                let is_port = component.kind == ComponentType::Port;
                if (filter.object_kind == SelectionBulkObjectKind::InstancesAndParameters
                    && is_port)
                    || (filter.object_kind == SelectionBulkObjectKind::PortsAndPins && !is_port)
                    || !on_current_sheet(component.id)
                    || !component_matches_model_cell(component, &filter.model_cell)
                    || !component_matches_current_property(component, &filter.current_property)
                {
                    continue;
                }
                let kind = if is_port {
                    SelectionBulkRowKind::Port
                } else {
                    SelectionBulkRowKind::Instance
                };
                rows.push(component_row(
                    view_key,
                    display_path,
                    component,
                    kind,
                    active_owner,
                    property,
                ));
                if filter.object_kind == SelectionBulkObjectKind::PortsAndPins {
                    for (index, (terminal, _)) in component.terminal_positions().iter().enumerate()
                    {
                        rows.push(SelectionBulkRow {
                            key: SelectionBulkRowKey {
                                view_key: view_key.to_owned(),
                                kind: SelectionBulkRowKind::Pin,
                                object_id: component.id,
                                member: index as u32,
                            },
                            object: format!("Pin {terminal}"),
                            path: format!(
                                "{display_path}/{}:{terminal}",
                                if component.name.is_empty() {
                                    component.id.to_string()
                                } else {
                                    component.name.clone()
                                }
                            ),
                            value: "terminal contract".to_owned(),
                            search_text: format!(
                                "{terminal} {} {} terminal pin port",
                                component.name, component.params
                            ),
                            lock: Some("Master-owned".to_owned()),
                        });
                    }
                }
            }
        }
        SelectionBulkObjectKind::NetsAndLabels => {
            if !filter.model_cell.trim().is_empty() || !filter.current_property.trim().is_empty() {
                return;
            }
            for wire in &schematic.wires {
                if on_current_sheet(wire.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::Wire,
                        wire.id,
                        "Wire",
                        format!("{} vertices", wire.points.len()),
                        active_owner,
                    ));
                }
            }
            for bus in &schematic.buses {
                if on_current_sheet(bus.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::Bus,
                        bus.id,
                        "Bus",
                        bus.declaration
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| "untyped".to_owned()),
                        active_owner,
                    ));
                }
            }
            for tap in &schematic.bus_taps {
                if on_current_sheet(tap.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::BusTap,
                        tap.id,
                        "Bus tap",
                        tap.slice.to_string(),
                        active_owner,
                    ));
                }
            }
            for junction in &schematic.junctions {
                if on_current_sheet(junction.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::Junction,
                        junction.id,
                        "Junction",
                        format!("{}, {}", junction.pos.x, junction.pos.y),
                        active_owner,
                    ));
                }
            }
            for label in &schematic.net_labels {
                if on_current_sheet(label.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::NetLabel,
                        label.id,
                        &label.name,
                        "net label".to_owned(),
                        active_owner,
                    ));
                }
            }
        }
        SelectionBulkObjectKind::GraphicsAndNotes => {
            if !filter.model_cell.trim().is_empty() || !filter.current_property.trim().is_empty() {
                return;
            }
            for note in &schematic.design_notes {
                if on_current_sheet(note.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::DesignNote,
                        note.id,
                        "Design note",
                        note.text.clone(),
                        active_owner,
                    ));
                }
            }
            for shape in &schematic.documentation_shapes {
                if on_current_sheet(shape.id) {
                    rows.push(non_editable_row(
                        view_key,
                        display_path,
                        SelectionBulkRowKind::DocumentationShape,
                        shape.id,
                        "Documentation shape",
                        shape.kind().label().to_owned(),
                        active_owner,
                    ));
                }
            }
        }
    }
}

fn component_row(
    view_key: &str,
    display_path: &str,
    component: &Component,
    kind: SelectionBulkRowKind,
    active_owner: bool,
    property: SelectionBulkProperty,
) -> SelectionBulkRow {
    let displayed_value = if !component.value.trim().is_empty() {
        component.value.clone()
    } else if let Some(binding) = component.library_cell.as_ref() {
        binding.cell.clone()
    } else {
        component.kind.display_name().to_owned()
    };
    let lock = if !active_owner {
        Some("Other owner".to_owned())
    } else if (kind == SelectionBulkRowKind::Port && property != SelectionBulkProperty::Display)
        || (property == SelectionBulkProperty::ModelSection && component.library_cell.is_none())
    {
        Some("Not applicable".to_owned())
    } else if parse_replacement_parameters_strict(&component.params).is_err()
        && matches!(
            property,
            SelectionBulkProperty::Temperature
                | SelectionBulkProperty::Tolerance
                | SelectionBulkProperty::ParameterOverride
        )
    {
        Some("Invalid parameters".to_owned())
    } else {
        None
    };
    let binding_search = component
        .library_cell
        .as_ref()
        .map(|binding| {
            format!(
                "{} {} {} {}",
                binding.library,
                binding.cell,
                binding.view,
                binding.model_section.as_deref().unwrap_or("default")
            )
        })
        .unwrap_or_default();
    SelectionBulkRow {
        key: SelectionBulkRowKey {
            view_key: view_key.to_owned(),
            kind,
            object_id: component.id,
            member: 0,
        },
        object: if component.name.is_empty() {
            format!("{} #{}", component.kind.display_name(), component.id)
        } else {
            component.name.clone()
        },
        path: format!(
            "{display_path}/{}",
            if component.name.is_empty() {
                component.id.to_string()
            } else {
                component.name.clone()
            }
        ),
        value: displayed_value,
        search_text: format!(
            "{} {} {} {} {}",
            component.kind.display_name(),
            component.name,
            component.value,
            component.params,
            binding_search
        ),
        lock,
    }
}

fn non_editable_row(
    view_key: &str,
    display_path: &str,
    kind: SelectionBulkRowKind,
    object_id: u64,
    object: &str,
    value: String,
    active_owner: bool,
) -> SelectionBulkRow {
    SelectionBulkRow {
        key: SelectionBulkRowKey {
            view_key: view_key.to_owned(),
            kind,
            object_id,
            member: 0,
        },
        object: object.to_owned(),
        path: format!("{display_path}/{object_id}"),
        search_text: format!("{object} {value}"),
        value,
        lock: Some(if active_owner {
            "Not applicable".to_owned()
        } else {
            "Other owner".to_owned()
        }),
    }
}

fn component_matches_model_cell(component: &Component, pattern: &str) -> bool {
    let pattern = pattern.trim();
    if pattern.is_empty() {
        return true;
    }
    let mut candidates = vec![
        component.kind.display_name().to_owned(),
        component.value.clone(),
    ];
    if let Some(binding) = component.library_cell.as_ref() {
        candidates.push(binding.cell.clone());
        candidates.push(format!(
            "{}/{}/{}",
            binding.library, binding.cell, binding.view
        ));
    }
    candidates
        .iter()
        .any(|candidate| wildcard_match(pattern, candidate))
}

fn component_matches_current_property(component: &Component, expression: &str) -> bool {
    let expression = expression.trim();
    if expression.is_empty() {
        return true;
    }
    let (key, expected) = expression
        .split_once('=')
        .map(|(key, expected)| (key.trim(), Some(expected.trim())))
        .unwrap_or((expression, None));
    let value = component_named_property(component, key);
    match (value, expected) {
        (Some(_), None) => true,
        (Some(value), Some(expected)) => wildcard_match(expected, &value),
        (None, _) => false,
    }
}

fn component_named_property(component: &Component, name: &str) -> Option<String> {
    let name = name.trim().to_ascii_lowercase().replace(['_', '-'], " ");
    match name.as_str() {
        "section" | "model section" => component
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.clone())
            .or_else(|| Some("default".to_owned())),
        "display" => Some(component.display_mode.label().to_owned()),
        "model" | "cell" => component
            .library_cell
            .as_ref()
            .map(|binding| binding.cell.clone())
            .or_else(|| Some(component.value.clone())),
        "value" => Some(component.value.clone()),
        other => parse_replacement_parameters_strict(&component.params)
            .ok()
            .and_then(|parameters| parameters.get(other).cloned()),
    }
}

fn row_matches_query(row: &SelectionBulkRow, filter: &SelectionBulkFilter) -> bool {
    let query = filter.query.trim();
    query.is_empty()
        || wildcard_match(query, &row.object)
        || wildcard_match(query, &row.path)
        || wildcard_match(query, &row.value)
        || wildcard_match(query, &row.search_text)
}

fn wildcard_match(pattern: &str, candidate: &str) -> bool {
    let pattern = pattern.trim().to_ascii_lowercase();
    let candidate = candidate.to_ascii_lowercase();
    if pattern.is_empty() {
        return true;
    }
    if !pattern.contains('*') {
        return candidate.contains(&pattern);
    }
    let mut remainder = candidate.as_str();
    let anchored_start = !pattern.starts_with('*');
    let anchored_end = !pattern.ends_with('*');
    let parts = pattern
        .split('*')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    for (index, part) in parts.iter().enumerate() {
        let Some(position) = remainder.find(part) else {
            return false;
        };
        if index == 0 && anchored_start && position != 0 {
            return false;
        }
        remainder = &remainder[position + part.len()..];
    }
    !anchored_end || remainder.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{LibraryCellInstance, Point};

    fn component(id: u64, name: &str, section: Option<&str>) -> Component {
        let mut component =
            Component::new(id, ComponentType::CellInstance, Point::new(id as i32, 0))
                .with_name_value(name, "OPA189");
        let mut binding = LibraryCellInstance::new("analog", "OPA189", "symbol");
        binding.model_section = section.map(str::to_owned);
        component.library_cell = Some(binding);
        component
    }

    /// The four result columns and the select box fit the pane they are laid
    /// in. They were fixed widths that summed past it, so the last column
    /// painted over the impact panel beside it.
    #[test]
    fn the_result_columns_fit_inside_their_pane() {
        // A 28 pt select box and a 14 pt scroll bar.
        let fixed = 42.0;
        for pane in [240.0_f32, 300.0, 360.0, 520.0, 900.0] {
            let columns = result_columns(pane, fixed);
            let used = fixed + RESULT_COLUMN_GAP * 4.0 + columns.iter().sum::<f32>();
            assert!(
                used <= pane.max(fixed + RESULT_COLUMN_GAP * 4.0 + 140.0),
                "{pane} pt pane lays out {used} pt of columns"
            );
        }
    }

    #[test]
    fn the_scope_line_names_the_occurrence_below_the_implicit_root() {
        let mut state = AppState::default();
        assert_eq!(active_hierarchy_path(&state), "/");

        state.model.workspace.descend_into(
            "XAFE".to_owned(),
            crate::state::CellViewRef::new("user", "afe_core", "schematic"),
            crate::state::ViewType::Schematic,
        );
        assert_eq!(active_hierarchy_path(&state), "/XAFE");

        state.model.workspace.descend_into(
            "XBIAS".to_owned(),
            crate::state::CellViewRef::new("user", "bias", "schematic"),
            crate::state::ViewType::Schematic,
        );
        assert_eq!(active_hierarchy_path(&state), "/XAFE/XBIAS");

        assert_eq!(
            schematic_display_path("analog/opamp/schematic"),
            "analog/opamp",
            "a buffer outside the occurrence is named as its library cell"
        );
    }

    #[test]
    fn wildcard_filter_is_case_insensitive_and_anchored_when_requested() {
        assert!(wildcard_match("OPA*", "opa189/tt"));
        assert!(wildcard_match("*189*", "OPA189/tt"));
        assert!(!wildcard_match("OPA*", "xOPA189"));
        assert!(!wildcard_match("*189", "OPA189/tt"));
    }

    #[test]
    fn free_text_query_searches_hidden_instance_parameters_and_binding_fields() {
        let mut state = AppState::default();
        let mut instance = component(1, "U1", Some("tt"));
        instance.params = "gain=100".to_owned();
        state.model.schematic.document_mut_for_test().components = vec![instance];

        let mut filter = SelectionBulkFilter {
            query: "gain=100".to_owned(),
            ..SelectionBulkFilter::default()
        };
        assert_eq!(
            build_rows(&state, &filter, SelectionBulkProperty::ModelSection).len(),
            1
        );

        filter.query = "analog".to_owned();
        assert_eq!(
            build_rows(&state, &filter, SelectionBulkProperty::ModelSection).len(),
            1
        );
    }

    #[test]
    fn exact_mockup_default_section_filter_matches_an_inherited_binding() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components = vec![component(1, "U1", None)];
        let filter = SelectionBulkFilter {
            model_cell: "OPA189*".to_owned(),
            current_property: "section = default".to_owned(),
            ..SelectionBulkFilter::default()
        };

        let rows = build_rows(&state, &filter, SelectionBulkProperty::ModelSection);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].object, "U1");
        assert!(rows[0].editable());
    }

    /// The dialog proposes nothing: it opens with no value and Apply says
    /// what it is waiting for.
    #[test]
    fn bulk_edit_opens_with_no_value_and_apply_waits_for_one() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];

        open_selection_bulk_edit_dialog(&mut state);

        let dialog = &state.dialogs.selection_bulk_edit;
        assert!(dialog.open);
        assert_eq!(dialog.new_value, "");
        assert!(missing_value_reason(dialog).is_some());
    }

    /// The sections offered are the bound library's own, not a fixed list,
    /// and a library that defines none says so.
    #[test]
    fn bulk_edit_offers_the_bound_librarys_sections() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];

        open_selection_bulk_edit_dialog(&mut state);
        assert!(state.dialogs.selection_bulk_edit.bound_sections.is_empty());
        assert_eq!(
            missing_value_reason(&state.dialogs.selection_bulk_edit),
            Some(NO_BOUND_SECTIONS)
        );

        let mut library = crate::state::model_library::ModelLibrary::new("analog");
        for (name, default) in [("TT_PRE", true), ("mc_ss", false)] {
            library.corners.insert(
                name.to_owned(),
                rspice_model_library::ProcessCorner::from_composite_section(
                    name,
                    std::path::PathBuf::from("analog.lib"),
                    default,
                ),
            );
        }
        state.model.model_library_manager.add_library(library);

        open_selection_bulk_edit_dialog(&mut state);
        let dialog = &state.dialogs.selection_bulk_edit;
        assert_eq!(dialog.bound_sections, ["mc_ss", "TT_PRE"]);
        assert_eq!(
            missing_value_reason(dialog),
            Some("Choose a section to apply")
        );
        assert_eq!(
            validate_bulk_value(
                SelectionBulkProperty::ModelSection,
                "TT_PRE",
                SelectionBulkUnsetBehavior::SetExplicitValue,
                None,
            ),
            Ok("TT_PRE".to_owned()),
            "a library's section is applied as it names it"
        );
    }

    #[test]
    fn invalid_target_preflight_never_partially_mutates() {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = vec![
            component(1, "X1", Some("tt")),
            component(2, "X2", Some("tt")),
        ];
        schematic.document_mut_for_test().components[1].library_cell = None;
        let before = schematic.document().components.clone();
        let ids = BTreeSet::from([1, 2]);

        assert!(
            apply_bulk_edit(
                &mut schematic,
                &ids,
                SelectionBulkProperty::ModelSection,
                "ff",
                SelectionBulkUnsetBehavior::SetExplicitValue,
                None,
            )
            .is_err()
        );
        assert_eq!(schematic.document().components, before);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn model_section_bulk_edit_is_one_undo_boundary() {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = vec![
            component(1, "X1", Some("tt")),
            component(2, "X2", Some("tt")),
        ];
        let ids = BTreeSet::from([1, 2]);
        let receipt = apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::ModelSection,
            "ff",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        )
        .expect("valid bulk edit");
        assert_eq!(receipt.changed, 2);
        assert!(schematic.can_undo());
        assert_eq!(
            schematic.document().components[0]
                .library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref()),
            Some("ff")
        );
        assert!(schematic.undo());
        assert_eq!(
            schematic.document().components[0]
                .library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref()),
            Some("tt")
        );
        assert!(!schematic.can_undo());
    }

    #[test]
    fn restore_and_leave_unchanged_have_distinct_semantics() {
        let mut schematic = SchematicState::default();
        let mut first = component(1, "X1", None);
        first.params = "temp=27".to_owned();
        let second = component(2, "X2", None);
        schematic.document_mut_for_test().components = vec![first, second];
        let ids = BTreeSet::from([1, 2]);

        let receipt = apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::Temperature,
            "85",
            SelectionBulkUnsetBehavior::LeaveUnchanged,
            None,
        )
        .expect("leave unchanged");
        assert_eq!(receipt.changed, 1);
        assert_eq!(schematic.document().components[0].params, "temp=85");
        assert!(schematic.document().components[1].params.is_empty());

        let receipt = apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::Temperature,
            "",
            SelectionBulkUnsetBehavior::RestoreInheritedValue,
            None,
        )
        .expect("restore");
        assert_eq!(receipt.changed, 1);
        assert!(schematic.document().components[0].params.is_empty());
    }

    #[test]
    fn display_override_changes_durable_component_state() {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
        let ids = BTreeSet::from([1]);
        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::Display,
            "hidden",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        )
        .expect("display edit");
        assert_eq!(
            schematic.document().components[0].display_mode,
            ComponentDisplayMode::Hidden
        );
    }

    #[test]
    fn parameter_override_uses_current_property_key_and_restores_inheritance() {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
        let ids = BTreeSet::from([1]);
        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::ParameterOverride,
            "4",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            Some("m=1"),
        )
        .expect("set parameter override");
        assert_eq!(schematic.document().components[0].params, "m=4");

        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::ParameterOverride,
            "",
            SelectionBulkUnsetBehavior::RestoreInheritedValue,
            Some("m=4"),
        )
        .expect("restore parameter inheritance");
        assert!(schematic.document().components[0].params.is_empty());
    }

    #[test]
    fn invalid_numeric_value_leaves_document_and_undo_history_unchanged() {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
        let before = schematic.document().components.clone();
        let ids = BTreeSet::from([1]);
        assert!(
            apply_bulk_edit(
                &mut schematic,
                &ids,
                SelectionBulkProperty::Temperature,
                "-300",
                SelectionBulkUnsetBehavior::SetExplicitValue,
                None,
            )
            .is_err()
        );
        assert_eq!(schematic.document().components, before);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn impact_preview_is_exact_without_cloning_an_undo_transaction() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components = vec![
            component(1, "X1", Some("tt")),
            component(2, "X2", Some("tt")),
        ];
        let rows = build_rows(
            &state,
            &SelectionBulkFilter::default(),
            SelectionBulkProperty::ModelSection,
        );
        let selected = rows.iter().map(|row| row.key.clone()).collect();

        let impact = preview_impact(
            &state.model.schematic,
            &rows,
            &selected,
            SelectionBulkProperty::ModelSection,
            "ff",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            "",
        );
        assert_eq!(impact.changed, 2);
        assert_eq!(impact.locked_or_excluded, 0);
        assert!(impact.error.is_none());
        assert!(!state.model.schematic.can_undo());
        assert!(
            state
                .model
                .schematic
                .document()
                .components
                .iter()
                .all(|component| {
                    component
                        .library_cell
                        .as_ref()
                        .and_then(|binding| binding.model_section.as_deref())
                        == Some("tt")
                })
        );
    }

    #[test]
    fn authority_rejects_active_selection_and_external_buffer_drift() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(1);
        state
            .model
            .workspace
            .insert_schematic_editor("user/child/schematic".to_owned(), SchematicState::default());
        let authority = SelectionBulkEditAuthority::capture(&state);
        assert!(authority.stale_reason(&state).is_none());

        state.model.schematic.session.editor.selection.clear();
        assert!(authority.stale_reason(&state).is_some());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(1);
        state
            .model
            .workspace
            .schematic_editor_mut("user/child/schematic")
            .expect("external buffer")
            .editor
            .document_mut_for_test()
            .components
            .push(component(2, "X2", Some("tt")));
        assert!(authority.stale_reason(&state).is_some());
    }

    #[test]
    fn complete_project_rows_are_visible_but_other_owners_are_locked() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        let mut external = SchematicState::default();
        external.document_mut_for_test().components = vec![component(2, "X2", Some("ff"))];
        state
            .model
            .workspace
            .insert_schematic_editor("user/child/schematic".to_owned(), external);
        let filter = SelectionBulkFilter {
            hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
            ..SelectionBulkFilter::default()
        };
        let rows = build_rows(&state, &filter, SelectionBulkProperty::ModelSection);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows.iter().filter(|row| row.editable()).count(), 1);
        assert!(
            rows.iter()
                .any(|row| row.lock.as_deref() == Some("Other owner"))
        );
    }

    #[test]
    fn port_rows_only_accept_the_display_property() {
        let mut state = AppState::default();
        state.model.schematic.document_mut_for_test().components = vec![
            Component::new(1, ComponentType::Port, Point::new(0, 0)).with_name_value("VIN", ""),
        ];
        let filter = SelectionBulkFilter {
            object_kind: SelectionBulkObjectKind::PortsAndPins,
            ..SelectionBulkFilter::default()
        };

        let temperature_rows = build_rows(&state, &filter, SelectionBulkProperty::Temperature);
        assert_eq!(temperature_rows[0].lock.as_deref(), Some("Not applicable"));
        let display_rows = build_rows(&state, &filter, SelectionBulkProperty::Display);
        assert!(display_rows[0].editable());
    }

    #[test]
    fn accessibility_tree_exposes_only_the_mockup_bulk_edit_surface_actions() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        open_selection_bulk_edit_dialog(&mut app.state);

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| app.render_selection_bulk_edit_dialog(ctx),
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("bulk-edit access tree")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
        }));
        for label in [PRIMARY, "Cancel", "Save filter\u{2026}"] {
            assert!(
                nodes.iter().any(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                }),
                "missing action {label}"
            );
        }
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::CheckBox
                && node.label() == Some("Select X1")
                && node.toggled() == Some(egui::accesskit::Toggled::True)
        }));
    }

    /// Nothing in the body reaches past the footer's right edge. With no
    /// section to offer, New value used to state why in a line wider than
    /// its column; the body grew about 90 pt past the surface, every row
    /// under it took the new width, and the split rule moved with it.
    #[test]
    fn the_body_stays_inside_the_dialog_when_no_section_is_defined() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        open_selection_bulk_edit_dialog(&mut app.state);
        assert!(
            app.state
                .dialogs
                .selection_bulk_edit
                .bound_sections
                .is_empty()
        );

        // Access updates are incremental: keep the latest of every node.
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..4 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| app.render_selection_bulk_edit_dialog(ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let apply = nodes
            .values()
            .find(|node| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(PRIMARY)
            })
            .and_then(egui::accesskit::Node::bounds)
            .expect("Apply has bounds");
        let footer_right = apply.x1 + 12.0;
        for node in nodes.values() {
            let Some(bounds) = node.bounds() else {
                continue;
            };
            if matches!(
                node.role(),
                egui::accesskit::Role::ComboBox
                    | egui::accesskit::Role::TextInput
                    | egui::accesskit::Role::CheckBox
                    | egui::accesskit::Role::Button
            ) {
                assert!(
                    bounds.x1 <= footer_right + 0.5,
                    "{:?} {:?} ends at {} past the dialog's {footer_right}",
                    node.role(),
                    node.label(),
                    bounds.x1,
                );
            }
        }
    }
}
