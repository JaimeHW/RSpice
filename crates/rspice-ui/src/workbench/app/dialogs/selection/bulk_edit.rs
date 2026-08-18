//! Mockup-owned filtered selection and atomic schematic bulk editing.
//!
//! Queries may inspect the active hierarchy or complete project, but edits are
//! deliberately limited to the active schematic owner. A commit preflights
//! every selected target, validates its frozen document authority, and
//! publishes all component changes through one schematic undo boundary.

use std::collections::{BTreeSet, HashSet};

use egui::{ComboBox, Context, Grid, RichText, ScrollArea, TextEdit, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    Component, ComponentDisplayMode, ComponentType, SchematicSnapshot, SchematicState,
    format_replacement_parameters, parse_replacement_parameters_strict,
    valid_replacement_parameter_name,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::design_system::{
    PANEL_SECTION_H, WorkbenchIcon, property_row, property_row_toned, section_header,
};
use crate::workbench::{SelectionBulkFilter, SelectionBulkHierarchyScope, SelectionBulkObjectKind};

use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} FILTERED SELECTION \u{00b7} EXACT DIFFERENCE";
const TITLE: &str = "Selection and bulk property editing";
const PRIMARY: &str = "Commit bulk edit";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SelectionBulkProperty {
    #[default]
    ModelSection,
    Temperature,
    Tolerance,
    Display,
    ParameterOverride,
}

impl SelectionBulkProperty {
    const ALL: [Self; 5] = [
        Self::ModelSection,
        Self::Temperature,
        Self::Tolerance,
        Self::Display,
        Self::ParameterOverride,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::ModelSection => "Model section",
            Self::Temperature => "Temperature",
            Self::Tolerance => "Tolerance",
            Self::Display => "Display",
            Self::ParameterOverride => "Parameter override",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SelectionBulkUnsetBehavior {
    #[default]
    LeaveUnchanged,
    SetExplicitValue,
    RestoreInheritedValue,
}

impl SelectionBulkUnsetBehavior {
    const ALL: [Self; 3] = [
        Self::LeaveUnchanged,
        Self::SetExplicitValue,
        Self::RestoreInheritedValue,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::LeaveUnchanged => "Leave unchanged",
            Self::SetExplicitValue => "Set explicit value",
            Self::RestoreInheritedValue => "Restore inherited value",
        }
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
        let active_key = state.workspace.active_schematic_reference().key();
        let mut project_buffers = state
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active_key))
            .map(|(key, schematic)| (key.clone(), SchematicSnapshot::capture(schematic)))
            .collect::<Vec<_>>();
        project_buffers.sort_by(|(left, _), (right, _)| left.cmp(right));
        Self {
            active: SchematicEditAuthority::capture(state),
            active_key,
            design_management_revision: state.workspace.design_management.revision(),
            project_buffers,
        }
    }

    fn stale_reason(&self, state: &AppState) -> Option<String> {
        let reopen = |reason: &str| {
            format!("{reason}. Close and reopen Selection and bulk property editing.")
        };
        if self.active.design_execution_epoch != state.design_execution_epoch {
            return Some(reopen("The design document changed"));
        }
        if self.active.active_schematic_epoch != state.active_schematic_epoch {
            return Some(reopen("The active schematic buffer changed"));
        }
        if self.active.topology_version != state.schematic.topology_version()
            || !self.active.snapshot.is_equal_state(&state.schematic)
        {
            return Some(reopen("The active schematic changed"));
        }
        if self.active.view_path != state.workspace.active_view.display_path()
            || self.active_key != state.workspace.active_schematic_reference().key()
        {
            return Some(reopen("The active cell/view changed"));
        }
        if self.active.grid_size != state.schematic.grid_size
            || self.active.document_policy != state.schematic.document_policy
        {
            return Some(reopen("The schematic grid or editing policy changed"));
        }
        if self.active.selection != state.schematic.selection {
            return Some(reopen("The selected-object set changed"));
        }
        if self.design_management_revision != state.workspace.design_management.revision() {
            return Some(reopen("The sheet ownership catalog changed"));
        }
        let live_external = state
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&self.active_key))
            .collect::<Vec<_>>();
        if live_external.len() != self.project_buffers.len() {
            return Some(reopen("The project schematic set changed"));
        }
        for (key, expected) in &self.project_buffers {
            let Some(live) = state.workspace.schematic_buffers.get(key) else {
                return Some(reopen("A project schematic was closed"));
            };
            if !expected.is_equal_state(live) {
                return Some(reopen("A project schematic changed"));
            }
        }
        None
    }

    fn validate_commit(&self, state: &AppState) -> Result<(), String> {
        self.active
            .validate(state, "Selection and bulk property editing")?;
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
    authority: Option<SelectionBulkEditAuthority>,
    pub(crate) error: Option<String>,
    pub(crate) receipt: Option<String>,
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
            new_value: "tt".to_owned(),
            unset_behavior: SelectionBulkUnsetBehavior::SetExplicitValue,
            authority: None,
            error: None,
            receipt: None,
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
    state.dialogs.selection_bulk_edit = SelectionBulkEditDialogState {
        open: true,
        filter,
        selected,
        property,
        new_value: "tt".to_owned(),
        unset_behavior: SelectionBulkUnsetBehavior::SetExplicitValue,
        authority: Some(authority),
        error: None,
        receipt: None,
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
            &self.state.schematic,
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
        let error = self
            .state
            .dialogs
            .selection_bulk_edit
            .error
            .as_deref()
            .or(stale.as_deref())
            .or(impact.error.as_deref())
            .map(str::to_owned);
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(
                "Filter authored schematic objects, review the exact impact, and commit one governed bulk-edit transaction.",
            )
            .size(DialogSize::SimulationWorkflow)
            .initial_height(640.0)
            .flush_body()
            .ghost("Cancel")
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The active schematic or project is read-only. Filters and exact impact review remain available; commit is disabled.",
            );
        } else if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Bulk edit cannot be committed",
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
        let analyses = self.state.sim_setup.enabled_analysis_instance_count();
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
                self.state.dialogs.selection_bulk_edit.selected =
                    build_rows(&self.state, &filter, property)
                        .into_iter()
                        .filter(SelectionBulkRow::editable)
                        .map(|row| row.key)
                        .collect();
                self.state.dialogs.selection_bulk_edit.error = None;
                self.state.dialogs.selection_bulk_edit.receipt = None;
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
                        self.state.dialogs.selection_bulk_edit.receipt =
                            Some("Selection filter saved to this UI session.".to_owned());
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
                    self.state.dialogs.selection_bulk_edit.receipt =
                        Some(format!("Loaded saved selection filter '{name}'."));
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
                    self.state.dialogs.selection_bulk_edit.receipt =
                        Some(format!("Deleted saved selection filter '{name}'."));
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
            &mut self.state.schematic,
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
        let analyses = self.state.sim_setup.enabled_analysis_instance_count();
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Bulk edit committed {} object{} in one undo transaction; {} enabled analysis task{} affected.",
            receipt.changed,
            if receipt.changed == 1 { "" } else { "s" },
            analyses,
            if analyses == 1 { "" } else { "s" },
        )));
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SelectionBulkReceipt {
    changed: usize,
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
        for component in &schematic.components {
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

fn apply_bulk_edit(
    schematic: &mut SchematicState,
    target_ids: &BTreeSet<u64>,
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: Option<&str>,
) -> Result<SelectionBulkReceipt, String> {
    let normalized = validate_bulk_value(property, value, unset, current_property)?;
    let parameter_key = if property == SelectionBulkProperty::ParameterOverride {
        parameter_key_from_filter(value, None).ok()
    } else {
        None
    };
    let mut replacements = Vec::new();
    let mut found = HashSet::new();
    for (index, component) in schematic.components.iter().enumerate() {
        if !target_ids.contains(&component.id) {
            continue;
        }
        found.insert(component.id);
        let mut candidate = component.clone();
        let changed = mutate_component(
            &mut candidate,
            property,
            &normalized,
            unset,
            parameter_key.as_deref(),
        )?;
        if changed {
            replacements.push((index, candidate));
        }
    }
    if found.len() != target_ids.len() {
        return Err("At least one selected component is no longer present.".to_owned());
    }
    if replacements.is_empty() {
        return Ok(SelectionBulkReceipt { changed: 0 });
    }
    schematic.begin_operation("Bulk edit schematic properties");
    let changed_ids = replacements
        .iter()
        .map(|(_, component)| component.id)
        .collect::<Vec<_>>();
    for (index, component) in replacements {
        schematic.components[index] = component;
    }
    schematic.is_dirty = true;
    let committed = schematic.end_operation();
    if !committed {
        return Err("The bulk-edit transaction did not produce a document change.".to_owned());
    }
    schematic.selection.clear();
    for id in changed_ids {
        schematic.selection.select_component(id);
    }
    Ok(SelectionBulkReceipt {
        changed: schematic.selection.components.len(),
    })
}

fn mutate_component(
    component: &mut Component,
    property: SelectionBulkProperty,
    normalized: &str,
    unset: SelectionBulkUnsetBehavior,
    parameter_key_override: Option<&str>,
) -> Result<bool, String> {
    let restore = unset == SelectionBulkUnsetBehavior::RestoreInheritedValue
        || normalized.eq_ignore_ascii_case("inherit");
    match property {
        SelectionBulkProperty::ModelSection => {
            let Some(binding) = component.library_cell.as_mut() else {
                return Err(format!(
                    "{} has no library/cell binding for a model section.",
                    component.name
                ));
            };
            if unset == SelectionBulkUnsetBehavior::LeaveUnchanged
                && binding.model_section.is_none()
            {
                return Ok(false);
            }
            let next = (!restore).then(|| normalized.to_owned());
            if binding.model_section == next {
                return Ok(false);
            }
            binding.model_section = next;
            Ok(true)
        }
        SelectionBulkProperty::Temperature | SelectionBulkProperty::Tolerance => {
            let key = if property == SelectionBulkProperty::Temperature {
                "temp"
            } else {
                "tol"
            };
            mutate_parameter(component, key, normalized, unset, restore)
        }
        SelectionBulkProperty::Display => {
            if unset == SelectionBulkUnsetBehavior::LeaveUnchanged
                && component.display_mode == ComponentDisplayMode::Inherit
            {
                return Ok(false);
            }
            let next = if restore {
                ComponentDisplayMode::Inherit
            } else {
                ComponentDisplayMode::parse(normalized).ok_or_else(|| {
                    "Display must be name and value, name only, value only, hidden, or inherit."
                        .to_owned()
                })?
            };
            if component.display_mode == next {
                return Ok(false);
            }
            component.display_mode = next;
            Ok(true)
        }
        SelectionBulkProperty::ParameterOverride => {
            let (key, explicit_value) = normalized
                .split_once('=')
                .ok_or_else(|| "Parameter override must use name=value syntax.".to_owned())?;
            let key = parameter_key_override
                .unwrap_or(key)
                .trim()
                .to_ascii_lowercase();
            mutate_parameter(component, &key, explicit_value.trim(), unset, restore)
        }
    }
}

fn mutate_parameter(
    component: &mut Component,
    key: &str,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    restore: bool,
) -> Result<bool, String> {
    let mut parameters = parse_replacement_parameters_strict(&component.params)
        .map_err(|error| format!("{} has invalid parameters: {error}", component.name))?;
    let key = key.to_ascii_lowercase();
    if unset == SelectionBulkUnsetBehavior::LeaveUnchanged && !parameters.contains_key(&key) {
        return Ok(false);
    }
    if restore {
        if parameters.remove(&key).is_none() {
            return Ok(false);
        }
    } else if parameters.get(&key).is_some_and(|current| current == value) {
        return Ok(false);
    } else {
        parameters.insert(key, value.to_owned());
    }
    component.params = format_replacement_parameters(&parameters);
    Ok(true)
}

fn validate_bulk_value(
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: Option<&str>,
) -> Result<String, String> {
    let value = value.trim();
    if unset == SelectionBulkUnsetBehavior::RestoreInheritedValue {
        return if property == SelectionBulkProperty::ParameterOverride {
            parameter_key_from_filter(value, current_property).map(|key| format!("{key}=inherit"))
        } else {
            Ok("inherit".to_owned())
        };
    }
    if value.is_empty() {
        return Err("A new value is required.".to_owned());
    }
    match property {
        SelectionBulkProperty::ModelSection => {
            let normalized = value.to_ascii_lowercase();
            if !matches!(normalized.as_str(), "tt" | "ff" | "ss" | "inherit") {
                return Err("Model section must be tt, ff, ss, or inherit.".to_owned());
            }
            Ok(normalized)
        }
        SelectionBulkProperty::Temperature => {
            let parsed = value
                .parse::<f64>()
                .map_err(|_| "Temperature must be a finite Celsius value.".to_owned())?;
            if !parsed.is_finite() || parsed < -273.15 {
                return Err("Temperature must be finite and no lower than -273.15 °C.".to_owned());
            }
            Ok(value.to_owned())
        }
        SelectionBulkProperty::Tolerance => {
            let number = value.strip_suffix('%').unwrap_or(value).trim();
            let parsed = number
                .parse::<f64>()
                .map_err(|_| "Tolerance must be a non-negative finite number.".to_owned())?;
            if !parsed.is_finite() || parsed < 0.0 {
                return Err("Tolerance must be a non-negative finite number.".to_owned());
            }
            Ok(value.to_owned())
        }
        SelectionBulkProperty::Display => ComponentDisplayMode::parse(value)
            .map(|mode| mode.label().to_owned())
            .ok_or_else(|| {
                "Display must be name and value, name only, value only, hidden, or inherit."
                    .to_owned()
            }),
        SelectionBulkProperty::ParameterOverride => {
            let key = parameter_key_from_filter(value, current_property)?;
            let explicit_value = value
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(value)
                .trim();
            if !valid_replacement_parameter_name(&key) {
                return Err("Parameter override has an invalid parameter name.".to_owned());
            }
            if explicit_value.is_empty() {
                return Err("Parameter override requires a value after '='.".to_owned());
            }
            Ok(format!("{key}={explicit_value}"))
        }
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
    egui::Frame::NONE
        .fill(tokens.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let width = (ui.available_width() - 112.0).max(180.0);
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
                if Button::new("Save filter\u{2026}").show(ui).clicked() {
                    dialog.save_filter_open = !dialog.save_filter_open;
                }
            });
            if dialog.save_filter_open {
                saved_filter_controls(ui, dialog, saved_filters, &tokens, &mut action);
            }
        });
    ui.separator();

    let split = egui::Frame::NONE
        .fill(tokens.color.bg_inset)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.columns(2, |columns| {
                columns[0].spacing_mut().item_spacing.x = 8.0;
                columns[1].spacing_mut().item_spacing.x = 8.0;
                selection_filter_panel(&mut columns[0], dialog, rows, hierarchy_path, &mut action);
                egui::Frame::NONE.inner_margin(egui::Margin::same(10)).show(
                    &mut columns[1],
                    |ui| {
                        bulk_edit_panel(ui, dialog, impact, affected_analyses, &mut action);
                    },
                );
            });
        });
    let split_rect = split.response.rect;
    ui.painter().vline(
        split_rect.center().x,
        split_rect.y_range(),
        egui::Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.painter().hline(
        split_rect.x_range(),
        split_rect.bottom(),
        egui::Stroke::new(1.0, tokens.color.border),
    );
    if let Some(receipt) = dialog.receipt.as_deref() {
        ui.add_space(4.0);
        ui.label(
            RichText::new(receipt)
                .font(theme::sans(11.0, FontWeight::Medium))
                .color(tokens.color.ok),
        );
    }
    action
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
                    .hint_text("OPA189*"),
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
                    .hint_text("section = default"),
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
    selection_filter_header(ui, rows.len());
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(10, 10))
        .show(ui, |ui| {
            selection_filter_fields(ui, dialog, hierarchy_path, action);
        });
    Grid::new("selection-bulk-result-header")
        .num_columns(5)
        .striped(false)
        .min_col_width(0.0)
        .spacing(Vec2::new(6.0, 4.0))
        .show(ui, |ui| {
            ui.allocate_space(Vec2::new(20.0, 16.0));
            ui.add_sized(
                [78.0, 16.0],
                egui::Label::new(RichText::new("Object").strong()),
            );
            ui.add_sized(
                [158.0, 16.0],
                egui::Label::new(RichText::new("Path").strong()),
            );
            ui.add_sized(
                [82.0, 16.0],
                egui::Label::new(RichText::new("Value").strong()),
            );
            ui.add_sized(
                [88.0, 16.0],
                egui::Label::new(RichText::new("Lock").strong()),
            );
            ui.end_row();
        });
    ScrollArea::vertical()
        .id_salt("selection-bulk-result-scroll")
        .max_height(300.0)
        .auto_shrink([false, false])
        .show_rows(ui, 24.0, rows.len(), |ui, range| {
            Grid::new("selection-bulk-result-grid")
                .num_columns(5)
                .striped(true)
                .min_col_width(0.0)
                .spacing(Vec2::new(6.0, 4.0))
                .show(ui, |ui| {
                    for row in &rows[range] {
                        let mut selected = dialog.selected.contains(&row.key);
                        let response = ui
                            .add_enabled(
                                row.editable(),
                                egui::Checkbox::without_text(&mut selected),
                            )
                            .on_disabled_hover_text(row.lock.as_deref().unwrap_or("not editable"));
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
                        ui.add_sized([78.0, 20.0], egui::Label::new(&row.object).truncate())
                            .on_hover_text(&row.object);
                        ui.add_sized(
                            [158.0, 20.0],
                            egui::Label::new(RichText::new(&row.path).monospace()).truncate(),
                        )
                        .on_hover_text(&row.path);
                        ui.add_sized([82.0, 20.0], egui::Label::new(&row.value).truncate())
                            .on_hover_text(&row.value);
                        let lock = row.lock.as_deref().unwrap_or("editable");
                        ui.add_sized([88.0, 20.0], egui::Label::new(lock).truncate())
                            .on_hover_text(lock);
                        ui.end_row();
                    }
                });
        });
}

fn bulk_edit_panel(
    ui: &mut Ui,
    dialog: &mut SelectionBulkEditDialogState,
    impact: &SelectionBulkImpact,
    affected_analyses: usize,
    action: &mut SelectionBulkBodyAction,
) {
    ui.spacing_mut().item_spacing.x = 12.0;
    section_header(ui, "Bulk edit and impact", None);
    ui.columns(2, |columns| {
        columns[0].label("Property");
        let before = dialog.property;
        ComboBox::from_id_salt("selection-bulk-property")
            .width(columns[0].available_width())
            .selected_text(dialog.property.label())
            .show_ui(&mut columns[0], |ui| {
                for property in SelectionBulkProperty::ALL {
                    ui.selectable_value(&mut dialog.property, property, property.label());
                }
            });
        if before != dialog.property {
            dialog.new_value = default_property_value(dialog.property).to_owned();
            *action = SelectionBulkBodyAction::FilterChanged;
        }
        columns[1].label("New value");
        if new_value_control(&mut columns[1], dialog) {
            dialog.error = None;
        }
    });
    ui.add_space(9.0);
    ui.columns(2, |columns| {
        columns[0].label("Unset behavior");
        ComboBox::from_id_salt("selection-bulk-unset")
            .width(columns[0].available_width())
            .selected_text(dialog.unset_behavior.label())
            .show_ui(&mut columns[0], |ui| {
                for behavior in SelectionBulkUnsetBehavior::ALL {
                    ui.selectable_value(&mut dialog.unset_behavior, behavior, behavior.label());
                }
            });
    });
    ui.add_space(8.0);
    property_row(ui, "Objects changed", &impact.changed.to_string());
    let warning = Tokens::get(ui.ctx()).color.warn;
    property_row_toned(
        ui,
        "Locked / excluded",
        &impact.locked_or_excluded.to_string(),
        warning,
    );
    property_row(
        ui,
        "Simulation tasks affected",
        if impact.changed == 0 {
            "0".to_owned()
        } else {
            affected_analyses.to_string()
        }
        .as_str(),
    );
    property_row(ui, "Undo boundary", "one transaction");
}

fn new_value_control(ui: &mut Ui, dialog: &mut SelectionBulkEditDialogState) -> bool {
    match dialog.property {
        SelectionBulkProperty::ModelSection => {
            let before = dialog.new_value.clone();
            ComboBox::from_id_salt("selection-bulk-model-section")
                .width(ui.available_width())
                .selected_text(&dialog.new_value)
                .show_ui(ui, |ui| {
                    for value in ["tt", "ff", "ss", "inherit"] {
                        ui.selectable_value(&mut dialog.new_value, value.to_owned(), value);
                    }
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

fn selection_filter_header(ui: &mut Ui, matched: usize) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(
            tokens.color.bg_panel_2.r(),
            tokens.color.bg_panel_2.g(),
            tokens.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        egui::Stroke::new(1.0, tokens.color.border),
    );
    ui.painter().text(
        rect.left_center() + egui::vec2(10.0, 0.0),
        egui::Align2::LEFT_CENTER,
        "SELECTION FILTER",
        theme::sans(crate::ui::tokens::FS_0, FontWeight::SemiBold),
        tokens.color.text_dim,
    );
    ui.painter().text(
        rect.right_center() - egui::vec2(10.0, 0.0),
        egui::Align2::RIGHT_CENTER,
        format!("{matched} matched"),
        theme::mono(crate::ui::tokens::FS_0, FontWeight::Regular),
        tokens.color.ok,
    );
}

fn default_property_value(property: SelectionBulkProperty) -> &'static str {
    match property {
        SelectionBulkProperty::ModelSection => "tt",
        SelectionBulkProperty::Temperature => "27",
        SelectionBulkProperty::Tolerance => "1%",
        SelectionBulkProperty::Display => "name and value",
        SelectionBulkProperty::ParameterOverride => "m=1",
    }
}

/// The occurrence the active schematic is open at — the dialog's scope line and
/// the display path of every row it owns. The design root is implicit, so the
/// top sheet is `/` rather than the name of the cell that happens to be top.
fn active_hierarchy_path(state: &AppState) -> String {
    state.workspace.occurrence_path().to_string()
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
    let active_key = state.workspace.active_schematic_reference().key();
    let mut buffers = Vec::new();
    buffers.push((
        active_key.clone(),
        active_hierarchy_path(state),
        &state.schematic,
        true,
    ));
    match filter.hierarchy_scope {
        SelectionBulkHierarchyScope::ActiveHierarchyPath => {
            let hierarchy_keys = state
                .workspace
                .hierarchy_stack
                .iter()
                .map(|reference| reference.key())
                .collect::<HashSet<_>>();
            for (key, schematic) in &state.workspace.schematic_buffers {
                if !key.eq_ignore_ascii_case(&active_key) && hierarchy_keys.contains(key) {
                    buffers.push((key.clone(), schematic_display_path(key), schematic, false));
                }
            }
        }
        SelectionBulkHierarchyScope::CurrentSheet => {}
        SelectionBulkHierarchyScope::CompleteProject => {
            for (key, schematic) in &state.workspace.schematic_buffers {
                if !key.eq_ignore_ascii_case(&active_key) {
                    buffers.push((key.clone(), schematic_display_path(key), schematic, false));
                }
            }
        }
    }
    buffers.sort_by(|(left, _, _, _), (right, _, _, _)| left.cmp(right));
    let active_sheet = state
        .workspace
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
            &state.workspace.design_management,
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
    schematic: &SchematicState,
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
                            lock: Some("master-owned".to_owned()),
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
                        format!("{:?}", shape.kind()),
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
        Some("other owner".to_owned())
    } else if (kind == SelectionBulkRowKind::Port && property != SelectionBulkProperty::Display)
        || (property == SelectionBulkProperty::ModelSection && component.library_cell.is_none())
    {
        Some("property not applicable".to_owned())
    } else if parse_replacement_parameters_strict(&component.params).is_err()
        && matches!(
            property,
            SelectionBulkProperty::Temperature
                | SelectionBulkProperty::Tolerance
                | SelectionBulkProperty::ParameterOverride
        )
    {
        Some("invalid parameters".to_owned())
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
            "property not applicable".to_owned()
        } else {
            "other owner".to_owned()
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

fn parameter_key_from_filter(
    new_value: &str,
    current_property: Option<&str>,
) -> Result<String, String> {
    let key = new_value
        .split_once('=')
        .map(|(key, _)| key)
        .or_else(|| {
            current_property.and_then(|expression| {
                expression
                    .split_once('=')
                    .map(|(key, _)| key)
                    .or(Some(expression))
            })
        })
        .unwrap_or_default()
        .trim();
    if !valid_replacement_parameter_name(key) {
        return Err("Parameter override requires a valid name=value property.".to_owned());
    }
    Ok(key.to_ascii_lowercase())
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

    #[test]
    fn the_scope_line_names_the_occurrence_below_the_implicit_root() {
        let mut state = AppState::default();
        assert_eq!(active_hierarchy_path(&state), "/");

        state.workspace.descend_into(
            "XAFE".to_owned(),
            crate::state::CellViewRef::new("user", "afe_core", "schematic"),
            crate::state::ViewType::Schematic,
        );
        assert_eq!(active_hierarchy_path(&state), "/XAFE");

        state.workspace.descend_into(
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
        state.schematic.components = vec![instance];

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
        state.schematic.components = vec![component(1, "U1", None)];
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

    #[test]
    fn invalid_target_preflight_never_partially_mutates() {
        let mut schematic = SchematicState::default();
        schematic.components = vec![
            component(1, "X1", Some("tt")),
            component(2, "X2", Some("tt")),
        ];
        schematic.components[1].library_cell = None;
        let before = schematic.components.clone();
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
        assert_eq!(schematic.components, before);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn model_section_bulk_edit_is_one_undo_boundary() {
        let mut schematic = SchematicState::default();
        schematic.components = vec![
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
            schematic.components[0]
                .library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref()),
            Some("ff")
        );
        assert!(schematic.undo());
        assert_eq!(
            schematic.components[0]
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
        schematic.components = vec![first, second];
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
        assert_eq!(schematic.components[0].params, "temp=85");
        assert!(schematic.components[1].params.is_empty());

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
        assert!(schematic.components[0].params.is_empty());
    }

    #[test]
    fn display_override_changes_durable_component_state() {
        let mut schematic = SchematicState::default();
        schematic.components = vec![component(1, "X1", None)];
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
            schematic.components[0].display_mode,
            ComponentDisplayMode::Hidden
        );
    }

    #[test]
    fn parameter_override_uses_current_property_key_and_restores_inheritance() {
        let mut schematic = SchematicState::default();
        schematic.components = vec![component(1, "X1", None)];
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
        assert_eq!(schematic.components[0].params, "m=4");

        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::ParameterOverride,
            "",
            SelectionBulkUnsetBehavior::RestoreInheritedValue,
            Some("m=4"),
        )
        .expect("restore parameter inheritance");
        assert!(schematic.components[0].params.is_empty());
    }

    #[test]
    fn invalid_numeric_value_leaves_document_and_undo_history_unchanged() {
        let mut schematic = SchematicState::default();
        schematic.components = vec![component(1, "X1", None)];
        let before = schematic.components.clone();
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
        assert_eq!(schematic.components, before);
        assert!(!schematic.can_undo());
    }

    #[test]
    fn impact_preview_is_exact_without_cloning_an_undo_transaction() {
        let mut state = AppState::default();
        state.schematic.components = vec![
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
            &state.schematic,
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
        assert!(!state.schematic.can_undo());
        assert!(state.schematic.components.iter().all(|component| {
            component
                .library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref())
                == Some("tt")
        }));
    }

    #[test]
    fn authority_rejects_active_selection_and_external_buffer_drift() {
        let mut state = AppState::default();
        state.schematic.components = vec![component(1, "X1", Some("tt"))];
        state.schematic.selection.select_component(1);
        state
            .workspace
            .schematic_buffers
            .insert("user/child/schematic".to_owned(), SchematicState::default());
        let authority = SelectionBulkEditAuthority::capture(&state);
        assert!(authority.stale_reason(&state).is_none());

        state.schematic.selection.clear();
        assert!(authority.stale_reason(&state).is_some());
        state.schematic.selection.select_component(1);
        state
            .workspace
            .schematic_buffers
            .get_mut("user/child/schematic")
            .expect("external buffer")
            .components
            .push(component(2, "X2", Some("tt")));
        assert!(authority.stale_reason(&state).is_some());
    }

    #[test]
    fn complete_project_rows_are_visible_but_other_owners_are_locked() {
        let mut state = AppState::default();
        state.schematic.components = vec![component(1, "X1", Some("tt"))];
        let mut external = SchematicState::default();
        external.components = vec![component(2, "X2", Some("ff"))];
        state
            .workspace
            .schematic_buffers
            .insert("user/child/schematic".to_owned(), external);
        let filter = SelectionBulkFilter {
            hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
            ..SelectionBulkFilter::default()
        };
        let rows = build_rows(&state, &filter, SelectionBulkProperty::ModelSection);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows.iter().filter(|row| row.editable()).count(), 1);
        assert!(
            rows.iter()
                .any(|row| row.lock.as_deref() == Some("other owner"))
        );
    }

    #[test]
    fn port_rows_only_accept_the_display_property() {
        let mut state = AppState::default();
        state.schematic.components = vec![
            Component::new(1, ComponentType::Port, Point::new(0, 0)).with_name_value("VIN", ""),
        ];
        let filter = SelectionBulkFilter {
            object_kind: SelectionBulkObjectKind::PortsAndPins,
            ..SelectionBulkFilter::default()
        };

        let temperature_rows = build_rows(&state, &filter, SelectionBulkProperty::Temperature);
        assert_eq!(
            temperature_rows[0].lock.as_deref(),
            Some("property not applicable")
        );
        let display_rows = build_rows(&state, &filter, SelectionBulkProperty::Display);
        assert!(display_rows[0].editable());
    }

    #[test]
    fn accessibility_tree_exposes_only_the_mockup_bulk_edit_surface_actions() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.schematic.components = vec![component(1, "X1", Some("tt"))];
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
}
