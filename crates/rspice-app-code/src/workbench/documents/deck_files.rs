//! The decks a session has open.
//!
//! A deck is a file. The project keeps where it is ([`ProjectDeck`]); the
//! session keeps its text while it is open, read when it opens and written
//! back, in its own encoding and with its own line separators, when it is
//! saved. An include the deck reads is a file too, opened on its own tab
//! and saved the same way.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

use crate::state::{DeckId, NetlistDocumentId, ProjectDeck, TextEncoding};
use crate::workbench::documents::netlist_document::{ActiveNetlistDocument, NetlistDocumentState};
use crate::workbench::state::WorkspaceDocumentId;

/// The line separators a file was read with, which the inspector shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEnding {
    #[default]
    None,
    Lf,
    CrLf,
    Cr,
    Mixed,
}

impl LineEnding {
    /// The separators' name, as the inspector shows it.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Lf => "LF",
            Self::CrLf => "CRLF",
            Self::Cr => "CR",
            Self::Mixed => "Mixed",
        }
    }

    /// The separators `text` uses.
    #[must_use]
    pub fn detect(text: &str) -> Self {
        let bytes = text.as_bytes();
        let (mut lf, mut crlf, mut cr) = (0usize, 0usize, 0usize);
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    crlf += 1;
                    index += 1;
                }
                b'\r' => cr += 1,
                b'\n' => lf += 1,
                _ => {}
            }
            index += 1;
        }
        match (lf > 0, crlf > 0, cr > 0) {
            (false, false, false) => Self::None,
            (true, false, false) => Self::Lf,
            (false, true, false) => Self::CrLf,
            (false, false, true) => Self::Cr,
            _ => Self::Mixed,
        }
    }

    /// `text` as the editor holds it, and the separators it was written
    /// with: every line ends in `\n`, except in a file whose lines end in
    /// more than one way, which is kept exactly as it is.
    #[must_use]
    pub fn read(text: String) -> (String, Self) {
        let ending = Self::detect(&text);
        let text = match ending {
            Self::CrLf => text.replace("\r\n", "\n"),
            Self::Cr => text.replace('\r', "\n"),
            Self::None | Self::Lf | Self::Mixed => text,
        };
        (text, ending)
    }

    /// `text`, as the editor holds it, with these separators again: what is
    /// written to the file.
    #[must_use]
    pub fn write(self, text: &str) -> std::borrow::Cow<'_, str> {
        match self {
            Self::CrLf => text.replace('\n', "\r\n").into(),
            Self::Cr => text.replace('\n', "\r").into(),
            Self::None | Self::Lf | Self::Mixed => text.into(),
        }
    }
}

/// What a file on disk was when the session last read or wrote it, so a
/// save can tell that something else changed it meanwhile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskStamp {
    pub digest: crate::product::ContentDigest,
    pub modified: Option<std::time::SystemTime>,
    pub len: u64,
}

/// One open file: a deck or an include it reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFile {
    /// Where the file is.
    pub path: PathBuf,
    /// Its text, every line ending in `\n` (see [`LineEnding::read`]).
    pub text: String,
    /// The encoding it was read in and is written back in.
    pub encoding: TextEncoding,
    /// The separators it was read with.
    pub line_ending: LineEnding,
    /// What was last read from or written to disk.
    pub disk: Option<DiskStamp>,
    /// Whether the text holds edits not yet saved.
    pub dirty: bool,
}

impl OpenFile {
    /// The file at `path` whose content is `bytes`.
    pub fn from_bytes(path: PathBuf, bytes: &[u8]) -> Result<Self, String> {
        let (text, encoding) = TextEncoding::decode(bytes)
            .map_err(|error| format!("{} could not be read: {error}", file_name(&path)))?;
        let (text, line_ending) = LineEnding::read(text);
        Ok(Self {
            line_ending,
            path,
            text,
            encoding,
            disk: None,
            dirty: false,
        })
    }

    /// The file's name, as its tab shows it.
    #[must_use]
    pub fn name(&self) -> String {
        file_name(&self.path)
    }

    /// The identity the session names the file by: its path.
    #[must_use]
    pub fn key(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    /// A UTF-8 file of `text` at `path`, read from nowhere: what a test
    /// opens.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn in_memory(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        let text = text.into();
        Self {
            line_ending: LineEnding::detect(&text),
            path: path.into(),
            text,
            encoding: TextEncoding::Utf8,
            disk: None,
            dirty: false,
        }
    }

    /// Replace the text with `text`; whether that changed it.
    pub fn edit(&mut self, text: String) -> bool {
        if self.text == text {
            return false;
        }
        self.text = text;
        self.dirty = true;
        true
    }
}

/// The deck open in the Netlist workspace, with the include files opened
/// beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDeck {
    pub id: DeckId,
    pub root: OpenFile,
    pub includes: Vec<OpenFile>,
}

impl OpenDeck {
    /// The identity the deck's tabs go by.
    #[must_use]
    pub fn tab_id(&self) -> NetlistDocumentId {
        NetlistDocumentId::try_from_uuid(self.id.as_uuid()).unwrap_or_default()
    }

    /// The open include named `key`.
    #[must_use]
    pub fn include(&self, key: &str) -> Option<&OpenFile> {
        self.includes.iter().find(|file| file.key() == key)
    }

    /// The first of the deck's open files holding edits not yet saved.
    #[must_use]
    pub fn unsaved(&self) -> Option<&OpenFile> {
        std::iter::once(&self.root)
            .chain(&self.includes)
            .find(|file| file.dirty)
    }

    /// The deck itself with no `include`, else the open include named so.
    pub fn file_mut(&mut self, include: Option<&str>) -> Option<&mut OpenFile> {
        match include {
            None => Some(&mut self.root),
            Some(key) => self.includes.iter_mut().find(|file| file.key() == key),
        }
    }

    /// The deck itself with no `include`, else the open include named so.
    #[must_use]
    pub fn file(&self, include: Option<&str>) -> Option<&OpenFile> {
        match include {
            None => Some(&self.root),
            Some(key) => self.include(key),
        }
    }

    /// A deck of `text` at `path`, read from nowhere: what a test opens.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn in_memory(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        Self {
            id: DeckId::new(),
            root: OpenFile::in_memory(path, text),
            includes: Vec::new(),
        }
    }
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// Read the file at `path`.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_file(path: &Path) -> Result<OpenFile, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("{} could not be read: {error}", path.display()))?;
    let mut file = OpenFile::from_bytes(path.to_path_buf(), &bytes)?;
    file.disk = Some(disk_stamp(path, &file.text));
    Ok(file)
}

#[cfg(not(target_arch = "wasm32"))]
fn disk_stamp(path: &Path, text: &str) -> DiskStamp {
    let metadata = std::fs::metadata(path).ok();
    DiskStamp {
        digest: crate::state::content_digest(text),
        modified: metadata
            .as_ref()
            .and_then(|metadata| metadata.modified().ok()),
        len: metadata.map_or(0, |metadata| metadata.len()),
    }
}

/// Whether the file on disk is no longer what the session last read or
/// wrote: same length and time means unchanged, else its text decides.
#[cfg(not(target_arch = "wasm32"))]
fn changed_on_disk(file: &OpenFile) -> bool {
    let Some(stamp) = &file.disk else {
        return false;
    };
    let Ok(metadata) = std::fs::metadata(&file.path) else {
        return false;
    };
    if metadata.len() == stamp.len && metadata.modified().ok() == stamp.modified {
        return false;
    }
    std::fs::read(&file.path)
        .ok()
        .and_then(|bytes| TextEncoding::decode(&bytes).ok())
        .is_none_or(|(text, _)| {
            crate::state::content_digest(&LineEnding::read(text).0) != stamp.digest
        })
}

/// Write `file` to its path now: its text, with the separators it was read
/// with, in its encoding, through a temporary file beside it renamed over
/// it. A file something else changed since it was read is not overwritten.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_file(file: &mut OpenFile) -> Result<(), String> {
    let name = file.name();
    // A file that had one line and no separator takes the editor's.
    if file.line_ending == LineEnding::None {
        file.line_ending = LineEnding::detect(&file.text);
    }
    let bytes = file
        .encoding
        .encode(&file.line_ending.write(&file.text))
        .map_err(|error| format!("{name} was not saved: {error}"))?;
    if changed_on_disk(file) {
        return Err(format!(
            "{name} changed on disk since you opened it, so it was not saved."
        ));
    }
    let temporary = file.path.with_file_name(format!(".{name}.saving"));
    std::fs::write(&temporary, &bytes)
        .and_then(|()| std::fs::rename(&temporary, &file.path))
        .map_err(|error| {
            let _ = std::fs::remove_file(&temporary);
            format!("{name} was not saved: {error}")
        })?;
    file.disk = Some(disk_stamp(&file.path, &file.text));
    file.dirty = false;
    Ok(())
}

/// Save the file in front of the Netlist workspace: the deck, or the
/// include on its tab.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_shown_file(netlist: &mut NetlistDocumentState) -> Result<(), String> {
    if netlist.active_document != ActiveNetlistDocument::OwnedSource {
        return Err("Only a deck or an include it reads is saved to its file.".to_owned());
    }
    let include = netlist.active_dependency_identity.clone();
    let file = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include.as_deref()))
        .ok_or_else(|| "The file is no longer open.".to_owned())?;
    save_file(file)
}

/// Open the deck at `path`: add it to the project unless it is one of the
/// project's decks already, and bring it to the front. The deck open before
/// it goes, so one with unsaved edits stops this.
#[cfg(not(target_arch = "wasm32"))]
pub fn open_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    path: &Path,
) -> Result<(), String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    if let Some(open) = &netlist.open_deck {
        if open.root.path == path {
            show_deck(model, netlist);
            return Ok(());
        }
        if let Some(unsaved) = open.unsaved() {
            return Err(format!(
                "Save {} before opening another deck.",
                unsaved.name()
            ));
        }
    }
    let root = read_file(&path)?;
    let folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    let id = match model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.is_file(&path, folder.as_deref()))
    {
        Some(deck) => deck.id,
        None => {
            let deck = ProjectDeck::for_file(&path, folder.as_deref());
            let id = deck.id;
            let content = &mut *model.workspace.content;
            content.decks.push(deck);
            content.decks_dirty = true;
            id
        }
    };
    install_open_deck(
        model,
        netlist,
        open_documents,
        runs,
        OpenDeck {
            id,
            root,
            includes: Vec::new(),
        },
    );
    Ok(())
}

/// Open the project's first deck when the project has decks and none is
/// open, once per project: a deck that cannot be read is reported once.
#[cfg(not(target_arch = "wasm32"))]
pub fn open_project_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
) {
    if netlist.open_deck.is_some() {
        return;
    }
    let Some(deck) = model.workspace.content.decks.first().cloned() else {
        return;
    };
    if netlist.unreadable_deck == Some(deck.id) {
        return;
    }
    let path = deck.file_path(model.workspace.content.project.data_root());
    match read_file(&path) {
        Ok(root) => install_open_deck(
            model,
            netlist,
            open_documents,
            runs,
            OpenDeck {
                id: deck.id,
                root,
                includes: Vec::new(),
            },
        ),
        Err(error) => {
            netlist.unreadable_deck = Some(deck.id);
            model.record(
                crate::diagnostics::LogSeverity::Error,
                crate::diagnostics::LogSource::Netlist,
                error,
            );
        }
    }
}

/// Make `deck` the open deck and put it in front: the tabs of the deck it
/// replaces close with it.
pub fn install_open_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    deck: OpenDeck,
) {
    if let Some(previous) = netlist.open_deck.take() {
        let previous = previous.tab_id();
        open_documents.retain(|tab| match tab {
            WorkspaceDocumentId::NetlistOwned(id) => *id != previous,
            WorkspaceDocumentId::NetlistDependency { root, .. } => *root != previous,
            _ => true,
        });
    }
    netlist.unreadable_deck = None;
    netlist.open_deck = Some(deck);
    show_deck(model, netlist);
    runs.manual_run.edited_lines.clear();
}

/// Bring the open deck to the front of the editor; whether there is one.
pub fn show_deck(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    let Some(deck) = netlist.open_deck.as_ref() else {
        return false;
    };
    // Asking again for the deck already in front changes nothing: a fresh
    // revision would throw away the deck's settled check.
    if netlist.active_document == ActiveNetlistDocument::OwnedSource
        && netlist.active_dependency_identity.is_none()
        && model.simulation.source.netlist_content == deck.root.text
    {
        return true;
    }
    model.simulation.source.netlist_content = deck.root.text.clone();
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document_initialized = true;
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Bring the include file at `path` to the front on its own tab, reading it
/// first when it is not open yet.
pub fn show_include(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    path: &Path,
) -> Result<(), String> {
    let key = path.to_string_lossy().into_owned();
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| "No deck is open.".to_owned())?;
    if deck.include(&key).is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        deck.includes.push(read_file(path)?);
        #[cfg(target_arch = "wasm32")]
        return Err(format!("{} is not open.", file_name(path)));
    }
    let root = deck.tab_id();
    let text = deck
        .include(&key)
        .map(|file| file.text.clone())
        .unwrap_or_default();
    crate::workbench::documents::netlist_document::put_dependency_in_front(
        model,
        netlist,
        open_documents,
        runs,
        ActiveNetlistDocument::OwnedSource,
        root,
        &key,
        text,
    )
}

/// The file an include line of the open deck names as `requested`: beside
/// the deck, else in the project's include search chain, the first one that
/// exists.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn resolve_include(model: &Model, deck: &OpenDeck, requested: &str) -> Option<PathBuf> {
    let requested = Path::new(requested.trim_matches(|c| c == '"' || c == '\''));
    if requested.is_absolute() {
        return requested.is_file().then(|| requested.to_path_buf());
    }
    deck.root
        .path
        .parent()
        .map(Path::to_path_buf)
        .into_iter()
        .chain(model.include_search_chain().directories())
        .map(|directory| directory.join(requested))
        .find(|candidate| candidate.is_file())
}
