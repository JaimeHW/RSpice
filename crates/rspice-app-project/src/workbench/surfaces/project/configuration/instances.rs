//! The tables under the global bindings, on the kit's row list. Instances
//! lists the pattern bindings, since one names many instances, then every
//! instance of the expanded design as a tree: what the shown configuration
//! resolved it to, which list chose that, and the view it binds it to. Cells
//! lists each library/cell once, with the cell binding every instance of it
//! follows.
//!
//! Rows select as a file list does, a double-click or Enter opens the view a
//! row is bound to, and the row menu sets, removes, copies and pastes the
//! selected rows' bindings in one edit.

use super::*;

use rspice_design::hierarchy::{
    HierarchyBindingRule, HierarchyBindingStatus, ResolvedHierarchyBinding,
};

use crate::state::{ConfigurationPlatform, ConfigurationSetDefinition, ConfigurationSetOverride};
use crate::ui::widgets::table::{
    Cell, Column, DataTable, Group, RowKey, TableResponse, TableState,
};
use crate::ui::widgets::{FILTER_CLEAR_INSET, SegmentedWidth, filter_field_clear, segmented};
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationRow, ConfigurationTab, ConfigurationTables, INHERITED,
    InstanceRow, PatternRow,
};

/// The widest the header's filter field grows.
const FILTER_WIDTH: f32 = 200.0;

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

pub(super) fn tables(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) {
    let catalog = &cx.model.workspace.content.configuration_sets;
    let id = match selection {
        ConfigurationSelection::Configuration(id) => Some(id),
        ConfigurationSelection::Default => None,
    };
    let editable = id.is_some_and(|id| cx.state.configuration.definition(catalog, id).is_some())
        && !cx.model.safe_mode.project_read_only();
    let visible = cx.state.configuration.visible_instances();
    let page = &mut cx.state.configuration;
    let mut actions = tab_header(ui, resolution, page, editable)
        .into_iter()
        .collect::<Vec<_>>();
    actions.extend(match page.tab {
        ConfigurationTab::Instances => instance_table(ui, page, resolution, &visible),
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

/// The band over the tables: which table, how many rows it holds, the filter
/// that finds rows in it, and on Instances the one action that adds to it.
fn tab_header(
    ui: &mut Ui,
    resolution: &ConfigurationResolution,
    page: &mut ConfigurationPageState,
    editable: bool,
) -> Option<TableAction> {
    let t = Tokens::get(ui.ctx());
    let tab = page.tab;
    let meta = match tab {
        ConfigurationTab::Instances => {
            let count = resolution.instances.len();
            format!("{count} instance{}", plural_suffix(count))
        }
        ConfigurationTab::Cells => {
            let count = resolution.cells.len();
            let cells = format!("{count} cell{}", plural_suffix(count));
            match resolution.tables.bindings_not_in_design {
                0 => cells,
                unused => format!(
                    "{cells} \u{b7} {unused} binding{} not in this design",
                    plural_suffix(unused)
                ),
            }
        }
    };
    let adds = tab == ConfigurationTab::Instances && editable;
    // A control tall, and the room around it, so the band holds its controls
    // at every density. Where the switch, the count, the action and the
    // least filter do not fit on one row, the action and the filter take a
    // second.
    let width = ui.available_width().max(1.0);
    let row_height = t.metrics.ctl_h + 6.0;
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let segments = ["Instances", "Cells"]
        .iter()
        .map(|label| {
            (text_width(ui, label, &theme::sans(tokens::FS_1, FontWeight::Medium)) + 16.0).max(54.0)
        })
        .sum::<f32>();
    let add_width = if adds {
        Button::new("Add pattern binding").measured_width(ui) + 10.0
    } else {
        0.0
    };
    let one_row = 16.0
        + segments
        + 10.0
        + text_width(ui, &meta, &meta_font)
        + 10.0
        + add_width
        + FILTER_MIN_WIDTH
        <= width;
    let rows = if one_row { 1.0 } else { 2.0 };
    let (rect, _) = ui.allocate_exact_size(vec2(width, row_height * rows), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let first = Rect::from_min_size(rect.min, vec2(width, row_height));
    let mut action = None;
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(first.shrink2(vec2(8.0, 3.0)))
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let mut index = match tab {
                ConfigurationTab::Instances => 0,
                ConfigurationTab::Cells => 1,
            };
            if segmented(
                ui,
                "configuration-tables",
                &["Instances", "Cells"],
                &mut index,
                SegmentedWidth::Natural,
            ) {
                action = Some(TableAction::Tab(if index == 0 {
                    ConfigurationTab::Instances
                } else {
                    ConfigurationTab::Cells
                }));
            }
            ui.label(
                egui::RichText::new(&meta)
                    .font(meta_font.clone())
                    .color(t.color.text_faint),
            );
            if one_row {
                filter_and_add(ui, page, tab, adds, &mut action);
            }
        },
    );
    if !one_row {
        let second = Rect::from_min_size(first.left_bottom(), vec2(width, row_height));
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(second.shrink2(vec2(8.0, 3.0)))
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                filter_and_add(ui, page, tab, adds, &mut action);
            },
        );
    }
    action
}

/// The least the filter field is made before the band takes a second row.
const FILTER_MIN_WIDTH: f32 = 120.0;

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
}

/// The filter at the band's right end, and the action that adds a pattern
/// binding before it.
fn filter_and_add(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    tab: ConfigurationTab,
    adds: bool,
    action: &mut Option<TableAction>,
) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        let (filter, name) = match tab {
            ConfigurationTab::Instances => (&mut page.instances_table.filter, "Filter instances"),
            ConfigurationTab::Cells => (&mut page.cells_table.filter, "Filter cells"),
        };
        filter_field(ui, filter, name);
        if adds && Button::new("Add pattern binding").show(ui).clicked() {
            *action = Some(TableAction::AddPattern);
        }
    });
}

/// The field that keeps only the rows whose words contain what it holds.
fn filter_field(ui: &mut Ui, filter: &mut String, name: &str) {
    let t = Tokens::get(ui.ctx());
    // egui fades the body tone for a hint; the palette's tertiary text keeps
    // its contrast on the field.
    let restored = ui.visuals_mut().weak_text_color.replace(t.color.text_faint);
    // On a row of its own beside one button it takes what the button leaves.
    let width = FILTER_WIDTH
        .min(ui.available_width() * 0.4)
        .max(FILTER_MIN_WIDTH.min(ui.available_width()))
        .max(1.0);
    let field = ui.add_sized(
        [width, t.metrics.ctl_h],
        egui::TextEdit::singleline(filter)
            .id_salt(("configuration-filter", name))
            .hint_text("Filter\u{2026}")
            .font(theme::field_sans())
            .vertical_align(egui::Align::Center)
            .margin(egui::Margin {
                left: 6,
                right: FILTER_CLEAR_INSET,
                top: 2,
                bottom: 2,
            }),
    );
    ui.visuals_mut().weak_text_color = restored;
    ui.ctx()
        .accesskit_node_builder(field.id, |node| node.set_label(name));
    filter_field_clear(ui, &field, filter);
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
        Column::words("View found", 96.0).unsortable(),
        Column::name("View to use", 110.0).unsortable(),
        // A pattern's row says which platforms it is limited to here, so
        // the column takes a share of the room the page has.
        Column::name("Bound by", 132.0).unsortable(),
        Column::words("Model section", 104.0).unsortable(),
        Column::name("Status", 120.0).unsortable(),
    ]
}

/// One row of the Instances table, in the order the table shows them.
#[derive(Debug, Clone, Copy)]
enum Line {
    /// A pattern binding, by its place among the definition's patterns.
    Pattern(usize),
    /// An instance, by its place in the tree.
    Instance(usize),
}

/// The Instances table's rows: the pattern bindings, then the tree. While a
/// filter is set every instance is a candidate, folded or not, and each is
/// named by its whole path.
struct InstanceLines<'a> {
    patterns: &'a [PatternRow],
    instances: std::borrow::Cow<'a, [usize]>,
}

impl InstanceLines<'_> {
    fn len(&self) -> usize {
        self.patterns.len() + self.instances.len()
    }

    fn line(&self, index: usize) -> Line {
        match index.checked_sub(self.patterns.len()) {
            None => Line::Pattern(index),
            Some(at) => Line::Instance(self.instances[at]),
        }
    }
}

fn instance_table(
    ui: &mut Ui,
    page: &mut ConfigurationPageState,
    resolution: &ConfigurationResolution,
    visible: &[usize],
) -> Vec<TableAction> {
    let tables = &resolution.tables;
    let hierarchy = resolution.hierarchy.as_ref().ok();
    let flat = !page.instances_table.filter.trim().is_empty();
    let instances = match hierarchy {
        None => std::borrow::Cow::Borrowed(&[][..]),
        Some(_) if flat => std::borrow::Cow::Owned((0..resolution.instances.len()).collect()),
        Some(_) => std::borrow::Cow::Borrowed(visible),
    };
    let lines = InstanceLines {
        patterns: &tables.patterns,
        instances,
    };
    let keys = (0..lines.len())
        .map(|index| {
            RowKey::Number(match lines.line(index) {
                Line::Pattern(at) => tables.patterns[at].key,
                Line::Instance(at) => tables.instance_keys[at],
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
                instance_cells(
                    tables,
                    &hierarchy.bindings,
                    at,
                    &resolution.instances[at],
                    collapsed(&resolution.instances[at].path),
                    flat,
                )
            }),
        });
    page.instances_table = state;
    let identity = |index: usize| match lines.line(index) {
        Line::Pattern(at) => ConfigurationRow::Pattern(tables.patterns[at].pattern.clone()),
        Line::Instance(at) => ConfigurationRow::Instance(resolution.instances[at].path.clone()),
    };
    let mut actions = answer(&page.instances_table, &response, &identity);
    if let Some((index, open)) = response.expand
        && let Line::Instance(at) = lines.line(index)
    {
        let row = &resolution.instances[at];
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
        view_list(&row.views),
        Cell::text(bound_by),
        row.model_section
            .as_deref()
            .map_or_else(Cell::absent, ident),
        Cell::empty(),
    ]
}

/// A view list as a cell: Inherited in words, views as identifiers.
pub(super) fn view_list(views: &str) -> Cell {
    if views == INHERITED {
        Cell::text(views)
    } else {
        ident(views)
    }
}

/// The short reason a row is not bound, or `None` when it is.
fn unbound_reason(status: HierarchyBindingStatus) -> Option<&'static str> {
    match status {
        HierarchyBindingStatus::Resolved | HierarchyBindingStatus::Modified => None,
        HierarchyBindingStatus::Unresolved => Some("no view found"),
        HierarchyBindingStatus::Recursive => Some("instantiates itself"),
        HierarchyBindingStatus::DepthLimit => Some("hierarchy too deep"),
        HierarchyBindingStatus::InstanceLimit => Some("too many instances"),
    }
}

/// One instance of the tree: indented by its depth behind its twist, or,
/// while a filter is set, named by its whole path at the left edge.
fn instance_cells(
    tables: &ConfigurationTables,
    hierarchy: &[ResolvedHierarchyBinding],
    index: usize,
    row: &InstanceRow,
    collapsed: bool,
    flat: bool,
) -> Vec<Cell> {
    let binding = &hierarchy[row.binding];
    let reference = &binding.reference;
    let instance = if flat {
        ident(&row.path)
    } else {
        let name = if row.depth == 0 {
            reference.cell.as_str()
        } else {
            row.path.rsplit('/').next().unwrap_or(&row.path)
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
    let to_use = tables.instance_to_use[index]
        .as_deref()
        .map_or_else(|| Cell::text(INHERITED), ident);
    let status = match unbound_reason(binding.status) {
        Some(reason) => Cell::problem(reason, Problem::Failure),
        None => match binding.warnings.first() {
            Some(warning) => Cell::problem(warning.as_str(), Problem::Warning),
            None => Cell::empty(),
        },
    };
    let status = match &binding.diagnostic {
        Some(diagnostic) => status.hint(diagnostic.as_str()),
        None => status,
    };
    vec![
        instance,
        ident(format!("{}/{}", reference.library, reference.cell)),
        found,
        to_use,
        Cell::text(rule_label(binding.rule)),
        if binding.model_section.is_empty() {
            Cell::absent()
        } else {
            ident(&binding.model_section)
        },
        status,
    ]
}
