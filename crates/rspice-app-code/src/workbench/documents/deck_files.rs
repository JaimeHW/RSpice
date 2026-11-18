//! The decks a session has open.
//!
//! A deck is a file. The project keeps where it is
//! ([`ProjectDeck`](crate::state::ProjectDeck)); the
//! session keeps its text while it is open, read when it opens and written
//! back, in its own encoding and with its own line separators, when it is
//! saved. An include the deck reads is a file too, opened on its own tab
//! and saved the same way.
//!
//! Where there is no file system, the project keeps a deck's text
//! ([`StoredDeck`](crate::state::StoredDeck)) and the text of the files its
//! include lines read. An edit to one of those is the project's at once,
//! and saving the project keeps it.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

use crate::state::{DeckId, NetlistDocumentId, StoredDeck, StoredDeckFile, TextEncoding};
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, NetlistRoot, NetlistTab, OwnedNetlistReplacement,
};
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
    /// What the file on disk became while the tab held edits, until the
    /// reader reloads it, keeps theirs or compares the two.
    pub disk_change: Option<Box<DiskChange>>,
}

/// A file as it stands on disk after something else changed it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskChange {
    pub text: String,
    pub encoding: TextEncoding,
    pub line_ending: LineEnding,
    pub stamp: DiskStamp,
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
            disk_change: None,
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
            disk_change: None,
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
    /// For a deck the project keeps as text, the files its include lines
    /// read, as the project keeps them; `None` for a deck that is a file.
    pub stored_files: Option<Vec<StoredDeckFile>>,
}

impl OpenDeck {
    /// The deck `id` the project keeps as `deck`. Its file is named by the
    /// deck's name and read from nowhere.
    #[must_use]
    pub fn from_stored(id: DeckId, deck: &StoredDeck) -> Self {
        let (text, line_ending) = LineEnding::read(deck.text.clone());
        Self {
            id,
            root: OpenFile {
                path: PathBuf::from(&deck.name),
                text,
                encoding: deck.encoding,
                line_ending,
                disk: None,
                dirty: false,
                disk_change: None,
            },
            includes: Vec::new(),
            stored_files: Some(deck.files.clone()),
        }
    }

    /// Whether the project keeps the deck as text rather than as a file.
    #[must_use]
    pub const fn is_stored(&self) -> bool {
        self.stored_files.is_some()
    }

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
            stored_files: None,
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

/// What a look at the disk found for one open file.
#[cfg(not(target_arch = "wasm32"))]
enum OnDisk {
    /// What the session last read or wrote.
    Same,
    /// The same text, written again: the stamp to keep from now on.
    Touched(DiskStamp),
    /// Other text.
    Changed(DiskChange),
    /// No file there, or none that reads.
    Gone,
}

/// Look at the file on disk: same length and time means unchanged, else its
/// text decides.
#[cfg(not(target_arch = "wasm32"))]
fn look_at_disk(file: &OpenFile) -> OnDisk {
    let Some(stamp) = &file.disk else {
        return OnDisk::Same;
    };
    let Ok(metadata) = std::fs::metadata(&file.path) else {
        return OnDisk::Gone;
    };
    if metadata.len() == stamp.len && metadata.modified().ok() == stamp.modified {
        return OnDisk::Same;
    }
    let Ok(read) = read_file(&file.path) else {
        return OnDisk::Gone;
    };
    let Some(now) = read.disk else {
        return OnDisk::Gone;
    };
    if now.digest == stamp.digest {
        return OnDisk::Touched(now);
    }
    OnDisk::Changed(DiskChange {
        text: read.text,
        encoding: read.encoding,
        line_ending: read.line_ending,
        stamp: now,
    })
}

/// The open deck's files, the deck first, each with the include key it goes
/// by; none for a deck the project keeps as text.
#[cfg(not(target_arch = "wasm32"))]
fn deck_file_keys(netlist: &NetlistDocumentState) -> Vec<Option<String>> {
    netlist
        .open_deck
        .as_ref()
        .filter(|deck| !deck.is_stored())
        .map(|deck| {
            std::iter::once(None)
                .chain(deck.includes.iter().map(|file| Some(file.key())))
                .collect()
        })
        .unwrap_or_default()
}

/// Look at the disk for every file of the open deck. A file changed on disk
/// whose tab holds no edits is read again, as an edit Undo takes back; one
/// whose tab holds edits keeps them and its tab shows the change. The
/// notices to show: one for each file read again.
#[cfg(not(target_arch = "wasm32"))]
pub fn check_disk(model: &mut Model, netlist: &mut NetlistDocumentState) -> Vec<String> {
    let mut notices = Vec::new();
    for include in deck_file_keys(netlist) {
        let Some(file) = netlist
            .open_deck
            .as_mut()
            .and_then(|deck| deck.file_mut(include.as_deref()))
        else {
            continue;
        };
        match look_at_disk(file) {
            OnDisk::Same | OnDisk::Gone => {}
            OnDisk::Touched(stamp) => file.disk = Some(stamp),
            OnDisk::Changed(change) if file.dirty => file.disk_change = Some(Box::new(change)),
            OnDisk::Changed(change) => {
                let name = file.name();
                take_disk_text(model, netlist, include.as_deref(), change);
                notices.push(format!("Reloaded {name}: changed on disk."));
            }
        }
    }
    notices
}

/// Where there is no file system, nothing on disk changes.
#[cfg(target_arch = "wasm32")]
pub fn check_disk(_model: &mut Model, _netlist: &mut NetlistDocumentState) -> Vec<String> {
    Vec::new()
}

/// Put what is on disk in the tab of the file `include` names (the deck for
/// `None`), as an edit Undo takes back; the file then holds no edits.
fn take_disk_text(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    include: Option<&str>,
    change: DiskChange,
) {
    let Some(original) = netlist
        .open_deck
        .as_ref()
        .and_then(|deck| deck.file(include))
        .map(|file| file.text.clone())
    else {
        return;
    };
    let replacement = match include {
        None => OwnedNetlistReplacement::root(&original, change.text.clone(), 1),
        Some(key) => OwnedNetlistReplacement::dependency(key, &original, change.text.clone(), 1),
    };
    let _ = crate::workbench::documents::netlist_document::replace_deck_files(
        model,
        netlist,
        vec![replacement],
    );
    if let Some(file) = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))
    {
        file.encoding = change.encoding;
        file.line_ending = change.line_ending;
        file.disk = Some(change.stamp);
        file.dirty = false;
        file.disk_change = None;
    }
}

/// Reload: drop the edits on the tab of the file `include` names (the deck
/// for `None`) for what is on disk.
pub fn reload_from_disk(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    include: Option<&str>,
) -> Result<(), String> {
    let change = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))
        .and_then(|file| file.disk_change.take())
        .ok_or_else(|| "Nothing on disk is newer than the tab.".to_owned())?;
    take_disk_text(model, netlist, include, *change);
    Ok(())
}

/// Keep mine: the tab keeps its edits, and the next Save writes them over
/// what is on disk.
pub fn keep_mine(netlist: &mut NetlistDocumentState, include: Option<&str>) {
    if let Some(file) = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))
        && let Some(change) = file.disk_change.take()
    {
        file.disk = Some(change.stamp);
    }
}

/// Compare: the file on disk against the tab's text, side by side in the
/// comparison view, the tab's edits kept.
pub fn compare_with_disk(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    include: Option<&str>,
) -> Result<(), String> {
    let file = netlist
        .open_deck
        .as_ref()
        .and_then(|deck| deck.file(include))
        .ok_or_else(|| "The file is no longer open.".to_owned())?;
    let change = file
        .disk_change
        .as_deref()
        .ok_or_else(|| "Nothing on disk is newer than the tab.".to_owned())?;
    let name = file.name();
    let deck = netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.id)
        .ok_or_else(|| "The file is no longer open.".to_owned())?;
    let disk = format!("{name} on disk");
    let yours = format!("{name} in RSpice");
    let diff = similar::TextDiff::from_lines(change.text.as_str(), file.text.as_str())
        .unified_diff()
        .context_radius(3)
        .header(&disk, &yours)
        .to_string();
    netlist.generated_diff_source = diff;
    // Closing the view goes back to the tab it was opened from.
    if !netlist.front.is_comparison() && !netlist.front.is_run_input() {
        netlist.comparison_return = netlist.front.clone();
    }
    netlist.comparison_sides = Some((
        crate::workbench::documents::netlist_document::CompareSide::DiskFile(deck),
        crate::workbench::documents::netlist_document::CompareSide::Deck(deck),
    ));
    netlist.comparison_include = include.map(str::to_owned);
    crate::workbench::documents::netlist_document::open_netlist_comparison(model, netlist);
    Ok(())
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
    if file.disk_change.is_none() {
        match look_at_disk(file) {
            OnDisk::Changed(change) => file.disk_change = Some(Box::new(change)),
            OnDisk::Touched(stamp) => file.disk = Some(stamp),
            OnDisk::Same | OnDisk::Gone => {}
        }
    }
    if file.disk_change.is_some() {
        return Err(format!(
            "{name} changed on disk since you opened it, so it was not saved: reload it, \
             keep yours or compare the two."
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

/// Save As: write the open deck to `path`, in its own encoding and line
/// endings, and read it from there from now on. The project lists the new
/// file; the old one stays as it was. A deck the project keeps as text is
/// saved as files: the deck at `path` and each file kept with it where the
/// deck reads it, and the project keeps the deck as that file from then on.
/// The new file's name.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_deck_as(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    path: &Path,
) -> Result<String, String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    if netlist.open_deck.as_ref().is_some_and(OpenDeck::is_stored) {
        return save_stored_deck_as_files(model, netlist, open_documents, runs, &path);
    }
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| "Open a deck to save it as another.".to_owned())?;
    let was = (deck.root.path.clone(), deck.root.disk.take());
    deck.root.path = path.clone();
    deck.root.disk_change = None;
    if let Err(error) = save_file(&mut deck.root) {
        (deck.root.path, deck.root.disk) = was;
        return Err(error);
    }
    let id = deck.id;
    let name = deck.root.name();
    let content = &mut *model.workspace.content;
    if let Some(entry) = content.decks.iter_mut().find(|entry| entry.id == id) {
        entry.source = crate::state::ProjectDeck::for_file(&path, folder.as_deref()).source;
        content.decks_dirty = true;
    }
    // The deck is read under its new name, and its includes from its new
    // folder.
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    Ok(name)
}

/// Save as files…: write the deck the project keeps as text to `path`, and
/// each file kept with it where a line of the deck reads it from there,
/// each in its own encoding; the project then keeps the deck as the file
/// at `path`, and the deck is read from there. A file of the deck already
/// on disk with other text is not overwritten, and nothing is written then.
#[cfg(not(target_arch = "wasm32"))]
fn save_stored_deck_as_files(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    path: &Path,
) -> Result<String, String> {
    let deck = netlist
        .open_deck
        .as_ref()
        .ok_or_else(|| "No deck is open.".to_owned())?;
    let mut writes = vec![(
        path.to_path_buf(),
        deck.root
            .encoding
            .encode(&deck.root.line_ending.write(&deck.root.text))?,
    )];
    for (at, key) in stored_file_places(model, deck, path)? {
        let Some(file) = deck
            .stored_files
            .iter()
            .flatten()
            .find(|file| file.path == key)
        else {
            continue;
        };
        let bytes = file.encoding.encode(&file.text)?;
        if let Some((_, other)) = writes.iter().find(|(there, _)| *there == at) {
            if *other == bytes {
                continue;
            }
            return Err(format!(
                "{} would be written twice, with other text: {} cannot be saved as files.",
                at.display(),
                deck.root.name()
            ));
        }
        if std::fs::read(&at).is_ok_and(|there| there != bytes) {
            return Err(format!(
                "{} is already there, with other text: choose another folder for {}.",
                at.display(),
                deck.root.name()
            ));
        }
        writes.push((at, bytes));
    }
    for (at, bytes) in &writes {
        if let Some(parent) = at.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("{} could not be made: {error}", parent.display()))?;
        }
        std::fs::write(at, bytes)
            .map_err(|error| format!("{} could not be written: {error}", at.display()))?;
    }
    let id = deck.id;
    let project_folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    let content = &mut *model.workspace.content;
    let entry = content
        .decks
        .iter_mut()
        .find(|entry| entry.id == id)
        .ok_or_else(|| "The deck is no longer in the project.".to_owned())?;
    entry.source = crate::state::ProjectDeck::for_file(path, project_folder.as_deref()).source;
    content.decks_dirty = true;
    let entry = entry.clone();
    let open = read_project_deck(&entry, project_folder.as_deref())?;
    let name = open.root.name();
    close_open_deck(model, netlist, open_documents);
    install_open_deck(model, netlist, open_documents, runs, open);
    Ok(name)
}

/// Where each file kept with the stored `deck` goes when the deck is saved
/// at `path`, with its path in the deck: where each line of the deck, or of
/// one of its files, reads it from the file the line is in, so the saved
/// deck reads the same files; a file no line reads goes beside the deck at
/// its path. A line that reaches above the top of the disk is refused.
#[cfg(not(target_arch = "wasm32"))]
fn stored_file_places(
    model: &Model,
    deck: &OpenDeck,
    path: &Path,
) -> Result<Vec<(PathBuf, String)>, String> {
    let files = deck.stored_files.as_deref().unwrap_or_default();
    let name = deck.root.name();
    let text = deck.root.line_ending.write(&deck.root.text).into_owned();
    let (root, sources) = rspice_design::deck::stored_deck_sources(&name, &text, files);
    let bundle = model
        .model_library_manager
        .seal_execution_sources()
        .and_then(|sealed| sealed.relative_bundle_with(sources.clone()))
        .or_else(|_| {
            rspice_core::netlist::SealedSourceBundle::try_new_relative(sources)
                .map_err(|error| error.to_string())
        })?;
    let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(&root, bundle);
    // A line the deck cannot read stops the reading; the files after it go
    // beside the deck.
    let _ = processor.expand_content_with_abort(&text, &root, &rspice_core::abort_signal::NoAbort);
    let root_key = rspice_design::deck::stored_file_path(&root);
    let mut read_at = vec![(root_key, path.to_path_buf())];
    let mut places = Vec::<(PathBuf, String)>::new();
    for dependency in processor.resolved_dependencies() {
        let key = rspice_design::deck::stored_file_path(dependency.resolved_path());
        if !files.iter().any(|file| file.path == key) {
            continue;
        }
        let owner = rspice_design::deck::stored_file_path(dependency.owner_path());
        let owners = read_at
            .iter()
            .filter(|(at_key, _)| *at_key == owner)
            .map(|(_, at)| at.clone())
            .collect::<Vec<_>>();
        let requested = dependency.requested_path().trim_matches(['"', '\'']);
        for owner_at in owners {
            let folder = owner_at.parent().unwrap_or(Path::new("."));
            let at = disk_join(folder, requested).ok_or_else(|| {
                format!(
                    "Line {} of {owner} reads {requested}, above the top of the disk from {}: \
                     save {name} in a folder further down.",
                    dependency.directive_line(),
                    folder.display()
                )
            })?;
            let place = (at.clone(), key.clone());
            if !places.contains(&place) {
                places.push(place);
                read_at.push((key.clone(), at));
            }
        }
    }
    let folder = path.parent().unwrap_or(Path::new("."));
    for file in files {
        if !places.iter().any(|(_, key)| *key == file.path) {
            places.push((
                folder.join(file.path.split('/').collect::<PathBuf>()),
                file.path.clone(),
            ));
        }
    }
    Ok(places)
}

/// `requested`, a path a line of a deck names, from `folder` on disk: `..`
/// goes up a folder, and none goes above the top of the disk.
#[cfg(not(target_arch = "wasm32"))]
fn disk_join(folder: &Path, requested: &str) -> Option<PathBuf> {
    let mut at = folder.to_path_buf();
    for part in requested.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." => {
                if !at.pop() {
                    return None;
                }
            }
            part => at.push(part),
        }
    }
    Some(at)
}

/// Export deck…: the name and bytes of the open deck the project keeps as
/// text, alone in its own encoding, or with `with_files` with the files
/// kept with it as one ZIP named after it.
pub fn export_stored_deck(
    model: &Model,
    netlist: &NetlistDocumentState,
    with_files: bool,
) -> Result<(String, Vec<u8>), String> {
    let id = netlist
        .open_deck
        .as_ref()
        .filter(|deck| deck.is_stored())
        .map(|deck| deck.id)
        .ok_or_else(|| "Open a deck kept in the project to export it.".to_owned())?;
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .and_then(crate::state::ProjectDeck::stored_deck)
        .ok_or_else(|| "The deck is no longer in the project.".to_owned())?;
    if !with_files {
        return Ok((deck.name.clone(), deck.encoding.encode(&deck.text)?));
    }
    let stem = deck
        .name
        .rsplit_once('.')
        .map_or(deck.name.as_str(), |(stem, _)| stem);
    Ok((
        format!("{stem}.zip"),
        crate::workbench::workflows::deck_import::export_zip(deck)?,
    ))
}

/// The text of a deck saved from the generated netlist `source`: a first
/// line naming the design and when, then the netlist as generated.
#[must_use]
pub fn deck_from_generated(source: &str, design: &str, when: &str) -> String {
    let mut deck = format!("* {design} netlist, saved as a deck {when}\n");
    deck.push_str(source);
    if !deck.ends_with('\n') {
        deck.push('\n');
    }
    deck
}

/// Before a run of the open deck: look at the disk, then save each of its
/// files that holds edits, so the run reads what its tabs show. The notices
/// to show, or why the run waits.
pub fn sync_before_run(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
) -> Result<Vec<String>, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (model, netlist);
        Ok(Vec::new())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(missing) = missing_deck(model, netlist) {
            return Err(format!(
                "{} is not at {}.",
                missing.name,
                missing.path.display()
            ));
        }
        let notices = check_disk(model, netlist);
        let Some(deck) = netlist.open_deck.as_mut().filter(|deck| !deck.is_stored()) else {
            return Ok(notices);
        };
        if let Some(changed) = std::iter::once(&deck.root)
            .chain(&deck.includes)
            .find(|file| file.disk_change.is_some())
        {
            return Err(format!(
                "Run is waiting: {} changed on disk. Reload it, keep yours or compare the two.",
                changed.name()
            ));
        }
        for file in std::iter::once(&mut deck.root).chain(deck.includes.iter_mut()) {
            if file.dirty {
                save_file(file)?;
            }
        }
        Ok(notices)
    }
}

/// How many of the open deck's files hold edits a run saves first.
#[must_use]
pub fn unsaved_before_run(netlist: &NetlistDocumentState) -> usize {
    netlist
        .open_deck
        .as_ref()
        .filter(|deck| !deck.is_stored())
        .map_or(0, |deck| {
            std::iter::once(&deck.root)
                .chain(&deck.includes)
                .filter(|file| file.dirty)
                .count()
        })
}

/// A deck of the project whose file is not where the project says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingDeck {
    pub id: DeckId,
    pub name: String,
    pub path: PathBuf,
}

/// The deck the page could not open because its file is not where the
/// project says, when that is why no deck is open.
#[must_use]
pub fn missing_deck(model: &Model, netlist: &NetlistDocumentState) -> Option<MissingDeck> {
    if netlist.open_deck.is_some() {
        return None;
    }
    let id = netlist.unreadable_deck?;
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)?;
    let path = deck.file_path(model.workspace.content.project.data_root())?;
    // In the browser a file deck is not missing: it is on the desktop.
    if cfg!(target_arch = "wasm32") || path.exists() {
        return None;
    }
    Some(MissingDeck {
        id,
        name: deck.name(),
        path,
    })
}

/// The deck of the project the browser could not open because it is a
/// file on the desktop, when that is why no deck is open.
#[must_use]
pub fn desktop_deck(model: &Model, netlist: &NetlistDocumentState) -> Option<MissingDeck> {
    desktop_deck_in(model, netlist, cfg!(target_arch = "wasm32"))
}

/// [`desktop_deck`], where `browser` says whether there is no file system.
#[must_use]
pub fn desktop_deck_in(
    model: &Model,
    netlist: &NetlistDocumentState,
    browser: bool,
) -> Option<MissingDeck> {
    if !browser || netlist.open_deck.is_some() {
        return None;
    }
    let id = netlist.unreadable_deck?;
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)?;
    Some(MissingDeck {
        id,
        name: deck.name(),
        path: deck.file_path(model.workspace.content.project.data_root())?,
    })
}

/// Remove from project: the project no longer lists the deck `id`, and its
/// tabs close; its file stays where it is. A deck holding unsaved edits is
/// not removed.
pub fn remove_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    id: DeckId,
) -> Result<String, String> {
    let name = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .map(crate::state::ProjectDeck::name)
        .ok_or_else(|| "The deck is no longer in the project.".to_owned())?;
    if let Some(open) = netlist.open_deck.as_ref().filter(|deck| deck.id == id) {
        if let Some(unsaved) = open.unsaved() {
            return Err(format!(
                "Save {} before removing {name} from the project.",
                unsaved.name()
            ));
        }
        close_open_deck(model, netlist, open_documents);
    }
    let content = &mut *model.workspace.content;
    content.decks.retain(|deck| deck.id != id);
    content.decks_dirty = true;
    if netlist.unreadable_deck == Some(id) {
        netlist.unreadable_deck = None;
    }
    Ok(name)
}

/// Close the open deck and its tabs; the generated netlist comes to the
/// front when there is one.
fn close_open_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
) {
    let Some(previous) = netlist.open_deck.take() else {
        return;
    };
    let previous = previous.tab_id();
    open_documents.retain(|tab| match tab {
        WorkspaceDocumentId::NetlistOwned(id) => *id != previous,
        WorkspaceDocumentId::NetlistDependency { root, .. } => *root != previous,
        _ => true,
    });
    netlist.front = NetlistTab::Generated;
    netlist.run_the_plan();
    model.simulation.source.netlist_content = model.netlist_artifacts.generated_source.clone();
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
}

/// Locate: the deck `id` is read from `path` from now on, and opens.
#[cfg(not(target_arch = "wasm32"))]
pub fn locate_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    id: DeckId,
    path: &Path,
) -> Result<(), String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let root = read_file(&path)?;
    let folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    let content = &mut *model.workspace.content;
    let deck = content
        .decks
        .iter_mut()
        .find(|deck| deck.id == id)
        .ok_or_else(|| "The deck is no longer in the project.".to_owned())?;
    deck.source = crate::state::ProjectDeck::for_file(&path, folder.as_deref()).source;
    content.decks_dirty = true;
    install_open_deck(
        model,
        netlist,
        open_documents,
        runs,
        OpenDeck {
            id,
            root,
            includes: Vec::new(),
            stored_files: None,
        },
    );
    Ok(())
}

/// Save the file in front of the Netlist workspace: the deck, or the
/// include on its tab.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_shown_file(netlist: &mut NetlistDocumentState) -> Result<(), String> {
    if !netlist.front.shows_deck() {
        return Err("Only a deck or an include it reads is saved to its file.".to_owned());
    }
    let include = netlist.front.include().map(str::to_owned);
    let file = netlist
        .open_deck
        .as_mut()
        .filter(|deck| !deck.is_stored())
        .and_then(|deck| deck.file_mut(include.as_deref()))
        .ok_or_else(|| "The file is no longer open.".to_owned())?;
    save_file(file)
}

/// For a deck the project keeps as text, in front of the page with `shown`
/// the text of the file in front: where the engine reads that file, and
/// every file of the deck where the engine reads it, the one in front with
/// `shown`. `None` for a deck that is a file.
#[must_use]
pub fn stored_sources(
    netlist: &NetlistDocumentState,
    shown: &str,
) -> Option<(PathBuf, Vec<(PathBuf, String)>)> {
    let deck = netlist.open_deck.as_ref()?;
    let files = deck.stored_files.as_ref()?;
    let include = netlist.front.include();
    let root_text = match include {
        None => shown.to_owned(),
        Some(_) => deck.root.line_ending.write(&deck.root.text).into_owned(),
    };
    let files = files
        .iter()
        .map(|file| StoredDeckFile {
            text: if include == Some(file.path.as_str()) {
                shown.to_owned()
            } else {
                file.text.clone()
            },
            ..file.clone()
        })
        .collect::<Vec<_>>();
    let (root, sources) =
        rspice_design::deck::stored_deck_sources(&deck.root.name(), &root_text, &files);
    let at = include.map_or(root, rspice_design::deck::stored_file_identity);
    Some((at, sources))
}

/// Whether the file in front of the Netlist workspace is a file on disk,
/// which Save writes: a deck that is a file, or an include of one. A deck
/// the project keeps as text is saved with the project.
#[must_use]
pub fn shown_file_is_on_disk(netlist: &NetlistDocumentState) -> bool {
    netlist
        .open_deck
        .as_ref()
        .is_some_and(|deck| !deck.is_stored())
        && crate::workbench::documents::netlist_document::active_deck_file_in(netlist).is_some()
}

/// Put an edit to the file `include` names, or to the deck itself, of a
/// deck the project keeps as text into the project at once, with the file's
/// own line separators: the project then has unsaved changes, and nothing
/// waits for a Save of the file. A deck that is a file waits for Save.
pub fn keep_stored_edit(model: &mut Model, deck: &mut OpenDeck, include: Option<&str>) {
    let OpenDeck {
        id,
        root,
        includes,
        stored_files: Some(files),
    } = deck
    else {
        return;
    };
    let file = match include {
        None => root,
        Some(key) => match includes.iter_mut().find(|file| file.key() == key) {
            Some(file) => file,
            None => return,
        },
    };
    file.dirty = false;
    let text = file.line_ending.write(&file.text).into_owned();
    if let Some(key) = include
        && let Some(stored) = files.iter_mut().find(|stored| stored.path == key)
    {
        stored.text.clone_from(&text);
    }
    let content = &mut *model.workspace.content;
    let Some(stored) = content
        .decks
        .iter_mut()
        .find(|deck| deck.id == *id)
        .and_then(crate::state::ProjectDeck::stored_deck_mut)
    else {
        return;
    };
    match include {
        None => stored.text = text,
        Some(key) => {
            if let Some(stored) = stored.files.iter_mut().find(|stored| stored.path == key) {
                stored.text = text;
            }
        }
    }
    content.decks_dirty = true;
}

/// Add a new deck the project keeps as text, named `deck<N>.cir` after the
/// first number no deck has, and bring it to the front: what New deck makes
/// where there is no file system, a title line and `.end`, or `text`. Its
/// name, or why there is none: the deck open before it goes, so one with
/// unsaved edits stops this.
pub fn new_stored_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    text: Option<String>,
) -> Result<String, String> {
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(format!(
            "Save {} before opening another deck.",
            unsaved.name()
        ));
    }
    let content = &mut *model.workspace.content;
    let names = content
        .decks
        .iter()
        .map(crate::state::ProjectDeck::name)
        .collect::<Vec<_>>();
    let name = crate::state::new_stored_deck_name(names.iter().map(String::as_str));
    let mut stored = StoredDeck::new(&name);
    if let Some(text) = text {
        stored.text = text;
    }
    let deck = crate::state::ProjectDeck::stored(stored.clone());
    let open = OpenDeck::from_stored(deck.id, &stored);
    content.decks.push(deck);
    content.decks_dirty = true;
    install_open_deck(model, netlist, open_documents, runs, open);
    Ok(name)
}

/// Add `decks`, decks the project keeps as text, to the project, each under
/// a name no other deck of it has, and bring the first to the front. Their
/// names, or why none was added: the deck open before goes, so one with
/// unsaved edits stops this.
pub fn add_stored_decks(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    decks: Vec<StoredDeck>,
) -> Result<Vec<String>, String> {
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(format!(
            "Save {} before opening another deck.",
            unsaved.name()
        ));
    }
    let content = &mut *model.workspace.content;
    let mut names = content
        .decks
        .iter()
        .map(crate::state::ProjectDeck::name)
        .collect::<Vec<_>>();
    let mut added = Vec::with_capacity(decks.len());
    let mut first = None;
    for mut deck in decks {
        deck.name = unused_name(&deck.name, &names);
        names.push(deck.name.clone());
        let entry = crate::state::ProjectDeck::stored(deck.clone());
        first.get_or_insert_with(|| OpenDeck::from_stored(entry.id, &deck));
        added.push(deck.name);
        content.decks.push(entry);
    }
    let Some(open) = first else {
        return Err("No deck was found to add.".to_owned());
    };
    content.decks_dirty = true;
    install_open_deck(model, netlist, open_documents, runs, open);
    Ok(added)
}

/// `name`, or where a deck of `taken` has it, `name` with the first number
/// after its stem that none has: `amp.cir`, `amp_2.cir`.
fn unused_name(name: &str, taken: &[String]) -> String {
    let free = |candidate: &str| {
        !taken
            .iter()
            .any(|other| other.eq_ignore_ascii_case(candidate))
    };
    if free(name) {
        return name.to_owned();
    }
    let (stem, extension) = name
        .rsplit_once('.')
        .map_or((name, String::new()), |(stem, extension)| {
            (stem, format!(".{extension}"))
        });
    (2_u32..)
        .map(|number| format!("{stem}_{number}{extension}"))
        .find(|candidate| free(candidate))
        .unwrap_or_else(|| name.to_owned())
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
            let deck = crate::state::ProjectDeck::for_file(&path, folder.as_deref());
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
            stored_files: None,
        },
    );
    Ok(())
}

/// The project's deck `deck`, opened: its text as the project keeps it, or
/// its file read from disk. A file is not read where there is no file
/// system, and the refusal says where to open it.
fn read_project_deck(
    deck: &crate::state::ProjectDeck,
    project_folder: Option<&Path>,
) -> Result<OpenDeck, String> {
    if let Some(stored) = deck.stored_deck() {
        return Ok(OpenDeck::from_stored(deck.id, stored));
    }
    let path = deck.file_path(project_folder).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    {
        read_file(&path).map(|root| OpenDeck {
            id: deck.id,
            root,
            includes: Vec::new(),
            stored_files: None,
        })
    }
    #[cfg(target_arch = "wasm32")]
    Err(format!(
        "{} is a file on the desktop ({}). Open the project on the desktop to edit or run it.",
        deck.name(),
        path.display()
    ))
}

/// Bring the project's deck `id` to the front, reading it when it is not
/// the open one. The open deck's edits are never dropped for another; a
/// deck that cannot be read leaves the page saying why.
pub fn open_deck_by_id(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    id: DeckId,
) -> Result<(), String> {
    if netlist.open_deck.as_ref().is_some_and(|deck| deck.id == id) {
        show_deck(model, netlist);
        return Ok(());
    }
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(format!(
            "Save {} before opening another deck.",
            unsaved.name()
        ));
    }
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .ok_or_else(|| "The deck is no longer in the project.".to_owned())?;
    match read_project_deck(deck, model.workspace.content.project.data_root()) {
        Ok(open) => {
            install_open_deck(model, netlist, open_documents, runs, open);
            Ok(())
        }
        Err(error) => {
            close_open_deck(model, netlist, open_documents);
            netlist.unreadable_deck = Some(id);
            Err(error)
        }
    }
}

/// Open the project's first deck when the project has decks and none is
/// open, once per project: a deck that cannot be read is reported once.
pub fn open_project_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
) {
    if netlist.open_deck.is_some() {
        return;
    }
    let Some(deck) = model.workspace.content.decks.first() else {
        return;
    };
    if netlist.unreadable_deck == Some(deck.id) {
        return;
    }
    let id = deck.id;
    match read_project_deck(deck, model.workspace.content.project.data_root()) {
        Ok(open) => install_open_deck(model, netlist, open_documents, runs, open),
        Err(error) => {
            netlist.unreadable_deck = Some(id);
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
    // The deck in front is the deck Run runs, on its tab again.
    netlist.run_target = super::netlist_tabs::RunTarget::Deck(deck.id);
    netlist.deck_tab_closed = false;
    // Asking again for the deck already in front changes nothing: a fresh
    // revision would throw away the deck's settled check.
    if netlist.front == NetlistTab::Deck(deck.id)
        && model.simulation.source.netlist_content == deck.root.text
    {
        return true;
    }
    model.simulation.source.netlist_content = deck.root.text.clone();
    netlist.front = NetlistTab::Deck(deck.id);
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
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| "No deck is open.".to_owned())?;
    // A file of a deck kept as text is named by its path from the deck,
    // whether asked for so or by where the engine read it.
    let key = if deck.is_stored() {
        rspice_design::deck::stored_file_path(path)
    } else {
        path.to_string_lossy().into_owned()
    };
    let path = &if deck.is_stored() {
        PathBuf::from(&key)
    } else {
        path.to_path_buf()
    };
    if deck.include(&key).is_none() {
        // A deck the project keeps as text reads its includes from what the
        // project keeps beside it.
        if let Some(files) = &deck.stored_files {
            let stored = files
                .iter()
                .find(|file| file.path == key)
                .ok_or_else(|| format!("{} is not kept with this deck.", file_name(path)))?;
            let (text, line_ending) = LineEnding::read(stored.text.clone());
            deck.includes.push(OpenFile {
                path: path.to_path_buf(),
                text,
                encoding: stored.encoding,
                line_ending,
                disk: None,
                dirty: false,
                disk_change: None,
            });
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            deck.includes.push(read_file(path)?);
            #[cfg(target_arch = "wasm32")]
            return Err(format!("{} is not open.", file_name(path)));
        }
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
        NetlistRoot::Deck,
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
