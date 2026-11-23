//! Everything the project reads, as one table: its technology, its design
//! and model libraries, the folders its include lines are searched in, and
//! its decks with the files they include.
//!
//! Each row says what the dependency is, what it provides and where it is.
//! A sound one says nothing more; a problem marks its name and is named at
//! the right end of its location, in the inspector's words for the same
//! dependency: both read the rows built once per change of what a status
//! reads. The exported manifest serializes the same rows.
//!
//! One bar over the table holds what the page does: the filter and the kind
//! it lists, Add, and Change… and Remove… on the selected row; the row's
//! context menu holds the same two with the same enablement, beside what
//! only that kind of row offers. Enter takes the row's first action and
//! Delete removes it.
//!
//! The table is the kit's row list: a head click sorts it, a head edge
//! sizes its column, and only the rows in view are drawn, so a site install
//! with hundreds of model libraries costs a frame what ten do.

mod include_paths;

use std::sync::Arc;

use serde::Serialize;

pub(crate) use include_paths::{IncludePathEdit, edit_include_paths};

use super::*;
use crate::state::model_library::ModelSourceAuthority;
use crate::ui::widgets::table::{Cell, Column, DataTable, RowKey, SortKey, fitted_width};
use crate::workbench::app_state::model::LibraryMode;
use crate::workbench::project::dependency_status::{
    DependencyKind, DependencyLibrary, DependencyRow, DependencyStatusKey, UNSET, dependency_rows,
    dependency_status_key, refresh_source_drift, reread_files, settle_selection, shown_path,
    visible_dependency_rows,
};
use crate::workbench::project::library_dialogs::library_files::{
    LibraryFileIntent, library_file_blocker, library_file_label, pick_folder,
    run_library_file_intent, take_browser_folder,
};
use crate::workbench::project::properties::project_properties_refusal;
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::overview::Tone;
use rspice_ui_kit::widgets::{PageBar, PageBarFilter, page_bar_kind};

#[derive(Debug, Clone, Serialize)]
struct DependencyManifest {
    schema_version: u16,
    project_id: String,
    project_revision: u64,
    technology: Option<TechnologyManifest>,
    dependencies: Vec<ManifestDependency>,
}

#[derive(Debug, Clone, Serialize)]
struct TechnologyManifest {
    label: String,
    model_library: String,
    model_file: String,
    source_files: usize,
}

#[derive(Debug, Clone, Serialize)]
struct ManifestDependency {
    name: String,
    kind: &'static str,
    version: String,
    source: String,
    resolved: Option<String>,
    access: &'static str,
    digest: String,
    status: &'static str,
    detail: Option<String>,
}

/// The table as the page reads it: every row, and the rows the filter and
/// the kind let through, by index.
#[derive(Debug, Clone)]
struct LedgerView {
    rows: Arc<[DependencyRow]>,
    visible: Arc<[usize]>,
    /// Each visible row's key, in the same order: the table keeps its
    /// selection by it through a sort or a change of the rows.
    keys: Arc<[RowKey]>,
    /// What the rows were built from: the columns sized to what they hold
    /// are measured again only when it moves.
    status: DependencyStatusKey,
}

impl LedgerView {
    fn row(&self, library: &DependencyLibrary) -> Option<&DependencyRow> {
        self.rows.iter().find(|row| &row.library == library)
    }
}

/// What the reader asked of the page this frame, from its bar, a row's
/// menu or a key.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Asked {
    Add(Addition),
    Change(DependencyLibrary),
    Remove(DependencyLibrary),
    Refresh,
    Export,
    /// Where the dependency is edited: a design library in the Library
    /// Manager, a model library in Models, a deck in Netlist.
    Open(DependencyLibrary),
    SetAccess(String, LibraryMode),
    /// An include search path one place earlier in the search (`true`) or
    /// later.
    Move(String, bool),
    ShowInFolder(DependencyLibrary),
    /// A model library's files read again from where it was attached.
    Reload(String),
}

/// What Add brings into the project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Addition {
    Library,
    Technology,
    IncludePath,
}

impl Addition {
    const ALL: [Self; 3] = [Self::Library, Self::Technology, Self::IncludePath];

    const fn label(self) -> &'static str {
        match self {
            Self::Library => "Library\u{2026}",
            Self::Technology => "Technology\u{2026}",
            Self::IncludePath => "Include path\u{2026}",
        }
    }
}

pub fn dependencies(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    // Read from folder… is offered here too; what the browser read for it
    // goes in before the page draws.
    take_browser_folder(cx);
    refresh_source_drift(ui.ctx(), cx.model);
    let filter = cx.state.dependencies.filter.trim().to_owned();
    // A kind the project no longer has a row of lists everything again,
    // as the kind select then says.
    let rows = dependency_rows(ui.ctx(), cx.model);
    if let Some(kind) = cx.state.dependencies.kind
        && !rows.iter().any(|row| row.library.kind == kind)
    {
        cx.state.dependencies.kind = None;
    }
    let kind = cx.state.dependencies.kind;
    let (rows, visible) = visible_dependency_rows(ui.ctx(), cx.model, &filter, kind);
    settle_selection(&rows, &mut cx.state.project_dependency_selection);
    let status = dependency_status_key(ui.ctx(), cx.model);
    let keys = derived(
        ui.ctx(),
        "dependencies.keys",
        (status.clone(), filter.to_lowercase(), kind),
        || {
            visible
                .iter()
                .map(|&index| ledger_key(&rows[index].library))
                .collect::<Arc<[RowKey]>>()
        },
    );
    let view = LedgerView {
        rows,
        visible,
        keys,
        status,
    };
    let width = visible_workspace_width(ui);
    let height = visible_workspace_height(ui);
    let asked = ui
        .allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 0.0;
            let asked = dependency_bar(ui, cx, &view);
            let table = ledger(ui, cx, &view, width);
            asked.or(table)
        })
        .inner;
    if let Some(asked) = asked {
        run(ui.ctx(), cx, &view, asked);
    }
}

/// The page's one bar: the filter and the kind on the left, then Add, the
/// selected row's Change… and Remove…, Refresh and Export manifest…; how
/// many dependencies there are and whether all are found on the right.
fn dependency_bar(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    view: &LedgerView,
) -> Option<Asked> {
    let selected = cx
        .state
        .project_dependency_selection
        .as_ref()
        .and_then(|selected| view.row(selected))
        .cloned();
    let (report, problem) = bar_report(cx, view);
    PageBar::new().report(report, problem).show(ui, |ui| {
        let mut asked = None;
        PageBarFilter::new("workbench.project.dependencies", "Filter dependencies")
            .show(ui, &mut cx.state.dependencies.filter);
        let kinds = present_kinds(&view.rows);
        let choices: Vec<String> = std::iter::once(ANY_KIND.to_owned())
            .chain(kinds.iter().map(|kind| kind.label().to_owned()))
            .collect();
        let current = cx
            .state
            .dependencies
            .kind
            .and_then(|kind| kinds.iter().position(|listed| *listed == kind))
            .map_or(0, |index| index + 1);
        if let Some(index) = page_bar_kind(
            ui,
            "workbench.project.dependencies.kind",
            "Kind",
            &choices,
            current,
        ) {
            cx.state.dependencies.kind = index
                .checked_sub(1)
                .and_then(|index| kinds.get(index).copied());
        }
        // The kit's button, so Add reads as its neighbours do, with the
        // chevron of a button that opens a menu.
        let add = Button::new("Add").menu().show(ui);
        egui::Popup::menu(&add).show(|ui| {
            ui.set_min_width(160.0);
            for addition in Addition::ALL {
                let refusal = addition_refusal(cx, addition);
                if menu_item(ui, addition.label(), None, refusal.as_deref()) {
                    asked = Some(Asked::Add(addition));
                }
            }
        });
        let (change, remove) = match &selected {
            Some(row) => (change_refusal(cx, row), remove_refusal(cx, row)),
            None => (
                Some(NOTHING_SELECTED.to_owned()),
                Some(NOTHING_SELECTED.to_owned()),
            ),
        };
        if bar_button(ui, CHANGE, change.as_deref())
            && let Some(row) = &selected
        {
            asked = Some(Asked::Change(row.library.clone()));
        }
        if bar_button(ui, REMOVE, remove.as_deref())
            && let Some(row) = &selected
        {
            asked = Some(Asked::Remove(row.library.clone()));
        }
        if bar_button(ui, "Refresh", None) {
            asked = Some(Asked::Refresh);
        }
        if bar_button(ui, "Export manifest\u{2026}", None) {
            asked = Some(Asked::Export);
        }
        asked
    })
}

const ANY_KIND: &str = "Any kind";
const CHANGE: &str = "Change\u{2026}";
const REMOVE: &str = "Remove\u{2026}";
const NOTHING_SELECTED: &str = "Select a dependency first.";

/// The kinds the project has a row of, in the table's order: the kind
/// select offers only what it can show.
fn present_kinds(rows: &[DependencyRow]) -> Vec<DependencyKind> {
    DependencyKind::ALL
        .into_iter()
        .filter(|kind| rows.iter().any(|row| row.library.kind == *kind))
        .collect()
}

/// A button of the bar, disabled with its reason under the pointer.
fn bar_button(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    let response = Button::new(label).enabled(refusal.is_none()).show(ui);
    let response = match refusal {
        Some(refusal) => response.on_disabled_hover_text(refusal),
        None => response,
    };
    response.clicked()
}

/// One item of a menu, disabled with its reason under the pointer, its key
/// at the row's right end; taking it closes the menu.
fn menu_item(ui: &mut Ui, label: &str, key: Option<&str>, refusal: Option<&str>) -> bool {
    let mut button = egui::Button::new(label);
    if let Some(key) = key {
        button = button.shortcut_text(key);
    }
    let response = ui.add_enabled(refusal.is_none(), button);
    if let Some(key) = key {
        // The key is told as the item's shortcut, not as part of its name.
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(label);
            node.set_keyboard_shortcut(key);
        });
    }
    let taken = response
        .on_disabled_hover_text(refusal.unwrap_or_default())
        .clicked();
    if taken {
        ui.close();
    }
    taken
}

/// What the bar says on its right: how many dependencies there are, how
/// many the filter shows, and whether all are found or how many are not,
/// behind the worst one's mark.
fn bar_report(cx: &Cx<'_, ProjectFeature>, view: &LedgerView) -> (String, Option<Problem>) {
    let total = view.rows.len();
    let noun = if total == 1 {
        "dependency"
    } else {
        "dependencies"
    };
    let count = if view.visible.len() == total {
        format!("{total} {noun}")
    } else {
        format!("{} of {total} {noun}", view.visible.len())
    };
    let problems = view
        .rows
        .iter()
        .filter(|row| row.status.mark != Tone::Neutral)
        .count();
    let worst = view
        .rows
        .iter()
        .map(|row| row.status.mark)
        .max_by_key(|mark| severity(*mark))
        .and_then(Tone::problem);
    match cx.state.dependencies.reading() {
        Some(reading) => (format!("{count} \u{00b7} {reading}"), None),
        None => {
            let state = match problems {
                0 => "all found".to_owned(),
                1 => "1 problem".to_owned(),
                problems => format!("{problems} problems"),
            };
            (format!("{count} \u{00b7} {state}"), worst)
        }
    }
}

/// How bad a status is: the bar names the worst.
const fn severity(mark: Tone) -> u8 {
    match mark {
        Tone::Neutral => 0,
        Tone::Warn => 1,
        Tone::Error => 2,
    }
}

/// The table's columns across `width`. Name takes what the others leave;
/// Contents and Location are as wide as what they hold, Location never so
/// wide that Name is left less than its least. A column no row has a value
/// for is left out.
fn ledger_columns(ui: &Ui, view: &LedgerView, width: f32) -> Vec<(LedgerColumn, Column)> {
    let rows = &view.rows;
    let has = |value: fn(&DependencyRow) -> bool| rows.iter().any(value);
    let [contents, location] = fitted_widths(ui, view);
    let shown: Vec<(LedgerColumn, f32)> = [
        (LedgerColumn::Name, NAME_LEAST),
        (LedgerColumn::Kind, 104.0),
        (LedgerColumn::Version, 72.0),
        (LedgerColumn::Contents, contents),
        (LedgerColumn::Location, location),
        (LedgerColumn::Access, 80.0),
    ]
    .into_iter()
    .filter(|(column, _)| match column {
        LedgerColumn::Version => has(|row| row.version != UNSET),
        LedgerColumn::Contents => has(|row| row.contents.is_some()),
        LedgerColumn::Access => has(|row| row.access != UNSET),
        _ => true,
    })
    .collect();
    let others: f32 = shown
        .iter()
        .filter(|(column, _)| *column != LedgerColumn::Location)
        .map(|(_, least)| least)
        .sum();
    // A path cut short shows whole under the pointer.
    let location = location.min((width - others).max(LOCATION_LEAST));
    shown
        .into_iter()
        .map(|(column, least)| {
            let head = column.head();
            let shown = match column {
                LedgerColumn::Name => Column::name(head, least),
                LedgerColumn::Location => Column::words(head, location),
                _ => Column::words(head, least),
            };
            (column, shown)
        })
        .collect()
}

/// The least the Name column takes, which it is given before the rest.
const NAME_LEAST: f32 = 120.0;

/// The least Location is cut to beside a long name: a short path whole.
const LOCATION_LEAST: f32 = 120.0;

/// The widths Contents and Location take to show every row whole, measured
/// when the rows change, not on every frame.
fn fitted_widths(ui: &Ui, view: &LedgerView) -> [f32; 2] {
    derived(
        ui.ctx(),
        "dependencies.widths",
        (view.status.clone(), ui.ctx().pixels_per_point().to_bits()),
        || {
            [LedgerColumn::Contents, LedgerColumn::Location].map(|column| {
                let cells: Vec<Cell> = view
                    .rows
                    .iter()
                    .map(|row| ledger_cell(row, column))
                    .collect();
                fitted_width(ui, column.head(), &cells)
            })
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerColumn {
    Name,
    Kind,
    Version,
    Contents,
    Location,
    Access,
}

impl LedgerColumn {
    const fn head(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Kind => "Kind",
            Self::Version => "Version",
            Self::Contents => "Contents",
            Self::Location => "Location",
            Self::Access => "Access",
        }
    }
}

/// Two hints in one, the first above the second.
fn joined(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => Some(format!("{first}\n{second}")),
        (first, second) => first.or(second).map(str::to_owned),
    }
}

/// One row's cell under `column`. A sound row's cells state what it is; a
/// problem marks its name, which reads its detail out with the row, and
/// names itself at the right end of its location, where the pointer finds
/// the detail too.
fn ledger_cell(row: &DependencyRow, column: LedgerColumn) -> Cell {
    let absent = || Cell::absent().sort_by(SortKey::Missing);
    let problem = row.status.mark.problem();
    let detail = problem.and(row.status.detail.as_deref());
    match column {
        LedgerColumn::Name => {
            let name = if row.library.name.is_empty() {
                absent()
            } else {
                Cell::text(row.library.shown_name())
            };
            let whole = (!row.library.name.is_empty()
                && row.library.shown_name() != row.library.name)
                .then_some(row.library.name.as_str());
            let name = match problem {
                Some(problem) => name.marked(problem),
                None => name,
            };
            match joined(whole, detail) {
                Some(hint) => name.hint(hint),
                None => name,
            }
        }
        LedgerColumn::Kind => Cell::text(row.library.kind.label()),
        LedgerColumn::Version if row.version == UNSET => absent(),
        LedgerColumn::Version => Cell::number(row.version.as_str()),
        LedgerColumn::Contents => match &row.contents {
            Some(contents) => {
                let cell = Cell::text(contents.words.as_str())
                    .sort_by(SortKey::Number(contents.count as f64));
                match contents.more.as_deref() {
                    Some(more) => cell.suffix(more),
                    None => cell,
                }
            }
            None => absent(),
        },
        LedgerColumn::Location => {
            let location = if row.location == UNSET {
                absent()
            } else {
                Cell::text(row.location.as_str())
            };
            let location = match problem {
                Some(_) => location.state(row.status.label, None),
                None => location,
            };
            let whole = row
                .resolved
                .as_deref()
                .filter(|whole| *whole != row.location);
            match joined(whole, detail) {
                Some(hint) => location.hint(hint),
                None => location,
            }
        }
        LedgerColumn::Access if row.access == UNSET => absent(),
        LedgerColumn::Access => Cell::text(row.access),
    }
}

/// A row's key: its kind and its name, since a design and a model library
/// may share a name.
fn ledger_key(library: &DependencyLibrary) -> RowKey {
    RowKey::Text(format!("{}\u{1f}{}", library.kind.label(), library.name))
}

/// The table, under the bar to the floor of the page, its selection kept
/// with the inspector's, and what the reader asked of a row: Enter or a
/// double-click its first action, the context menu what can be done to it,
/// Delete removes it, Alt+Up and Alt+Down move an include search path.
fn ledger(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    view: &LedgerView,
    width: f32,
) -> Option<Asked> {
    if view.rows.is_empty() {
        workspace_empty_table_row(ui, width, "The project reads nothing.");
        return None;
    }
    if view.visible.is_empty() {
        workspace_empty_table_row(ui, width, "No dependency matches the filter.");
        return None;
    }
    let keys = &view.keys;
    let selected_library = cx.state.project_dependency_selection.clone();
    // The keys the table read last frame: Alt+Up and Alt+Down move the
    // selected include search path before the table takes the arrows as
    // a move of the selection.
    let moved = if cx.state.dependencies.focused {
        selected_library
            .as_ref()
            .filter(|library| library.kind == DependencyKind::IncludePath)
            .and_then(|library| {
                ui.input_mut(|input| {
                    if input.consume_key(Modifiers::ALT, Key::ArrowUp) {
                        Some(Asked::Move(library.name.clone(), true))
                    } else if input.consume_key(Modifiers::ALT, Key::ArrowDown) {
                        Some(Asked::Move(library.name.clone(), false))
                    } else {
                        None
                    }
                })
            })
    } else {
        None
    };
    let selected = selected_library.as_ref().and_then(|selected| {
        view.visible
            .iter()
            .position(|&index| &view.rows[index].library == selected)
    });
    let table = &mut cx.state.dependencies.table;
    match selected {
        Some(row) if table.selected_key() != Some(&keys[row]) => {
            table.select(row, Some(keys[row].clone()));
        }
        None if table.selected_row().is_some() => {
            table.selection = None;
            table.others.clear();
            table.anchor = None;
        }
        _ => {}
    }
    let columns = ledger_columns(ui, view, width);
    let heads: Vec<Column> = columns.iter().map(|(_, column)| column.clone()).collect();
    // Narrower than the columns' least, the table scrolls sideways and
    // keeps the location, where a problem is named, beside the name.
    let location_column = columns
        .iter()
        .position(|(column, _)| *column == LedgerColumn::Location)
        .unwrap_or(heads.len() - 1);
    let response = DataTable::new(
        "workbench.project.dependencies.ledger",
        &heads,
        view.visible.len(),
    )
    .state(table)
    .keys(keys)
    .freeze(1)
    .pin_when_narrow(location_column)
    .max_height(ui.available_height().max(1.0))
    .show(ui, |row| {
        let row = &view.rows[view.visible[row]];
        columns
            .iter()
            .map(|(column, _)| ledger_cell(row, *column))
            .collect()
    });
    cx.state.dependencies.focused = response.focused;
    let library_at = |row: usize| {
        view.visible
            .get(row)
            .map(|&index| view.rows[index].library.clone())
    };
    if response.selection_changed {
        cx.state.project_dependency_selection = cx
            .state
            .dependencies
            .table
            .selected_row()
            .and_then(library_at);
    }
    if moved.is_some() {
        return moved;
    }
    if let Some(library) = response.activate.and_then(library_at) {
        return Some(first_action(library));
    }
    if response.delete
        && let Some(library) = cx
            .state
            .dependencies
            .table
            .selected_row()
            .and_then(library_at)
    {
        return Some(Asked::Remove(library));
    }
    ledger_menu(
        ui,
        cx,
        view,
        response.context_menu.and_then(|request| {
            library_at(request.cell.row).map(|library| (library, request.position))
        }),
    )
}

/// What Enter and a double-click ask of a row: a design library opens in
/// the Library Manager and a deck in Netlist, where each is edited; any
/// other row is changed.
fn first_action(library: DependencyLibrary) -> Asked {
    match library.kind {
        DependencyKind::Design | DependencyKind::Deck => Asked::Open(library),
        _ => Asked::Change(library),
    }
}

/// A row's context menu: opened where the reader asked for it, on the row
/// they asked it of, and kept open until an item is taken or they click
/// elsewhere. An item that cannot be taken now is shown, disabled, with
/// its reason under the pointer.
fn ledger_menu(
    ui: &Ui,
    cx: &Cx<'_, ProjectFeature>,
    view: &LedgerView,
    request: Option<(DependencyLibrary, egui::Pos2)>,
) -> Option<Asked> {
    let id = ui.id().with("workbench.project.dependencies.row menu");
    let at = id.with("at");
    let opened = request.is_some();
    if let Some(request) = request {
        ui.ctx().data_mut(|data| data.insert_temp(at, request));
    }
    let (library, position) = ui
        .ctx()
        .data(|data| data.get_temp::<(DependencyLibrary, egui::Pos2)>(at))?;
    if !opened && !egui::Popup::is_id_open(ui.ctx(), id) {
        return None;
    }
    let row = view.row(&library)?.clone();
    let items = menu_items(cx, &row);
    let mut taken = None;
    egui::Popup::new(
        id,
        ui.ctx().clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    // A menu, not a stack of buttons: one column of full-width rows in the
    // menu style, as every desktop tool's context menu reads.
    .layout(egui::Layout::top_down_justified(egui::Align::Min))
    .style(egui::containers::menu::menu_style)
    .open_memory(opened.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        ui.set_min_width(180.0);
        for item in items {
            match item {
                MenuItem::Separator => {
                    ui.separator();
                }
                MenuItem::Action(action) => {
                    if menu_item(ui, &action.label, action.key, action.refusal.as_deref()) {
                        taken = Some(action.asked);
                    }
                }
            }
        }
    });
    taken
}

enum MenuItem {
    Action(MenuAction),
    Separator,
}

/// A command of a row's menu: its words, the key that takes it from the
/// table, what it asks for and why it cannot now.
struct MenuAction {
    label: String,
    key: Option<&'static str>,
    asked: Asked,
    refusal: Option<String>,
}

impl MenuItem {
    fn action(label: impl Into<String>, asked: Asked, refusal: Option<String>) -> Self {
        Self::Action(MenuAction {
            label: label.into(),
            key: None,
            asked,
            refusal,
        })
    }

    fn key(mut self, key: &'static str) -> Self {
        if let Self::Action(action) = &mut self {
            action.key = Some(key);
        }
        self
    }
}

/// What a row's menu offers, in order: where it is edited, Change… and
/// Remove… as the bar offers them, then what only its kind offers.
fn menu_items(cx: &Cx<'_, ProjectFeature>, row: &DependencyRow) -> Vec<MenuItem> {
    let library = &row.library;
    let mut items = Vec::new();
    let open = match library.kind {
        DependencyKind::Design => Some("Open in Library Manager"),
        DependencyKind::Model => Some("Open in Models"),
        DependencyKind::Deck => Some("Open in Netlist"),
        _ => None,
    };
    // Enter takes what activating the row does: opening a design library
    // or a deck where it is edited, changing anything else.
    let enter_opens = matches!(library.kind, DependencyKind::Design | DependencyKind::Deck);
    if let Some(open) = open {
        let item = MenuItem::action(open, Asked::Open(library.clone()), None);
        items.push(if enter_opens { item.key("Enter") } else { item });
    }
    let change = MenuItem::action(
        CHANGE,
        Asked::Change(library.clone()),
        change_refusal(cx, row),
    );
    items.push(if enter_opens {
        change
    } else {
        change.key("Enter")
    });
    items.push(
        MenuItem::action(
            REMOVE,
            Asked::Remove(library.clone()),
            remove_refusal(cx, row),
        )
        .key("Delete"),
    );
    let mut more = Vec::new();
    match library.kind {
        DependencyKind::Design => {
            if let Some(reference) = cx.model.library_manager.reference(&library.name) {
                let mode = if reference.mode.is_editable() {
                    LibraryMode::ReadOnly
                } else {
                    LibraryMode::Editable
                };
                let intent = LibraryFileIntent::SetMode(library.name.clone(), mode);
                more.push(MenuItem::action(
                    library_file_label(&intent),
                    Asked::SetAccess(library.name.clone(), mode),
                    library_file_blocker(cx.model, &intent),
                ));
            }
        }
        DependencyKind::Model if cfg!(not(target_arch = "wasm32")) => {
            if let Some(found) = cx.model.model_library_manager.get_library(&library.name)
                && found.has_external_root()
                && matches!(found.source_authority, ModelSourceAuthority::External)
            {
                more.push(MenuItem::action(
                    "Reload",
                    Asked::Reload(library.name.clone()),
                    reading_refusal(cx),
                ));
            }
        }
        DependencyKind::IncludePath => {
            for (label, key, up) in [
                ("Move up", "Alt+Up", true),
                ("Move down", "Alt+Down", false),
            ] {
                let refusal = (!include_paths::can_move(cx.model, &library.name, up)).then(|| {
                    format!(
                        "It is searched {} already.",
                        if up { "first" } else { "last" }
                    )
                });
                more.push(
                    MenuItem::action(
                        label,
                        Asked::Move(library.name.clone(), up),
                        read_only_refusal(cx).or(refusal),
                    )
                    .key(key),
                );
            }
        }
        _ => {}
    }
    if cfg!(not(target_arch = "wasm32")) && shown_folder(cx, row).is_some() {
        more.push(MenuItem::action(
            "Show in folder",
            Asked::ShowInFolder(library.clone()),
            None,
        ));
    }
    if !more.is_empty() {
        items.push(MenuItem::Separator);
        items.extend(more);
    }
    items
}

/// Why nothing can change the project's properties now.
fn read_only_refusal(cx: &Cx<'_, ProjectFeature>) -> Option<String> {
    project_properties_refusal(cx.model).map(str::to_owned)
}

/// Why a model library cannot be read now: another is being read.
fn reading_refusal(cx: &Cx<'_, ProjectFeature>) -> Option<String> {
    cx.state
        .dependencies
        .reading()
        .map(|reading| format!("{reading} Wait until it is in."))
}

/// Why Add cannot bring `addition` in now, if it cannot.
fn addition_refusal(cx: &Cx<'_, ProjectFeature>, addition: Addition) -> Option<String> {
    match addition {
        Addition::Library => library_file_blocker(cx.model, &LibraryFileIntent::Attach),
        Addition::Technology => cx
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_some()
            .then(|| "A technology is attached: change it from its row.".to_owned()),
        Addition::IncludePath => read_only_refusal(cx).or_else(|| {
            cfg!(target_arch = "wasm32")
                .then(|| "A browser cannot pick a folder of this computer.".to_owned())
        }),
    }
}

/// What Change… does to a design library: a library kept in a folder of its
/// own is relinked to another folder, or in a browser read from one.
fn relink_intent(name: &str) -> LibraryFileIntent {
    if cfg!(target_arch = "wasm32") {
        LibraryFileIntent::ReadFromFolder(name.to_owned())
    } else {
        LibraryFileIntent::Relink(name.to_owned())
    }
}

/// Why Change… cannot act on `row` now, if it cannot.
fn change_refusal(cx: &Cx<'_, ProjectFeature>, row: &DependencyRow) -> Option<String> {
    let name = &row.library.name;
    match row.library.kind {
        DependencyKind::Technology => None,
        DependencyKind::Design => {
            if cx.model.library_manager.is_referenced(name) {
                library_file_blocker(cx.model, &relink_intent(name))
            } else {
                Some(
                    "The project file holds this library: there is no folder to relink.".to_owned(),
                )
            }
        }
        DependencyKind::Model => {
            if cfg!(target_arch = "wasm32") {
                return Some("A browser cannot read a model file of this computer.".to_owned());
            }
            let found = cx.model.model_library_manager.get_library(name)?;
            if !found.source_authority.has_execution_source() {
                return Some("It ships with RSpice.".to_owned());
            }
            let relinkable = found.pack_id.is_none()
                && matches!(
                    found.source_authority,
                    ModelSourceAuthority::External | ModelSourceAuthority::RetainedImport { .. }
                );
            if relinkable {
                reading_refusal(cx)
            } else {
                Some("Only a library read from a file can be relinked.".to_owned())
            }
        }
        DependencyKind::IncludePath => read_only_refusal(cx).or_else(|| {
            cfg!(target_arch = "wasm32")
                .then(|| "A browser cannot pick a folder of this computer.".to_owned())
        }),
        DependencyKind::Deck => Some("A deck is edited in Netlist.".to_owned()),
        DependencyKind::Include => {
            Some("An include is changed in the deck line that names it.".to_owned())
        }
    }
}

/// Why Remove… cannot act on `row` now, if it cannot.
fn remove_refusal(cx: &Cx<'_, ProjectFeature>, row: &DependencyRow) -> Option<String> {
    let name = &row.library.name;
    match row.library.kind {
        DependencyKind::Technology => cx
            .model
            .workspace
            .content
            .project
            .technology_binding()
            .is_none()
            .then(|| "No technology is attached.".to_owned()),
        DependencyKind::Design => {
            if cx.model.library_manager.is_referenced(name) {
                library_file_blocker(cx.model, &LibraryFileIntent::Detach(name.clone()))
            } else {
                None
            }
        }
        DependencyKind::Model => {
            let found = cx.model.model_library_manager.get_library(name)?;
            if !found.source_authority.has_execution_source() {
                return Some("It ships with RSpice.".to_owned());
            }
            if cx
                .model
                .workspace
                .content
                .project
                .technology_binding()
                .is_some_and(|binding| binding.model_library() == name)
            {
                return Some(
                    "The project's technology simulates from it: detach the technology first."
                        .to_owned(),
                );
            }
            reading_refusal(cx)
        }
        DependencyKind::IncludePath => read_only_refusal(cx),
        DependencyKind::Deck => Some("A deck is removed in Netlist.".to_owned()),
        DependencyKind::Include => {
            Some("An include is removed with the deck line that names it.".to_owned())
        }
    }
}

/// The folder Show in folder opens for `row`: a design library's or an
/// include search path's own, the one a file is in; none for what is not
/// on this computer's disk.
fn shown_folder(cx: &Cx<'_, ProjectFeature>, row: &DependencyRow) -> Option<std::path::PathBuf> {
    if row.status.label == "Not found" {
        return None;
    }
    let resolved = std::path::PathBuf::from(row.resolved.as_deref()?);
    match row.library.kind {
        DependencyKind::Design => cx
            .model
            .library_manager
            .is_referenced(&row.library.name)
            .then_some(resolved),
        DependencyKind::IncludePath => Some(resolved),
        _ => resolved.parent().map(std::path::Path::to_path_buf),
    }
}

/// Carry out what the reader asked, if the page still offers it: a refusal
/// found now is stated on the status line rather than acted over.
fn run(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, view: &LedgerView, asked: Asked) {
    match asked {
        Asked::Add(addition) => {
            if let Some(refusal) = addition_refusal(cx, addition) {
                cx.wb.prompt(refusal);
                return;
            }
            match addition {
                Addition::Library => run_library_file_intent(cx, LibraryFileIntent::Attach),
                Addition::Technology => open_technology_attachment_dialog(cx),
                Addition::IncludePath => {
                    if let Some(folder) = pick_folder("Add include search path") {
                        let stored = include_paths::stored_include_path(
                            &folder,
                            cx.model.workspace.content.project.data_root(),
                        );
                        let selected =
                            DependencyLibrary::include_path(stored.display().to_string());
                        if edit_include_path(cx, IncludePathEdit::Add(stored)) {
                            cx.state.project_dependency_selection = Some(selected);
                        }
                    }
                }
            }
        }
        Asked::Change(library) => {
            let Some(row) = view.row(&library).cloned() else {
                return;
            };
            if let Some(refusal) = change_refusal(cx, &row) {
                cx.wb.prompt(refusal);
                return;
            }
            change(ctx, cx, &row);
        }
        Asked::Remove(library) => {
            let Some(row) = view.row(&library).cloned() else {
                return;
            };
            if let Some(refusal) = remove_refusal(cx, &row) {
                cx.wb.prompt(refusal);
                return;
            }
            remove(cx, &row);
        }
        Asked::Refresh => refresh(ctx, cx),
        Asked::Export => export_dependency_manifest(cx, &view.rows),
        Asked::Open(library) => open_dependency(ctx, cx, &library),
        Asked::SetAccess(name, mode) => {
            run_library_file_intent(cx, LibraryFileIntent::SetMode(name, mode));
        }
        Asked::Move(written, up) => {
            if include_paths::can_move(cx.model, &written, up) {
                edit_include_path(cx, IncludePathEdit::Move(written, up));
            }
        }
        Asked::ShowInFolder(library) => {
            let Some(row) = view.row(&library).cloned() else {
                return;
            };
            show_row_folder(cx, &row);
        }
        Asked::Reload(name) => {
            if let Some(refusal) = reading_refusal(cx) {
                cx.wb.prompt(refusal);
                return;
            }
            #[cfg(not(target_arch = "wasm32"))]
            start_model_library_read(ctx, cx, &name, None);
            #[cfg(target_arch = "wasm32")]
            let _ = name;
        }
    }
}

/// Change… on `row`: the technology through its change flow, a library
/// relinked, an include search path pointed at another folder.
fn change(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, row: &DependencyRow) {
    let name = row.library.name.clone();
    match row.library.kind {
        DependencyKind::Technology => open_technology_attachment_dialog(cx),
        DependencyKind::Design => run_library_file_intent(cx, relink_intent(&name)),
        DependencyKind::Model => {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(file) = pick_model_file(&name) {
                start_model_library_read(ctx, cx, &name, Some(file));
            }
            #[cfg(target_arch = "wasm32")]
            let _ = ctx;
        }
        DependencyKind::IncludePath => {
            if let Some(folder) = pick_folder("Change include search path") {
                let stored = include_paths::stored_include_path(
                    &folder,
                    cx.model.workspace.content.project.data_root(),
                );
                let selected = DependencyLibrary::include_path(stored.display().to_string());
                if edit_include_path(cx, IncludePathEdit::Change(name, stored)) {
                    cx.state.project_dependency_selection = Some(selected);
                }
            }
        }
        DependencyKind::Deck | DependencyKind::Include => {}
    }
}

/// Remove… on `row`, through each kind's own flow: the technology detached
/// after its review, a library kept in a folder of its own detached after
/// its review and one the project file holds deleted after its review, a
/// model library and an include search path taken out as one step of Undo.
fn remove(cx: &mut Cx<'_, ProjectFeature>, row: &DependencyRow) {
    let name = row.library.name.clone();
    match row.library.kind {
        DependencyKind::Technology => open_technology_detach_dialog(cx),
        DependencyKind::Design => {
            if cx.model.library_manager.is_referenced(&name) {
                run_library_file_intent(cx, LibraryFileIntent::Detach(name));
            } else if let Err(refusal) = cx.state.open_delete_library_review(cx.model, &name) {
                cx.wb.prompt(refusal);
            }
        }
        DependencyKind::Model => remove_model_library(cx, &name),
        DependencyKind::IncludePath => {
            edit_include_path(cx, IncludePathEdit::Remove(name));
        }
        DependencyKind::Deck | DependencyKind::Include => {}
    }
}

/// Edit the include search paths as one step of Undo, stating what it did
/// in the log, or the refusal on the status line. Whether it was done.
fn edit_include_path(cx: &mut Cx<'_, ProjectFeature>, edit: IncludePathEdit) -> bool {
    match edit_include_paths(cx.model, edit) {
        Ok(done) => {
            cx.model.record(LogSeverity::Info, LogSource::Studio, done);
            true
        }
        Err(refusal) => {
            cx.wb.prompt(refusal);
            false
        }
    }
}

/// Read everything the page states from the disk again: each library
/// folder, each attached model file's bytes, the include search paths and
/// the decks with their includes. What changed outside RSpice shows in the
/// rows; the status line says where the project stands.
fn refresh(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    cx.model.check_library_files();
    cx.model.scan_source_drift();
    reread_files(ctx);
    let rows = crate::workbench::project::dependency_status::dependency_rows(ctx, cx.model);
    let problems = rows
        .iter()
        .filter(|row| row.status.mark != Tone::Neutral)
        .count();
    cx.wb.prompt(match problems {
        0 => "Read every dependency again: all found.".to_owned(),
        1 => "Read every dependency again: 1 problem.".to_owned(),
        problems => format!("Read every dependency again: {problems} problems."),
    });
}

/// Show the folder of `row` in the file manager.
fn show_row_folder(cx: &mut Cx<'_, ProjectFeature>, row: &DependencyRow) {
    if row.library.kind == DependencyKind::Design {
        run_library_file_intent(
            cx,
            LibraryFileIntent::ShowInFolder(row.library.name.clone()),
        );
        return;
    }
    let Some(folder) = shown_folder(cx, row) else {
        return;
    };
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(error) = show_folder(&folder) {
        cx.model.report_failure("Folder not shown", error);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = folder;
}

/// Open `library` where it is edited: a design library selected in the
/// Library Manager, a model library in Models, a deck in Netlist.
pub(in crate::workbench) fn open_dependency(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    library: &DependencyLibrary,
) {
    match library.kind {
        DependencyKind::Design => {
            super::library::apply_library_intent(
                ctx,
                cx,
                super::library::LibraryIntent::SelectLibrary(library.name.clone()),
            );
            cx.state.project_page = ProjectPage::Library;
        }
        DependencyKind::Model => open_model_library(cx, &library.name),
        DependencyKind::Deck => cx.execute(ids::open_workspace(Workspace::Netlist)),
        _ => {}
    }
}

/// Show model library `name` in Models. The catalog lists models, not
/// libraries, so the library opens on its first model in the catalog's own
/// order; a library that holds none opens the catalog with it selected.
fn open_model_library(cx: &mut Cx<'_, ProjectFeature>, name: &str) {
    let first = cx
        .model
        .model_library_manager
        .get_library(name)
        .and_then(|library| {
            library
                .models
                .keys()
                .min_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()).then(a.cmp(b)))
                .cloned()
        });
    match first {
        Some(model) => cx
            .wb
            .outbox
            .push(crate::workbench::framework::requests::RevealModel {
                library: name.to_owned(),
                model,
            }),
        None => match cx.model.model_library_manager.select_library(name) {
            Ok(()) => cx.execute(ids::MODELS_CATALOG),
            Err(refusal) => cx.wb.prompt(refusal),
        },
    }
}

/// Take model library `name` out of the project, as one step of Undo. The
/// library the project's technology simulates from stays until the
/// technology is detached.
fn remove_model_library(cx: &mut Cx<'_, ProjectFeature>, name: &str) {
    if cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .is_some_and(|binding| binding.model_library() == name)
    {
        cx.wb.prompt(format!(
            "The project's technology simulates from '{name}'. Detach the technology before removing it."
        ));
        return;
    }
    let mut candidate = cx.model.model_library_manager.clone();
    if candidate.remove_library(name).is_none() {
        cx.wb.prompt(format!(
            "Model library '{name}' is no longer in the project."
        ));
        return;
    }
    let result = if cx.platform.project_lifecycle.is_open() {
        crate::workbench::app_state::design_history::publish_model_library_set_candidate(
            cx.model,
            candidate,
            format!("remove model library {name}"),
        )
        .map(|revision| {
            revision.map_or_else(
                || format!("Removed model library '{name}'."),
                |revision| {
                    format!(
                        "Removed model library '{name}' at project revision {}.",
                        revision.get()
                    )
                },
            )
        })
    } else {
        cx.model.model_library_manager = candidate;
        Ok(format!("Removed model library '{name}'."))
    };
    match result {
        Ok(message) => {
            cx.model
                .record(LogSeverity::Info, LogSource::Studio, message);
        }
        Err(error) => cx.model.report_failure("Model library not removed", error),
    }
}

#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    crate::workbench::lifecycle::recovery::reveal_directory(folder)
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
fn show_folder(folder: &std::path::Path) -> Result<(), String> {
    crate::workbench::app::dialogs::project::SHOWN_FOLDERS
        .with(|shown| shown.borrow_mut().push(folder.to_path_buf()));
    Ok(())
}

/// The model file the reader chooses for library `name`.
#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn pick_model_file(name: &str) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_title(format!("Relink model library {name}"))
        .add_filter(
            "SPICE model source",
            crate::state::pdk_config::MODEL_FILE_EXTENSIONS,
        )
        .add_filter("All files", &["*"])
        .pick_file()
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
fn pick_model_file(_: &str) -> Option<std::path::PathBuf> {
    crate::workbench::app::dialogs::project::RELINK_PICK.with(|pick| pick.borrow_mut().take())
}

/// Read model library `name` again, from `relink` when the reader chose a
/// new file and else from where it was attached, on a background thread.
/// The page states the one read in progress; the library changes as one
/// step of Undo when it is in.
#[cfg(not(target_arch = "wasm32"))]
fn start_model_library_read(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    name: &str,
    relink: Option<std::path::PathBuf>,
) {
    let Some(library) = cx.model.model_library_manager.get_library(name) else {
        return;
    };
    let Some(root) = relink.clone().or_else(|| library.root_path.clone()) else {
        return;
    };
    // A library is named by its file: the file it is relinked to keeps the
    // name, so every model bound to it still finds it.
    if relink.is_some() && root.file_stem().and_then(|stem| stem.to_str()) != Some(name) {
        cx.wb.prompt(format!(
            "Choose a file named '{name}': model library '{name}' is named by its file."
        ));
        return;
    }
    let section = library.selected_corner.clone();
    let pack = library.pack_id.clone().filter(|_| relink.is_none());
    let mut candidate = cx.model.model_library_manager.clone();
    let expected = name.to_owned();
    let file = root.file_name().map_or_else(
        || root.display().to_string(),
        |file| file.to_string_lossy().into_owned(),
    );
    let relinked = relink.is_some();
    let reading = crate::workbench::app::dialogs::project::ModelLibraryRead::start(
        ctx,
        name.to_owned(),
        file,
        relinked,
        cx.model.workspace.content.project.revision().get(),
        move || {
            if relinked {
                candidate.remove_library(&expected);
            }
            let loaded = match pack {
                Some(pack) => candidate.refresh_spice_pack(&pack),
                None => candidate.load_library_file(&root, section.as_deref()),
            }?;
            if loaded != expected {
                return Err(format!(
                    "The file read as model library '{loaded}', not '{expected}'."
                ));
            }
            Ok(candidate)
        },
    );
    cx.state.dependencies.read = Some(reading);
}

/// Take a model library read once it is in: the library as its files now
/// say, as one step of Undo, or what stopped it.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn take_model_library_read(cx: &mut Cx<'_, ProjectFeature>) {
    let Some(answer) = cx
        .state
        .dependencies
        .read
        .as_ref()
        .and_then(crate::workbench::app::dialogs::project::ModelLibraryRead::take_answer)
    else {
        return;
    };
    let Some(read) = cx.state.dependencies.read.take() else {
        return;
    };
    let name = read.library;
    let failure = if read.relink {
        "Model library not relinked"
    } else {
        "Model library not reloaded"
    };
    if cx.model.workspace.content.project.revision().get() != read.project_revision {
        cx.model.report_failure(
            failure,
            format!("The project changed while '{name}' was read; nothing was changed."),
        );
        return;
    }
    let candidate = match answer {
        Ok(candidate) => candidate,
        Err(error) => {
            cx.model.report_failure(failure, error);
            return;
        }
    };
    let unchanged = cx
        .model
        .model_library_manager
        .get_library(&name)
        .zip(candidate.get_library(&name))
        .is_some_and(|(before, after)| {
            before.root_path == after.root_path
                && before.source_closure == after.source_closure
                && before.source_authority == after.source_authority
        });
    if unchanged {
        cx.model.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!("Model library '{name}' already matches its files."),
        );
        return;
    }
    let verb = if read.relink { "Relinked" } else { "Reloaded" };
    let result = if cx.platform.project_lifecycle.is_open() {
        crate::workbench::app_state::design_history::publish_model_library_candidate(
            cx.model,
            candidate,
            &name,
            format!("{} model library {name}", verb.to_lowercase()),
        )
        .map(|revision| {
            format!(
                "{verb} model library '{name}' from {} at project revision {}.",
                read.file,
                revision.get()
            )
        })
    } else {
        cx.model.model_library_manager = candidate;
        Ok(format!("{verb} model library '{name}' from {}.", read.file))
    };
    match result {
        Ok(message) => {
            cx.model
                .record(LogSeverity::Info, LogSource::Studio, message);
        }
        Err(error) => cx.model.report_failure(failure, error),
    }
}

fn dependency_manifest(cx: &Cx<'_, ProjectFeature>, rows: &[DependencyRow]) -> DependencyManifest {
    let technology = cx
        .model
        .workspace
        .content
        .project
        .technology_binding()
        .map(|binding| TechnologyManifest {
            label: binding.display_label(),
            model_library: binding.model_library().to_owned(),
            model_file: shown_path(binding.root_source()),
            source_files: binding.source_closure().len(),
        });
    DependencyManifest {
        schema_version: 3,
        project_id: cx.model.workspace.content.project.id().to_string(),
        project_revision: cx.model.workspace.content.project.revision().get(),
        technology,
        dependencies: rows
            .iter()
            .map(|row| ManifestDependency {
                name: row.library.name.clone(),
                kind: row.library.kind.label(),
                version: row.version.clone(),
                source: row.source.clone(),
                resolved: row.resolved.clone(),
                access: row.access,
                digest: row.digest.clone(),
                status: row.status.label,
                detail: row.status.detail.clone(),
            })
            .collect(),
    }
}

fn export_dependency_manifest(cx: &mut Cx<'_, ProjectFeature>, rows: &[DependencyRow]) {
    let contents = match serde_json::to_vec_pretty(&dependency_manifest(cx, rows)) {
        Ok(contents) => contents,
        Err(error) => {
            cx.model.report_failure(
                "Manifest export failed",
                format!("Dependency manifest serialization failed: {error}"),
            );
            return;
        }
    };
    let filename = format!(
        "{}-dependencies.json",
        cx.model
            .workspace
            .content
            .project
            .display_name()
            .chars()
            .map(|character| if character.is_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
            .trim_matches('-')
    );

    match crate::workbench::workflows::export_workflow::publish_project_manifest(
        &filename, &contents,
    ) {
        Ok(Some(exported)) => cx.wb.prompt(exported),
        Ok(None) => {}
        Err(error) => cx.model.report_failure("Manifest export failed", error),
    }
}
