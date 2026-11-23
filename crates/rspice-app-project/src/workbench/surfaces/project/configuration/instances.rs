//! The tables under the global bindings, on the kit's row list. Instances
//! lists the pattern bindings, since one names many instances, then the
//! expanded design as a tree that opens an instance at a time: what the shown
//! configuration resolved each instance to, which list chose that, and the
//! view it binds it to. Cells
//! lists each library/cell once, with the cell binding every instance of it
//! follows.
//!
//! Rows select as a file list does, a double-click or Enter opens the view a
//! row is bound to, and the row menu sets, removes, copies and pastes the
//! selected rows' bindings in one edit.

use super::*;

use std::collections::BTreeMap;
use std::sync::Arc;

use rspice_design::hierarchy::{
    HierarchyBindingRule, HierarchyBindingStatus, ResolvedHierarchyBinding,
};

use crate::state::{ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetOverride};
use crate::ui::widgets::table::{
    Cell, Column, DataTable, Group, RowKey, TableResponse, TableState,
};
use crate::workbench::app_state::model::UnreadLibrary;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationRow, ConfigurationTab, ConfigurationTables, INHERITED,
    InstanceRow, PatternRow, instance_key,
};

/// What the reader asked of the tables this frame.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum TableAction {
    Tab(ConfigurationTab),
    AddPattern,
    /// The lead of the selection moved: the inspector shows this row.
    Lead(Option<ConfigurationRow>),
    Toggle(String),
    /// Open the view a row is bound to.
    Open(ConfigurationRow),
    /// The row menu, for the selected rows, at a place on the screen.
    Menu(Vec<ConfigurationRow>, egui::Pos2),
    /// Delete: remove the selected rows' own bindings.
    Remove(Vec<ConfigurationRow>),
}

/// Which list chose a row's view, in the page's words.
fn rule_label(rule: HierarchyBindingRule) -> &'static str {
    // Spelled from the crate root: the variant names are also item names in
    // other features, and the full path says which these are.
    match rule {
        rspice_design::hierarchy::HierarchyBindingRule::Default => "Default",
        rspice_design::hierarchy::HierarchyBindingRule::ViewList => "View list",
        rspice_design::hierarchy::HierarchyBindingRule::CellBinding => "Cell binding",
        rspice_design::hierarchy::HierarchyBindingRule::InstanceBinding => "Instance binding",
    }
}

/// The table the bar's switch shows, and what the bar asked of it: the
/// switch itself, or Add pattern binding.
pub(super) fn tables(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
    asked: Option<TableAction>,
) {
    let catalog = &cx.model.workspace.content.configuration_sets;
    let id = match selection {
        ConfigurationSelection::Configuration(id) => Some(id),
        ConfigurationSelection::Default => None,
    };
    let editable = id.is_some_and(|id| cx.state.configuration.definition(catalog, id).is_some())
        && !cx.model.safe_mode.project_read_only();
    let visible = cx.state.configuration.visible_instances();
    // A cell of a library this session could not read is unbound because
    // the library is missing, not a view of it.
    let unread = cx.model.unread_libraries();
    let page = &mut cx.state.configuration;
    let mut actions = asked.into_iter().collect::<Vec<_>>();
    actions.extend(match page.tab {
        ConfigurationTab::Instances => instance_table(ui, page, resolution, visible, &unread),
        ConfigurationTab::Cells => super::cells::cells_table(ui, page, &resolution.tables),
    });
    let mut menu_at = None;
    for action in actions {
        match action {
            TableAction::Menu(rows, at) => menu_at = Some((rows, at)),
            action => apply(cx, id, resolution, editable, action),
        }
    }
    super::menu::row_menu(ui, cx, id, resolution, editable, menu_at);
}

fn apply(
    cx: &mut Cx<'_, ProjectFeature>,
    id: Option<crate::state::ConfigurationSetId>,
    resolution: &ConfigurationResolution,
    editable: bool,
    action: TableAction,
) {
    match action {
        TableAction::Tab(tab) => {
            cx.state.configuration.tab = tab;
            cx.state.configuration.row = None;
        }
        TableAction::Lead(row) => cx.state.configuration.row = row,
        TableAction::Toggle(path) => cx.state.configuration.toggle_collapsed(&path),
        TableAction::Open(row) => super::menu::open_view(cx, resolution, &row),
        TableAction::Remove(rows) => {
            if editable && let Some(id) = id {
                super::menu::remove_bindings(cx, id, &rows);
            }
        }
        TableAction::Menu(..) => {}
        TableAction::AddPattern => {
            if let Some(id) = id {
                let mut added = None;
                if cx.state.configuration.edit(
                    &cx.model.workspace.content.configuration_sets,
                    id,
                    |definition| added = Some(add_pattern_binding(definition)),
                ) {
                    cx.state.configuration.row = added.map(ConfigurationRow::Pattern);
                }
            }
        }
    }
}

/// Bind every instance a new pattern names, starting from the first scope
/// no binding uses yet, and return the pattern so the inspector can edit it.
fn add_pattern_binding(definition: &mut ConfigurationSetDefinition) -> String {
    let taken = |pattern: &str| {
        definition
            .overrides
            .iter()
            .any(|binding| binding.instance_path.eq_ignore_ascii_case(pattern))
    };
    let pattern = (1..)
        .map(|depth| format!("/{}", vec!["*"; depth].join("/")))
        .find(|pattern| !taken(pattern))
        .expect("pattern depth is unbounded");
    let views = definition
        .executable_view_policy
        .first()
        .cloned()
        .into_iter()
        .collect();
    definition.overrides.push(ConfigurationSetOverride {
        instance_path: pattern.clone(),
        executable_views: views,
        stop_view: None,
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
        target: None,
    });
    pattern
}

/// An identifier in a cell: a path, a cell or a view name, set in mono as
/// the rest of the page sets them.
pub(super) fn ident(text: impl Into<String>) -> Cell {
    Cell::number(text)
}

/// The Instances table's columns: the tree's, which the pattern rows above
/// it share. The tree's order is the hierarchy's, so no head sorts it.
fn instance_columns() -> [Column; 7] {
    [
        Column::name("Instance", 140.0).unsortable(),
        Column::name("Cell", 120.0).unsortable(),
        Column::name("Cell to use", 120.0)
            .unsortable()
            .tooltip("The cell a binding runs in the placed cell's place"),
        Column::words("View found", 96.0).unsortable(),
        Column::name("View to use", 110.0).unsortable(),
        // A pattern's row says which platforms it is limited to here, so
        // the column takes a share of the room the page has.
        Column::name("Bound by", 132.0).unsortable(),
        Column::words("Model section", 104.0).unsortable(),
    ]
}

/// One row of the Instances table, in the order the table shows them.
#[derive(Debug, Clone, Copy)]
enum Line {
    /// A pattern binding, by its place among the definition's patterns.
    Pattern(usize),
    /// An instance, by its place among the rows the tree shows.
    Instance(usize),
}

/// The Instances table's rows: the pattern bindings, then the tree. While a
/// filter is set every instance is a candidate, folded or not, and each is
/// named by its whole path.
struct InstanceLines<'a> {
    patterns: &'a [PatternRow],
    instances: Arc<[InstanceRow]>,
}

impl InstanceLines<'_> {
    fn len(&self) -> usize {
        self.patterns.len() + self.instances.len()
    }

    fn line(&self, index: usize) -> Line {
        match index.checked_sub(self.patterns.len()) {
            None => Line::Pattern(index),
            Some(at) => Line::Instance(at),
        }
    }
}

fn instance_table(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    resolution: &ConfigurationResolution,
    visible: Arc<[InstanceRow]>,
    unread: &BTreeMap<String, UnreadLibrary>,
) -> Vec<TableAction> {
    let tables = &resolution.tables;
    let hierarchy = resolution.hierarchy.as_ref().ok();
    let filter = page.instances_table.filter.trim().to_lowercase();
    let flat = !filter.is_empty();
    // A filter searches the whole design once, for the rows the table then
    // shows; the table's own filter keeps the same rows.
    let instances = match hierarchy {
        None => Arc::from(Vec::new()),
        Some(hierarchy) if flat => page.matching_instances(&filter, &|row| {
            instance_cells(tables, &hierarchy.bindings, row, false, true, unread)
                .iter()
                .any(|cell| cell.text.to_lowercase().contains(&filter))
        }),
        Some(_) => visible,
    };
    let lines = InstanceLines {
        patterns: &tables.patterns,
        instances,
    };
    let keys = (0..lines.len())
        .map(|index| {
            RowKey::Number(match lines.line(index) {
                Line::Pattern(at) => tables.patterns[at].key,
                Line::Instance(at) => instance_key(&lines.instances[at].path),
            })
        })
        .collect::<Vec<_>>();
    let groups = if lines.patterns.is_empty() {
        Vec::new()
    } else {
        vec![
            Group::new("Pattern bindings", 0),
            Group::new("Instances", lines.patterns.len()),
        ]
    };
    let wanted = page.row.as_ref().and_then(|row| tables.row_key(row));
    lead_follows(&mut page.instances_table, &keys, wanted);
    let columns = instance_columns();
    let mut state = std::mem::take(&mut page.instances_table);
    let collapsed = |path: &str| !flat && page.is_collapsed(path);
    let response = DataTable::new("configuration.instances", &columns, lines.len())
        .state(&mut state)
        .keys(&keys)
        .groups(&groups)
        .multi_select()
        .freeze(1)
        .max_height(table_height(ui))
        .show(ui, |index| match lines.line(index) {
            Line::Pattern(at) => pattern_cells(&tables.patterns[at]),
            Line::Instance(at) => hierarchy.map_or_else(Vec::new, |hierarchy| {
                let row = &lines.instances[at];
                instance_cells(
                    tables,
                    &hierarchy.bindings,
                    row,
                    collapsed(&row.path),
                    flat,
                    unread,
                )
            }),
        });
    page.instances_table = state;
    let identity = |index: usize| match lines.line(index) {
        Line::Pattern(at) => ConfigurationRow::Pattern(tables.patterns[at].pattern.clone()),
        Line::Instance(at) => ConfigurationRow::Instance(lines.instances[at].path.clone()),
    };
    let mut actions = answer(&page.instances_table, &response, &identity);
    if let Some((index, open)) = response.expand
        && let Line::Instance(at) = lines.line(index)
    {
        let row = &lines.instances[at];
        if row.children > 0 && page.is_collapsed(&row.path) == open {
            actions.push(TableAction::Toggle(row.path.clone()));
        }
    }
    actions
}

/// How tall a table may be: what the page leaves under the band, and never
/// less than a few rows.
pub(super) fn table_height(ui: &Ui) -> f32 {
    let row = Tokens::get(ui.ctx()).metrics.row_h;
    ui.available_height().max(row * 6.0)
}

/// Put the table's lead on the row the inspector shows when something other
/// than the table chose that row: the inspector, a new pattern binding, a
/// test. A row the table does not show leaves the table with no selection.
pub(super) fn lead_follows(state: &mut TableState, keys: &[RowKey], wanted: Option<u64>) {
    let wanted = wanted.map(RowKey::Number);
    if state.selected_key() == wanted.as_ref() {
        return;
    }
    match wanted.and_then(|key| Some((keys.iter().position(|shown| *shown == key)?, key))) {
        Some((row, key)) => state.select(row, Some(key)),
        None => {
            state.selection = None;
            state.others.clear();
            state.anchor = None;
        }
    }
}

/// What a table's response asks of the page, with `identity` naming the row
/// at each index in the table's own order.
pub(super) fn answer(
    state: &TableState,
    response: &TableResponse,
    identity: &dyn Fn(usize) -> ConfigurationRow,
) -> Vec<TableAction> {
    let selected = || {
        state
            .selected_rows()
            .into_iter()
            .map(identity)
            .collect::<Vec<_>>()
    };
    let mut actions = Vec::new();
    if response.selection_changed {
        actions.push(TableAction::Lead(state.selected_row().map(identity)));
    }
    if let Some(row) = response.activate {
        actions.push(TableAction::Open(identity(row)));
    }
    if let Some(request) = response.context_menu {
        actions.push(TableAction::Menu(selected(), request.position));
    }
    if response.delete {
        actions.push(TableAction::Remove(selected()));
    }
    actions
}

/// A pattern binding on the tree's columns: the pattern where an instance
/// names itself, its views where an instance's view to use is.
fn pattern_cells(row: &PatternRow) -> Vec<Cell> {
    let bound_by = match row.only_on {
        Some(only_on) => format!("Pattern binding \u{b7} only on {only_on}"),
        None => "Pattern binding".to_owned(),
    };
    vec![
        ident(&row.pattern).leaf(),
        Cell::absent(),
        Cell::absent(),
        Cell::absent(),
        view_list(&row.views),
        Cell::text(bound_by),
        row.model_section
            .as_deref()
            .map_or_else(Cell::absent, ident),
    ]
}

/// A view list as a cell: the views, or a faint Inherited when the row
/// names none of its own. The column holds both, so it reads in one face,
/// the interface's, as its words do.
pub(super) fn view_list(views: &str) -> Cell {
    if views == INHERITED {
        Cell::faint(INHERITED)
    } else {
        Cell::text(views)
    }
}

/// Why a row is not bound, or `None` when it is.
fn unbound_reason(status: HierarchyBindingStatus) -> Option<&'static str> {
    match status {
        HierarchyBindingStatus::Resolved | HierarchyBindingStatus::Modified => None,
        HierarchyBindingStatus::Unresolved => Some("Not bound: no view found."),
        HierarchyBindingStatus::Incompatible => Some("Not bound: its pins do not fit."),
        HierarchyBindingStatus::Recursive => Some("Not bound: it instantiates itself."),
        HierarchyBindingStatus::DepthLimit => Some("Not bound: the hierarchy is too deep."),
    }
}

/// One instance of the tree: indented by its depth behind its twist, or,
/// while a filter is set, named by its whole path at the left edge. An
/// instance that is not bound, or binds with a warning, carries the
/// problem's mark on its name, and why under the pointer.
fn instance_cells(
    tables: &ConfigurationTables,
    hierarchy: &[ResolvedHierarchyBinding],
    row: &InstanceRow,
    collapsed: bool,
    flat: bool,
    unread: &BTreeMap<String, UnreadLibrary>,
) -> Vec<Cell> {
    let binding = &hierarchy[row.binding];
    let reference = &binding.reference;
    // A binding to another cell runs `reference` in the placed cell's place.
    let placed = binding.placed.as_ref().unwrap_or(reference);
    let instance = if flat {
        ident(&row.path)
    } else {
        let name = if row.depth == 0 {
            reference.cell.as_str()
        } else {
            row.name.as_str()
        };
        let named = ident(name).indent(u8::try_from(row.depth).unwrap_or(u8::MAX));
        if row.children > 0 {
            named.twist(!collapsed)
        } else {
            named.leaf()
        }
    };
    let found = if binding.status.is_resolved() {
        ident(&reference.view)
    } else {
        Cell::absent()
    };
    let to_use = view_list(tables.instance_to_use(&row.path).unwrap_or(INHERITED));
    // A cell of a library this session could not read is unbound because
    // the library is missing; that sentence says what to do about it.
    let problem = match unbound_reason(binding.status) {
        Some(reason) => Some((
            Problem::Failure,
            match (unread.get(&reference.library), &binding.diagnostic) {
                (Some(unread), _) => unread.sentence.clone(),
                (None, Some(diagnostic)) => format!("{reason}\n{diagnostic}"),
                (None, None) => reason.to_owned(),
            },
        )),
        None => binding.warnings.first().map(|warning| {
            (
                Problem::Warning,
                match &binding.diagnostic {
                    Some(diagnostic) => format!("{warning}\n{diagnostic}"),
                    None => warning.clone(),
                },
            )
        }),
    };
    let instance = match problem {
        Some((problem, why)) => instance.marked(problem).hint(why),
        None => instance,
    };
    vec![
        instance,
        ident(format!("{}/{}", placed.library, placed.cell)),
        if binding.placed.is_some() {
            ident(format!("{}/{}", reference.library, reference.cell))
        } else {
            Cell::faint(INHERITED)
        },
        found,
        to_use,
        Cell::text(rule_label(binding.rule)),
        if binding.model_section.is_empty() {
            Cell::absent()
        } else {
            ident(&binding.model_section)
        },
    ]
}
