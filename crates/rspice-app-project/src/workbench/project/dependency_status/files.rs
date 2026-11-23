//! The files the project reads besides its libraries: its include search
//! paths, in the order they are searched, and its decks with the files
//! their include lines name.
//!
//! Each is a file or a folder of the host, so its status reads the disk.
//! That is done when the project, its decks or where it is saved change, or
//! when the reader asks to read them again; never on a frame. A browser has
//! no host files: there a deck is kept in the project with the files its
//! include lines read, and an include search path is listed in its order
//! with nothing said of a folder it cannot see.

use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use rspice_simulation::netlist_preparation::IncludeSearchChain;

use super::{
    DependencyKind, DependencyLibrary, DependencyRow, LibraryStatus, PROJECT_FILE_SOURCE, UNSET,
    project_relative, shown_path,
};
use crate::state::{DeckSource, ProjectDeck, StoredDeck};
use crate::workbench::app_state::Model;
use crate::workbench::surfaces::project::overview::Tone;

/// What the files' rows were read under: the project's revision, where it
/// is saved, its include search paths and its decks as the project holds
/// them, and how many times the reader asked to read them again. Each is
/// hashed from what the project holds, never from the disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FilesKey {
    revision: u64,
    held: u64,
    reread: u64,
}

fn reread_id() -> egui::Id {
    egui::Id::new("workbench.project.dependencies.reread")
}

/// Read the project's include search paths, decks and include files again
/// the next time a dependency is asked for: the reader's Refresh, for a
/// change made outside RSpice.
pub fn reread_files(ctx: &egui::Context) {
    ctx.data_mut(|data| *data.get_temp_mut_or_default::<u64>(reread_id()) += 1);
}

pub(super) fn files_key(ctx: &egui::Context, model: &Model) -> FilesKey {
    let content = &model.workspace.content;
    let mut held = DefaultHasher::new();
    content.project.path.hash(&mut held);
    content.project.include_search_paths().hash(&mut held);
    content.decks_dirty.hash(&mut held);
    for deck in &content.decks {
        deck.id.hash(&mut held);
        match &deck.source {
            DeckSource::File { path } => path.hash(&mut held),
            // The text is not read on a frame: each text is told apart by
            // its identity, which any edit to it changes.
            DeckSource::Stored(stored) => {
                stored.name.hash(&mut held);
                stored.text.identity().hash(&mut held);
                for file in &stored.files {
                    file.path.hash(&mut held);
                    file.text.identity().hash(&mut held);
                }
            }
        }
    }
    FilesKey {
        revision: content.project.revision().get(),
        held: held.finish(),
        reread: ctx.data(|data| data.get_temp::<u64>(reread_id()).unwrap_or_default()),
    }
}

/// The include search paths in the order they are searched, then each
/// deck and the files its include lines name, a file two lines or two
/// decks name listed once.
pub(super) fn file_rows(model: &Model) -> Vec<DependencyRow> {
    let project = &model.workspace.content.project;
    let folder = project.data_root();
    let mut rows = include_path_rows(project.include_search_paths(), folder);
    let mut includes: Vec<DependencyRow> = Vec::new();
    let mut decks: Vec<DependencyRow> = Vec::new();
    for deck in &model.workspace.content.decks {
        let (row, named) = deck_rows(model, deck, folder);
        decks.push(row);
        for include in named {
            if !includes
                .iter()
                .any(|listed| listed.library == include.library)
            {
                includes.push(include);
            }
        }
    }
    let by_name = |left: &DependencyRow, right: &DependencyRow| {
        left.library
            .name
            .to_lowercase()
            .cmp(&right.library.name.to_lowercase())
            .then_with(|| left.library.name.cmp(&right.library.name))
    };
    decks.sort_by(by_name);
    includes.sort_by(by_name);
    rows.extend(decks);
    rows.extend(includes);
    rows
}

fn found(detail: Option<String>) -> LibraryStatus {
    LibraryStatus::neutral("Found", detail)
}

fn not_found(detail: String) -> LibraryStatus {
    LibraryStatus {
        mark: Tone::Error,
        label: "Not found",
        detail: Some(detail),
    }
}

/// What a browser says of a host file or folder it cannot see.
fn not_checked() -> LibraryStatus {
    LibraryStatus::neutral(
        "Not checked",
        Some("A browser cannot see the folders of this computer".to_owned()),
    )
}

/// One row per include search path, as the project stores it and in its
/// order: where it resolves to, and whether a folder is there.
fn include_path_rows(authored: &[PathBuf], folder: Option<&Path>) -> Vec<DependencyRow> {
    let chain = IncludeSearchChain::resolve(authored, folder);
    chain
        .entries()
        .iter()
        .map(|entry| {
            let written = entry.authored().display().to_string();
            let placed = entry.authored().is_absolute() || folder.is_some();
            let status = if !IncludeSearchChain::states_presence() {
                not_checked()
            } else if entry.exists() {
                found(None)
            } else if placed {
                not_found(format!("No folder at {}", shown_path(entry.resolved())))
            } else {
                not_found(
                    "The project is not saved, so a path relative to its folder names no folder"
                        .to_owned(),
                )
            };
            // Read from the project folder as every other row's location is,
            // whichever way the project stores it.
            let location = if placed {
                project_relative(entry.resolved(), folder)
            } else {
                written.clone()
            };
            DependencyRow {
                library: DependencyLibrary::include_path(written.clone()),
                version: UNSET.to_owned(),
                location,
                source: written,
                resolved: placed.then(|| shown_path(entry.resolved())),
                access: UNSET,
                digest: UNSET.to_owned(),
                contents: None,
                status,
            }
        })
        .collect()
}

/// A deck's row and a row for each file its include lines name.
fn deck_rows(
    model: &Model,
    deck: &ProjectDeck,
    folder: Option<&Path>,
) -> (DependencyRow, Vec<DependencyRow>) {
    let name = deck.name();
    let row = |source: String, location: String, resolved: Option<String>, status| DependencyRow {
        library: DependencyLibrary {
            kind: DependencyKind::Deck,
            name: name.clone(),
        },
        version: UNSET.to_owned(),
        source,
        location,
        resolved,
        access: UNSET,
        digest: UNSET.to_owned(),
        contents: None,
        status,
    };
    match &deck.source {
        DeckSource::Stored(stored) => (
            row(
                PROJECT_FILE_SOURCE.to_owned(),
                PROJECT_FILE_SOURCE.to_owned(),
                None,
                LibraryStatus::neutral("In project", None),
            ),
            stored_includes(stored),
        ),
        DeckSource::File { path } => {
            let file = deck.file_path(folder).unwrap_or_else(|| path.clone());
            let written = path.display().to_string();
            let location = if path.is_relative() {
                written.clone()
            } else {
                project_relative(&file, folder)
            };
            let (status, includes) = read_deck(model, &name, &file);
            (
                row(written, location, Some(shown_path(&file)), status),
                includes,
            )
        }
    }
}

/// A row for each file a deck the project keeps as text names: in the
/// project when it keeps that file, else not found.
fn stored_includes(stored: &StoredDeck) -> Vec<DependencyRow> {
    crate::state::parse_include_directives(&stored.text)
        .into_iter()
        .map(|directive| {
            let locator = directive.locator();
            let wanted = rspice_design::deck::stored_file_path(Path::new(locator));
            let wanted = wanted.trim_start_matches("./");
            let kept = stored.files.iter().any(|file| file.path == wanted);
            let status = if kept {
                LibraryStatus::neutral(
                    "In project",
                    Some(format!("{} line {}", stored.name, directive.line())),
                )
            } else {
                not_found(format!(
                    "{} line {} names it; the project keeps no file of that name",
                    stored.name,
                    directive.line()
                ))
            };
            include_row(locator, PROJECT_FILE_SOURCE.to_owned(), None, status)
        })
        .collect()
}

fn include_row(
    locator: &str,
    location: String,
    resolved: Option<String>,
    status: LibraryStatus,
) -> DependencyRow {
    DependencyRow {
        library: DependencyLibrary {
            kind: DependencyKind::Include,
            name: locator.to_owned(),
        },
        version: UNSET.to_owned(),
        source: locator.to_owned(),
        location,
        resolved,
        access: UNSET,
        digest: UNSET.to_owned(),
        contents: None,
        status,
    }
}

/// Whether the deck file is there, and a row for each file its include
/// lines name, found as the engine finds it: beside the deck, then along
/// the project's include search chain.
#[cfg(not(target_arch = "wasm32"))]
fn read_deck(model: &Model, deck: &str, file: &Path) -> (LibraryStatus, Vec<DependencyRow>) {
    let Ok(text) = std::fs::read_to_string(file) else {
        return (
            not_found(format!("No file at {}", shown_path(file))),
            Vec::new(),
        );
    };
    let folder = model.workspace.content.project.data_root();
    let searched: Vec<PathBuf> = file
        .parent()
        .map(Path::to_path_buf)
        .into_iter()
        .chain(model.include_search_chain().directories())
        .collect();
    let includes = crate::state::parse_include_directives(&text)
        .into_iter()
        .map(|directive| {
            let locator = directive.locator();
            let where_named = format!("{deck} line {}", directive.line());
            let requested = Path::new(locator.trim_matches(['"', '\'']));
            let at = if requested.is_absolute() {
                requested.is_file().then(|| requested.to_path_buf())
            } else {
                searched
                    .iter()
                    .map(|directory| directory.join(requested))
                    .find(|candidate| candidate.is_file())
            };
            match at.map(|at| super::lexical(&at)) {
                Some(at) => include_row(
                    locator,
                    project_relative(&at, folder),
                    Some(shown_path(&at)),
                    found(Some(where_named)),
                ),
                None => include_row(
                    locator,
                    locator.to_owned(),
                    None,
                    not_found(format!(
                        "{where_named} names it; no folder searched holds it"
                    )),
                ),
            }
        })
        .collect();
    (found(None), includes)
}

/// A browser has no deck files to read.
#[cfg(target_arch = "wasm32")]
fn read_deck(_: &Model, _: &str, _: &Path) -> (LibraryStatus, Vec<DependencyRow>) {
    (not_checked(), Vec::new())
}
