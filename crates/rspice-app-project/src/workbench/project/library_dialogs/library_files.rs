//! Libraries kept in folders of their own, on the pages: Attach library…,
//! the commands Library actions and the Dependencies rows offer over them,
//! Refresh…, the choice a cellview that changed on disk since editing began
//! asks for, Detach library…, Break lock…, a relink to a folder that holds
//! the library under another identity, and the cellviews a publication
//! restore found moved on. Each acts through the model's library-file
//! operations, and those that change the project are each one step of Undo.
//!
//! Nothing here reads the disk on a frame: a check runs when Refresh… opens,
//! on open, when the window regains focus and on Save.

use std::path::{Path, PathBuf};

use egui::{Context, Ui};

use crate::ui::widgets::table::{Cell, Column, DataTable, RowKey, TableState};
use crate::ui::widgets::{
    CommandForm, Dialog, DialogChoice, DialogInitialFocus, DialogSize, Problem, TableColumn,
};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{
    FolderPreview, LibraryFile, LibraryFileLock, LibraryMode, LibraryPath, RefreshRow,
    RelinkOutcome, WorkspacePruned,
};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

/// What a reader can ask of a library kept in a folder of its own, or of an
/// embedded one on its way to or from a folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryFileIntent {
    /// Pick a library folder and attach it, read-only.
    Attach,
    /// Write an embedded library to a folder of its own.
    MoveToFolder(String),
    /// Copy every cellview of a referenced library into the project.
    Embed(String),
    /// Embed a copy of a picked library folder.
    EmbedFromFolder,
    /// Make a referenced library editable or read-only.
    SetMode(String, LibraryMode),
    /// Point a referenced library at another folder.
    Relink(String),
    /// List the cellviews that changed on disk and reload the picked ones.
    Refresh,
    /// Drop the reference, after a review naming what it leaves unresolved.
    Detach(String),
    /// Show the library's folder.
    ShowInFolder(String),
    /// In a browser, read a referenced library from a picked folder for this
    /// session only.
    ReadFromFolder(String),
    /// Remove another holder's lock on one file, after naming them.
    BreakLock { library: String, file: LibraryFile },
    /// Choose Keep my version or Take the file's version for the cellviews
    /// that changed on disk since editing began.
    ChooseChangedOnDisk,
}

/// The dialog a library-file command has open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryFilesDialog {
    /// The folder picked, and the paths that name it: the one Attach stores
    /// first, the others that also work after it.
    ///
    /// What the folder holds is read once, when the dialog opens, for it to
    /// name the library it will attach.
    Attach {
        folder: PathBuf,
        forms: Vec<LibraryPath>,
        chosen: usize,
        preview: Result<FolderPreview, String>,
    },
    Refresh,
    ChangedOnDisk,
    /// The library and the drawings outside it that place its cells.
    Detach {
        library: String,
        drawings: Vec<String>,
    },
    BreakLock {
        library: String,
        file: LibraryFile,
        holder: LibraryFileLock,
    },
    /// The picked folder holds the library under another identity.
    Relink {
        library: String,
        folder: PathBuf,
    },
    /// The cellviews a publication restore found moved on in their folders.
    MovedOn,
}

/// The library-file dialogs: the one open, its list's selection and the
/// last refusal it states.
#[derive(Debug, Clone, Default)]
pub struct LibraryFilesDialogs {
    pub open: Option<LibraryFilesDialog>,
    pub table: TableState,
    pub error: Option<String>,
    /// The cellviews a publication restore found moved on, each with the
    /// revision the publication holds and the one in its folder.
    pub moved_on: Vec<RefreshRow>,
}

impl LibraryFilesDialogs {
    pub const fn is_open(&self) -> bool {
        self.open.is_some()
    }

    fn show(&mut self, dialog: LibraryFilesDialog) {
        *self = Self {
            open: Some(dialog),
            moved_on: std::mem::take(&mut self.moved_on),
            ..Self::default()
        };
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// Open the choice for the cellviews that changed on disk since editing
    /// began, when there are any: Save refused them.
    pub fn open_changed_on_disk(&mut self, model: &Model) {
        if !model.library_files_changed_since_editing().is_empty() {
            self.show(LibraryFilesDialog::ChangedOnDisk);
        }
    }

    /// Name the cellviews a publication restore found moved on, and offer
    /// to restore the picked ones from the publication's copies.
    pub fn open_moved_on(&mut self, moved_on: Vec<RefreshRow>) {
        if moved_on.is_empty() {
            return;
        }
        self.show(LibraryFilesDialog::MovedOn);
        self.moved_on = moved_on;
    }
}

/// Why a library-file command cannot be taken here and now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Blocker {
    BrowserKeepsNoLink,
    BrowserReachesNoPath,
    ReadsItsFolder,
    ReadOnlyProject,
    Already(LibraryMode),
}

impl Blocker {
    /// The reason as a menu's hover and a refusal state it.
    const fn sentence(self) -> &'static str {
        match self {
            Self::BrowserKeepsNoLink => {
                "A browser cannot keep a link to a folder; use Embed library from folder\u{2026}"
            }
            Self::BrowserReachesNoPath => "A browser cannot reach a folder by path.",
            Self::ReadsItsFolder => "This computer reads the library from its folder.",
            Self::ReadOnlyProject => "The project is open read-only.",
            Self::Already(LibraryMode::Editable) => "The library is already editable.",
            Self::Already(LibraryMode::ReadOnly) => "The library is already read-only.",
        }
    }

    /// The reason as the clause a command's refusal is stated in.
    const fn clause(self) -> &'static str {
        match self {
            Self::BrowserKeepsNoLink => "a browser cannot keep a link to a folder",
            Self::BrowserReachesNoPath => "a browser cannot reach a folder by path",
            Self::ReadsItsFolder => "this computer reads the library from its folder",
            Self::ReadOnlyProject => "the project is open read-only",
            Self::Already(LibraryMode::Editable) => "the library is already editable",
            Self::Already(LibraryMode::ReadOnly) => "the library is already read-only",
        }
    }
}

fn blocker(model: &Model, intent: &LibraryFileIntent) -> Option<Blocker> {
    let browser = cfg!(target_arch = "wasm32");
    let read_only_project = model.safe_mode.project_read_only();
    match intent {
        LibraryFileIntent::Attach if browser => Some(Blocker::BrowserKeepsNoLink),
        LibraryFileIntent::MoveToFolder(_)
        | LibraryFileIntent::ShowInFolder(_)
        | LibraryFileIntent::Relink(_)
            if browser =>
        {
            Some(Blocker::BrowserReachesNoPath)
        }
        LibraryFileIntent::ReadFromFolder(_) if !browser => Some(Blocker::ReadsItsFolder),
        LibraryFileIntent::Attach
        | LibraryFileIntent::MoveToFolder(_)
        | LibraryFileIntent::Embed(_)
        | LibraryFileIntent::EmbedFromFolder
        | LibraryFileIntent::SetMode(..)
        | LibraryFileIntent::Relink(_)
        | LibraryFileIntent::Detach(_)
            if read_only_project =>
        {
            Some(Blocker::ReadOnlyProject)
        }
        LibraryFileIntent::SetMode(library, mode) => model
            .library_manager
            .reference(library)
            .filter(|reference| reference.mode == *mode)
            .map(|_| Blocker::Already(*mode)),
        _ => None,
    }
}

/// Why a library-file command cannot be taken here and now, if it cannot.
pub fn library_file_blocker(model: &Model, intent: &LibraryFileIntent) -> Option<String> {
    blocker(model, intent).map(|blocker| blocker.sentence().to_owned())
}

/// The same reason, as the clause a command's refusal is stated in.
pub fn library_file_refusal(model: &Model, intent: &LibraryFileIntent) -> Option<&'static str> {
    blocker(model, intent).map(Blocker::clause)
}

/// The label a library-file command goes by in a menu.
pub fn library_file_label(intent: &LibraryFileIntent) -> &'static str {
    match intent {
        LibraryFileIntent::Attach => "Attach library\u{2026}",
        LibraryFileIntent::MoveToFolder(_) => "Move to its own folder\u{2026}",
        LibraryFileIntent::Embed(_) => "Embed in project",
        LibraryFileIntent::EmbedFromFolder => "Embed library from folder\u{2026}",
        LibraryFileIntent::SetMode(_, LibraryMode::Editable) => "Make editable",
        LibraryFileIntent::SetMode(_, LibraryMode::ReadOnly) => "Make read-only",
        LibraryFileIntent::Relink(_) => "Relink\u{2026}",
        LibraryFileIntent::Refresh => "Refresh\u{2026}",
        LibraryFileIntent::Detach(_) => "Detach library\u{2026}",
        LibraryFileIntent::ShowInFolder(_) => "Show in folder",
        LibraryFileIntent::ReadFromFolder(_) => "Read from folder\u{2026}",
        LibraryFileIntent::BreakLock { .. } => "Break lock\u{2026}",
        LibraryFileIntent::ChooseChangedOnDisk => "Changed on disk\u{2026}",
    }
}

/// The commands Library actions offers over `library`, in order: for one
/// kept in a folder of its own its mode, Refresh…, Embed in project and
/// Detach library…; for one held in the project, Move to its own folder….
pub fn library_file_intents(model: &Model, library: &str) -> Vec<LibraryFileIntent> {
    match model.library_manager.reference(library) {
        Some(reference) => {
            let mode = if reference.mode.is_editable() {
                LibraryMode::ReadOnly
            } else {
                LibraryMode::Editable
            };
            let mut intents = vec![LibraryFileIntent::SetMode(library.to_owned(), mode)];
            if cfg!(target_arch = "wasm32") {
                intents.push(LibraryFileIntent::ReadFromFolder(library.to_owned()));
            } else {
                intents.push(LibraryFileIntent::Relink(library.to_owned()));
            }
            intents.extend([
                LibraryFileIntent::Refresh,
                LibraryFileIntent::Embed(library.to_owned()),
                LibraryFileIntent::Detach(library.to_owned()),
            ]);
            intents
        }
        None => vec![LibraryFileIntent::MoveToFolder(library.to_owned())],
    }
}

/// Carry out `intent`: a command that needs a folder asks for one first, a
/// command that needs a review opens it, and the rest act at once. A
/// refusal is stated where the command was taken.
pub fn run_library_file_intent(cx: &mut Cx<'_, ProjectFeature>, intent: LibraryFileIntent) {
    if let Some(blocker) = library_file_blocker(cx.model, &intent) {
        cx.wb.prompt(blocker);
        return;
    }
    match intent {
        LibraryFileIntent::Attach => {
            if let Some(folder) = pick_folder("Attach library") {
                open_attach(cx, &folder);
            }
        }
        LibraryFileIntent::MoveToFolder(library) => {
            if let Some(picked) = pick_folder("Move library to its own folder") {
                let folder = library_folder_in(&picked, &library);
                let result = cx.model.move_library_to_folder(&library, &folder);
                announce(
                    cx,
                    result.map(|()| WorkspacePruned::default()),
                    format!(
                        "Moved library '{library}' to its own folder {}; it is editable.",
                        folder.display()
                    ),
                );
            }
        }
        LibraryFileIntent::Embed(library) => {
            let result = cx.model.embed_library(&library);
            announce(
                cx,
                result.map(|()| WorkspacePruned::default()),
                format!("Embedded library '{library}' in the project."),
            );
        }
        LibraryFileIntent::EmbedFromFolder => embed_from_folder(cx),
        LibraryFileIntent::SetMode(library, mode) => {
            let result = cx.model.set_library_mode(&library, mode);
            announce(
                cx,
                result.map(|()| WorkspacePruned::default()),
                format!(
                    "Library '{library}' is now {}.",
                    mode.label().to_lowercase()
                ),
            );
        }
        LibraryFileIntent::Relink(library) => {
            if let Some(folder) = pick_folder("Relink library") {
                relink(cx, &library, &folder, false);
            }
        }
        LibraryFileIntent::Refresh => {
            cx.model.check_library_files();
            cx.state.library_files.show(LibraryFilesDialog::Refresh);
            preselect_refresh(cx);
        }
        LibraryFileIntent::Detach(library) => {
            let drawings = cx.model.library_placements_outside(&library);
            cx.state
                .library_files
                .show(LibraryFilesDialog::Detach { library, drawings });
        }
        LibraryFileIntent::ShowInFolder(library) => show_in_folder(cx, &library),
        LibraryFileIntent::ReadFromFolder(library) => read_from_folder(cx, &library),
        LibraryFileIntent::BreakLock { library, file } => {
            match cx.model.library_lock_elsewhere(&library, &file).cloned() {
                Some(holder) => cx.state.library_files.show(LibraryFilesDialog::BreakLock {
                    library,
                    file,
                    holder,
                }),
                None => cx
                    .wb
                    .prompt(format!("Nobody else holds {library}/{file} now.")),
            }
        }
        LibraryFileIntent::ChooseChangedOnDisk => {
            cx.state.library_files.open_changed_on_disk(cx.model);
            if !cx.state.library_files.is_open() {
                cx.wb
                    .prompt("No cellview has changed on disk since you started editing it.");
            }
        }
    }
}

/// Open Attach library on `folder`: the path it would store, and the other
/// forms that name the same folder.
pub fn open_attach(cx: &mut Cx<'_, ProjectFeature>, folder: &Path) {
    match cx.model.library_path_choice(folder) {
        Ok(choice) => {
            let mut forms = vec![choice.stored];
            forms.extend(choice.others);
            let preview = cx.model.library_folder_preview(folder);
            cx.state.library_files.show(LibraryFilesDialog::Attach {
                folder: folder.to_path_buf(),
                forms,
                chosen: 0,
                preview,
            });
        }
        Err(refusal) => cx.wb.prompt(format!("Library not attached: {refusal}")),
    }
}

/// Where Move to its own folder writes `library` when the reader picked
/// `picked`: there, when it is empty, else a folder named for the library
/// inside it.
fn library_folder_in(picked: &Path, library: &str) -> PathBuf {
    let empty = std::fs::read_dir(picked).is_ok_and(|mut entries| entries.next().is_none());
    if empty {
        picked.to_path_buf()
    } else {
        picked.join(library)
    }
}

/// State what a command did in the log once the shell has carried out what
/// it left outside the model, or its refusal where it was taken.
fn announce(
    cx: &mut Cx<'_, ProjectFeature>,
    result: Result<WorkspacePruned, String>,
    notice: String,
) {
    match result {
        Ok(pruned) => cx.wb.outbox.push(ProjectRequest::CatalogChanged {
            pruned,
            mutation: None,
            notice,
        }),
        Err(refusal) => cx.wb.prompt(refusal),
    }
}

fn relink(cx: &mut Cx<'_, ProjectFeature>, library: &str, folder: &Path, accept: bool) {
    match cx.model.relink_library(library, folder, accept) {
        Ok(RelinkOutcome::Relinked(rebound)) => {
            cx.state.library_files.close();
            let closed = rebound.closed_sentence();
            announce(
                cx,
                Ok(rebound.pruned),
                format!(
                    "Relinked library '{library}' to {}.{closed}",
                    folder.display()
                ),
            );
        }
        Ok(RelinkOutcome::AsksFirst { .. }) => {
            cx.state.library_files.show(LibraryFilesDialog::Relink {
                library: library.to_owned(),
                folder: folder.to_path_buf(),
            });
        }
        Err(refusal) => {
            if cx.state.library_files.is_open() {
                cx.state.library_files.error = Some(refusal);
            } else {
                cx.wb.prompt(refusal);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn embed_from_folder(cx: &mut Cx<'_, ProjectFeature>) {
    let Some(folder) = pick_folder("Embed library from folder") else {
        return;
    };
    let result = cx.model.embed_library_from_folder(&folder);
    finish_embed_from_folder(cx, result);
}

#[cfg(target_arch = "wasm32")]
fn embed_from_folder(cx: &mut Cx<'_, ProjectFeature>) {
    let _ = cx;
    browser_folder::pick(browser_folder::Purpose::Embed);
}

fn finish_embed_from_folder(cx: &mut Cx<'_, ProjectFeature>, result: Result<String, String>) {
    match result {
        Ok(name) => {
            select_library(cx, &name);
            announce(
                cx,
                Ok(WorkspacePruned::default()),
                format!("Embedded a copy of library '{name}' in the project."),
            );
        }
        Err(refusal) => cx.wb.prompt(refusal),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_from_folder(cx: &mut Cx<'_, ProjectFeature>, library: &str) {
    cx.wb.prompt(format!(
        "Library '{library}' is read from its folder on this computer."
    ));
}

#[cfg(target_arch = "wasm32")]
fn read_from_folder(cx: &mut Cx<'_, ProjectFeature>, library: &str) {
    let _ = cx;
    browser_folder::pick(browser_folder::Purpose::ReadForSession(library.to_owned()));
}

/// Take the folder a browser finished reading for Embed library from
/// folder… or Read from folder…, if one is waiting. Polled as the Library
/// and Dependencies pages draw, where those commands are taken.
pub fn take_browser_folder(cx: &mut Cx<'_, ProjectFeature>) {
    #[cfg(target_arch = "wasm32")]
    if let Some((purpose, picked)) = browser_folder::take() {
        match (purpose, picked) {
            (_, Err(refusal)) => cx.wb.prompt(refusal),
            (browser_folder::Purpose::Embed, Ok((_, files))) => {
                let result = cx.model.embed_library_from_files(files);
                finish_embed_from_folder(cx, result);
            }
            (browser_folder::Purpose::ReadForSession(library), Ok((folder, files))) => {
                let result = cx.model.read_library_for_session(&library, &folder, files);
                let closed = result
                    .as_ref()
                    .map(|rebound| rebound.closed_sentence())
                    .unwrap_or_default();
                announce(
                    cx,
                    result.map(|rebound| rebound.pruned),
                    format!(
                        "Read library '{library}' from {folder} for this session, read-only.{closed}"
                    ),
                );
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = cx;
}

#[cfg(not(target_arch = "wasm32"))]
fn show_in_folder(cx: &mut Cx<'_, ProjectFeature>, library: &str) {
    let Some(folder) = cx.model.library_folder(library).map(Path::to_path_buf) else {
        cx.wb.prompt(format!(
            "Library '{library}' has no folder on this computer."
        ));
        return;
    };
    if let Err(error) = reveal(&folder) {
        cx.model.report_failure("Folder not shown", error);
    }
}

#[cfg(target_arch = "wasm32")]
fn show_in_folder(cx: &mut Cx<'_, ProjectFeature>, _: &str) {
    cx.wb.prompt("A browser cannot reach a folder by path.");
}

#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn reveal(folder: &Path) -> Result<(), String> {
    crate::workbench::lifecycle::recovery::reveal_directory(folder)
}

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
fn reveal(folder: &Path) -> Result<(), String> {
    REVEALED.with(|revealed| *revealed.borrow_mut() = Some(folder.to_path_buf()));
    Ok(())
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static PICKED: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
    static REVEALED: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

/// The folder the next folder picker answers with, as a reader would pick it.
#[cfg(any(test, feature = "test-support"))]
pub fn answer_next_folder_pick(folder: &Path) {
    PICKED.with(|picked| *picked.borrow_mut() = Some(folder.to_path_buf()));
}

/// The folder Show in folder last showed.
#[cfg(any(test, feature = "test-support"))]
pub fn take_revealed_folder() -> Option<PathBuf> {
    REVEALED.with(|revealed| revealed.borrow_mut().take())
}

#[cfg(any(test, feature = "test-support"))]
fn pick_folder(_: &str) -> Option<PathBuf> {
    PICKED.with(|picked| picked.borrow_mut().take())
}

#[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
fn pick_folder(title: &str) -> Option<PathBuf> {
    rfd::FileDialog::new().set_title(title).pick_folder()
}

#[cfg(all(not(any(test, feature = "test-support")), target_arch = "wasm32"))]
fn pick_folder(_: &str) -> Option<PathBuf> {
    None
}

/// Select `library` on the Library page.
fn select_library(cx: &mut Cx<'_, ProjectFeature>, library: &str) {
    cx.state.builtin_models.open = false;
    cx.model.library_manager.select_library(library);
}

/// A row's key: its library and its file.
fn row_key(library: &str, file: &LibraryFile) -> RowKey {
    RowKey::Text(format!("{library}\u{1f}{file}"))
}

/// Pick every changed cellview Refresh can reload: those with no edits here.
fn preselect_refresh(cx: &mut Cx<'_, ProjectFeature>) {
    let rows = cx.model.refresh_rows();
    let table = &mut cx.state.library_files.table;
    let mut first = true;
    for (index, row) in rows.iter().enumerate().filter(|(_, row)| !row.edited) {
        let selection = crate::ui::widgets::table::Selection {
            row: index,
            key: Some(row_key(&row.library, &row.file)),
            column: None,
        };
        if first {
            table.selection = Some(selection.clone());
            table.anchor = Some(selection);
            first = false;
        } else {
            table.others.push(selection);
        }
    }
}

/// The rows a table has picked, in its order.
fn picked<'a>(table: &TableState, rows: &'a [RefreshRow]) -> Vec<&'a RefreshRow> {
    let keys = table.selected_keys();
    rows.iter()
        .filter(|row| keys.contains(&&row_key(&row.library, &row.file)))
        .collect()
}

/// A file's cell and view, as a row names them: the library's own file and
/// a cell's own file have no view.
fn cell_and_view(file: &LibraryFile) -> (String, String) {
    match file {
        LibraryFile::Manifest => ("\u{2014}".to_owned(), "library".to_owned()),
        LibraryFile::Cell { cell } => (cell.clone(), "cell".to_owned()),
        LibraryFile::CellView { cell, view, .. } => (cell.clone(), view.clone()),
    }
}

fn revision(revision: Option<u64>) -> String {
    revision.map_or_else(|| "\u{2014}".to_owned(), |revision| revision.to_string())
}

/// A list of changed cellviews, flush to the dialog's edges: library, cell,
/// view, the revision loaded and the one on disk, and where each stands.
/// A dialog too narrow for six columns, on a phone, names each cellview by
/// its path and its two revisions on one line, so where it stands is on
/// screen without scrolling sideways.
fn cellview_table(
    ui: &mut Ui,
    id: &str,
    rows: &[RefreshRow],
    table: &mut TableState,
    heads: (&str, &str, &str),
    status_width: f32,
    status: impl Fn(&RefreshRow) -> Cell,
) {
    let narrow = ui.available_width() < NARROW_LIST;
    let wide = [
        Column::name("Library", 90.0),
        Column::name("Cell", 90.0),
        Column::name("View", 80.0),
        Column::number(heads.0, 64.0),
        Column::number(heads.1, 64.0),
        Column::words(heads.2, status_width),
    ];
    let compact = [
        Column::name("Cellview", 110.0),
        Column::number("Revision", 76.0),
        Column::words(heads.2, status_width),
    ];
    let columns: &[Column] = if narrow { &compact } else { &wide };
    let keys = rows
        .iter()
        .map(|row| row_key(&row.library, &row.file))
        .collect::<Vec<_>>();
    let _ = DataTable::new(id, columns, rows.len())
        .state(table)
        .keys(&keys)
        .multi_select()
        .show(ui, |index| {
            let row = &rows[index];
            let (cell, view) = cell_and_view(&row.file);
            if narrow {
                return vec![
                    Cell::text(format!("{}/{cell}/{view}", row.library)),
                    Cell::number(format!(
                        "{} \u{2192} {}",
                        revision(row.loaded),
                        revision(row.on_disk)
                    )),
                    status(row),
                ];
            }
            vec![
                Cell::text(row.library.as_str()),
                Cell::text(cell),
                Cell::text(view),
                Cell::number(revision(row.loaded)),
                Cell::number(revision(row.on_disk)),
                status(row),
            ]
        });
}

/// Narrower than this, a list of cellviews takes three columns, not six.
const NARROW_LIST: f32 = 560.0;

fn changed_status(row: &RefreshRow) -> Cell {
    let (label, problem) = changed_words(row);
    Cell::problem(label, problem)
}

fn changed_words(row: &RefreshRow) -> (&'static str, Problem) {
    if row.edited {
        (
            rspice_design::library::folder::CHANGED_SINCE_EDITING,
            Problem::Failure,
        )
    } else {
        ("Changed", Problem::Warning)
    }
}

/// The least width of the lists' last column, its head's, and the most, as
/// a share of the dialog's body: between them it fits its widest words.
const STATUS_LEAST: f32 = 72.0;
const STATUS_MOST: f32 = 0.45;

/// The width of a list's last column, fitted to `statuses` in `ui`.
fn last_column_width<'a>(ui: &Ui, statuses: impl IntoIterator<Item = (&'a str, bool)>) -> f32 {
    crate::workbench::project::dependency_status::status_column_width(
        ui,
        STATUS_LEAST,
        ui.available_width() * STATUS_MOST,
        statuses,
    )
}

impl ProjectFeature {
    /// Draw the library-file dialog that is open, if one is.
    pub(in crate::workbench) fn process_library_files_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let Some(open) = cx.state.library_files.open.clone() else {
            return;
        };
        match open {
            LibraryFilesDialog::Attach {
                folder,
                forms,
                chosen,
                preview,
            } => attach_dialog(cx, ctx, &folder, &forms, chosen, &preview),
            LibraryFilesDialog::Refresh => refresh_dialog(cx, ctx),
            LibraryFilesDialog::ChangedOnDisk => changed_on_disk_dialog(cx, ctx),
            LibraryFilesDialog::Detach { library, drawings } => {
                detach_dialog(cx, ctx, &library, &drawings);
            }
            LibraryFilesDialog::BreakLock {
                library,
                file,
                holder,
            } => break_lock_dialog(cx, ctx, &library, &file, &holder),
            LibraryFilesDialog::Relink { library, folder } => {
                relink_dialog(cx, ctx, &library, &folder);
            }
            LibraryFilesDialog::MovedOn => moved_on_dialog(cx, ctx),
        }
    }
}

fn attach_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
    ctx: &Context,
    folder: &Path,
    forms: &[LibraryPath],
    chosen: usize,
    preview: &Result<FolderPreview, String>,
) {
    let host = rspice_design::library::folder::HostPlatform::current();
    // The choice names each form in words; the path the picked one stores
    // is the row under it, cut from the middle to the dialog's width, so a
    // long absolute path never runs past the dialog's edge.
    let options = forms
        .iter()
        .map(|path| form_words(path, host))
        .collect::<Vec<_>>();
    let options = options.iter().map(String::as_str).collect::<Vec<_>>();
    let mut picked = chosen.min(forms.len().saturating_sub(1));
    let error = cx
        .state
        .library_files
        .error
        .clone()
        .or_else(|| preview.as_ref().err().cloned());
    let shown_folder = folder.display().to_string();
    let choice = CommandForm::new("Attach library", "Attach")
        .describe("Attach a library kept in a folder of its own, read-only.")
        .reading_column()
        .primary_enabled(preview.is_ok())
        .status(error.as_deref().map_or(
            Ok(Some(
                "It opens read-only; make it editable from Library actions.",
            )),
            Err,
        ))
        .show(ctx, |rows| {
            if let Ok(preview) = preview {
                rows.value_mono("Library", &preview.library);
                rows.value(
                    "Cells",
                    &format!(
                        "{} cell{}",
                        preview.cells,
                        if preview.cells == 1 { "" } else { "s" }
                    ),
                );
                if preview.technology.is_empty() {
                    rows.value("Technology", "None named");
                } else {
                    rows.value_mono("Technology", &preview.technology);
                }
            }
            rows.value_mono("Folder", &shown_folder);
            rows.choice("Stored as", &options, &mut picked);
            if let Some(path) = forms.get(picked) {
                rows.value_mono("Path", &path.shown(host));
            }
            None
        });
    if let Some(LibraryFilesDialog::Attach { chosen, .. }) = &mut cx.state.library_files.open {
        *chosen = picked;
    }
    match choice {
        DialogChoice::Primary => {
            let Some(path) = forms.get(picked).cloned() else {
                return;
            };
            match cx.model.attach_library(folder, path.clone()) {
                Ok(name) => {
                    cx.state.library_files.close();
                    select_library(cx, &name);
                    announce(
                        cx,
                        Ok(WorkspacePruned::default()),
                        format!(
                            "Attached library '{name}' from {}, read-only.",
                            path.shown(host)
                        ),
                    );
                }
                Err(refusal) => cx.state.library_files.error = Some(refusal),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// A stored form in words: under a named root, which root.
fn form_words(path: &LibraryPath, host: rspice_design::library::folder::HostPlatform) -> String {
    use rspice_design::library::folder::LibraryPathForm;
    match path.form() {
        LibraryPathForm::Root => {
            let shown = path.shown(host);
            let root = shown.split(['/', '\\']).next().unwrap_or_default();
            format!("Under {root}")
        }
        form => form.label().to_owned(),
    }
}

fn refresh_dialog(cx: &mut Cx<'_, ProjectFeature>, ctx: &Context) {
    let rows = cx.model.refresh_rows();
    let picked_rows = picked(&cx.state.library_files.table, &rows);
    let edited_picked = picked_rows.iter().any(|row| row.edited);
    let any_edited = rows.iter().any(|row| row.edited);
    let refusal = cx.state.library_files.error.clone().or_else(|| {
        edited_picked.then(|| {
            "A picked cellview has edits here: choose Keep my version or Take the file's version for it."
                .to_owned()
        })
    });
    let empty = rows.is_empty();
    let mut dialog = Dialog::new(
        "Refresh cellviews",
        if empty { "Close" } else { "Reload picked" },
    )
    .description(if empty {
        "No cellview has changed on disk since this session read it."
    } else {
        "These cellviews changed on disk since this session read them. The picked ones are read again; the rest keep what is loaded."
    })
    .size(DialogSize::Transaction)
    .flush_body()
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(empty || (!picked_rows.is_empty() && !edited_picked));
    if !empty {
        dialog = dialog.ghost("Cancel");
    }
    if any_edited {
        dialog = dialog.secondary("Changed on disk\u{2026}");
    }
    if let Some(refusal) = refusal.as_deref() {
        dialog = dialog.hint(refusal);
    }
    let table = &mut cx.state.library_files.table;
    let choice = dialog.show(ctx, |ui| {
        if !empty {
            let width = last_column_width(ui, rows.iter().map(|row| (changed_words(row).0, true)));
            cellview_table(
                ui,
                "workbench.project.library-files.refresh",
                &rows,
                table,
                ("Loaded", "On disk", "Status"),
                width,
                changed_status,
            );
        }
    });
    match choice {
        DialogChoice::Primary if empty => cx.state.library_files.close(),
        DialogChoice::Primary => {
            let picked = picked(&cx.state.library_files.table, &rows)
                .into_iter()
                .map(|row| (row.library.clone(), row.file.clone()))
                .collect::<Vec<_>>();
            match cx.model.refresh_library_files(&picked) {
                Ok((count, pruned)) => {
                    cx.state.library_files.close();
                    announce(
                        cx,
                        Ok(pruned),
                        format!(
                            "Read {count} cellview{} again from disk.",
                            if count == 1 { "" } else { "s" }
                        ),
                    );
                }
                Err(refusal) => cx.state.library_files.error = Some(refusal),
            }
        }
        DialogChoice::Secondary => cx
            .state
            .library_files
            .show(LibraryFilesDialog::ChangedOnDisk),
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::None => {}
    }
}

fn changed_on_disk_dialog(cx: &mut Cx<'_, ProjectFeature>, ctx: &Context) {
    let rows = cx.model.library_files_changed_since_editing();
    if rows.is_empty() {
        cx.state.library_files.close();
        return;
    }
    let table = &mut cx.state.library_files.table;
    if table.selection.is_none() {
        let first = &rows[0];
        table.select(0, Some(row_key(&first.library, &first.file)));
    }
    let selected = picked(table, &rows).first().map(|row| (*row).clone());
    let kept = |row: &RefreshRow| {
        cx.model
            .library_files
            .library(&row.library)
            .and_then(|session| session.files.get(&row.file))
            .is_some_and(|file| file.keep_mine)
    };
    let selected_kept = selected.as_ref().is_some_and(&kept);
    let kept_rows = rows.iter().map(kept).collect::<Vec<_>>();
    let mut dialog = Dialog::new(
        rspice_design::library::folder::CHANGED_SINCE_EDITING,
        "Keep my version",
    )
    .description(
        "Another writer saved these cellviews while you were editing them here, and Save refuses each until you choose. Keep my version writes your edits over the file at the next save; Take the file's version discards your edits to it and reads the file.",
    )
    .size(DialogSize::Transaction)
    .flush_body()
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(selected.is_some() && !selected_kept)
    .secondary("Take the file's version")
    .secondary_enabled(selected.is_some())
    .ghost("Close");
    if let Some(error) = cx.state.library_files.error.as_deref() {
        dialog = dialog.hint(error);
    }
    let choice = dialog.show(ctx, |ui| {
        // Every row here changed on disk since its edits began, as the
        // title says: the column states what is chosen for each, and Save
        // refuses each not yet chosen.
        let width = last_column_width(ui, [("Not chosen", true), ("Keep my version", false)]);
        cellview_table(
            ui,
            "workbench.project.library-files.changed-on-disk",
            &rows,
            table,
            ("Loaded", "On disk", "Choice"),
            width,
            |row| {
                let index = rows.iter().position(|each| each == row).unwrap_or_default();
                if kept_rows[index] {
                    Cell::text("Keep my version")
                } else {
                    Cell::problem("Not chosen", Problem::Failure)
                }
            },
        );
    });
    let Some(row) = selected else {
        if matches!(choice, DialogChoice::Ghost | DialogChoice::Cancelled) {
            cx.state.library_files.close();
        }
        return;
    };
    let shown = format!("{}/{}", row.library, row.file);
    match choice {
        DialogChoice::Primary => match cx.model.keep_my_library_file(&row.library, &row.file) {
            Ok(()) => {
                cx.state.library_files.error = None;
                cx.wb.prompt(format!(
                    "{shown}: your version is written at the next save."
                ));
            }
            Err(refusal) => cx.state.library_files.error = Some(refusal),
        },
        DialogChoice::Secondary => {
            match cx
                .model
                .take_library_file_from_disk(&row.library, &row.file)
            {
                Ok(pruned) => {
                    cx.state.library_files.error = None;
                    cx.state.library_files.table = TableState::default();
                    announce(
                        cx,
                        Ok(pruned),
                        format!(
                            "Took the file's version of {shown}; your edits to it are discarded."
                        ),
                    );
                }
                Err(refusal) => cx.state.library_files.error = Some(refusal),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::None => {}
    }
}

fn detach_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
    ctx: &Context,
    library: &str,
    drawings: &[String],
) {
    let host = rspice_design::library::folder::HostPlatform::current();
    let source = cx
        .model
        .library_manager
        .reference(library)
        .map(|reference| reference.path.shown(host))
        .unwrap_or_default();
    let error = cx.state.library_files.error.clone();
    let title = format!("Detach library \u{2018}{library}\u{2019}");
    let placements = match drawings.len() {
        0 => "Nothing outside the library places its cells.".to_owned(),
        1 => "1 drawing keeps its placements, unresolved.".to_owned(),
        count => format!("{count} drawings keep their placements, unresolved."),
    };
    let rows = drawings
        .iter()
        .map(|drawing| vec![drawing.clone()])
        .collect::<Vec<_>>();
    let choice = CommandForm::new(&title, "Detach")
        .describe("Drop the project's reference to a library kept in a folder of its own.")
        .reading_column()
        .status(error.as_deref().map_or(
            Ok(Some(
                "The folder and its files stay as they are. Undo attaches it again.",
            )),
            Err,
        ))
        .show(ctx, |rows_form| {
            rows_form.value("Library", library);
            rows_form.value_mono("Folder", &source);
            rows_form.value("Placements", &placements);
            if !rows.is_empty() {
                rows_form.table(
                    &[TableColumn {
                        header: "Drawing",
                        mono: true,
                    }],
                    &rows,
                );
            }
            None
        });
    match choice {
        DialogChoice::Primary => match cx.model.detach_library(library) {
            Ok(left) => {
                cx.state.library_files.close();
                let notice = match left.len() {
                    0 => format!("Detached library '{library}'."),
                    count => format!(
                        "Detached library '{library}'; {count} drawing{} keep{} its placements unresolved: {}.",
                        if count == 1 { "" } else { "s" },
                        if count == 1 { "s" } else { "" },
                        left.join(", ")
                    ),
                };
                announce(cx, Ok(WorkspacePruned::default()), notice);
            }
            Err(refusal) => cx.state.library_files.error = Some(refusal),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Who holds a lock and since when, as a refusal and Break lock… name them:
/// "jsmith on LAB-PC3 since 2026-11-17 10:42".
pub fn lock_holder(lock: &LibraryFileLock) -> String {
    match crate::time_compat::local_date_time(lock.since_unix_ms) {
        Some(stamp) => format!(
            "{} since {:04}-{:02}-{:02} {:02}:{:02}",
            lock.holder(),
            stamp.year(),
            u8::from(stamp.month()),
            stamp.day(),
            stamp.hour(),
            stamp.minute()
        ),
        None => lock.holder(),
    }
}

fn break_lock_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
    ctx: &Context,
    library: &str,
    file: &LibraryFile,
    holder: &LibraryFileLock,
) {
    let error = cx.state.library_files.error.clone();
    let held = lock_holder(holder);
    let shown = format!("{library}/{file}");
    let choice = CommandForm::new("Break lock", "Break lock")
        .describe("Remove another holder's lock on one cellview file.")
        .reading_column()
        .status(error.as_deref().map_or(
            Ok(Some(
                "Their unsaved edits stay theirs; their next save of it finds it changed.",
            )),
            Err,
        ))
        .show(ctx, |rows| {
            rows.value_mono("File", &shown);
            rows.value("Being edited by", &held);
            rows.value("In project", &holder.project);
            None
        });
    match choice {
        DialogChoice::Primary => match cx.model.break_library_lock(library, file) {
            Ok(()) => {
                cx.state.library_files.close();
                announce(
                    cx,
                    Ok(WorkspacePruned::default()),
                    format!("Broke the lock {} held on {shown}.", holder.holder()),
                );
            }
            Err(refusal) => cx.state.library_files.error = Some(refusal),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn relink_dialog(cx: &mut Cx<'_, ProjectFeature>, ctx: &Context, library: &str, folder: &Path) {
    let error = cx.state.library_files.error.clone();
    let shown = folder.display().to_string();
    let choice = CommandForm::new("Relink to another copy?", "Relink")
        .describe("Relink a library to a folder that holds it under another identity.")
        .reading_column()
        .status(error.as_deref().map_or(
            Ok(Some(
                "The folder holds a library of this name made separately, not this one moved.",
            )),
            Err,
        ))
        .show(ctx, |rows| {
            rows.value("Library", library);
            rows.value_mono("Folder", &shown);
            None
        });
    match choice {
        DialogChoice::Primary => relink(cx, library, folder, true),
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn moved_on_dialog(cx: &mut Cx<'_, ProjectFeature>, ctx: &Context) {
    let rows = cx.state.library_files.moved_on.clone();
    let picked_rows = picked(&cx.state.library_files.table, &rows);
    let mut dialog = Dialog::new(
        "Cellviews moved on since the publication",
        "Restore picked",
    )
    .description(
        "The project is rebuilt as published. These cellviews' folders hold newer revisions, which read Changed until you refresh them. The picked ones are restored from the publication's copies, each written under its own lock.",
    )
    .size(DialogSize::Transaction)
    .flush_body()
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(!picked_rows.is_empty())
    .ghost("Keep the folders' versions");
    if let Some(error) = cx.state.library_files.error.as_deref() {
        dialog = dialog.hint(error);
    }
    let table = &mut cx.state.library_files.table;
    let choice = dialog.show(ctx, |ui| {
        let width = last_column_width(ui, [("Changed", true)]);
        cellview_table(
            ui,
            "workbench.project.library-files.moved-on",
            &rows,
            table,
            ("Published", "In folder", "Status"),
            width,
            |_| Cell::problem("Changed", Problem::Warning),
        );
    });
    match choice {
        DialogChoice::Primary => {
            let picked = picked_rows.into_iter().cloned().collect::<Vec<_>>();
            restore_moved_on(cx, &picked);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.library_files.close(),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Write the publication's copy of each picked cellview over its folder's
/// newer one. A cellview that cannot be locked or written is named and left
/// as it is.
#[cfg(not(target_arch = "wasm32"))]
fn restore_moved_on(cx: &mut Cx<'_, ProjectFeature>, picked: &[RefreshRow]) {
    let mut restored = Vec::new();
    let mut refused = Vec::new();
    for row in picked {
        match cx
            .model
            .restore_library_file_from_project(&row.library, &row.file)
        {
            Ok(()) => restored.push(format!("{}/{}", row.library, row.file)),
            Err(refusal) => refused.push(refusal),
        }
    }
    let left = cx
        .state
        .library_files
        .moved_on
        .iter()
        .filter(|row| {
            !restored
                .iter()
                .any(|done| *done == format!("{}/{}", row.library, row.file))
        })
        .cloned()
        .collect::<Vec<_>>();
    if !restored.is_empty() {
        cx.model.record(
            crate::diagnostics::LogSeverity::Info,
            crate::diagnostics::LogSource::Studio,
            format!(
                "Restored {} from the publication: {}.",
                if restored.len() == 1 {
                    "1 cellview".to_owned()
                } else {
                    format!("{} cellviews", restored.len())
                },
                restored.join(", ")
            ),
        );
    }
    if refused.is_empty() {
        cx.state.library_files.close();
    } else {
        cx.state.library_files.moved_on = left;
        cx.state.library_files.table = TableState::default();
        cx.state.library_files.error = Some(format!("Left as they are: {}.", refused.join("; ")));
    }
}

#[cfg(target_arch = "wasm32")]
fn restore_moved_on(cx: &mut Cx<'_, ProjectFeature>, _: &[RefreshRow]) {
    cx.state.library_files.error = Some("A browser cannot write a library folder.".to_owned());
}

/// A browser's folder upload: the files of one picked folder, each by its
/// path inside it, read in the background and taken once the read is done.
#[cfg(target_arch = "wasm32")]
mod browser_folder {
    use std::cell::RefCell;

    #[derive(Debug, Clone)]
    pub(super) enum Purpose {
        Embed,
        ReadForSession(String),
    }

    type Picked = Result<(String, Vec<(String, Vec<u8>)>), String>;

    thread_local! {
        static DONE: RefCell<Option<(Purpose, Picked)>> = const { RefCell::new(None) };
    }

    pub(super) fn pick(purpose: Purpose) {
        rspice_app_services::workbench::browser::folder_upload::pick_folder(move |picked| {
            let picked = match picked {
                Ok(Some(folder)) => Ok(folder),
                Ok(None) => return,
                Err(error) => Err(error),
            };
            DONE.with(|done| *done.borrow_mut() = Some((purpose, picked)));
        });
    }

    pub(super) fn take() -> Option<(Purpose, Picked)> {
        DONE.with(|done| done.borrow_mut().take())
    }
}
