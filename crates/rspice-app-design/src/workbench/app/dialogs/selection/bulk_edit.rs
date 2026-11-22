//! Filtered selection and atomic schematic bulk editing.
//!
//! Queries may inspect the active hierarchy or complete project, but edits are
//! deliberately limited to the active schematic owner. A commit preflights
//! every selected target, validates its frozen document authority, and
//! publishes all component changes through one schematic undo boundary.

use std::collections::{BTreeSet, HashSet};

use egui::{Context, Grid, RichText, ScrollArea, TextEdit, Ui, Vec2};
use rspice_design::schematic::bom::BomField;
use rspice_design::schematic::tolerance::{ToleranceDistribution, supports_tolerance};

use super::bulk_filter::{
    SelectionBulkFilter, SelectionBulkFilterSession, SelectionBulkHierarchyScope,
    SelectionBulkObjectKind,
};
use crate::state::bulk_edit::{SelectionBulkProperty, SelectionBulkUnsetBehavior};
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
    name_control, select, text_field,
};
use crate::ui::widgets::{DIALOG_BODY_INSET, band_header};
use crate::workbench::design_system::WorkbenchIcon;
pub(crate) use rspice_design::schematic::owned::bulk_edit::takes_instance_temperature;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

pub const TITLE: &str = "Bulk edit";
pub const PRIMARY: &str = "Apply";

const BULK_PROPERTIES: [SelectionBulkProperty; 14] = [
    SelectionBulkProperty::Value,
    SelectionBulkProperty::ModelSection,
    SelectionBulkProperty::Temperature,
    SelectionBulkProperty::Tolerance,
    SelectionBulkProperty::ToleranceDistribution,
    SelectionBulkProperty::Display,
    SelectionBulkProperty::ParameterOverride,
    SelectionBulkProperty::Bom(BomField::Manufacturer),
    SelectionBulkProperty::Bom(BomField::Mpn),
    SelectionBulkProperty::Bom(BomField::Description),
    SelectionBulkProperty::Bom(BomField::Package),
    SelectionBulkProperty::Bom(BomField::VoltageRating),
    SelectionBulkProperty::Bom(BomField::CurrentRating),
    SelectionBulkProperty::Bom(BomField::PowerRating),
];

const fn bulk_property_label(value: SelectionBulkProperty) -> &'static str {
    match value {
        SelectionBulkProperty::Value => "Value",
        SelectionBulkProperty::ModelSection => "Model section",
        SelectionBulkProperty::Temperature => "Temperature",
        SelectionBulkProperty::Tolerance => "Tolerance",
        SelectionBulkProperty::ToleranceDistribution => "Tolerance distribution",
        SelectionBulkProperty::Display => "Display",
        SelectionBulkProperty::ParameterOverride => "Parameter override",
        SelectionBulkProperty::Bom(field) => field.label(),
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
    fn capture(model: &Model) -> Self {
        let active_key = model.workspace.content.active_schematic_reference().key();
        let mut project_buffers = model
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
            active: SchematicEditAuthority::capture(model),
            active_key,
            design_management_revision: model.workspace.content.design_management.revision(),
            project_buffers,
        }
    }

    fn stale_reason(&self, model: &Model) -> Option<String> {
        let reopen = |reason: &str| format!("{reason}. Close Bulk edit and open it again.");
        let current = crate::workbench::app::schematic_editor_request_source(model);
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
                .is_equal_document(model.schematic.document())
        {
            return Some(reopen("The active schematic changed"));
        }
        if self.active.view_path != model.workspace.content.active_view.display_path()
            || self.active_key != model.workspace.content.active_schematic_reference().key()
        {
            return Some(reopen("The active cell/view changed"));
        }
        if !self.active.matches_source(&current) {
            return Some(reopen("The schematic source or editing scope changed"));
        }
        if self.active.grid_size != model.schematic.document().grid_size
            || self.active.document_policy != model.schematic.document().document_policy
        {
            return Some(reopen("The schematic grid or editing policy changed"));
        }
        if self.active.selection != model.schematic.session.editor.selection {
            return Some(reopen("The selected-object set changed"));
        }
        if self.design_management_revision != model.workspace.content.design_management.revision() {
            return Some(reopen("The sheet ownership catalog changed"));
        }
        let live_external = model
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
            let Some(live) = model.workspace.content.schematic_buffers.get(key) else {
                return Some(reopen("A project schematic was closed"));
            };
            if !expected.is_equal_document(live.document()) {
                return Some(reopen("A project schematic changed"));
            }
        }
        None
    }

    fn validate_commit(&self, model: &Model) -> Result<(), String> {
        self.active.validate(model, TITLE)?;
        if let Some(reason) = model.safe_mode.read_only_reason() {
            return Err(reason);
        }
        if let Some(reason) = self.stale_reason(model) {
            return Err(reason);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SelectionBulkEditDialogState {
    pub open: bool,
    pub(crate) filter: SelectionBulkFilter,
    selected: BTreeSet<SelectionBulkRowKey>,
    pub(crate) property: SelectionBulkProperty,
    pub(crate) new_value: String,
    pub(crate) unset_behavior: SelectionBulkUnsetBehavior,
    /// The sections a model section can be set to: those every ticked
    /// instance's library defines. Resolved when the filter, the property or
    /// a tick changes.
    pub bound_sections: Vec<String>,
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
    /// The filter changed: what it now finds is ticked.
    FilterChanged,
    /// The property changed: the ticks stand, less the rows it cannot reach.
    PropertyChanged,
    /// A row was ticked or unticked.
    TicksChanged,
    SaveFilter,
    LoadFilter,
    DeleteFilter,
}

/// The rows Bulk edit opens with ticked: the parts and ports selected on the
/// canvas, and nothing when nothing is selected, as Virtuoso's Edit
/// Properties and KiCad's "selected only" scope start from the selection.
/// The filter lists what else could be ticked; it ticks nothing by itself
/// until it is changed. A selected row the property cannot reach stays
/// chosen, so the next property that can reach it finds it ticked.
fn selected_rows(model: &Model, rows: &[SelectionBulkRow]) -> BTreeSet<SelectionBulkRowKey> {
    let active_key = model.workspace.content.active_schematic_reference().key();
    let selection = &model.schematic.session.editor.selection;
    rows.iter()
        .filter(|row| {
            row.key.view_key.eq_ignore_ascii_case(&active_key)
                && matches!(
                    row.key.kind,
                    SelectionBulkRowKind::Instance | SelectionBulkRowKind::Port
                )
                && selection.components.contains(&row.key.object_id)
        })
        .map(|row| row.key.clone())
        .collect()
}

pub fn open_selection_bulk_edit_dialog(
    dialog: &mut SelectionBulkEditDialogState,
    filters: &SelectionBulkFilterSession,
    model: &Model,
) {
    let filter = filters.active.clone();
    let authority = SelectionBulkEditAuthority::capture(model);
    let property = SelectionBulkProperty::default();
    let rows = build_rows(model, &filter, property);
    let selected = selected_rows(model, &rows);
    let bound_sections = bound_library_sections(model, &rows, &selected);
    // Nothing is proposed: the value is the user's to choose, and Apply
    // waits for it.
    *dialog = SelectionBulkEditDialogState {
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

pub fn render_selection_bulk_edit_dialog(
    bulk_edit: &mut SelectionBulkEditDialogState,
    filters: &mut SelectionBulkFilterSession,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !bulk_edit.open {
        return;
    }
    let filter = bulk_edit.filter.clone();
    let property = bulk_edit.property;
    let rows = build_rows(model, &filter, property);
    let impact = preview_impact(
        &model.schematic,
        &rows,
        &bulk_edit.selected,
        property,
        &bulk_edit.new_value,
        bulk_edit.unset_behavior,
        &bulk_edit.filter.current_property,
    );
    let stale = bulk_edit
        .authority
        .as_ref()
        .and_then(|authority| authority.stale_reason(model));
    let write_allowed = !model.schematic_edit_read_only() && !model.safe_mode.project_read_only();
    let primary_enabled =
        write_allowed && stale.is_none() && impact.error.is_none() && impact.changed > 0;
    let mut body_scroll_offset = bulk_edit.body_scroll_offset;
    // A value not yet chosen is not a failed edit: the footer says what
    // Apply is waiting for, and no "nothing was changed" banner opens
    // over a dialog nobody has touched.
    let missing_value = missing_value_reason(bulk_edit);
    let error = bulk_edit
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
        dialog =
            dialog.transaction_state(DialogTransactionTone::Error, "Nothing was changed", error);
    }

    let saved = filters
        .saved()
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<Vec<_>>();
    let analyses = model.sim_setup.enabled_analysis_instance_count();
    let hierarchy_path = active_hierarchy_path(model);
    let mut action = SelectionBulkBodyAction::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        action = selection_bulk_body(
            ui,
            &mut *bulk_edit,
            &rows,
            &impact,
            analyses,
            &saved,
            &hierarchy_path,
        );
        None
    });
    bulk_edit.body_scroll_offset = body_scroll_offset;
    handle_selection_bulk_body_action(bulk_edit, filters, model, action);

    match choice {
        DialogChoice::Primary => {
            if let Err(error) = commit_selection_bulk_edit(bulk_edit, model, workbench) {
                bulk_edit.error = Some(error);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            bulk_edit.close();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn handle_selection_bulk_body_action(
    bulk_edit: &mut SelectionBulkEditDialogState,
    filters: &mut SelectionBulkFilterSession,
    model: &Model,
    action: SelectionBulkBodyAction,
) {
    match action {
        SelectionBulkBodyAction::None => {}
        SelectionBulkBodyAction::FilterChanged => {
            filters.active = bulk_edit.filter.clone();
            tick_what_the_filter_finds(bulk_edit, model);
        }
        SelectionBulkBodyAction::PropertyChanged | SelectionBulkBodyAction::TicksChanged => {
            let rows = build_rows(model, &bulk_edit.filter, bulk_edit.property);
            refresh_bound_sections(bulk_edit, model, &rows);
            bulk_edit.error = None;
        }
        SelectionBulkBodyAction::SaveFilter => {
            let name = bulk_edit.save_filter_name.clone();
            filters.active = bulk_edit.filter.clone();
            match filters.save_active(&name) {
                Ok(()) => {
                    bulk_edit.selected_saved_filter = Some(name.trim().to_owned());
                    bulk_edit.save_filter_name.clear();
                    bulk_edit.save_filter_open = false;
                    bulk_edit.error = None;
                }
                Err(error) => bulk_edit.error = Some(error),
            }
        }
        SelectionBulkBodyAction::LoadFilter => {
            let name = bulk_edit.selected_saved_filter.clone();
            if let Some(name) = name
                && filters.load(&name)
            {
                bulk_edit.filter = filters.active.clone();
                tick_what_the_filter_finds(bulk_edit, model);
            }
        }
        SelectionBulkBodyAction::DeleteFilter => {
            let name = bulk_edit.selected_saved_filter.clone();
            if let Some(name) = name
                && filters.remove(&name)
            {
                bulk_edit.selected_saved_filter = None;
                bulk_edit.error = None;
            }
        }
    }
}

/// After the filter changed, by typing or by loading a saved one: what it
/// finds is ticked, and everything read from the ticks is read again, the
/// model sections they can take among it.
fn tick_what_the_filter_finds(bulk_edit: &mut SelectionBulkEditDialogState, model: &Model) {
    let rows = build_rows(model, &bulk_edit.filter, bulk_edit.property);
    bulk_edit.selected = rows
        .iter()
        .filter(|row| row.editable())
        .map(|row| row.key.clone())
        .collect();
    refresh_bound_sections(bulk_edit, model, &rows);
    bulk_edit.error = None;
    bulk_edit.authority = Some(SelectionBulkEditAuthority::capture(model));
}

fn commit_selection_bulk_edit(
    bulk_edit: &mut SelectionBulkEditDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<(), String> {
    let dialog = &*bulk_edit;
    let authority = dialog
        .authority
        .as_ref()
        .ok_or_else(|| "Bulk-edit document authority is missing.".to_owned())?;
    authority.validate_commit(model)?;
    let rows = build_rows(model, &dialog.filter, dialog.property);
    let section = dialog.new_value.trim();
    if dialog.property == SelectionBulkProperty::ModelSection
        && dialog.unset_behavior != SelectionBulkUnsetBehavior::RestoreInheritedValue
        && !section.is_empty()
        && !section.eq_ignore_ascii_case("inherit")
        && !bound_library_sections(model, &rows, &dialog.selected)
            .iter()
            .any(|bound| bound.eq_ignore_ascii_case(section))
    {
        return Err(format!(
            "Section {section} is not defined by every ticked part's library."
        ));
    }
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
        &mut model.schematic,
        &selected_active_ids,
        dialog.property,
        &dialog.new_value,
        dialog.unset_behavior,
        Some(&dialog.filter.current_property),
    )?;
    if receipt.changed == 0 {
        return Err("The selected objects already satisfy the requested value.".to_owned());
    }
    sync_active_schematic(model, workbench);
    let changed = format!(
        "Bulk edit changed {} object{}",
        receipt.changed,
        if receipt.changed == 1 { "" } else { "s" },
    );
    let analyses = model.sim_setup.enabled_analysis_instance_count();
    workbench.prompt(if changes_the_circuit(dialog.property) && analyses > 0 {
        format!(
            "{changed}; {analyses} enabled analys{} affected.",
            if analyses == 1 { "is" } else { "es" },
        )
    } else {
        format!("{changed}.")
    });
    bulk_edit.close();
    Ok(())
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
                    let response = text_field(
                        ui,
                        TextEdit::singleline(&mut dialog.filter.query)
                            .hint_text("Find instance, property, model, expression or path\u{2026}")
                            .margin(egui::Margin {
                                left: 29,
                                right: 8,
                                top: 5,
                                bottom: 5,
                            }),
                        width,
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
                saved_filter_controls(ui, dialog, saved_filters, &mut action);
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

/// The kit's select over `options`, writing the one picked to `value`;
/// whether that changed it. The dialog's choices were raw combo boxes with
/// egui's own triangle, the only controls in it the kit did not draw.
fn pick<T: Copy + PartialEq>(
    ui: &mut Ui,
    id: &str,
    label: &str,
    value: &mut T,
    options: &[(T, &str)],
    width: f32,
) -> bool {
    let selected = options
        .iter()
        .find_map(|(candidate, label)| (*candidate == *value).then_some(*label))
        .unwrap_or_default();
    let labels = options
        .iter()
        .map(|(_, label)| (*label).to_owned())
        .collect::<Vec<_>>();
    match select(ui, id, label, selected, &labels, width) {
        Some(index) if options[index].0 != *value => {
            *value = options[index].0;
            true
        }
        _ => false,
    }
}

fn saved_filter_controls(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    saved_filters: &[String],
    action: &mut SelectionBulkBodyAction,
) {
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.label("Filter name");
        let response = text_field(
            ui,
            TextEdit::singleline(&mut dialog.save_filter_name),
            220.0,
        );
        name_control(ui, &response, "Filter name");
        if Button::new("Save").accent().show(ui).clicked() {
            *action = SelectionBulkBodyAction::SaveFilter;
        }
        if !saved_filters.is_empty() {
            if let Some(index) = select(
                ui,
                "selection-bulk-saved-filter",
                "Saved filters",
                dialog
                    .selected_saved_filter
                    .as_deref()
                    .unwrap_or("Saved filters"),
                saved_filters,
                180.0,
            ) {
                dialog.selected_saved_filter = Some(saved_filters[index].clone());
            }
            let has_saved = dialog.selected_saved_filter.is_some();
            if Button::new("Load").enabled(has_saved).show(ui).clicked() {
                *action = SelectionBulkBodyAction::LoadFilter;
            }
            if Button::new("Delete").enabled(has_saved).show(ui).clicked() {
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
        let width = columns[0].available_width();
        if pick(
            &mut columns[0],
            "selection-bulk-object-kind",
            "Object kinds",
            &mut dialog.filter.object_kind,
            &SelectionBulkObjectKind::ALL.map(|kind| (kind, kind.label())),
            width,
        ) {
            *action = SelectionBulkBodyAction::FilterChanged;
        }
        columns[1].label("Hierarchy scope");
        let width = columns[1].available_width();
        if pick(
            &mut columns[1],
            "selection-bulk-hierarchy-scope",
            "Hierarchy scope",
            &mut dialog.filter.hierarchy_scope,
            &SelectionBulkHierarchyScope::ALL.map(|scope| {
                let label = if scope == SelectionBulkHierarchyScope::ActiveHierarchyPath {
                    hierarchy_path
                } else {
                    scope.label()
                };
                (scope, label)
            }),
            width,
        ) {
            *action = SelectionBulkBodyAction::FilterChanged;
        }
    });
    ui.add_space(9.0);
    ui.columns(2, |columns| {
        columns[0].label("Model / cell");
        if field(
            &mut columns[0],
            "Model / cell",
            &mut dialog.filter.model_cell,
            "Any",
        )
        .changed()
        {
            *action = SelectionBulkBodyAction::FilterChanged;
        }
        columns[1].label("Current property");
        if field(
            &mut columns[1],
            "Current property",
            &mut dialog.filter.current_property,
            "name = value",
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
        selection_results(ui, dialog, rows, action);
    });
}

fn selection_results(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    rows: &[SelectionBulkRow],
    action: &mut SelectionBulkBodyAction,
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
                        // A row the property cannot reach reads unticked,
                        // though it stays chosen for the next property.
                        let mut selected = row.editable() && dialog.selected.contains(&row.key);
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
                            *action = SelectionBulkBodyAction::TicksChanged;
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
        let width = columns[0].available_width();
        if pick(
            &mut columns[0],
            "selection-bulk-property",
            "Property",
            &mut dialog.property,
            &BULK_PROPERTIES.map(|property| (property, bulk_property_label(property))),
            width,
        ) {
            dialog.new_value.clear();
            *action = SelectionBulkBodyAction::PropertyChanged;
        }
        columns[1].label("New value");
        if new_value_control(&mut columns[1], dialog) {
            dialog.error = None;
        }
    });
    ui.add_space(9.0);
    ui.label("Unset behavior");
    let width = ui.available_width();
    pick(
        ui,
        "selection-bulk-unset",
        "Unset behavior",
        &mut dialog.unset_behavior,
        &BULK_UNSET_BEHAVIORS.map(|behavior| (behavior, bulk_unset_label(behavior))),
        width,
    );
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
    // A bill-of-materials field or how a part is lettered changes no
    // analysis, so the count is shown only for what the netlist carries.
    if changes_the_circuit(dialog.property) {
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
}

/// Whether `property` reaches the netlist, so that an edit of it changes
/// what the enabled analyses compute.
const fn changes_the_circuit(property: SelectionBulkProperty) -> bool {
    !matches!(
        property,
        SelectionBulkProperty::Display | SelectionBulkProperty::Bom(_)
    )
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

/// A text field across `ui`'s width at the selects' height, named by the
/// label its column states above it.
fn field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> egui::Response {
    let width = ui.available_width();
    let response = text_field(
        ui,
        TextEdit::singleline(value)
            .font(theme::field_mono())
            .hint_text(hint)
            .margin(egui::Margin::symmetric(8, 4)),
        width,
    );
    name_control(ui, &response, label);
    response
}

fn new_value_control(ui: &mut Ui, dialog: &mut SelectionBulkEditDialogState) -> bool {
    match dialog.property {
        SelectionBulkProperty::ModelSection => {
            let before = dialog.new_value.clone();
            // The footer says why the list is empty; the control says only
            // that it is.
            let shown = if dialog.bound_sections.is_empty() {
                "None defined"
            } else if dialog.new_value.is_empty() {
                "Choose a section"
            } else {
                dialog.new_value.as_str()
            }
            .to_owned();
            ui.add_enabled_ui(!dialog.bound_sections.is_empty(), |ui| {
                let width = ui.available_width();
                if let Some(index) = select(
                    ui,
                    "selection-bulk-model-section",
                    "Model section",
                    &shown,
                    &dialog.bound_sections,
                    width,
                ) {
                    dialog.new_value = dialog.bound_sections[index].clone();
                }
            });
            before != dialog.new_value
        }
        SelectionBulkProperty::Display => {
            let before = dialog.new_value.clone();
            let modes = [
                ComponentDisplayMode::NameAndValue,
                ComponentDisplayMode::NameOnly,
                ComponentDisplayMode::ValueOnly,
                ComponentDisplayMode::Hidden,
                ComponentDisplayMode::Inherit,
            ]
            .map(|mode| mode.label().to_owned());
            let width = ui.available_width();
            let shown = dialog.new_value.clone();
            if let Some(index) = select(
                ui,
                "selection-bulk-display",
                "Display",
                &shown,
                &modes,
                width,
            ) {
                dialog.new_value = modes[index].clone();
            }
            before != dialog.new_value
        }
        SelectionBulkProperty::ToleranceDistribution => {
            let before = dialog.new_value.clone();
            let labels =
                ToleranceDistribution::ALL.map(|distribution| distribution.label().to_owned());
            let shown = ToleranceDistribution::ALL
                .into_iter()
                .find(|distribution| distribution.keyword() == dialog.new_value)
                .map_or("Choose a distribution", ToleranceDistribution::label);
            let width = ui.available_width();
            if let Some(index) = select(
                ui,
                "selection-bulk-tolerance-distribution",
                "Distribution",
                shown,
                &labels,
                width,
            ) {
                ToleranceDistribution::ALL[index]
                    .keyword()
                    .clone_into(&mut dialog.new_value);
            }
            before != dialog.new_value
        }
        SelectionBulkProperty::Value
        | SelectionBulkProperty::Temperature
        | SelectionBulkProperty::Tolerance
        | SelectionBulkProperty::ParameterOverride
        | SelectionBulkProperty::Bom(_) => {
            field(ui, "New value", &mut dialog.new_value, "").changed()
        }
    }
}

const NO_BOUND_SECTIONS: &str = "No model section is defined by every bound library.";

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

/// Re-reads the sections the ticked parts can take, and lets go of a chosen
/// section that one of them no longer has.
fn refresh_bound_sections(
    dialog: &mut SelectionBulkEditDialogState,
    model: &Model,
    rows: &[SelectionBulkRow],
) {
    dialog.bound_sections = bound_library_sections(model, rows, &dialog.selected);
    if dialog.property == SelectionBulkProperty::ModelSection
        && !dialog.new_value.is_empty()
        && !dialog
            .bound_sections
            .iter()
            .any(|section| section.eq_ignore_ascii_case(&dialog.new_value))
    {
        dialog.new_value.clear();
    }
}

/// The sections every ticked instance's model library defines, once each.
///
/// A section is written to every ticked part, so it is offered only when
/// each of their libraries has it, as Virtuoso offers only the sections of
/// the library being edited. With nothing ticked, the matched instances
/// stand in for the ticks. Read from the same library records Models ▸
/// Corners & sections lists: the sections each library's corners bind, and
/// the sections its model metadata declares. A library this project does
/// not hold defines nothing, so a part bound to one leaves nothing to offer.
fn bound_library_sections(
    model: &Model,
    rows: &[SelectionBulkRow],
    selected: &BTreeSet<SelectionBulkRowKey>,
) -> Vec<String> {
    let instance_rows = || {
        rows.iter()
            .filter(|row| row.key.kind == SelectionBulkRowKind::Instance)
    };
    let mut instances = instance_rows()
        .filter(|row| row.editable() && selected.contains(&row.key))
        .map(|row| row.key.object_id)
        .collect::<BTreeSet<_>>();
    if instances.is_empty() {
        instances = instance_rows().map(|row| row.key.object_id).collect();
    }
    let libraries = model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| instances.contains(&component.id))
        .filter_map(|component| component.library_cell.as_ref())
        .map(|binding| binding.library.as_str())
        .collect::<BTreeSet<_>>();
    let mut common: Option<Vec<String>> = None;
    for name in libraries {
        let mut sections = model
            .model_library_manager
            .get_library(name)
            .map(|library| {
                library
                    .corners
                    .values()
                    .flat_map(|corner| corner.effective_section_bindings())
                    .map(|binding| binding.section)
                    .chain(
                        library
                            .model_definition_metadata
                            .values()
                            .flat_map(|metadata| metadata.sections.iter())
                            .map(|section| section.name.clone()),
                    )
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        sections.retain(|section| !section.trim().is_empty());
        common = Some(match common {
            None => sections,
            Some(mut kept) => {
                kept.retain(|section| {
                    sections
                        .iter()
                        .any(|other| other.eq_ignore_ascii_case(section))
                });
                kept
            }
        });
    }
    let mut sections = common.unwrap_or_default();
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    sections
}

/// The occurrence the active schematic is open at — the dialog's scope line and
/// the display path of every row it owns. The design root is implicit, so the
/// top sheet is `/` rather than the name of the cell that happens to be top.
fn active_hierarchy_path(model: &Model) -> String {
    model.workspace.content.occurrence_path().to_string()
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
    model: &Model,
    filter: &SelectionBulkFilter,
    property: SelectionBulkProperty,
) -> Vec<SelectionBulkRow> {
    let active_key = model.workspace.content.active_schematic_reference().key();
    let mut buffers = Vec::new();
    buffers.push((
        active_key.clone(),
        active_hierarchy_path(model),
        model.schematic.document(),
        true,
    ));
    match filter.hierarchy_scope {
        SelectionBulkHierarchyScope::ActiveHierarchyPath => {
            let hierarchy_keys = model
                .workspace
                .content
                .hierarchy_stack
                .iter()
                .map(|reference| reference.key())
                .collect::<HashSet<_>>();
            for (key, schematic) in &model.workspace.content.schematic_buffers {
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
            for (key, schematic) in &model.workspace.content.schematic_buffers {
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
    let active_sheet = model
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
            &model.workspace.content.design_management,
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
    for component in &schematic.components {
        let is_port = component.kind == ComponentType::Port;
        if (filter.object_kind == SelectionBulkObjectKind::InstancesAndParameters && is_port)
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
            for (index, (terminal, _)) in component.terminal_positions().iter().enumerate() {
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
        || (property == SelectionBulkProperty::Value && component.kind == ComponentType::Ground)
        || (property == SelectionBulkProperty::Temperature
            && !takes_instance_temperature(component.kind))
        || (property == SelectionBulkProperty::ModelSection && component.library_cell.is_none())
        || (matches!(
            property,
            SelectionBulkProperty::Tolerance | SelectionBulkProperty::ToleranceDistribution
        ) && !supports_tolerance(component.kind))
    {
        Some("Not applicable".to_owned())
    } else if parse_replacement_parameters_strict(&component.params).is_err()
        && matches!(
            property,
            SelectionBulkProperty::Temperature
                | SelectionBulkProperty::Tolerance
                | SelectionBulkProperty::ToleranceDistribution
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
            "{} {} {} {} {} {} {} {}",
            component.kind.display_name(),
            component.name,
            component.value,
            component.params,
            binding_search,
            component.bom.manufacturer,
            component.bom.mpn,
            component.bom.description
        ),
        lock,
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
        other => match BomField::ALL
            .into_iter()
            .find(|field| field.label().eq_ignore_ascii_case(other))
        {
            Some(field) => Some(component.bom.get(field).to_owned()),
            None => parse_replacement_parameters_strict(&component.params)
                .ok()
                .and_then(|parameters| parameters.get(other).cloned()),
        },
    }
}

fn row_matches_query(row: &SelectionBulkRow, filter: &SelectionBulkFilter) -> bool {
    let query = filter.query.trim();
    query.is_empty()
        || [&row.object, &row.path, &row.value, &row.search_text]
            .into_iter()
            .any(|text| search_match(query, text))
}

/// The result search finds as it is typed: a query without `*` is found
/// anywhere in the text, and one with `*` is a pattern.
fn search_match(query: &str, text: &str) -> bool {
    if query.contains('*') {
        wildcard_match(query, text)
    } else {
        text.to_ascii_lowercase()
            .contains(&query.trim().to_ascii_lowercase())
    }
}

/// Whether `candidate` matches `pattern`, ignoring case: exactly, unless the
/// pattern holds a `*`, which stands for any run of characters. `value = 1k`
/// finds 1k and not 5.1k; `R*0` finds R100, `*0` anything ending in 0.
fn wildcard_match(pattern: &str, candidate: &str) -> bool {
    let pattern = pattern.trim().to_ascii_lowercase();
    let candidate = candidate.to_ascii_lowercase();
    if pattern.is_empty() {
        return true;
    }
    let mut pieces = pattern.split('*');
    let head = pieces.next().unwrap_or_default();
    let Some(mut rest) = candidate.strip_prefix(head) else {
        return false;
    };
    let pieces = pieces.collect::<Vec<_>>();
    let Some((tail, middle)) = pieces.split_last() else {
        // No `*`: the whole text is the pattern.
        return rest.is_empty();
    };
    // Each middle piece at its first place after the last; the tail is held
    // to the end, so a later occurrence of it is the one that counts.
    for piece in middle {
        let Some(position) = rest.find(piece) else {
            return false;
        };
        rest = &rest[position + piece.len()..];
    }
    rest.ends_with(tail)
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests;
