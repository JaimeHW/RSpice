//! Governed schematic Cut, Duplicate, Delete, and Select All workflows.
//!
//! The upgraded workbench mockup makes the topology consequences and scope of
//! these otherwise terse edit commands explicit. Each dialog retains the exact
//! document and selection authority reviewed by the user, then delegates the
//! commit to the existing typed clipboard, undo, naming, and selection engines.

use std::collections::{BTreeSet, HashMap, HashSet};

use egui::{Context, Frame, Margin, Stroke, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::schematic::view::SchematicSymbolContext;
use crate::simulation::netlist_gen::{DesignNet, HierarchySource, design_nets_with_hierarchy};
use crate::state::{
    ClipboardData, ComponentType, Point, SchematicSelectionFilter, SchematicSnapshot,
    SchematicState, Selection,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, select_mono_with_response,
};
use crate::state::SchematicHierarchyVisibility;

use crate::workbench::app::{AppState, RSpiceApp, SchematicEditAuthority};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SelectionWorkflowKind {
    #[default]
    Cut,
    Duplicate,
    Delete,
    SelectAll,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DuplicateExternalNets {
    #[default]
    LeaveUnconnected,
    PreserveNamedNetAttachment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SelectAllScope {
    #[default]
    ActiveCellView,
    VisibleEditInPlaceHierarchy,
    CurrentLayerObjectFilter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectAllClasses {
    instances: bool,
    wires: bool,
    labels: bool,
    pins: bool,
}

impl Default for SelectAllClasses {
    fn default() -> Self {
        Self {
            instances: true,
            wires: true,
            labels: true,
            pins: true,
        }
    }
}

impl SelectAllClasses {
    const fn any(self) -> bool {
        self.instances || self.wires || self.labels || self.pins
    }

    const fn intersect_filter(self, filter: SchematicSelectionFilter) -> Self {
        Self {
            instances: self.instances && filter.instances,
            wires: self.wires && filter.wires,
            labels: self.labels && filter.labels,
            pins: self.pins && filter.instances,
        }
    }
}

#[derive(Debug, Clone)]
struct RetainedSchematicAuthority {
    key: String,
    topology_version: u64,
    snapshot: SchematicSnapshot,
    selection: Selection,
}

#[derive(Debug, Clone)]
struct SelectionWorkflowAuthority {
    active: SchematicEditAuthority,
    active_key: String,
    hierarchy_keys: Vec<String>,
    hierarchy_buffers: Vec<RetainedSchematicAuthority>,
    hierarchy_visibility: SchematicHierarchyVisibility,
    selection_filter: SchematicSelectionFilter,
}

impl SelectionWorkflowAuthority {
    fn capture(state: &AppState) -> Self {
        let active_key = state.workspace.active_schematic_reference().key();
        let hierarchy_keys = hierarchy_stack_keys(state);
        let hierarchy_buffers = hierarchy_keys
            .iter()
            .filter(|key| **key != active_key)
            .filter_map(|key| {
                state.workspace.schematic_buffers.get(key).map(|schematic| {
                    RetainedSchematicAuthority {
                        key: key.clone(),
                        topology_version: schematic.topology_version(),
                        snapshot: SchematicSnapshot::capture(schematic),
                        selection: schematic.selection.clone(),
                    }
                })
            })
            .collect();
        Self {
            active: SchematicEditAuthority::capture(state),
            active_key,
            hierarchy_keys,
            hierarchy_buffers,
            hierarchy_visibility: state.ui.schematic_visibility.hierarchy,
            selection_filter: state.ui.schematic_selection_filter,
        }
    }

    fn validate(
        &self,
        state: &AppState,
        kind: SelectionWorkflowKind,
        scope: SelectAllScope,
    ) -> Result<(), String> {
        let command = kind.title();
        if kind == SelectionWorkflowKind::SelectAll {
            self.active.validate_presentation(state, command)?;
        } else {
            self.active.validate(state, command)?;
        }
        if self.active_key != state.workspace.active_schematic_reference().key() {
            return Err(format!(
                "The active hierarchy owner changed. Close and reopen {command}."
            ));
        }
        if kind != SelectionWorkflowKind::SelectAll {
            return Ok(());
        }
        if self.hierarchy_visibility != state.ui.schematic_visibility.hierarchy {
            return Err(format!(
                "Hierarchy visibility changed. Close and reopen {command}."
            ));
        }
        if scope == SelectAllScope::CurrentLayerObjectFilter
            && self.selection_filter != state.ui.schematic_selection_filter
        {
            return Err(format!(
                "The current object filter changed. Close and reopen {command}."
            ));
        }
        if scope != SelectAllScope::VisibleEditInPlaceHierarchy {
            return Ok(());
        }
        if self.hierarchy_keys != hierarchy_stack_keys(state) {
            return Err(format!(
                "The visible edit-in-place hierarchy changed. Close and reopen {command}."
            ));
        }
        for retained in &self.hierarchy_buffers {
            let Some(schematic) = state.workspace.schematic_buffers.get(&retained.key) else {
                return Err(format!(
                    "A visible hierarchy buffer closed. Close and reopen {command}."
                ));
            };
            if retained.topology_version != schematic.topology_version()
                || retained.selection != schematic.selection
                || !retained.snapshot.is_equal_state(schematic)
            {
                return Err(format!(
                    "A visible hierarchy buffer changed. Close and reopen {command}."
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SelectionWorkflowDialogState {
    pub(crate) open: bool,
    kind: SelectionWorkflowKind,
    authority: Option<SelectionWorkflowAuthority>,
    duplicate_external_nets: DuplicateExternalNets,
    select_all_scope: SelectAllScope,
    select_all_classes: SelectAllClasses,
    duplicate_anchor: Point,
    transaction_selection: Selection,
    source_object_count: usize,
    cut_open_net_count: usize,
    pub(crate) error: Option<String>,
}

impl Default for SelectionWorkflowDialogState {
    fn default() -> Self {
        Self {
            open: false,
            kind: SelectionWorkflowKind::Cut,
            authority: None,
            duplicate_external_nets: DuplicateExternalNets::LeaveUnconnected,
            select_all_scope: SelectAllScope::ActiveCellView,
            select_all_classes: SelectAllClasses::default(),
            duplicate_anchor: Point::origin(),
            transaction_selection: Selection::default(),
            source_object_count: 0,
            cut_open_net_count: 0,
            error: None,
        }
    }
}

impl SelectionWorkflowDialogState {
    fn begin(
        &mut self,
        kind: SelectionWorkflowKind,
        authority: SelectionWorkflowAuthority,
        duplicate_anchor: Point,
        transaction_selection: Selection,
        source_object_count: usize,
        cut_open_net_count: usize,
    ) {
        *self = Self {
            open: true,
            kind,
            authority: Some(authority),
            duplicate_external_nets: DuplicateExternalNets::LeaveUnconnected,
            select_all_scope: SelectAllScope::ActiveCellView,
            select_all_classes: SelectAllClasses::default(),
            duplicate_anchor,
            transaction_selection,
            source_object_count,
            cut_open_net_count,
            error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    fn validate(&self, state: &AppState) -> Result<(), String> {
        let authority = self
            .authority
            .as_ref()
            .ok_or_else(|| "The retained schematic authority is unavailable.".to_owned())?;
        authority.validate(state, self.kind, self.select_all_scope)?;
        match self.kind {
            SelectionWorkflowKind::Cut
            | SelectionWorkflowKind::Duplicate
            | SelectionWorkflowKind::Delete
                if self.source_object_count == 0 =>
            {
                Err("The reviewed selection no longer contains a complete object.".to_owned())
            }
            SelectionWorkflowKind::SelectAll if !self.select_all_classes.any() => {
                Err("Select at least one object class.".to_owned())
            }
            SelectionWorkflowKind::SelectAll if self.estimated_count(state) == 0 => {
                Err("No objects match the selected scope and object classes.".to_owned())
            }
            _ => Ok(()),
        }
    }

    fn estimated_count(&self, state: &AppState) -> usize {
        select_all_targets(state, self.select_all_scope, self.select_all_classes)
            .iter()
            .map(|(_, selection)| selection.count())
            .sum()
    }
}

impl SelectionWorkflowKind {
    const fn eyebrow(self) -> &'static str {
        match self {
            Self::Cut => "EDIT \u{00b7} CONNECTIVITY SAFE",
            Self::Duplicate => "EDIT \u{00b7} GRID PLACEMENT",
            Self::Delete => "EDIT \u{00b7} CONNECTIVITY IMPACT",
            Self::SelectAll => "EDIT \u{00b7} SELECTION SCOPE",
        }
    }

    const fn title(self) -> &'static str {
        match self {
            Self::Cut => "Cut schematic selection",
            Self::Duplicate => "Duplicate schematic selection",
            Self::Delete => "Delete schematic selection",
            Self::SelectAll => "Select all in edit context",
        }
    }

    const fn primary(self) -> &'static str {
        match self {
            Self::Cut => "Cut to project clipboard",
            Self::Duplicate => "Duplicate and place",
            Self::Delete => "Delete selection",
            Self::SelectAll => "Select objects",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::Cut => {
                "Remove selected objects as one undoable transaction and retain explicit dangling-net review."
            }
            Self::Duplicate => {
                "Duplicate selected instances, parameters, labels, and internal connectivity with new stable identifiers."
            }
            Self::Delete => {
                "Delete as one undoable transaction after reviewing affected nets, probes, specifications, and comments."
            }
            Self::SelectAll => {
                "Choose whether selection is limited to the active cell, visible hierarchy, or filtered object classes."
            }
        }
    }
}

pub(crate) fn open_cut_selection_dialog(state: &mut AppState) -> bool {
    open_selection_workflow(state, SelectionWorkflowKind::Cut, None)
}

pub(crate) fn open_duplicate_selection_dialog(state: &mut AppState) -> bool {
    let anchor = state.schematic_paste_anchor() + Point::new(2, 2);
    open_selection_workflow(state, SelectionWorkflowKind::Duplicate, Some(anchor))
}

pub(crate) fn open_duplicate_selection_dialog_at(state: &mut AppState, anchor: Point) -> bool {
    open_selection_workflow(state, SelectionWorkflowKind::Duplicate, Some(anchor))
}

#[derive(Debug, Clone)]
struct DeleteDependencyImpact {
    affected_nets: String,
    dependent_records: String,
}

pub(crate) fn open_delete_selection_dialog(state: &mut AppState) -> bool {
    open_selection_workflow(state, SelectionWorkflowKind::Delete, None)
}

pub(crate) fn open_select_all_dialog(state: &mut AppState) -> bool {
    open_selection_workflow(state, SelectionWorkflowKind::SelectAll, None)
}

fn open_selection_workflow(
    state: &mut AppState,
    kind: SelectionWorkflowKind,
    duplicate_anchor: Option<Point>,
) -> bool {
    if state.dialogs.selection_workflow.open {
        return false;
    }
    if kind != SelectionWorkflowKind::SelectAll
        && (state.schematic.read_only || state.active_view_read_only())
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "{} is unavailable because the active schematic is read-only.",
            kind.title()
        )));
        return false;
    }

    let transaction_selection = if kind == SelectionWorkflowKind::SelectAll {
        state.schematic.selection.clone()
    } else {
        crate::schematic::view::sheet_visibility::selection_filtered_to_active_sheet(
            state,
            &state.schematic.selection,
        )
    };

    let captured_selection = (kind != SelectionWorkflowKind::SelectAll)
        .then(|| capture_selection(state, &transaction_selection));
    let source_object_count = if kind == SelectionWorkflowKind::SelectAll {
        0
    } else {
        complete_selection_count(&state.schematic, &transaction_selection)
    };
    if kind != SelectionWorkflowKind::SelectAll
        && (source_object_count == 0
            || captured_selection
                .as_ref()
                .is_none_or(ClipboardData::is_empty))
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "{} requires at least one complete selected object.",
            kind.title()
        )));
        return false;
    }

    let cut_open_net_count = if kind == SelectionWorkflowKind::Cut {
        cut_open_net_count(state, &transaction_selection)
    } else {
        0
    };
    let duplicate_anchor =
        duplicate_anchor.unwrap_or_else(|| state.schematic_paste_anchor() + Point::new(2, 2));
    let authority = SelectionWorkflowAuthority::capture(state);
    state.dialogs.selection_workflow.begin(
        kind,
        authority,
        duplicate_anchor,
        transaction_selection,
        source_object_count,
        cut_open_net_count,
    );
    true
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_selection_workflow_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.selection_workflow.open {
            return;
        }

        let draft = self.state.dialogs.selection_workflow.clone();
        let validation_error = draft.validate(&self.state).err();
        let can_commit = validation_error.is_none();
        let estimated_count = draft.estimated_count(&self.state);
        let visible_error = draft.error.as_deref().or(validation_error.as_deref());
        let kind = draft.kind;
        let delete_impact = (kind == SelectionWorkflowKind::Delete)
            .then(|| delete_dependency_impact(&self.state, &draft.transaction_selection));
        let recovery_available =
            kind == SelectionWorkflowKind::SelectAll && select_all_recovery_available(&self.state);

        let initial_focus = if kind == SelectionWorkflowKind::Delete {
            DialogInitialFocus::Ghost
        } else {
            DialogInitialFocus::BodyControl
        };
        let mut dialog = Dialog::new(kind.eyebrow(), kind.title(), kind.primary())
            .description(kind.description())
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .primary_enabled(can_commit)
            .initial_focus(initial_focus);
        if kind == SelectionWorkflowKind::Delete {
            dialog = dialog.destructive().hint("One schematic undo transaction");
        }
        if recovery_available {
            dialog = dialog.secondary("Restore prior selection");
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            Some(selection_workflow_body(
                ui,
                &mut self.state.dialogs.selection_workflow,
                estimated_count,
                visible_error,
                delete_impact.as_ref(),
            ))
        });

        match choice {
            DialogChoice::Primary => {
                let transaction = self.state.dialogs.selection_workflow.clone();
                match commit_selection_workflow(&mut self.state, &transaction) {
                    Ok(message) => {
                        self.state.dialogs.selection_workflow.close();
                        self.state.push_user_message(ConsoleMessage::info(message));
                    }
                    Err(error) => {
                        self.state.dialogs.selection_workflow.error = Some(error);
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.selection_workflow.close();
            }
            DialogChoice::Secondary if kind == SelectionWorkflowKind::SelectAll => {
                match restore_select_all_selection(&mut self.state) {
                    Ok(count) => {
                        self.state.dialogs.selection_workflow.close();
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Restored the prior selection across {count} schematic view(s)."
                        )));
                    }
                    Err(error) => {
                        self.state.dialogs.selection_workflow.error = Some(error);
                    }
                }
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn selection_workflow_body(
    ui: &mut Ui,
    draft: &mut SelectionWorkflowDialogState,
    estimated_count: usize,
    error: Option<&str>,
    delete_impact: Option<&DeleteDependencyImpact>,
) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let first = match draft.kind {
        SelectionWorkflowKind::Cut => {
            let selection = format!("{} complete typed objects", draft.source_object_count);
            let connectivity = match draft.cut_open_net_count {
                0 => "No retained net becomes open".to_owned(),
                1 => "1 net will become open".to_owned(),
                count => format!("{count} nets will become open"),
            };
            let first = read_only_row(
                ui,
                "Selection",
                "Exact active-sheet objects retained by stable identity.",
                &selection,
            );
            read_only_row(
                ui,
                "Connectivity",
                "Projected after the reviewed atomic removal.",
                &connectivity,
            );
            read_only_row(
                ui,
                "Clipboard",
                "Destination and object representation.",
                "Project-local typed objects",
            );
            first
        }
        SelectionWorkflowKind::Duplicate => {
            let selection = format!("{} complete typed objects", draft.source_object_count);
            let first = read_only_row(
                ui,
                "Selection",
                "Exact active-sheet objects retained by stable identity.",
                &selection,
            );
            read_only_row(
                ui,
                "Reference designators",
                "Collision-free instance naming at commit.",
                "Allocate next available",
            );
            let (picked, _) = select_row(
                ui,
                "External nets",
                "Whether duplicated terminals retain authored logical-net attachment.",
                "duplicate-external-net-policy",
                duplicate_net_label(draft.duplicate_external_nets),
                &["Leave unconnected", "Preserve named-net attachment"],
            );
            if let Some(index) = picked {
                draft.duplicate_external_nets = if index == 1 {
                    DuplicateExternalNets::PreserveNamedNetAttachment
                } else {
                    DuplicateExternalNets::LeaveUnconnected
                };
                draft.error = None;
            }
            first
        }
        SelectionWorkflowKind::Delete => {
            let impact = delete_impact.expect("delete workflow owns an impact report");
            let first = read_only_row(
                ui,
                "Selection",
                "Exact active-sheet objects retained by stable identity.",
                &format!("{} complete typed objects", draft.source_object_count),
            );
            read_only_row(
                ui,
                "Affected nets",
                "Named electrical connectivity touched by the reviewed objects.",
                &impact.affected_nets,
            );
            read_only_row(
                ui,
                "Dependent records",
                "Durable probes, specifications, and review comments that reference the affected design identities.",
                &impact.dependent_records,
            );
            first
        }
        SelectionWorkflowKind::SelectAll => {
            let (picked, first) = select_row(
                ui,
                "Scope",
                "Exact design ownership boundary for the resulting selection.",
                "select-all-scope",
                select_all_scope_label(draft.select_all_scope),
                &[
                    "Active cell view",
                    "Visible edit-in-place hierarchy",
                    "Current layer / object filter",
                ],
            );
            if let Some(index) = picked {
                draft.select_all_scope = match index {
                    1 => SelectAllScope::VisibleEditInPlaceHierarchy,
                    2 => SelectAllScope::CurrentLayerObjectFilter,
                    _ => SelectAllScope::ActiveCellView,
                };
                draft.error = None;
            }
            object_classes_row(ui, &mut draft.select_all_classes);
            read_only_row(
                ui,
                "Estimated",
                "Complete objects selected by the current scope and class policy.",
                &estimated_count.to_string(),
            );
            first
        }
    };

    if let Some(error) = error {
        let t = Tokens::get(ui.ctx());
        Frame::new()
            .fill(t.color.bg_app)
            .inner_margin(Margin::symmetric(12, 9))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.err),
                );
            });
    }
    first
}

fn read_only_row(ui: &mut Ui, title: &str, detail: &str, value: &str) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            ui.horizontal(|ui| {
                let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                ui.allocate_ui_with_layout(
                    vec2(copy_width, t.metrics.ctl_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| setting_copy(ui, title, detail),
                );
                ui.add_space(12.0);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), t.metrics.ctl_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new(value)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                        )
                    },
                )
                .inner
                .id
            })
            .inner
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    row.inner
}

fn select_row(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    id: &str,
    selected: &str,
    options: &[&str],
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
            ui.horizontal(|ui| {
                let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                ui.allocate_ui_with_layout(
                    vec2(copy_width, t.metrics.ctl_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| setting_copy(ui, title, detail),
                );
                ui.add_space(12.0);
                let control_width = ui.available_width();
                let output =
                    select_mono_with_response(ui, id, title, selected, &options, control_width);
                (output.picked, output.response.id)
            })
            .inner
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    row.inner
}

fn object_classes_row(ui: &mut Ui, classes: &mut SelectAllClasses) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            ui.horizontal(|ui| {
                let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                ui.allocate_ui_with_layout(
                    vec2(copy_width, t.metrics.ctl_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        setting_copy(
                            ui,
                            "Object classes",
                            "Mockup-defined selectable electrical object taxonomy.",
                        )
                    },
                );
                ui.add_space(12.0);
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut classes.instances, "instances");
                    ui.checkbox(&mut classes.wires, "wires");
                    ui.checkbox(&mut classes.labels, "labels");
                    ui.checkbox(&mut classes.pins, "pins");
                });
            });
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
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

const fn duplicate_net_label(value: DuplicateExternalNets) -> &'static str {
    match value {
        DuplicateExternalNets::LeaveUnconnected => "Leave unconnected",
        DuplicateExternalNets::PreserveNamedNetAttachment => "Preserve named-net attachment",
    }
}

const fn select_all_scope_label(value: SelectAllScope) -> &'static str {
    match value {
        SelectAllScope::ActiveCellView => "Active cell view",
        SelectAllScope::VisibleEditInPlaceHierarchy => "Visible edit-in-place hierarchy",
        SelectAllScope::CurrentLayerObjectFilter => "Current layer / object filter",
    }
}

fn commit_selection_workflow(
    state: &mut AppState,
    draft: &SelectionWorkflowDialogState,
) -> Result<String, String> {
    draft.validate(state)?;
    match draft.kind {
        SelectionWorkflowKind::Cut => commit_cut(state, draft),
        SelectionWorkflowKind::Duplicate => commit_duplicate(state, draft),
        SelectionWorkflowKind::Delete => commit_delete(state, draft),
        SelectionWorkflowKind::SelectAll => commit_select_all(state, draft),
    }
}

fn commit_delete(
    state: &mut AppState,
    draft: &SelectionWorkflowDialogState,
) -> Result<String, String> {
    let previous_selection = state.schematic.selection.clone();
    state.schematic.selection = draft.transaction_selection.clone();
    let committed = crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
        state,
        |schematic| schematic.delete_selection(),
    );
    if !committed {
        state.schematic.selection = previous_selection;
        return Err("The reviewed selection could not be removed atomically.".to_owned());
    }
    state.sync_active_schematic_to_workspace();
    Ok(format!(
        "Deleted {} typed objects in one undoable transaction.",
        draft.source_object_count
    ))
}

fn commit_cut(
    state: &mut AppState,
    draft: &SelectionWorkflowDialogState,
) -> Result<String, String> {
    let previous_clipboard = state.schematic.clipboard.clone();
    let previous_selection = state.schematic.selection.clone();
    state.schematic.selection = draft.transaction_selection.clone();
    if !state.copy_active_schematic_selection() {
        state.schematic.selection = previous_selection;
        return Err("The reviewed selection could not be copied.".to_owned());
    }
    let committed = crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
        state,
        |schematic| schematic.delete_selection(),
    );
    if !committed {
        state.schematic.clipboard = previous_clipboard;
        state.schematic.selection = previous_selection;
        return Err("The reviewed selection could not be removed atomically.".to_owned());
    }
    state.sync_active_schematic_to_workspace();
    Ok(format!(
        "Cut {} typed objects to the project clipboard in one undoable transaction; {}.",
        draft.source_object_count,
        match draft.cut_open_net_count {
            0 => "no retained net became open".to_owned(),
            1 => "1 retained net became open".to_owned(),
            count => format!("{count} retained nets became open"),
        }
    ))
}

fn commit_duplicate(
    state: &mut AppState,
    draft: &SelectionWorkflowDialogState,
) -> Result<String, String> {
    let previous_clipboard = state.schematic.clipboard.clone();
    let previous_selection = state.schematic.selection.clone();
    state.schematic.selection = draft.transaction_selection.clone();
    if !state.copy_active_schematic_selection() {
        state.schematic.selection = previous_selection;
        return Err("The reviewed selection could not be copied.".to_owned());
    }
    let attachment_count =
        if draft.duplicate_external_nets == DuplicateExternalNets::PreserveNamedNetAttachment {
            let attachments = named_external_attachments(state);
            state
                .schematic
                .clipboard
                .preserve_named_net_attachments(attachments)
        } else {
            0
        };
    if !state.schematic.paste_at(draft.duplicate_anchor) {
        state.schematic.clipboard = previous_clipboard;
        state.schematic.selection = previous_selection;
        return Err(
            "The duplicate could not be committed at the reviewed canvas target.".to_owned(),
        );
    }
    state.sync_active_schematic_to_workspace();
    Ok(format!(
        "Duplicated {} typed objects with next-available references{}.",
        draft.source_object_count,
        if attachment_count == 0 {
            String::new()
        } else {
            format!(" and {attachment_count} named-net attachments")
        }
    ))
}

fn commit_select_all(
    state: &mut AppState,
    draft: &SelectionWorkflowDialogState,
) -> Result<String, String> {
    let targets = select_all_targets(state, draft.select_all_scope, draft.select_all_classes);
    let selected_count = targets
        .iter()
        .map(|(_, selection)| selection.count())
        .sum::<usize>();
    if selected_count == 0 {
        return Err("No objects match the selected scope and object classes.".to_owned());
    }
    let active_key = state.workspace.active_schematic_reference().key();
    let mut previous_selections = HashMap::with_capacity(targets.len());
    for (key, _) in &targets {
        let selection = if key == &active_key {
            state.schematic.selection.clone()
        } else {
            state
                .workspace
                .schematic_buffers
                .get(key)
                .ok_or_else(|| {
                    format!("The hierarchy buffer {key} closed before selection could be applied.")
                })?
                .selection
                .clone()
        };
        previous_selections.insert(key.clone(), selection);
    }
    for (key, selection) in targets {
        if key == active_key {
            state.schematic.selection = selection;
        } else if let Some(schematic) = state.workspace.schematic_buffers.get_mut(&key) {
            schematic.selection = selection;
        } else {
            return Err(format!(
                "The hierarchy buffer {key} closed before selection could be applied."
            ));
        }
    }
    if let Some(buffer) = state.workspace.schematic_buffers.get_mut(&active_key) {
        buffer.selection = state.schematic.selection.clone();
    }
    state.ui.schematic_selection_recovery = Some(crate::workbench::SchematicSelectionRecovery {
        active_key,
        selections: previous_selections,
    });
    Ok(format!(
        "Selected {selected_count} objects in {}.",
        select_all_scope_label(draft.select_all_scope).to_ascii_lowercase()
    ))
}

fn select_all_recovery_available(state: &AppState) -> bool {
    let Some(recovery) = &state.ui.schematic_selection_recovery else {
        return false;
    };
    recovery.active_key == state.workspace.active_schematic_reference().key()
        && recovery.selections.keys().all(|key| {
            key == &recovery.active_key || state.workspace.schematic_buffers.contains_key(key)
        })
}

fn restore_select_all_selection(state: &mut AppState) -> Result<usize, String> {
    let recovery = state
        .ui
        .schematic_selection_recovery
        .clone()
        .ok_or_else(|| "No prior Select All selection is available.".to_owned())?;
    let active_key = state.workspace.active_schematic_reference().key();
    if recovery.active_key != active_key {
        return Err(
            "The active hierarchy owner changed; its prior selection cannot be restored."
                .to_owned(),
        );
    }
    if let Some(missing) = recovery.selections.keys().find(|key| {
        key.as_str() != active_key.as_str() && !state.workspace.schematic_buffers.contains_key(*key)
    }) {
        return Err(format!(
            "The hierarchy buffer {missing} closed; no selection was restored."
        ));
    }
    for (key, selection) in &recovery.selections {
        if key == &active_key {
            state.schematic.selection = selection.clone();
        } else if let Some(schematic) = state.workspace.schematic_buffers.get_mut(key) {
            schematic.selection = selection.clone();
        }
    }
    if let Some(buffer) = state.workspace.schematic_buffers.get_mut(&active_key) {
        buffer.selection = state.schematic.selection.clone();
    }
    state.ui.schematic_selection_recovery = None;
    Ok(recovery.selections.len())
}

fn capture_selection(state: &AppState, selection: &Selection) -> ClipboardData {
    let symbols = SchematicSymbolContext::from_state(state);
    let mut schematic = state.schematic.clone();
    schematic.selection = selection.clone();
    schematic.capture_complete_selection_resolved(|component| {
        symbols
            .named_terminal_points(component)
            .into_iter()
            .map(|(_, point)| point)
            .collect()
    })
}

fn complete_selection_count(schematic: &SchematicState, selection: &Selection) -> usize {
    schematic
        .components
        .iter()
        .filter(|object| selection.has_component(object.id))
        .count()
        .saturating_add(
            schematic
                .wires
                .iter()
                .filter(|object| selection.has_wire(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .junctions
                .iter()
                .filter(|object| selection.has_junction(object.pos))
                .count(),
        )
        .saturating_add(
            schematic
                .buses
                .iter()
                .filter(|object| selection.has_bus(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .bus_taps
                .iter()
                .filter(|object| selection.has_bus_tap(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .net_labels
                .iter()
                .filter(|object| selection.has_net_label(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .design_notes
                .iter()
                .filter(|object| selection.has_design_note(object.id))
                .count(),
        )
        .saturating_add(
            schematic
                .documentation_shapes
                .iter()
                .filter(|object| selection.has_documentation_shape(object.id))
                .count(),
        )
}

fn design_nets(state: &AppState) -> Vec<DesignNet> {
    let hierarchy =
        HierarchySource::from_workspace(&state.library_manager, &state.workspace.schematic_buffers);
    design_nets_with_hierarchy(&state.schematic, &hierarchy)
}

fn delete_dependency_impact(state: &AppState, selection: &Selection) -> DeleteDependencyImpact {
    let selected_junctions = selection
        .junctions
        .iter()
        .map(|junction| junction.pos)
        .collect::<HashSet<_>>();
    let selected_component_names = state
        .schematic
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .map(|component| component.name.clone())
        .collect::<BTreeSet<_>>();
    let selected_label_names = state
        .schematic
        .net_labels
        .iter()
        .filter(|label| selection.has_net_label(label.id))
        .map(|label| label.name.clone())
        .collect::<BTreeSet<_>>();
    let mut affected_nets = design_nets(state)
        .into_iter()
        .filter(|net| {
            net.terminals
                .iter()
                .any(|terminal| selection.components.contains(&terminal.component_id))
                || net
                    .wire_ids
                    .iter()
                    .any(|wire_id| selection.wires.contains(wire_id))
                || state.schematic.wires.iter().any(|wire| {
                    net.wire_ids.contains(&wire.id)
                        && selected_junctions
                            .iter()
                            .any(|position| wire.contains_point(*position))
                })
                || selected_label_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&net.name))
        })
        .collect::<Vec<_>>();
    affected_nets.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
    });
    affected_nets.dedup_by(|left, right| left.name.eq_ignore_ascii_case(&right.name));

    let mut affected_symbols = affected_nets
        .iter()
        .map(|net| net.name.clone())
        .chain(selected_component_names)
        .chain(selected_label_names)
        .collect::<BTreeSet<_>>();
    for bus in state
        .schematic
        .buses
        .iter()
        .filter(|bus| selection.has_bus(bus.id))
    {
        if let Some(declaration) = &bus.declaration {
            affected_symbols.insert(declaration.name.clone());
        }
    }
    for tap in state
        .schematic
        .bus_taps
        .iter()
        .filter(|tap| selection.has_bus_tap(tap.id))
    {
        affected_symbols.insert(tap.slice.to_string());
    }

    let affected_wire_ids = affected_nets
        .iter()
        .flat_map(|net| net.wire_ids.iter().copied())
        .collect::<HashSet<_>>();
    let mut affected_probes = BTreeSet::new();
    for probe in &state.schematic.probes {
        let references_symbol = probe
            .source_expression
            .as_deref()
            .is_some_and(|expression| references_any_symbol(expression, &affected_symbols));
        let lies_on_affected_wire = state.schematic.wires.iter().any(|wire| {
            affected_wire_ids.contains(&wire.id) && wire.contains_point(probe.position)
        });
        if references_symbol || lies_on_affected_wire {
            affected_probes.insert(probe.reference.clone());
        }
    }

    let mut affected_output_names = BTreeSet::new();
    for record in &state.workspace.simulation_plan_payloads {
        for output in &record.payload.saved_outputs {
            if references_any_symbol(&output.source_expression, &affected_symbols) {
                affected_output_names.insert(output.name.clone());
            }
        }
    }
    let mut affected_specifications = BTreeSet::new();
    for spec in &state.workspace.specs {
        if affected_output_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&spec.measurement))
        {
            affected_specifications.insert(spec.measurement.clone());
        }
    }
    for record in &state.workspace.simulation_plan_payloads {
        for spec in &record.payload.specs {
            if affected_output_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&spec.measurement))
            {
                affected_specifications.insert(spec.measurement.clone());
            }
        }
    }

    let mut affected_comments = BTreeSet::new();
    for note in state
        .schematic
        .design_notes
        .iter()
        .filter(|note| note.review.is_some())
    {
        let selected = selection.has_design_note(note.id);
        let references_symbol = references_any_symbol(&note.text, &affected_symbols)
            || note.review.as_ref().is_some_and(|review| {
                review
                    .messages
                    .iter()
                    .any(|message| references_any_symbol(&message.body, &affected_symbols))
                    || review.evidence.iter().any(|evidence| {
                        references_any_symbol(&evidence.label, &affected_symbols)
                            || references_any_symbol(&evidence.source_identity, &affected_symbols)
                    })
            });
        if selected || references_symbol {
            let identity = note.review.as_ref().map_or_else(
                || format!("review note #{}", note.id),
                |review| review.record_id.clone(),
            );
            affected_comments.insert(identity);
        }
    }

    let affected_net_names = affected_nets
        .iter()
        .map(|net| net.name.clone())
        .chain(
            state
                .schematic
                .buses
                .iter()
                .filter(|bus| selection.has_bus(bus.id))
                .filter_map(|bus| bus.declaration.as_ref().map(ToString::to_string)),
        )
        .chain(
            state
                .schematic
                .bus_taps
                .iter()
                .filter(|tap| selection.has_bus_tap(tap.id))
                .map(|tap| tap.slice.to_string()),
        )
        .collect::<BTreeSet<_>>();

    DeleteDependencyImpact {
        affected_nets: summarize_identities(&affected_net_names, "No named nets"),
        dependent_records: format!(
            "{}; {}; {}",
            summarize_dependency_class(&affected_probes, "probe", "probes"),
            summarize_dependency_class(&affected_specifications, "specification", "specifications"),
            summarize_dependency_class(&affected_comments, "review comment", "review comments")
        ),
    }
}

fn summarize_identities(identities: &BTreeSet<String>, empty: &str) -> String {
    if identities.is_empty() {
        return empty.to_owned();
    }
    const LIMIT: usize = 5;
    let shown = identities.iter().take(LIMIT).cloned().collect::<Vec<_>>();
    let remainder = identities.len().saturating_sub(shown.len());
    if remainder == 0 {
        shown.join(" \u{00b7} ")
    } else {
        format!("{} \u{00b7} +{remainder} more", shown.join(" \u{00b7} "))
    }
}

fn summarize_dependency_class(
    identities: &BTreeSet<String>,
    singular: &str,
    plural: &str,
) -> String {
    match identities.len() {
        0 => format!("0 {plural}"),
        1 => format!("1 {singular} ({})", summarize_identities(identities, "")),
        count => format!(
            "{count} {plural} ({})",
            summarize_identities(identities, "")
        ),
    }
}

fn references_any_symbol(value: &str, symbols: &BTreeSet<String>) -> bool {
    let value = value.to_ascii_lowercase();
    symbols.iter().any(|symbol| {
        let symbol = symbol.to_ascii_lowercase();
        if symbol.is_empty() {
            return false;
        }
        value.match_indices(&symbol).any(|(start, _)| {
            let end = start + symbol.len();
            let before = (start > 0).then(|| value.as_bytes()[start - 1]);
            let after = (end < value.len()).then(|| value.as_bytes()[end]);
            !before.is_some_and(identifier_byte) && !after.is_some_and(identifier_byte)
        })
    })
}

const fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

fn cut_open_net_count(state: &AppState, selection: &Selection) -> usize {
    let selected_components = &selection.components;
    let selected_wires = &selection.wires;
    let selected_junctions = selection
        .junctions
        .iter()
        .map(|junction| junction.pos)
        .collect::<HashSet<_>>();
    let before = design_nets(state);
    let mut after_schematic = state.schematic.clone();
    after_schematic.selection = selection.clone();
    let _ = after_schematic.delete_selection();
    let hierarchy =
        HierarchySource::from_workspace(&state.library_manager, &state.workspace.schematic_buffers);
    let after = design_nets_with_hierarchy(&after_schematic, &hierarchy);

    before
        .iter()
        .filter(|net| {
            let touched = net
                .terminals
                .iter()
                .any(|terminal| selected_components.contains(&terminal.component_id))
                || net
                    .wire_ids
                    .iter()
                    .any(|wire_id| selected_wires.contains(wire_id))
                || state.schematic.wires.iter().any(|wire| {
                    net.wire_ids.contains(&wire.id)
                        && selected_junctions
                            .iter()
                            .any(|point| wire.contains_point(*point))
                });
            if !touched {
                return false;
            }
            let remaining = net
                .terminals
                .iter()
                .filter(|terminal| !selected_components.contains(&terminal.component_id))
                .map(terminal_identity)
                .collect::<HashSet<_>>();
            if remaining.is_empty() {
                return false;
            }
            let largest_retained_group = after
                .iter()
                .map(|candidate| {
                    candidate
                        .terminals
                        .iter()
                        .map(terminal_identity)
                        .filter(|terminal| remaining.contains(terminal))
                        .count()
                })
                .max()
                .unwrap_or(0);
            remaining.len() == 1 || largest_retained_group < remaining.len()
        })
        .count()
}

fn terminal_identity(terminal: &crate::simulation::netlist_gen::NetTerminal) -> (u64, String) {
    (terminal.component_id, terminal.pin.clone())
}

fn named_external_attachments(state: &AppState) -> Vec<(Point, String)> {
    let selected_components = &state.schematic.selection.components;
    let captured_wires = state
        .schematic
        .clipboard
        .wires
        .iter()
        .map(|wire| wire.id)
        .collect::<HashSet<_>>();
    let captured_names = state
        .schematic
        .clipboard
        .net_labels
        .iter()
        .map(|label| label.name.as_str())
        .collect::<HashSet<_>>();
    let symbols = SchematicSymbolContext::from_state(state);
    let terminal_points = state
        .schematic
        .components
        .iter()
        .filter(|component| selected_components.contains(&component.id))
        .flat_map(|component| {
            symbols
                .named_terminal_points(component)
                .into_iter()
                .map(move |(pin, point)| ((component.id, pin), point))
        })
        .collect::<HashMap<_, _>>();

    let mut attachments = Vec::new();
    for net in design_nets(state) {
        if !net.authored_name
            || crate::state::NetLabel::validate_name(
                &net.name,
                state.schematic.document_policy.net_naming,
            )
            .is_err()
        {
            continue;
        }
        let selected_terminals = net
            .terminals
            .iter()
            .filter(|terminal| selected_components.contains(&terminal.component_id))
            .collect::<Vec<_>>();
        if selected_terminals.is_empty() {
            continue;
        }
        let external = net
            .terminals
            .iter()
            .any(|terminal| !selected_components.contains(&terminal.component_id))
            || net
                .wire_ids
                .iter()
                .any(|wire_id| !captured_wires.contains(wire_id))
            || !captured_names.contains(net.name.as_str());
        if !external {
            continue;
        }
        for terminal in selected_terminals {
            if let Some(point) = terminal_points.get(&(terminal.component_id, terminal.pin.clone()))
            {
                attachments.push((*point, net.name.clone()));
            }
        }
    }
    attachments.sort_by(|left, right| {
        (left.0.x, left.0.y, left.1.as_str()).cmp(&(right.0.x, right.0.y, right.1.as_str()))
    });
    attachments.dedup();
    attachments
}

fn select_all_targets(
    state: &AppState,
    scope: SelectAllScope,
    classes: SelectAllClasses,
) -> Vec<(String, Selection)> {
    let active_key = state.workspace.active_schematic_reference().key();
    match scope {
        SelectAllScope::ActiveCellView => vec![(
            active_key,
            selection_for_schematic(&state.schematic, classes, |_| true),
        )],
        SelectAllScope::CurrentLayerObjectFilter => {
            let classes = classes.intersect_filter(state.ui.schematic_selection_filter);
            vec![(
                active_key,
                selection_for_schematic(&state.schematic, classes, |id| {
                    crate::schematic::view::sheet_visibility::object_is_on_active_sheet(state, id)
                }),
            )]
        }
        SelectAllScope::VisibleEditInPlaceHierarchy => visible_hierarchy_keys(state)
            .into_iter()
            .filter_map(|key| {
                if key == active_key {
                    Some((
                        key,
                        selection_for_schematic(&state.schematic, classes, |_| true),
                    ))
                } else {
                    state
                        .workspace
                        .schematic_buffers
                        .get(&key)
                        .map(|schematic| {
                            (key, selection_for_schematic(schematic, classes, |_| true))
                        })
                }
            })
            .collect(),
    }
}

fn selection_for_schematic(
    schematic: &SchematicState,
    classes: SelectAllClasses,
    mut included: impl FnMut(u64) -> bool,
) -> Selection {
    let mut selection = Selection::default();
    for component in &schematic.components {
        let class_enabled = if component.kind == ComponentType::Port {
            classes.pins
        } else {
            classes.instances
        };
        if class_enabled && included(component.id) {
            selection.select_component(component.id);
        }
    }
    if classes.wires {
        for wire in &schematic.wires {
            if included(wire.id) {
                selection.select_wire(wire.id);
            }
        }
        for junction in &schematic.junctions {
            if included(junction.id) {
                selection.select_junction(junction.pos);
            }
        }
        for bus in &schematic.buses {
            if included(bus.id) {
                selection.select_bus(bus.id);
            }
        }
        for tap in &schematic.bus_taps {
            if included(tap.id) {
                selection.select_bus_tap(tap.id);
            }
        }
    }
    if classes.labels {
        for label in &schematic.net_labels {
            if included(label.id) {
                selection.select_net_label(label.id);
            }
        }
    }
    selection
}

fn hierarchy_stack_keys(state: &AppState) -> Vec<String> {
    let mut keys = state
        .workspace
        .hierarchy_stack
        .iter()
        .map(crate::state::CellViewRef::key)
        .collect::<Vec<_>>();
    let active = state.workspace.active_schematic_reference().key();
    if !keys.contains(&active) {
        keys.push(active);
    }
    keys
}

fn visible_hierarchy_keys(state: &AppState) -> Vec<String> {
    let keys = hierarchy_stack_keys(state);
    match state.ui.schematic_visibility.hierarchy {
        SchematicHierarchyVisibility::ActiveOnly => {
            vec![state.workspace.active_schematic_reference().key()]
        }
        SchematicHierarchyVisibility::ActiveAndParent => keys
            .into_iter()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        SchematicHierarchyVisibility::FullVisibleHierarchy => keys,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_classes_keep_ports_distinct_from_instances() {
        let mut schematic = SchematicState::default();
        let resistor = schematic.add_component(ComponentType::Resistor, Point::origin());
        let port = schematic.add_component(ComponentType::Port, Point::new(20, 0));

        let instances = selection_for_schematic(
            &schematic,
            SelectAllClasses {
                instances: true,
                wires: false,
                labels: false,
                pins: false,
            },
            |_| true,
        );
        assert!(instances.has_component(resistor));
        assert!(!instances.has_component(port));

        let pins = selection_for_schematic(
            &schematic,
            SelectAllClasses {
                instances: false,
                wires: false,
                labels: false,
                pins: true,
            },
            |_| true,
        );
        assert!(!pins.has_component(resistor));
        assert!(pins.has_component(port));
    }

    #[test]
    fn read_only_view_can_retain_select_all_authority() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.read_only = true;
        state.sync_active_schematic_to_workspace();

        assert!(open_select_all_dialog(&mut state));
        assert!(state.dialogs.selection_workflow.validate(&state).is_ok());
        state.schematic.selection.select_component(id);
        assert!(state.dialogs.selection_workflow.validate(&state).is_err());
    }

    #[test]
    fn delete_review_retains_authority_and_commits_one_undo_transaction() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.selection.select_only_component(id);
        state.schematic.init_undo_history();

        assert!(open_delete_selection_dialog(&mut state));
        assert_eq!(state.schematic.components.len(), 1);
        let draft = state.dialogs.selection_workflow.clone();
        commit_selection_workflow(&mut state, &draft).expect("reviewed delete");

        assert!(state.schematic.components.is_empty());
        assert_eq!(state.schematic.undo_description(), Some("delete selection"));
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components.len(), 1);
    }

    #[test]
    fn select_all_recovery_restores_the_prior_selection() {
        let mut state = AppState::default();
        let first = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let second = state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(20, 0));
        state.schematic.selection.select_only_component(first);
        state.sync_active_schematic_to_workspace();

        assert!(open_select_all_dialog(&mut state));
        let draft = state.dialogs.selection_workflow.clone();
        commit_selection_workflow(&mut state, &draft).expect("select all");
        assert!(state.schematic.selection.has_component(first));
        assert!(state.schematic.selection.has_component(second));
        assert!(select_all_recovery_available(&state));

        assert_eq!(restore_select_all_selection(&mut state).unwrap(), 1);
        assert!(state.schematic.selection.has_component(first));
        assert!(!state.schematic.selection.has_component(second));
    }
}
