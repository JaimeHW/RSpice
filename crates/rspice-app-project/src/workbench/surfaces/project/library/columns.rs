//! The Library page's three columns, each a list of the one table: the
//! libraries, the selected library's cells and the selected cell's views.
//!
//! The arrows and typed letters move within a column, Left and Right between
//! columns. Enter or a double-click opens what a row names (a library's cells,
//! a cell's default view, a view), F2 renames it, Delete reviews its deletion,
//! and the menu key or a right-click opens the band's menu for it.
//!
//! The library manager's selection is the one the lists show. Undo, the rail
//! and the band write it too, so each list follows it, and what the reader
//! selects in a list is written back to it.

use rspice_app_state::veriloga_library::{
    BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS, BuiltinVerilogAModel,
};

use super::*;
use crate::ui::widgets::table::{
    Cell as Entry, Column, ContextRequest, DataTable, Group, LineHeights, RowDrag, RowKey,
    Selection, TableResponse, TableState,
};

/// One of the page's three columns.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BrowserColumn {
    Library,
    #[default]
    Cell,
    View,
}

impl BrowserColumn {
    /// The column Left (`by` < 0) or Right reaches from this one.
    const fn beside(self, by: isize) -> Option<Self> {
        match (self, by > 0) {
            (Self::Library, true) => Some(Self::Cell),
            (Self::Cell, true) => Some(Self::View),
            (Self::Cell, false) => Some(Self::Library),
            (Self::View, false) => Some(Self::Cell),
            (Self::Library, false) | (Self::View, true) => None,
        }
    }
}

/// What the Library page's columns keep between frames: each list's
/// selection and keyboard memory, the column the keyboard moves into next and
/// the one the reader last worked in.
#[derive(Debug, Clone, Default)]
pub struct LibraryBrowserState {
    pub libraries: TableState,
    pub cells: TableState,
    pub views: TableState,
    /// The column that takes the keyboard on the next frame.
    pub focus: Option<BrowserColumn>,
    /// The column taking the keyboard this frame: the page moves `focus` here
    /// before it draws, so a request made while drawing waits a frame.
    pub(super) focusing: Option<BrowserColumn>,
    /// The frame the page last drew on: a page drawn after a gap has just
    /// arrived.
    pub(super) drawn: Option<u64>,
    /// The column that last held the keyboard.
    pub active: BrowserColumn,
    /// What a command asked of the page, carried out as the page next draws.
    pub(super) queued: Option<LibraryIntent>,
    /// The row menu a column asked for this frame, and where.
    pub(super) asked_menu: Option<(BrowserColumn, egui::Pos2)>,
    /// A list held the keyboard this frame.
    pub(super) holds_keyboard: bool,
    /// The view type the find asks for: its cells hold one, and only views
    /// of it are listed.
    pub view_type: Option<ViewType>,
}

/// What the context strip asks the columns to find, in every library: the
/// filter's words in a cell's name, description or view names, and a view
/// type the cell holds.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Find {
    /// The filter's words as typed, trimmed.
    words: String,
    /// The same words lower-cased, as they are matched.
    query: String,
    view_type: Option<ViewType>,
}

impl Find {
    pub(super) fn of(state: &ProjectState) -> Self {
        let words = state.project_library_filter.trim();
        Self {
            words: words.to_owned(),
            query: words.to_ascii_lowercase(),
            view_type: state.library_browser.view_type,
        }
    }

    /// Nothing is asked, so every cell and view is listed.
    pub(super) fn is_everything(&self) -> bool {
        self.query.is_empty() && self.view_type.is_none()
    }

    /// Whether the find turns `cell` up.
    pub(super) fn cell(&self, cell: &crate::state::Cell) -> bool {
        cell_matches(cell, &self.query)
            && self
                .view_type
                .is_none_or(|wanted| cell.views.values().any(|view| view.view_type == wanted))
    }

    /// Whether a view of `view_type` is listed.
    pub(super) fn view(&self, view_type: ViewType) -> bool {
        self.view_type.is_none_or(|wanted| wanted == view_type)
    }

    /// The built-in library's models it turns up, each a Verilog-A cell.
    pub(super) fn builtin_models(&self) -> Vec<&'static BuiltinVerilogAModel> {
        if self.view(ViewType::VerilogA) {
            builtin_models::shown_models(&self.query)
        } else {
            Vec::new()
        }
    }

    /// What a column the find leaves empty says.
    pub(super) fn nothing_found(&self) -> String {
        match (self.words.as_str(), self.view_type) {
            ("", Some(view_type)) => format!("No cells hold a {} view", view_type.caption()),
            (words, _) => format!("No cells match \u{201c}{words}\u{201d}"),
        }
    }
}

/// The key the built-in library's row goes by: no project library's name.
const BUILTIN_KEY: RowKey = RowKey::Number(0);

/// The Cell column's list of a project library's cells.
const CELLS_TABLE: &str = "workbench.project.library.cells";

/// The Cell column's list of the built-in library's models.
const MODELS_TABLE: &str = "workbench.project.library.models";

/// What a row dragged onto library `target` asks for: Copy Cell of the
/// cell, or of the built-in model, aimed at that library.
fn dropped_onto(
    ctx: &Context,
    cx: &Cx<'_, ProjectFeature>,
    find: &Find,
    drag: RowDrag,
    target: &str,
) -> Option<LibraryIntent> {
    let target = target.to_owned();
    if drag.is_from(CELLS_TABLE) {
        let manager = &cx.model.library_manager;
        let cells = cell_rows(ctx, manager, find);
        Some(LibraryIntent::CopyCellInto {
            library: manager.selected_library.clone()?,
            cell: cells.rows.get(drag.row)?.name.clone(),
            target,
        })
    } else if drag.is_from(MODELS_TABLE) {
        Some(LibraryIntent::CopyModelInto {
            registry_name: find.builtin_models().get(drag.row)?.registry_name,
            target,
        })
    } else {
        None
    }
}

/// One library as its column lists it.
pub(super) struct LibraryRow {
    pub(super) name: String,
    /// What the row reads: the library's display name, or its name.
    label: String,
    /// Its cells, or those the find turns up in it.
    cells: String,
    read_only: bool,
    /// The row is the built-in Verilog-A library's.
    builtin: bool,
}

/// One cell of the selected library that the filter lets through.
pub(super) struct CellRow {
    pub(super) name: String,
    views: String,
    description: String,
}

/// One view of the selected cell. Whether it is modified is read when the
/// row is drawn: an editor marks it without changing the catalog.
pub(super) struct ViewRow {
    pub(super) name: String,
    pub(super) view_type: ViewType,
    path: Option<String>,
}

/// A column's rows and each row's key, in the order the column lists them.
pub(super) struct Listed<R> {
    pub(super) rows: Arc<[R]>,
    pub(super) keys: Arc<[RowKey]>,
}

/// A column's rows, sorted once for the catalog content and the selection
/// and filter they were listed for, and kept in the context's memory until
/// one of those moves. A column draws only the rows in view.
struct ListedRows<K, R> {
    key: K,
    rows: Arc<[R]>,
    keys: Arc<[RowKey]>,
}

/// The rows remembered under `id` while `held` accepts their key, or the
/// rows `list` gives, remembered under `key` with the keys `keyed` gives.
fn listed<K, R>(
    ctx: &Context,
    id: &'static str,
    held: impl FnOnce(&K) -> bool,
    key: impl FnOnce() -> K,
    list: impl FnOnce() -> Vec<R>,
    keyed: impl Fn(&[R]) -> Vec<RowKey>,
) -> Listed<R>
where
    K: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    let id = egui::Id::new(id);
    if let Some(listed) = ctx.data(|data| data.get_temp::<Arc<ListedRows<K, R>>>(id))
        && held(&listed.key)
    {
        return Listed {
            rows: Arc::clone(&listed.rows),
            keys: Arc::clone(&listed.keys),
        };
    }
    #[cfg(any(test, feature = "test-support"))]
    LIBRARY_ROW_BUILDS.with(|builds| builds.set(builds.get() + 1));
    let rows: Arc<[R]> = list().into();
    let keys: Arc<[RowKey]> = keyed(&rows).into();
    let listed = Arc::new(ListedRows {
        key: key(),
        rows: Arc::clone(&rows),
        keys: Arc::clone(&keys),
    });
    ctx.data_mut(|data| data.insert_temp(id, listed));
    Listed { rows, keys }
}

/// The libraries, by name, as the manager shows them, then the built-in
/// library. While a find asks for something, only the libraries it turns a
/// cell up in are listed, each with how many, and `kept`, the one the page
/// shows, whatever it holds.
pub(super) fn library_rows(
    ctx: &Context,
    manager: &LibraryManager,
    find: &Find,
    kept: Option<&str>,
) -> Listed<LibraryRow> {
    let stamp = manager.catalog().stamp();
    listed(
        ctx,
        "workbench.project.library.rows.libraries",
        |held: &(u64, Find, Option<String>)| {
            held.0 == stamp && held.1 == *find && held.2.as_deref() == kept
        },
        || (stamp, find.clone(), kept.map(str::to_owned)),
        || {
            let everything = find.is_everything();
            let mut rows = manager
                .libraries_sorted()
                .into_iter()
                .filter_map(|library| {
                    let found = if everything {
                        library.cell_count()
                    } else {
                        library
                            .cells
                            .values()
                            .filter(|cell| find.cell(cell))
                            .count()
                    };
                    (everything || found > 0 || kept == Some(library.name.as_str())).then(|| {
                        LibraryRow {
                            name: library.name.clone(),
                            label: library.display_name().to_owned(),
                            cells: found.to_string(),
                            read_only: library.read_only,
                            builtin: false,
                        }
                    })
                })
                .collect::<Vec<_>>();
            let models = find.builtin_models().len();
            if everything || models > 0 {
                rows.push(LibraryRow {
                    name: BUILTIN_VERILOGA_LIBRARY.to_owned(),
                    label: BUILTIN_VERILOGA_LIBRARY.to_owned(),
                    cells: models.to_string(),
                    read_only: true,
                    builtin: true,
                });
            }
            rows
        },
        |rows| {
            rows.iter()
                .map(|row| {
                    if row.builtin {
                        BUILTIN_KEY
                    } else {
                        RowKey::from(row.name.as_str())
                    }
                })
                .collect()
        },
    )
}

/// Whether `query`, lower-cased and trimmed, finds `cell`: in its name, its
/// description or one of its views' names. An empty query finds every cell.
fn cell_matches(cell: &crate::state::Cell, query: &str) -> bool {
    query.is_empty()
        || cell.name.to_ascii_lowercase().contains(query)
        || cell.description.to_ascii_lowercase().contains(query)
        || cell
            .views
            .values()
            .any(|view| view.name.to_ascii_lowercase().contains(query))
}

/// The selected library's cells, by name, that `find` turns up.
pub(super) fn cell_rows(ctx: &Context, manager: &LibraryManager, find: &Find) -> Listed<CellRow> {
    let stamp = manager.catalog().stamp();
    let library = manager.selected_library.as_deref();
    listed(
        ctx,
        "workbench.project.library.rows.cells",
        |held: &(u64, Option<String>, Find)| {
            held.0 == stamp && held.1.as_deref() == library && held.2 == *find
        },
        || (stamp, library.map(str::to_owned), find.clone()),
        || {
            library
                .and_then(|library| manager.get_library(library))
                .map(|library| {
                    library
                        .cells_sorted()
                        .into_iter()
                        .filter(|cell| find.cell(cell))
                        .map(|cell| CellRow {
                            name: cell.name.clone(),
                            views: cell.view_count().to_string(),
                            description: cell.description.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default()
        },
        |rows| {
            rows.iter()
                .map(|row| RowKey::from(row.name.as_str()))
                .collect()
        },
    )
}

/// The selected cell's views, by name, of the type `find` asks for.
pub(super) fn view_rows(ctx: &Context, manager: &LibraryManager, find: &Find) -> Listed<ViewRow> {
    let stamp = manager.catalog().stamp();
    let library = manager.selected_library.as_deref();
    let cell = manager.selected_cell.as_deref();
    let view_type = find.view_type;
    listed(
        ctx,
        "workbench.project.library.rows.views",
        |held: &(u64, Option<String>, Option<String>, Option<ViewType>)| {
            held.0 == stamp
                && held.1.as_deref() == library
                && held.2.as_deref() == cell
                && held.3 == view_type
        },
        || {
            (
                stamp,
                library.map(str::to_owned),
                cell.map(str::to_owned),
                view_type,
            )
        },
        || {
            manager
                .current_cell()
                .map(|cell| {
                    cell.views_sorted()
                        .into_iter()
                        .filter(|view| find.view(view.view_type))
                        .map(|view| ViewRow {
                            name: view.name.clone(),
                            view_type: view.view_type,
                            path: view
                                .file_path
                                .as_ref()
                                .map(|path| path.display().to_string()),
                        })
                        .collect()
                })
                .unwrap_or_default()
        },
        |rows| {
            rows.iter()
                .map(|row| RowKey::from(row.name.as_str()))
                .collect()
        },
    )
}

/// Make the row keyed `lead` the one `state` selects, and nothing else,
/// unless it already leads the selection. No lead, or one the list does not
/// show, selects nothing.
fn follow_lead(state: &mut TableState, keys: &[RowKey], lead: Option<RowKey>) {
    if lead.is_some() && state.selected_key() == lead.as_ref() {
        return;
    }
    let selection = lead.and_then(|lead| {
        let row = keys.iter().position(|key| *key == lead)?;
        Some(Selection {
            row,
            key: Some(lead),
            column: None,
        })
    });
    state.anchor.clone_from(&selection);
    state.selection = selection;
    state.others.clear();
}

/// The names of the rows `state` selects, its lead first.
pub(super) fn chosen(state: &TableState) -> Vec<String> {
    state
        .selection
        .iter()
        .chain(&state.others)
        .filter_map(|selection| selection.key.as_ref().and_then(text))
        .map(str::to_owned)
        .collect()
}

/// The text a row is keyed by.
fn text(key: &RowKey) -> Option<&str> {
    match key {
        RowKey::Text(text) => Some(text),
        RowKey::Number(_) => None,
    }
}

/// The height of a column's head under the live metrics: a band that names
/// what the list below it holds, never pressed.
pub(super) fn column_head_height(ui: &Ui) -> f32 {
    LineHeights::list(&Tokens::get(ui.ctx())).band
}

/// What a list asked of the page that only the page's columns answer: the
/// keyboard moving to a neighbouring column, the menu it asked for, and the
/// column the reader is working in.
fn answer_moves(
    browser: &mut LibraryBrowserState,
    column: BrowserColumn,
    response: &TableResponse,
) {
    if response.focused {
        browser.active = column;
        browser.holds_keyboard = true;
    }
    if let Some(next) = response.across.and_then(|by| column.beside(by)) {
        browser.focus = Some(next);
    }
    if let Some(ContextRequest { position, .. }) = response.context_menu {
        browser.active = column;
        browser.asked_menu = Some((column, position));
    }
}

/// A column: its head, then the list `list` draws in the height left under
/// it, or what an empty column holds.
fn list_column(
    ui: &mut Ui,
    title: &str,
    count: ColumnCount<'_>,
    width: f32,
    height: f32,
    list: impl FnOnce(&mut Ui, f32) -> TableResponse,
) -> TableResponse {
    let body = (height - column_head_height(ui)).max(0.0);
    column_frame(ui, title, &count.header(), width, height, |ui| {
        if count.shown == 0 {
            empty_column_text(ui, count.empty);
            return TableResponse::default();
        }
        list(ui, body)
    })
}

/// A read-only library says so; a writable one says nothing.
fn access(read_only: bool) -> Entry {
    if read_only {
        Entry::text("read-only")
    } else {
        Entry::empty()
    }
}

pub(super) fn library_column(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    browser: &mut LibraryBrowserState,
    find: &Find,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    let manager = &cx.model.library_manager;
    let builtin_open = cx.state.builtin_models.open;
    let shown_library = manager
        .selected_library
        .as_deref()
        .filter(|_| !builtin_open);
    let libraries = library_rows(ui.ctx(), manager, find, shown_library);
    let lead = if builtin_open {
        Some(BUILTIN_KEY)
    } else {
        shown_library.map(RowKey::from)
    };
    follow_lead(&mut browser.libraries, &libraries.keys, lead);
    let count = libraries.keys.len();
    let nothing_found = find.nothing_found();
    let columns = [
        Column::name("Library", 60.0),
        Column::number("Cells", 44.0),
        Column::words("Access", 72.0),
    ];
    let focus = browser.focusing == Some(BrowserColumn::Library);
    let response = list_column(
        ui,
        "Library",
        ColumnCount {
            shown: count,
            total: manager.library_count() + 1,
            empty: &nothing_found,
        },
        width,
        height,
        |ui, body| {
            DataTable::new("workbench.project.library.libraries", &columns, count)
                .list()
                .state(&mut browser.libraries)
                .keys(&libraries.keys)
                .drops()
                .focus(focus)
                .max_height(body)
                .show(ui, |row| {
                    let library = &libraries.rows[row];
                    vec![
                        Entry::text(library.label.as_str()),
                        Entry::number(library.cells.as_str()),
                        access(library.read_only),
                    ]
                })
        },
    );
    answer_moves(browser, BrowserColumn::Library, &response);
    if response.activate.is_some() {
        browser.focus = Some(BrowserColumn::Cell);
    }
    if let Some((row, drag)) = response.dropped
        && let Some(target) = libraries.rows.get(row).filter(|target| !target.builtin)
    {
        return dropped_onto(ui.ctx(), cx, find, drag, &target.name);
    }
    let lead = browser.libraries.selected_key().cloned()?;
    let Some(library) = text(&lead).map(str::to_owned) else {
        return response
            .selection_changed
            .then_some(LibraryIntent::BuiltinModels(
                builtin_models::BuiltinModelsIntent::ShowLibrary,
            ));
    };
    if response.selection_changed {
        Some(LibraryIntent::SelectLibrary(library))
    } else if response.rename.is_some() {
        Some(LibraryIntent::RenameLibrary(library))
    } else if response.delete {
        Some(LibraryIntent::DeleteLibrary(library))
    } else {
        None
    }
}

pub(super) fn cell_column(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    browser: &mut LibraryBrowserState,
    find: &Find,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    if cx.state.builtin_models.open {
        return builtin_cell_column(ui, cx, browser, find, width, height);
    }
    let manager = &cx.model.library_manager;
    let cells = cell_rows(ui.ctx(), manager, find);
    follow_lead(
        &mut browser.cells,
        &cells.keys,
        manager.selected_cell.as_deref().map(RowKey::from),
    );
    let total = manager
        .current_library()
        .map_or(0, |library| library.cell_count());
    let no_match = if cells.rows.is_empty() && total > 0 {
        find.nothing_found()
    } else {
        String::new()
    };
    let count = ColumnCount {
        shown: cells.rows.len(),
        total,
        empty: if total == 0 { "No cells" } else { &no_match },
    };
    let columns = [Column::name("Cell", 60.0), Column::number("Views", 44.0)];
    let focus = browser.focusing == Some(BrowserColumn::Cell);
    let response = list_column(ui, "Cell", count, width, height, |ui, body| {
        DataTable::new(CELLS_TABLE, &columns, cells.rows.len())
            .list()
            .state(&mut browser.cells)
            .keys(&cells.keys)
            .multi_select()
            .drag_rows()
            .focus(focus)
            .max_height(body)
            .show(ui, |row| {
                let cell = &cells.rows[row];
                let name = Entry::text(cell.name.as_str());
                vec![
                    if cell.description.trim().is_empty() {
                        name
                    } else {
                        name.hint(cell.description.as_str())
                    },
                    Entry::number(cell.views.as_str()),
                ]
            })
    });
    answer_moves(browser, BrowserColumn::Cell, &response);
    let library = manager.selected_library.clone()?;
    let cell = browser.cells.selected_key().and_then(text)?.to_owned();
    if response.selection_changed {
        Some(LibraryIntent::SelectCell { library, cell })
    } else if response.activate.is_some() {
        Some(LibraryIntent::OpenCell { library, cell })
    } else if response.rename.is_some() {
        Some(LibraryIntent::RenameCell { library, cell })
    } else if response.delete {
        deletion_intent(&library, None, &chosen(&browser.cells))
    } else {
        None
    }
}

pub(super) fn view_column(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    browser: &mut LibraryBrowserState,
    find: &Find,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    if cx.state.builtin_models.open {
        return builtin_view_column(ui, cx, browser, width, height);
    }
    let manager = &cx.model.library_manager;
    // A cell the find hides is not the one the page acts on: its views
    // are not listed and the band offers nothing for it.
    let cell_shown = selected_cell_shown(cx.model, find);
    let views = if cell_shown {
        view_rows(ui.ctx(), manager, find)
    } else {
        Listed {
            rows: Arc::from([]),
            keys: Arc::from([]),
        }
    };
    follow_lead(
        &mut browser.views,
        &views.keys,
        manager.selected_view.as_deref().map(RowKey::from),
    );
    let shown_cell = manager.current_cell();
    let count = ColumnCount {
        shown: views.rows.len(),
        total: if cell_shown {
            shown_cell.map_or(0, |cell| cell.view_count())
        } else {
            0
        },
        empty: if cell_shown {
            "No views"
        } else {
            "No cell selected"
        },
    };
    let columns = [
        Column::name("View", 60.0),
        Column::words("Type", 92.0),
        Column::words("State", 68.0),
    ];
    let focus = browser.focusing == Some(BrowserColumn::View);
    let response = list_column(ui, "View", count, width, height, |ui, body| {
        DataTable::new(
            "workbench.project.library.views",
            &columns,
            views.rows.len(),
        )
        .list()
        .state(&mut browser.views)
        .keys(&views.keys)
        .multi_select()
        .focus(focus)
        .max_height(body)
        .show(ui, |row| {
            let view = &views.rows[row];
            let modified = shown_cell
                .and_then(|cell| cell.get_view(&view.name))
                .is_some_and(|view| view.modified);
            let name = Entry::text(view.name.as_str());
            vec![
                match &view.path {
                    Some(path) => name.hint(path.as_str()),
                    None => name,
                },
                Entry::text(view.view_type.caption()),
                if modified {
                    Entry::text("modified")
                } else {
                    Entry::empty()
                },
            ]
        })
    });
    answer_moves(browser, BrowserColumn::View, &response);
    let lead = browser.views.selected_key().and_then(text)?;
    let (library, cell) = (
        manager.selected_library.as_deref()?,
        manager.selected_cell.as_deref()?,
    );
    let reference = CellViewRef::new(library, cell, lead);
    let view_type = views
        .rows
        .iter()
        .find(|view| view.name == lead)
        .map(|view| view.view_type)?;
    if response.selection_changed {
        Some(LibraryIntent::SelectView(reference))
    } else if response.activate.is_some() {
        Some(LibraryIntent::OpenView(reference, view_type))
    } else if response.rename.is_some() {
        Some(LibraryIntent::RenameView(reference))
    } else if response.delete {
        deletion_intent(library, Some(cell), &chosen(&browser.views))
    } else {
        None
    }
}

/// The built-in library's models, under the device class each belongs to.
fn builtin_cell_column(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    browser: &mut LibraryBrowserState,
    find: &Find,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    let builtin = &cx.state.builtin_models;
    let shown = find.builtin_models();
    let keys = shown
        .iter()
        .map(|model| RowKey::from(model.registry_name))
        .collect::<Vec<_>>();
    let mut groups = Vec::new();
    for (row, model) in shown.iter().enumerate() {
        if row == 0 || shown[row - 1].class != model.class {
            groups.push(Group::new(model.class.label(), row));
        }
    }
    follow_lead(
        &mut browser.cells,
        &keys,
        builtin.model.as_deref().map(RowKey::from),
    );
    let no_match = if shown.is_empty() {
        find.nothing_found()
    } else {
        String::new()
    };
    let count = ColumnCount {
        shown: shown.len(),
        total: BUILTIN_VERILOGA_MODELS.len(),
        empty: &no_match,
    };
    let columns = [Column::name("Cell", 60.0), Column::words("Version", 64.0)];
    let focus = browser.focusing == Some(BrowserColumn::Cell);
    let response = list_column(ui, "Cell", count, width, height, |ui, body| {
        DataTable::new(MODELS_TABLE, &columns, shown.len())
            .list()
            .state(&mut browser.cells)
            .keys(&keys)
            .groups(&groups)
            .drag_rows()
            .focus(focus)
            .max_height(body)
            .show(ui, |row| {
                let model = shown[row];
                vec![
                    Entry::text(model.name)
                        .hint(format!("{} (module {})", model.summary, model.module)),
                    Entry::text(model.version),
                ]
            })
    });
    answer_moves(browser, BrowserColumn::Cell, &response);
    let lead = browser.cells.selected_key().and_then(text)?;
    let model = shown.iter().find(|model| model.registry_name == lead)?;
    let intent = if response.selection_changed {
        builtin_models::BuiltinModelsIntent::SelectModel(model.registry_name)
    } else if response.activate.is_some() {
        browser.focus = Some(BrowserColumn::View);
        return None;
    } else {
        return None;
    };
    Some(LibraryIntent::BuiltinModels(intent))
}

/// The selected model's files, its root first.
fn builtin_view_column(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    browser: &mut LibraryBrowserState,
    width: f32,
    height: f32,
) -> Option<LibraryIntent> {
    let builtin = &cx.state.builtin_models;
    let keys = builtin
        .files
        .iter()
        .map(|file| RowKey::from(file.as_str()))
        .collect::<Vec<_>>();
    follow_lead(
        &mut browser.views,
        &keys,
        builtin.file.as_deref().map(RowKey::from),
    );
    let count = ColumnCount {
        shown: builtin.files.len(),
        total: builtin.files.len(),
        empty: "No cell selected",
    };
    let columns = [Column::name("View", 60.0), Column::words("Type", 92.0)];
    let focus = browser.focusing == Some(BrowserColumn::View);
    let response = list_column(ui, "View", count, width, height, |ui, body| {
        DataTable::new("workbench.project.library.files", &columns, keys.len())
            .list()
            .state(&mut browser.views)
            .keys(&keys)
            .focus(focus)
            .max_height(body)
            .show(ui, |row| {
                vec![
                    Entry::text(builtin.files[row].as_str()),
                    Entry::text(if row == 0 { "Verilog-A" } else { "Include" }),
                ]
            })
    });
    answer_moves(browser, BrowserColumn::View, &response);
    let file = browser.views.selected_key().and_then(text)?.to_owned();
    let intent = if response.selection_changed {
        builtin_models::BuiltinModelsIntent::SelectFile(file)
    } else if response.activate.is_some() {
        builtin_models::BuiltinModelsIntent::OpenFile(file)
    } else {
        return None;
    };
    Some(LibraryIntent::BuiltinModels(intent))
}
