//! Everything the project reads, and what each stands at: one status per
//! dependency, read by every place that names one.
//!
//! A design library is read from the project file, so it is in the project
//! for as long as the catalogue holds it. A model library is built in, or it
//! executes from source files whose bytes the project pinned when they were
//! attached; whether those still hold is the retained drift report's answer,
//! which is decided on events and never on a frame. The technology states
//! its last proven verdict. None of these reads the disk or hashes a byte.
//! The project's include search paths, its decks and the files their include
//! lines name are files of the host: they are read when the project changes
//! or the reader asks to read them again, never on a frame.
//!
//! The table and the inspector beside the Dependencies page read the rows
//! built here, once per change of what a status reads, so the two list one
//! set of dependencies in one order and say the same thing about each.

mod files;

use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{LibraryMode, ModelSourceDriftScope};
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::overview::Tone;

pub use files::reread_files;

/// Painted in a column the library on that row has no value for.
pub(crate) const UNSET: &str = "\u{2014}";

/// What a dependency is, in the order the table lists the kinds. A design
/// library and a model library may share a name, so a dependency is named by
/// both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyKind {
    /// The process the project simulates with.
    Technology,
    Design,
    Model,
    /// A folder the project's include lines are searched in.
    IncludePath,
    /// A SPICE deck the project is made of.
    Deck,
    /// A file a deck's include line names.
    Include,
}

impl DependencyKind {
    pub const ALL: [Self; 6] = [
        Self::Technology,
        Self::Design,
        Self::Model,
        Self::IncludePath,
        Self::Deck,
        Self::Include,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Technology => "Technology",
            Self::Design => "Design library",
            Self::Model => "Model library",
            Self::IncludePath => "Include path",
            Self::Deck => "Deck",
            Self::Include => "Include",
        }
    }

    /// A design or a model library: what the catalogues hold.
    pub const fn is_library(self) -> bool {
        matches!(self, Self::Design | Self::Model)
    }
}

/// One dependency of the project, by kind and name. An include search path
/// is named by its path as the project stores it, an include by its path as
/// the deck writes it, and the technology by its package.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyLibrary {
    pub kind: DependencyKind,
    pub name: String,
}

impl DependencyLibrary {
    pub fn design(name: impl Into<String>) -> Self {
        Self {
            kind: DependencyKind::Design,
            name: name.into(),
        }
    }

    pub fn model(name: impl Into<String>) -> Self {
        Self {
            kind: DependencyKind::Model,
            name: name.into(),
        }
    }

    pub fn include_path(path: impl Into<String>) -> Self {
        Self {
            kind: DependencyKind::IncludePath,
            name: path.into(),
        }
    }

    /// What the table's Name column says: a folder or a file by its own
    /// name, the path that tells two of one name apart under Location.
    pub fn shown_name(&self) -> &str {
        match self.kind {
            DependencyKind::IncludePath | DependencyKind::Include => {
                last_component(&self.name).unwrap_or(&self.name)
            }
            _ => &self.name,
        }
    }
}

/// The last named part of a path as written, with either separator.
fn last_component(path: &str) -> Option<&str> {
    path.trim_end_matches(['/', '\\'])
        .rsplit(['/', '\\'])
        .next()
        .filter(|name| !name.is_empty() && *name != "." && *name != "..")
}

/// Where one library stands: the mark it earns, the word for it, and what
/// the reader needs to act on it when there is more to say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryStatus {
    pub mark: Tone,
    pub label: &'static str,
    pub detail: Option<String>,
}

impl LibraryStatus {
    fn neutral(label: &'static str, detail: Option<String>) -> Self {
        Self {
            mark: Tone::Neutral,
            label,
            detail,
        }
    }
}

/// Where `library` stands in `model`.
pub fn library_status(model: &Model, library: &DependencyLibrary) -> LibraryStatus {
    match library.kind {
        // A library held in the project reads In project; one kept in a
        // folder of its own reads where its folder stands.
        DependencyKind::Design => {
            model
                .reference_status(&library.name)
                .map_or_else(not_in_project, |status| LibraryStatus {
                    mark: tone(status.mark()),
                    label: status.label(),
                    detail: status.detail(),
                })
        }
        DependencyKind::Model => model
            .model_library_manager
            .get_library(&library.name)
            .map_or_else(not_in_project, |found| model_library_status(model, found)),
        DependencyKind::Technology => technology_status(model),
        // A file's status is read from the disk with its row; by name alone
        // nothing can be said of it.
        DependencyKind::IncludePath | DependencyKind::Deck | DependencyKind::Include => {
            not_in_project()
        }
    }
}

fn not_in_project() -> LibraryStatus {
    LibraryStatus::neutral("Not in project", None)
}

/// What a library, cell or view row of the Library page and its rail prints:
/// only its exception, in the words the one status function gives. A row
/// that reads Found, or In project, and that nobody else is editing, prints
/// nothing beyond Read-only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowException {
    pub mark: Tone,
    pub label: String,
    pub detail: Option<String>,
}

impl RowException {
    fn plain(label: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            mark: Tone::Neutral,
            label: label.into(),
            detail,
        }
    }

    fn of_status(status: LibraryStatus) -> Self {
        Self {
            mark: status.mark,
            label: status.label.to_owned(),
            detail: status.detail,
        }
    }

    fn of_file(status: &rspice_design::library::folder::CellViewStatus) -> Self {
        Self {
            mark: tone(status.mark()),
            label: status.label().to_owned(),
            detail: status.detail(),
        }
    }

    fn being_edited(lock: &rspice_design::library::folder::LibraryFileLock) -> Self {
        Self::plain(
            format!("Being edited by {}", lock.holder()),
            Some(format!(
                "Being edited by {} in {}",
                crate::workbench::project::library_dialogs::library_files::lock_holder(lock),
                lock.project
            )),
        )
    }
}

/// The width a column of statuses takes: its widest words whole, a mark
/// and the cell's padding included. Never under `least`, the head's own,
/// nor over `most`, past which the longest is cut after a whole word and
/// its detail shows under the pointer.
pub fn status_column_width<'a>(
    ui: &egui::Ui,
    least: f32,
    most: f32,
    statuses: impl IntoIterator<Item = (&'a str, bool)>,
) -> f32 {
    let font = crate::ui::theme::sans(
        crate::ui::tokens::FS_0,
        crate::ui::theme::FontWeight::Regular,
    );
    let mut measured = std::collections::BTreeMap::new();
    for (label, marked) in statuses {
        let mark = if marked {
            crate::ui::widgets::PROBLEM_MARK_ADVANCE
        } else {
            0.0
        };
        let entry = measured.entry(label).or_insert(mark);
        *entry = entry.max(mark);
    }
    let widest = measured
        .into_iter()
        .map(|(label, mark)| {
            ui.painter()
                .layout_no_wrap(label.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
                .size()
                .x
                + mark
        })
        .fold(0.0_f32, f32::max);
    (widest.ceil() + 2.0 * crate::ui::widgets::table::CELL_PAD + 2.0).clamp(least, most.max(least))
}

/// Every design library's row exception, by name, derived again only when
/// the catalogue, the references or what this session read of their folders
/// changes, so an idle frame reads them for nothing.
pub fn library_exceptions(
    ctx: &egui::Context,
    model: &Model,
) -> Arc<std::collections::BTreeMap<String, RowException>> {
    let key = [
        model.library_manager.catalog().stamp(),
        model.library_manager.references_stamp(),
        model.library_files.stamp(),
    ];
    derived(ctx, "library.exceptions", key, || {
        Arc::new(
            model
                .library_manager
                .libraries_sorted()
                .into_iter()
                .filter_map(|library| {
                    library_exception(model, &library.name)
                        .map(|exception| (library.name.clone(), exception))
                })
                .collect(),
        )
    })
}

fn tone(mark: rspice_design::library::folder::StatusMark) -> Tone {
    match mark {
        rspice_design::library::folder::StatusMark::None => Tone::Neutral,
        rspice_design::library::folder::StatusMark::Caution => Tone::Warn,
        rspice_design::library::folder::StatusMark::Error => Tone::Error,
    }
}

/// A library row's exception: its status when that is neither Found nor In
/// project, else who else is editing its own file, else Read-only.
pub fn library_exception(model: &Model, library: &str) -> Option<RowException> {
    let status = library_status(model, &DependencyLibrary::design(library));
    if !matches!(status.label, "Found" | "In project") {
        return Some(RowException::of_status(status));
    }
    if let Some(lock) = model.library_lock_elsewhere(
        library,
        &rspice_design::library::folder::LibraryFile::Manifest,
    ) {
        return Some(RowException::being_edited(lock));
    }
    model
        .library_manager
        .get_library(library)
        .filter(|found| found.read_only)
        .map(|_| RowException::plain("Read-only", None))
}

/// A file's exception: its status when that is not Found, else who else is
/// editing it.
fn file_exception(
    model: &Model,
    library: &str,
    file: &rspice_design::library::folder::LibraryFile,
) -> Option<RowException> {
    match model.library_file_status(library, file) {
        Some(status)
            if !matches!(
                status,
                rspice_design::library::folder::CellViewStatus::Found { .. }
            ) =>
        {
            Some(RowException::of_file(&status))
        }
        _ => model
            .library_lock_elsewhere(library, file)
            .map(RowException::being_edited),
    }
}

/// A cell row's exception: its own file's, else the first of its views'
/// that is not Found, so a cell holding a changed view says so.
pub fn cell_exception(model: &Model, library: &str, cell: &str) -> Option<RowException> {
    model.library_manager.reference(library)?;
    if let Some(exception) = file_exception(
        model,
        library,
        &rspice_design::library::folder::LibraryFile::cell(cell),
    ) {
        return Some(exception);
    }
    model
        .library_manager
        .get_library(library)?
        .get_cell(cell)?
        .views_sorted()
        .into_iter()
        .filter_map(|view| {
            let file = rspice_design::library::folder::LibraryFile::cell_view(cell, view);
            model
                .library_file_status(library, &file)
                .filter(|status| {
                    !matches!(
                        status,
                        rspice_design::library::folder::CellViewStatus::Found { .. }
                    )
                })
                .map(|status| RowException::of_file(&status))
        })
        .next()
}

/// A view row's exception: its file's status when that is not Found, else
/// who else is editing it.
pub fn view_exception(
    model: &Model,
    library: &str,
    cell: &str,
    view: &crate::state::View,
) -> Option<RowException> {
    model.library_manager.reference(library)?;
    file_exception(
        model,
        library,
        &rspice_design::library::folder::LibraryFile::cell_view(cell, view),
    )
}

fn model_library_status(model: &Model, library: &ModelLibrary) -> LibraryStatus {
    let authority = library.source_authority;
    if !authority.has_execution_source() {
        // Its models ship with RSpice and simulate from there: there is no
        // file to find, so there is nothing that can be missing.
        return LibraryStatus::neutral("Built in", Some("Ships with RSpice".to_owned()));
    }
    let files = library.source_closure.len();
    if library.root_path.is_none() || files == 0 {
        return LibraryStatus {
            mark: Tone::Error,
            label: "No model files",
            detail: Some("No model file has been read for this library".to_owned()),
        };
    }
    if authority.uses_retained_bytes() {
        let kept: BTreeSet<_> = library
            .source_contents
            .iter()
            .map(|content| &content.path)
            .collect();
        let missing = library
            .source_closure
            .iter()
            .filter(|pin| !kept.contains(&pin.path))
            .count();
        if missing > 0 {
            return LibraryStatus {
                mark: Tone::Error,
                label: "Incomplete",
                detail: Some(format!(
                    "{missing} of {} not kept in the project",
                    model_files(files)
                )),
            };
        }
    }
    let findings = model.source_drift_findings(&library.name);
    let more = |count: usize| match count {
        0 | 1 => String::new(),
        count => format!(" \u{00b7} and {} more", count - 1),
    };
    let unread: Vec<_> = findings
        .iter()
        .filter(|finding| finding.on_disk.is_none())
        .collect();
    if let Some(first) = unread.first() {
        return LibraryStatus {
            mark: Tone::Error,
            label: "Not found",
            detail: Some(format!(
                "{} cannot be read{}",
                file_name(&first.path),
                more(unread.len())
            )),
        };
    }
    if let Some(first) = findings.first() {
        return LibraryStatus {
            mark: Tone::Warn,
            label: "Changed",
            detail: Some(format!(
                "{} \u{00b7} attached {} \u{00b7} now {}{}",
                file_name(&first.path),
                first.pinned,
                first.on_disk.as_deref().unwrap_or(UNSET),
                more(findings.len())
            )),
        };
    }
    let label = if matches!(authority, ModelSourceAuthority::External) {
        "Found"
    } else {
        "In project"
    };
    LibraryStatus::neutral(label, Some(model_files(files)))
}

/// The file a finding is about, by name: the library's source row beside it
/// gives the whole path.
fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || shown_path(path),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// The source a library the project file holds is listed with: words, not
/// a path.
pub const PROJECT_FILE_SOURCE: &str = "Project file";

/// Where a library that ships with RSpice is.
pub const BUILT_IN_SOURCE: &str = "Built in";

/// A path as the reader knows it. Canonicalising a Windows drive path adds
/// the verbatim `\\?\` prefix, which is no part of the name they gave it.
pub(crate) fn shown_path(path: &Path) -> String {
    let text = path.display().to_string();
    let Some(rest) = text.strip_prefix(r"\\?\") else {
        return text;
    };
    if let Some(share) = rest.strip_prefix(r"UNC\") {
        return format!(r"\\{share}");
    }
    let drive = rest.as_bytes();
    if drive.first().is_some_and(u8::is_ascii_alphabetic) && drive.get(1) == Some(&b':') {
        return rest.to_owned();
    }
    text
}

/// `path` as it is found from the project: from the project folder when it
/// is inside it, or inside the folder that holds it, where a project keeps
/// the libraries and models beside it; else the whole path, since a climb
/// out of the project's neighbourhood names nothing the reader knows it by.
pub(crate) fn project_relative(path: &Path, project_folder: Option<&Path>) -> String {
    let whole = shown_path(&lexical(path));
    let Some(folder) = project_folder.map(|folder| shown_path(&lexical(folder))) else {
        return whole;
    };
    let path = Path::new(&whole);
    let folder = Path::new(&folder);
    if let Ok(inside) = path.strip_prefix(folder) {
        return if inside.as_os_str().is_empty() {
            ".".to_owned()
        } else {
            inside.display().to_string()
        };
    }
    folder
        .parent()
        .filter(|parent| parent.parent().is_some())
        .and_then(|parent| path.strip_prefix(parent).ok())
        .filter(|beside| !beside.as_os_str().is_empty())
        .map_or(whole.clone(), |beside| {
            Path::new("..").join(beside).display().to_string()
        })
}

/// `path` as the file system reads it, without asking the disk: each `.`
/// step dropped, each `..` taken against the step before it, and every step
/// joined with the platform's separator, so a path a deck wrote with `/`
/// reads as one the project wrote.
pub(crate) fn lexical(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut steps = PathBuf::new();
    for step in path.components() {
        match step {
            Component::CurDir => {}
            Component::ParentDir
                if matches!(steps.components().next_back(), Some(Component::Normal(_))) =>
            {
                steps.pop();
            }
            other => steps.push(other.as_os_str()),
        }
    }
    steps
}

fn model_files(count: usize) -> String {
    format!("{count} model file{}", if count == 1 { "" } else { "s" })
}

/// What every status reads: the two catalogues, the drift report, the
/// technology's status and the files the project reads. Each is read by its
/// stamp or by what it found, never by walking a catalogue or the disk, so
/// asking for the key costs an idle frame nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyStatusKey {
    /// The catalogue, the references and what this session read of each
    /// library's folder.
    design: [u64; 3],
    models: [u64; 3],
    drift: (Option<ModelSourceDriftScope>, u64),
    technology: u64,
    files: files::FilesKey,
}

pub fn dependency_status_key(ctx: &egui::Context, model: &Model) -> DependencyStatusKey {
    // The report holds only the libraries that drifted, so hashing what it
    // found is as cheap as the report is short, and catches a rescan that
    // found something new under the same scope.
    let mut findings = DefaultHasher::new();
    for (library, found) in &model.source_drift.libraries {
        library.hash(&mut findings);
        for finding in found {
            finding.path.hash(&mut findings);
            finding.pinned.hash(&mut findings);
            finding.on_disk.hash(&mut findings);
        }
    }
    let mut technology = DefaultHasher::new();
    let binding = model.workspace.content.project.technology_binding();
    binding
        .map(|binding| binding.display_label())
        .hash(&mut technology);
    binding
        .map(|binding| binding.root_source())
        .hash(&mut technology);
    let status = technology_status(model);
    (status.mark as u8, status.label, status.detail).hash(&mut technology);
    DependencyStatusKey {
        design: [
            model.library_manager.content_stamp(),
            model.library_manager.references_stamp(),
            model.library_files.stamp(),
        ],
        models: model.model_library_manager.content_stamp(),
        drift: (model.source_drift.scanned, findings.finish()),
        technology: technology.finish(),
        files: files::files_key(ctx, model),
    }
}

/// One row of what the project depends on, as the table and the inspector
/// show it.
#[derive(Debug, Clone)]
pub struct DependencyRow {
    pub library: DependencyLibrary,
    /// The version it states, or [`UNSET`].
    pub version: String,
    /// Where it is read from, as the project stores it: a path as written,
    /// or words for a library with none ([`PROJECT_FILE_SOURCE`],
    /// [`BUILT_IN_SOURCE`]).
    pub source: String,
    /// Where it is, as the Location column states it: from the project
    /// folder when that is near, as written for a path the project stores.
    pub location: String,
    /// The whole path it resolves to on this computer, for the hover over
    /// its location and the inspector's Resolves to; `None` for what is not
    /// a file or folder of the host.
    pub resolved: Option<String>,
    /// Editable or Read-only, or [`UNSET`] for what has no access of its
    /// own to set.
    pub access: &'static str,
    pub digest: String,
    pub status: LibraryStatus,
}

impl DependencyRow {
    /// Whether the reader's filter, trimmed and lowercased, finds this row
    /// in anything it shows.
    pub fn matches(&self, filter: &str) -> bool {
        filter.is_empty()
            || [
                self.library.name.as_str(),
                self.library.kind.label(),
                self.version.as_str(),
                self.source.as_str(),
                self.location.as_str(),
                self.digest.as_str(),
                self.status.label,
            ]
            .iter()
            .any(|value| value.to_lowercase().contains(filter))
    }
}

/// Everything the project depends on, by kind in [`DependencyKind`]'s order,
/// each kind by name without regard to case, except the include search
/// paths, which keep the order they are searched in. Built when a status
/// input changes, so an idle frame sorts nothing.
pub fn dependency_rows(ctx: &egui::Context, model: &Model) -> Arc<[DependencyRow]> {
    derived(
        ctx,
        "dependencies.rows",
        dependency_status_key(ctx, model),
        || build_rows(model).into(),
    )
}

/// The rows `filter` and `kind` let through, by index into
/// [`dependency_rows`], chosen when the filter, the kind or the rows change.
pub fn visible_dependency_rows(
    ctx: &egui::Context,
    model: &Model,
    filter: &str,
    kind: Option<DependencyKind>,
) -> (Arc<[DependencyRow]>, Arc<[usize]>) {
    let rows = dependency_rows(ctx, model);
    let filter = filter.trim().to_lowercase();
    let visible = derived(
        ctx,
        "dependencies.visible",
        (dependency_status_key(ctx, model), filter.clone(), kind),
        || {
            rows.iter()
                .enumerate()
                .filter(|(_, row)| kind.is_none_or(|kind| row.library.kind == kind))
                .filter(|(_, row)| row.matches(&filter))
                .map(|(index, _)| index)
                .collect::<Arc<[usize]>>()
        },
    );
    (rows, visible)
}

fn build_rows(model: &Model) -> Vec<DependencyRow> {
    let project_folder = model.workspace.content.project.data_root();
    let technology = technology_row(model, project_folder);
    let design = model
        .library_manager
        .catalog()
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let key = DependencyLibrary::design(&library.name);
            let source = model.library_manager.reference(&library.name).map_or_else(
                || PROJECT_FILE_SOURCE.to_owned(),
                |reference| {
                    reference
                        .path
                        .shown(rspice_design::library::folder::HostPlatform::current())
                },
            );
            DependencyRow {
                status: library_status(model, &key),
                library: key,
                // A design library has no version; the technology it is
                // drawn for is the inspector's to state.
                version: UNSET.to_owned(),
                location: source.clone(),
                source,
                resolved: model.library_folder(&library.name).map(shown_path),
                access: if library.read_only {
                    LibraryMode::ReadOnly.label()
                } else {
                    LibraryMode::Editable.label()
                },
                digest: UNSET.to_owned(),
            }
        });
    let models = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let key = DependencyLibrary::model(&library.name);
            let built_in = !library.source_authority.has_execution_source();
            DependencyRow {
                status: library_status(model, &key),
                library: key,
                version: nonempty_or_unset(&library.version),
                source: library.root_path.as_deref().map_or_else(
                    || {
                        if built_in {
                            BUILT_IN_SOURCE.to_owned()
                        } else {
                            UNSET.to_owned()
                        }
                    },
                    shown_path,
                ),
                location: library.root_path.as_deref().map_or_else(
                    || {
                        if built_in {
                            BUILT_IN_SOURCE.to_owned()
                        } else {
                            UNSET.to_owned()
                        }
                    },
                    |root| project_relative(root, project_folder),
                ),
                resolved: library.root_path.as_deref().map(shown_path),
                // RSpice reads a model file and never writes one.
                access: LibraryMode::ReadOnly.label(),
                digest: library
                    .source_closure
                    .iter()
                    .find(|pin| library.root_path.as_ref() == Some(&pin.path))
                    .or_else(|| library.source_closure.first())
                    .map_or_else(
                        || UNSET.to_owned(),
                        |pin| pin.digest.to_string().chars().take(8).collect(),
                    ),
            }
        });
    let mut libraries: Vec<_> = design.chain(models).collect();
    libraries.sort_by(|left, right| {
        left.library
            .kind
            .cmp(&right.library.kind)
            .then_with(|| {
                left.library
                    .name
                    .to_lowercase()
                    .cmp(&right.library.name.to_lowercase())
            })
            .then_with(|| left.library.name.cmp(&right.library.name))
    });
    technology
        .into_iter()
        .chain(libraries)
        .chain(files::file_rows(model))
        .collect()
}

/// The technology's row: the package it is attached from, the main model
/// file its runs read, and its last proven status. A project with none has
/// no row, unless its setup needs one, when the row is what says so.
fn technology_row(model: &Model, project_folder: Option<&Path>) -> Option<DependencyRow> {
    let status = technology_status(model);
    let Some(binding) = model.workspace.content.project.technology_binding() else {
        return status.mark.problem().map(|_| DependencyRow {
            library: DependencyLibrary {
                kind: DependencyKind::Technology,
                name: String::new(),
            },
            version: UNSET.to_owned(),
            source: UNSET.to_owned(),
            location: UNSET.to_owned(),
            resolved: None,
            access: UNSET,
            digest: UNSET.to_owned(),
            status,
        });
    };
    let root = binding.root_source();
    let version = binding
        .pdk_package()
        .map(|pin| pin.revision().to_owned())
        .or_else(|| binding.package_version().map(str::to_owned))
        .unwrap_or_else(|| UNSET.to_owned());
    Some(DependencyRow {
        library: DependencyLibrary {
            kind: DependencyKind::Technology,
            name: binding.package_name().to_owned(),
        },
        version,
        source: shown_path(root),
        location: project_relative(root, project_folder),
        resolved: Some(shown_path(root)),
        // Its sources are pinned when it is attached.
        access: LibraryMode::ReadOnly.label(),
        digest: binding
            .source_closure()
            .iter()
            .find(|pin| pin.path == root)
            .or_else(|| binding.source_closure().first())
            .map_or_else(
                || UNSET.to_owned(),
                |pin| pin.digest.to_string().chars().take(8).collect(),
            ),
        status,
    })
}

fn nonempty_or_unset(value: &str) -> String {
    if value.trim().is_empty() {
        UNSET.to_owned()
    } else {
        value.to_owned()
    }
}

/// Bring the drift report up to what the model catalogue holds now, when
/// either it or the project's revision has moved since it was last asked.
/// Scanning rehashes every pinned source, which is why it is asked here, on
/// a change, and never inside a status.
pub fn refresh_source_drift(ctx: &egui::Context, model: &mut Model) {
    let key = (
        model.model_library_manager.content_stamp(),
        model.workspace.content.project.revision().get(),
    );
    derived(ctx, "dependencies.drift", key, || {
        if model.source_drift_needs_scan() {
            model.scan_source_drift();
            // Whatever drew a status before this frame's scan draws it again.
            ctx.request_repaint();
        }
    });
}

/// What the Dependencies page has selected: kept while it names a
/// dependency of the project, else nothing, which the inspector reads as
/// the project itself.
pub fn settle_selection(rows: &[DependencyRow], selection: &mut Option<DependencyLibrary>) {
    let names_a_row = selection
        .as_ref()
        .is_some_and(|selected| rows.iter().any(|row| &row.library == selected));
    if !names_a_row {
        *selection = None;
    }
}

/// The technology the project's runs take their model sources from, as the
/// Dependencies page and its inspector state it. A project with none is
/// ordinary unless the simulation setup needs one, which is when a run would
/// be refused.
pub fn technology_status(model: &Model) -> LibraryStatus {
    if model
        .workspace
        .content
        .project
        .technology_binding()
        .is_none()
    {
        return match model.technology_inputs().technology_demand().block_reason() {
            Some(reason) => LibraryStatus {
                mark: Tone::Error,
                label: "Not attached",
                detail: Some(reason),
            },
            None => LibraryStatus::neutral("Not attached", None),
        };
    }
    if let Some(notice) = model
        .workspace
        .content
        .project
        .technology_binding()
        .and_then(|binding| model.technology_not_installed(binding))
    {
        return LibraryStatus {
            mark: Tone::Error,
            label: "Not installed",
            detail: Some(notice),
        };
    }
    // As last proven: proving it hashes the attached library's sources. A
    // binding that no longer resolves refuses every run.
    match model.project_technology_verdict() {
        Ok(()) => LibraryStatus::neutral("Attached", None),
        Err(error) => LibraryStatus {
            mark: Tone::Error,
            label: "Not resolved",
            detail: Some(error),
        },
    }
}

#[cfg(test)]
mod tests;
