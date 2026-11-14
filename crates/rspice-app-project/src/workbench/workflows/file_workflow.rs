//! Schematic file workflows.
//!
//! The parts phases of opening and saving a standalone schematic: the cell
//! name an import takes, the browser's file picker, and the open's settling.
//! The whole-app sequences, behind the same I/O trait split as the export
//! workflows, are the shell's, in `session::project_transactions`.

use std::path::Path;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::SchematicState;
use crate::workbench::app_state::{Model, PlatformState};
use rspice_app_services::workbench::slots::platform::remember_recent_file;

#[derive(Debug, Clone, Copy)]
pub(crate) enum SchematicLoadOrigin<'a> {
    PersistentPath(&'a Path),
    #[cfg(any(test, target_arch = "wasm32"))]
    BrowserImport(&'a str),
}

impl<'a> SchematicLoadOrigin<'a> {
    pub(crate) fn display_label(self) -> String {
        match self {
            Self::PersistentPath(path) => path.display().to_string(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) => name.to_string(),
        }
    }

    /// The file's name without its extension.
    pub(crate) fn file_stem(self) -> &'a str {
        match self {
            Self::PersistentPath(path) => path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) => Path::new(name)
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default(),
        }
    }

    fn recent_path(self) -> Option<&'a Path> {
        match self {
            Self::PersistentPath(path) => Some(path),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => None,
        }
    }

    fn success_prefix(self) -> &'static str {
        match self {
            Self::PersistentPath(_) => "Opened schematic",
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => "Imported schematic",
        }
    }
}

/// Remember the file a schematic replacing the drawing in front came from,
/// and record that it was opened.
pub(crate) fn settle_loaded_schematic(
    model: &mut Model,
    platform: &mut PlatformState,
    origin: SchematicLoadOrigin<'_>,
) {
    if let Some(path) = origin.recent_path() {
        remember_recent_file(
            model,
            platform,
            crate::services::recent_files::RecentKind::Schematic,
            path,
        );
    }
    model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("{} {}.", origin.success_prefix(), origin.display_label()),
    );
}

/// A cell name from a file stem: letters, digits and underscores kept, every
/// other run of characters one underscore, never empty.
pub(crate) fn cell_name_from_file(stem: &str) -> String {
    let mut name = String::new();
    for character in stem.chars() {
        if character.is_alphanumeric() || character == '_' {
            name.push(character);
        } else if !name.ends_with('_') {
            name.push('_');
        }
    }
    let name = name.trim_matches('_');
    if name.is_empty() {
        "imported_schematic".to_owned()
    } else {
        name.to_owned()
    }
}

/// `base`, or `base_2`, `base_3`, … : the first the library does not hold,
/// compared as cell identities are (ignoring letter case).
pub(crate) fn unique_cell_name(library: &crate::state::Library, base: &str) -> String {
    let taken = |candidate: &str| {
        let identity = crate::state::canonical_cell_view_owner_key(&library.name, candidate, "");
        library.cells.values().any(|cell| {
            crate::state::canonical_cell_view_owner_key(&library.name, &cell.name, "") == identity
        })
    };
    if !taken(base) {
        return base.to_owned();
    }
    (2..)
        .map(|suffix| format!("{base}_{suffix}"))
        .find(|candidate| !taken(candidate))
        .expect("an unbounded suffix range finds a free name")
}

/// Settle where a schematic replacing the drawing in front saves next: a
/// file opened from disk saves back to it, and a browser import keeps its
/// name as the next save's suggestion and no file binding.
pub(crate) fn prepare_loaded_schematic(
    platform: &mut PlatformState,
    schematic: SchematicState,
    origin: SchematicLoadOrigin<'_>,
) -> SchematicState {
    match origin {
        SchematicLoadOrigin::PersistentPath(_) => {
            platform.browser_schematic_save_name = None;
            schematic
        }
        #[cfg(any(test, target_arch = "wasm32"))]
        SchematicLoadOrigin::BrowserImport(name) => {
            let mut schematic = schematic;
            schematic.session.current_file = None;
            platform.browser_schematic_save_name = Some(name.to_string());
            schematic
        }
    }
}

pub(in crate::workbench) fn file_name_string(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
}

/// The name the schematic's Save As dialog suggests: its file's, or the
/// name the browser last read or downloaded it under.
pub(crate) fn schematic_save_dialog_default_name(
    model: &Model,
    platform: &PlatformState,
) -> Option<String> {
    model
        .schematic
        .session
        .current_file
        .as_deref()
        .and_then(file_name_string)
        .or_else(|| platform.browser_schematic_save_name.clone())
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserSchematicImportResult {
    Cancelled,
    Failed(String),
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserSchematicImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserSchematicImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SCHEMATIC_IMPORT_RESULT: std::cell::RefCell<Option<BrowserSchematicImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_schematic_import() -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Schematic,
    )?;

    crate::workbench::browser::file_import::pick_text_file(
        crate::io::schematic_io::SCHEMATIC_FILTER.0,
        crate::io::schematic_io::SCHEMATIC_FILTER.1,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let event = match result {
                Ok(Some(file)) => BrowserSchematicImportResult::Loaded(file),
                Ok(None) => BrowserSchematicImportResult::Cancelled,
                Err(error) => BrowserSchematicImportResult::Failed(error),
            };
            BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserSchematicImportCompletion {
                    token,
                    result: event,
                });
            });
        },
    );
    Ok(())
}

/// A schematic the browser read for opening, for the shell to apply.
#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserSchematic {
    pub(crate) schematic: SchematicState,
    /// The picked file's name, kept as the next save's suggestion.
    pub(crate) name: String,
}

/// Take the schematic the browser finished reading, if any, recording why
/// a read that failed opened nothing.
#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_schematic_import(model: &mut Model) -> Option<BrowserSchematic> {
    let completion = BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())?;
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return None;
    }
    match completion.result {
        BrowserSchematicImportResult::Loaded(file) => {
            match crate::io::schematic_io::load_schematic_text(&file.contents, None) {
                Ok(schematic) => Some(BrowserSchematic {
                    schematic,
                    name: file.name,
                }),
                Err(error) => {
                    model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("Schematic open failed: {error}"),
                    );
                    None
                }
            }
        }
        BrowserSchematicImportResult::Failed(error) => {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic open failed: {error}"),
            );
            None
        }
        BrowserSchematicImportResult::Cancelled => None,
    }
}
