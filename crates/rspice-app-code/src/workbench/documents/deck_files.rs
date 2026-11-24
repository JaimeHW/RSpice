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
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, mpsc};

use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

use crate::state::{
    DeckId, DeckText, NetlistDocumentId, RevisedText, StoredDeck, StoredDeckFile, TextEncoding,
};
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, NetlistRoot, NetlistTab, OwnedNetlistReplacement,
};
use crate::workbench::state::WorkspaceDocumentId;
use crate::workbench::{MessageCatalog, MessageId};

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
    pub fn label(self, messages: MessageCatalog) -> String {
        messages.text(match self {
            Self::None => MessageId::SourceLineEndingsNone,
            Self::Lf => MessageId::SourceLineEndingsLf,
            Self::CrLf => MessageId::SourceLineEndingsCrLf,
            Self::Cr => MessageId::SourceLineEndingsCr,
            Self::Mixed => MessageId::SourceLineEndingsMixedAny,
        })
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
    pub text: RevisedText,
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
    pub fn from_bytes(
        path: PathBuf,
        bytes: &[u8],
        messages: MessageCatalog,
    ) -> Result<Self, String> {
        let (text, encoding) = TextEncoding::decode(bytes)
            .map_err(|error| unreadable(messages, &file_name(&path), &error))?;
        let (text, line_ending) = LineEnding::read(text);
        Ok(Self {
            line_ending,
            path,
            text: RevisedText::new(text),
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
            text: RevisedText::new(text),
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
        self.text.set(text);
        self.dirty = true;
        true
    }

    /// Hold `text`, shared rather than copied; whether that changed it.
    /// Whichever way, the file and `text` then share one revision.
    pub fn edit_to(&mut self, text: &RevisedText) -> bool {
        let changed = !self.text.same_text(text);
        self.hold(text, changed)
    }

    /// Hold the text an editor left after an edit, shared rather than
    /// copied; whether that changed it. The editor gives a text a revision
    /// of its own only when it changes it, so the two revisions say so
    /// without the two texts being compared, as an edit that keeps the
    /// length, typing over a selection, would otherwise compare them.
    pub fn take_typed(&mut self, text: &RevisedText) -> bool {
        let changed = self.text.revision() != text.revision();
        self.hold(text, changed)
    }

    fn hold(&mut self, text: &RevisedText, changed: bool) -> bool {
        self.text = text.clone();
        self.dirty |= changed;
        changed
    }

    /// The file's text as the project keeps it, with the file's own
    /// separators: shared with the file, not copied.
    fn kept_text(&self) -> DeckText {
        DeckText::new(EditedText {
            text: self.text.clone(),
            line_ending: self.line_ending,
            written: OnceLock::new(),
        })
    }
}

/// The text of a file of a deck the project keeps, as the editor left it:
/// the editor's text, shared, and the separators the file is written with.
/// It is put in one piece with those separators only when something reads
/// it so, a save of the project, an export or a run, so a key typed in the
/// deck copies nothing.
struct EditedText {
    text: RevisedText,
    line_ending: LineEnding,
    written: OnceLock<String>,
}

impl crate::state::DeckTextBody for EditedText {
    fn text(&self) -> &str {
        match self.line_ending {
            LineEnding::CrLf | LineEnding::Cr => self
                .written
                .get_or_init(|| self.line_ending.write(&self.text).into_owned()),
            LineEnding::None | LineEnding::Lf | LineEnding::Mixed => self.text.as_str(),
        }
    }
}

/// A text the project keeps as the editor holds it, with the separators it
/// was written with: the editor's own text when the editor gave it, else
/// read from it.
fn editor_text(text: &DeckText) -> (RevisedText, LineEnding) {
    match text.body::<EditedText>() {
        Some(edited) => (edited.text.clone(), edited.line_ending),
        None => {
            let (text, line_ending) = LineEnding::read(text.as_str().to_owned());
            (RevisedText::new(text), line_ending)
        }
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
        let (text, line_ending) = editor_text(&deck.text);
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
        deck_tab_id(self.id)
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

/// Why the file `name` was not read.
fn unreadable(messages: MessageCatalog, name: &str, error: &dyn std::fmt::Display) -> String {
    messages.format(
        MessageId::NetlistFileUnreadable,
        &[("name", name), ("error", &error.to_string())],
    )
}

/// Why the file `name` was not saved.
#[cfg(not(target_arch = "wasm32"))]
fn not_saved(messages: MessageCatalog, name: &str, error: &dyn std::fmt::Display) -> String {
    messages.format(
        MessageId::NetlistFileNotSaved,
        &[("name", name), ("error", &error.to_string())],
    )
}

/// The file `include` names (the deck for `None`) is not open.
fn not_open(messages: MessageCatalog, include: Option<&str>) -> String {
    match include {
        Some(key) => messages.format(
            MessageId::NetlistIncludeNotOpen,
            &[("file", &file_name(Path::new(key)))],
        ),
        None => messages.text(MessageId::NetlistNoDeckOpen),
    }
}

/// `file` holds no change on disk to reload or compare.
fn nothing_newer(messages: MessageCatalog, file: &OpenFile) -> String {
    messages.format(
        MessageId::NetlistDiskNothingNewer,
        &[("name", &file.name())],
    )
}

/// Read the file at `path`.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_file(path: &Path, messages: MessageCatalog) -> Result<OpenFile, String> {
    if let Some(refused) = super::deck_memory::refusal_before_reading(path, messages) {
        return Err(refused);
    }
    let bytes = std::fs::read(path)
        .map_err(|error| unreadable(messages, &path.display().to_string(), &error))?;
    let mut file = OpenFile::from_bytes(path.to_path_buf(), &bytes, messages)?;
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
#[derive(Debug)]
enum OnDisk {
    /// What the session last read or wrote.
    Same,
    /// The same text, written again: the stamp to keep from now on.
    Touched(DiskStamp),
    /// Other text.
    Changed(DiskChange),
    /// A file too large for this machine to read, as an open would refuse
    /// it: why, in the words the open says it in.
    Refused(String),
    /// No file there, or none that reads.
    Gone,
}

/// What the file's length and time on disk say of it.
#[cfg(not(target_arch = "wasm32"))]
enum ByMetadata<'a> {
    /// Enough: the file is as last read or written, or not there.
    Settled(OnDisk),
    /// They moved since the stamp was taken: the file's text decides.
    Moved(&'a DiskStamp),
}

/// Look at the file's length and time on disk: the same as when it was
/// last read or written means unchanged. Nothing of the file is read.
#[cfg(not(target_arch = "wasm32"))]
fn by_metadata(file: &OpenFile) -> ByMetadata<'_> {
    let Some(stamp) = &file.disk else {
        return ByMetadata::Settled(OnDisk::Same);
    };
    let Ok(metadata) = std::fs::metadata(&file.path) else {
        return ByMetadata::Settled(OnDisk::Gone);
    };
    if metadata.len() == stamp.len && metadata.modified().ok() == stamp.modified {
        return ByMetadata::Settled(OnDisk::Same);
    }
    ByMetadata::Moved(stamp)
}

/// Read the file at `path` and tell its text against `since`, what was
/// last read from or written to it. A file grown past what this machine can
/// hold is not read, as an open would not read it.
#[cfg(not(target_arch = "wasm32"))]
fn read_against(path: &Path, since: &DiskStamp, messages: MessageCatalog) -> OnDisk {
    if let Some(refused) = super::deck_memory::refusal_before_reading(path, messages) {
        return OnDisk::Refused(refused);
    }
    let Ok(read) = read_file(path, messages) else {
        return OnDisk::Gone;
    };
    let Some(now) = read.disk else {
        return OnDisk::Gone;
    };
    if now.digest == since.digest {
        return OnDisk::Touched(now);
    }
    OnDisk::Changed(DiskChange {
        text: read.text.into_string(),
        encoding: read.encoding,
        line_ending: read.line_ending,
        stamp: now,
    })
}

/// Look at the file on disk: same length and time means unchanged, else its
/// text decides.
#[cfg(not(target_arch = "wasm32"))]
fn look_at_disk(file: &OpenFile, messages: MessageCatalog) -> OnDisk {
    match by_metadata(file) {
        ByMetadata::Settled(found) => found,
        ByMetadata::Moved(since) => read_against(&file.path, since, messages),
    }
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

/// The looks at the disk under way: each a file of the open deck whose
/// length or time moved since it was read, read away from the frame to
/// tell whether its text changed, until the frame loop takes the answer in
/// ([`land_disk_looks`]).
#[derive(Debug, Clone, Default)]
pub struct DiskLooks {
    #[cfg(not(target_arch = "wasm32"))]
    looks: Vec<DiskLook>,
}

impl DiskLooks {
    /// Whether no look at the disk is under way.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.looks.is_empty()
        }
        #[cfg(target_arch = "wasm32")]
        true
    }
}

/// One file's look at the disk, away from the frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct DiskLook {
    /// The include the file is, or `None` for the deck.
    include: Option<String>,
    path: PathBuf,
    /// What was last read from or written to the file as the look started:
    /// an answer for a file read, written or reloaded since is not taken.
    since: DiskStamp,
    read: ReadAway<OnDisk>,
}

/// Look at the disk for every file of the open deck: by its length and time
/// on the frame, and a file whose length or time moved since it was read is
/// read to tell whether its text changed, away from the frame where decks
/// are read away from it, its answer taken in by a later frame
/// ([`land_disk_looks`]). A file changed on disk whose tab holds no edits
/// is read again, as an edit Undo takes back; one whose tab holds edits
/// keeps them and its tab shows the change. The notices to show now: one
/// for each file read again.
#[cfg(not(target_arch = "wasm32"))]
pub fn check_disk(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    looks: &mut DiskLooks,
    messages: crate::workbench::MessageCatalog,
) -> Vec<String> {
    let mut notices = Vec::new();
    for include in deck_file_keys(netlist) {
        let Some(file) = netlist
            .open_deck
            .as_ref()
            .and_then(|deck| deck.file(include.as_deref()))
        else {
            continue;
        };
        let ByMetadata::Moved(since) = by_metadata(file) else {
            continue;
        };
        // A file already being looked at is answered by that look.
        if looks.looks.iter().any(|look| look.path == file.path) {
            continue;
        }
        let (path, since) = (file.path.clone(), since.clone());
        let found = if reads_away() {
            let (from, stamp) = (path.clone(), since.clone());
            match ReadAway::start(move || read_against(&from, &stamp, messages)) {
                Ok(read) => {
                    looks.looks.push(DiskLook {
                        include,
                        path,
                        since,
                        read,
                    });
                    continue;
                }
                Err(_) => read_against(&path, &since, messages),
            }
        } else {
            read_against(&path, &since, messages)
        };
        notices.extend(take_look(
            model,
            netlist,
            include.as_deref(),
            found,
            messages,
        ));
    }
    notices
}

/// Where there is no file system, nothing on disk changes.
#[cfg(target_arch = "wasm32")]
pub fn check_disk(
    _model: &mut Model,
    _netlist: &mut NetlistDocumentState,
    _looks: &mut DiskLooks,
    _messages: crate::workbench::MessageCatalog,
) -> Vec<String> {
    Vec::new()
}

/// Take in the looks at the disk that have ended, as [`check_disk`] takes
/// what it finds: each for a file of the open deck still as it was when its
/// look started; a file read, written or reloaded since keeps what that
/// gave it. The notices to show: one for each file read again.
#[cfg(not(target_arch = "wasm32"))]
pub fn land_disk_looks(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    looks: &mut DiskLooks,
    messages: crate::workbench::MessageCatalog,
) -> Vec<String> {
    if looks.looks.is_empty() {
        return Vec::new();
    }
    let mut notices = Vec::new();
    for look in std::mem::take(&mut looks.looks) {
        let found = match look.read.landed(false) {
            Landed::Reading => {
                looks.looks.push(look);
                continue;
            }
            Landed::Read(found) => found,
            Landed::Lost => continue,
        };
        let current = netlist
            .open_deck
            .as_ref()
            .and_then(|deck| deck.file(look.include.as_deref()))
            .is_some_and(|file| file.path == look.path && file.disk.as_ref() == Some(&look.since));
        if current {
            notices.extend(take_look(
                model,
                netlist,
                look.include.as_deref(),
                found,
                messages,
            ));
        }
    }
    notices
}

/// Where there is no file system, no look at the disk is under way.
#[cfg(target_arch = "wasm32")]
pub fn land_disk_looks(
    _model: &mut Model,
    _netlist: &mut NetlistDocumentState,
    _looks: &mut DiskLooks,
    _messages: crate::workbench::MessageCatalog,
) -> Vec<String> {
    Vec::new()
}

/// Take what a look at the disk found for the file `include` names (the
/// deck for `None`): the stamp of the same text written again; a change on
/// disk read into a tab with no edits, or shown on one holding edits. The
/// notice to show for a file read again, or for one grown past what this
/// machine can read, whose tab keeps its text.
#[cfg(not(target_arch = "wasm32"))]
fn take_look(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    include: Option<&str>,
    found: OnDisk,
    messages: crate::workbench::MessageCatalog,
) -> Option<String> {
    let file = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))?;
    match found {
        OnDisk::Same | OnDisk::Gone => None,
        OnDisk::Refused(why) => Some(why),
        OnDisk::Touched(stamp) => {
            file.disk = Some(stamp);
            None
        }
        OnDisk::Changed(change) if file.dirty => {
            file.disk_change = Some(Box::new(change));
            None
        }
        OnDisk::Changed(change) => {
            let name = file.name();
            take_disk_text(model, netlist, include, change);
            Some(messages.format(MessageId::NetlistDeckReloadedChanged, &[("name", &name)]))
        }
    }
}

/// The open deck's file that changed on disk while its tab held edits, and
/// waits for the reader to reload it or keep theirs.
fn changed_under_edits(netlist: &NetlistDocumentState) -> Option<&OpenFile> {
    let deck = netlist.open_deck.as_ref()?;
    std::iter::once(&deck.root)
        .chain(&deck.includes)
        .find(|file| file.disk_change.is_some())
}

/// Whether a file of the open deck changed on disk while its tab held
/// edits, and waits for the reader to reload it or keep theirs.
#[must_use]
pub fn waits_for_answer(netlist: &NetlistDocumentState) -> bool {
    changed_under_edits(netlist).is_some()
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
    let DiskChange {
        text,
        encoding,
        line_ending,
        stamp,
    } = change;
    let replacement = match include {
        None => OwnedNetlistReplacement::root(&original, text, 1),
        Some(key) => OwnedNetlistReplacement::dependency(key, &original, text, 1),
    };
    let _ = crate::workbench::documents::netlist_document::apply_deck_replacements(
        model,
        netlist,
        vec![replacement],
    );
    if let Some(file) = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))
    {
        file.encoding = encoding;
        file.line_ending = line_ending;
        file.disk = Some(stamp);
        file.dirty = false;
        file.disk_change = None;
    }
    // The deck read again is weighed as an open weighs it: its language
    // features are off when its indexes need more memory than is free.
    if let Some(deck) = netlist.open_deck.as_ref() {
        netlist.language_off = super::deck_memory::LanguageOff::as_opened(deck);
    }
}

/// Reload: drop the edits on the tab of the file `include` names (the deck
/// for `None`) for what is on disk.
pub fn reload_from_disk(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    include: Option<&str>,
    messages: MessageCatalog,
) -> Result<(), String> {
    let file = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include))
        .ok_or_else(|| not_open(messages, include))?;
    let change = file
        .disk_change
        .take()
        .ok_or_else(|| nothing_newer(messages, file))?;
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
    messages: MessageCatalog,
) -> Result<(), String> {
    let file = netlist
        .open_deck
        .as_ref()
        .and_then(|deck| deck.file(include))
        .ok_or_else(|| not_open(messages, include))?;
    let change = file
        .disk_change
        .as_deref()
        .ok_or_else(|| nothing_newer(messages, file))?;
    let name = file.name();
    let deck = netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.id)
        .ok_or_else(|| not_open(messages, include))?;
    let disk = messages.format(MessageId::NetlistDiffOnDisk, &[("name", &name)]);
    let yours = messages.format(MessageId::NetlistDiffInRSpice, &[("name", &name)]);
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
/// it. A file something else changed since it was read is not overwritten,
/// nor one grown past what this machine can read to tell.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_file(file: &mut OpenFile, messages: MessageCatalog) -> Result<(), String> {
    let name = file.name();
    // A file that had one line and no separator takes the editor's.
    if file.line_ending == LineEnding::None {
        file.line_ending = LineEnding::detect(&file.text);
    }
    let bytes = file
        .encoding
        .encode(&file.line_ending.write(&file.text))
        .map_err(|error| not_saved(messages, &name, &error))?;
    if file.disk_change.is_none() {
        match look_at_disk(file, messages) {
            OnDisk::Changed(change) => file.disk_change = Some(Box::new(change)),
            OnDisk::Touched(stamp) => file.disk = Some(stamp),
            OnDisk::Refused(why) => return Err(why),
            OnDisk::Same | OnDisk::Gone => {}
        }
    }
    if file.disk_change.is_some() {
        return Err(messages.format(MessageId::NetlistSaveChangedOnDisk, &[("name", &name)]));
    }
    ask_to_save(&file.path);
    {
        let _files = hold_deck_files();
        write_through_temporary(&file.path, &name, &bytes, messages)?;
    }
    file.disk = Some(disk_stamp(&file.path, &file.text));
    file.dirty = false;
    Ok(())
}

/// Write `bytes` to the file `name` at `path` whole: into a temporary file
/// beside it, renamed over it.
#[cfg(not(target_arch = "wasm32"))]
fn write_through_temporary(
    path: &Path,
    name: &str,
    bytes: &[u8],
    messages: MessageCatalog,
) -> Result<(), String> {
    let temporary = path.with_file_name(format!(".{name}.saving"));
    std::fs::write(&temporary, bytes)
        .and_then(|()| std::fs::rename(&temporary, path))
        .map_err(|error| {
            let _ = std::fs::remove_file(&temporary);
            not_saved(messages, name, &error)
        })
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
    messages: MessageCatalog,
) -> Result<String, String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    if netlist.open_deck.as_ref().is_some_and(OpenDeck::is_stored) {
        return save_stored_deck_as_files(model, netlist, open_documents, runs, &path, messages);
    }
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| messages.text(MessageId::NetlistNoDeckOpen))?;
    let was = (deck.root.path.clone(), deck.root.disk.take());
    deck.root.path = path.clone();
    deck.root.disk_change = None;
    if let Err(error) = save_file(&mut deck.root, messages) {
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
    messages: MessageCatalog,
) -> Result<String, String> {
    let deck = netlist
        .open_deck
        .as_ref()
        .ok_or_else(|| messages.text(MessageId::NetlistNoDeckOpen))?;
    let mut writes = vec![(
        path.to_path_buf(),
        deck.root
            .encoding
            .encode(&deck.root.line_ending.write(&deck.root.text))?,
    )];
    for (at, key) in stored_file_places(model, deck, path, messages)? {
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
            return Err(messages.format(
                MessageId::NetlistSaveAsFilesTwice,
                &[
                    ("name", &deck.root.name()),
                    ("path", &at.display().to_string()),
                ],
            ));
        }
        if std::fs::read(&at).is_ok_and(|there| there != bytes) {
            return Err(messages.format(
                MessageId::NetlistSaveAsFilesTaken,
                &[
                    ("path", &at.display().to_string()),
                    ("name", &deck.root.name()),
                ],
            ));
        }
        writes.push((at, bytes));
    }
    for (at, bytes) in &writes {
        if let Some(parent) = at.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                messages.format(
                    MessageId::NetlistFolderNotCreated,
                    &[
                        ("path", &parent.display().to_string()),
                        ("error", &error.to_string()),
                    ],
                )
            })?;
        }
        std::fs::write(at, bytes).map_err(|error| {
            messages.format(
                MessageId::NetlistFileNotWritten,
                &[
                    ("path", &at.display().to_string()),
                    ("error", &error.to_string()),
                ],
            )
        })?;
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
        .ok_or_else(|| messages.text(MessageId::NetlistDeckNotInProject))?;
    entry.source = crate::state::ProjectDeck::for_file(path, project_folder.as_deref()).source;
    content.decks_dirty = true;
    let entry = entry.clone();
    let open = read_project_deck(&entry, project_folder.as_deref(), messages)?;
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
    messages: MessageCatalog,
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
                messages.format(
                    MessageId::NetlistSaveAsFilesAboveTop,
                    &[
                        ("line", &dependency.directive_line().to_string()),
                        ("file", &owner),
                        ("path", requested),
                        ("folder", &folder.display().to_string()),
                        ("name", &name),
                    ],
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

/// Where in `text` the zero-based `line` writes the path the engine reads as
/// `requested`: the path alone, inside any quotes.
#[must_use]
pub fn named_path_span(text: &str, line: usize, requested: &str) -> Option<Range<usize>> {
    let start = text
        .split_inclusive('\n')
        .take(line)
        .map(str::len)
        .sum::<usize>();
    let row = text.get(start..)?.split('\n').next()?;
    let mut index = 0;
    while index < row.len() {
        let rest = &row[index..];
        let Some(first) = rest.chars().next() else {
            break;
        };
        if first.is_ascii_whitespace() {
            index += 1;
            continue;
        }
        let (from, to, next) = if matches!(first, '"' | '\'') {
            let to = rest[1..].find(first).map_or(row.len(), |at| index + 1 + at);
            (index + 1, to, to + 1)
        } else {
            let to = rest
                .find(|character: char| character.is_ascii_whitespace())
                .map_or(row.len(), |at| index + at);
            (index, to, to)
        };
        if rspice_core::netlist::normalize_source_path_literal(&row[from..to])
            .is_ok_and(|path| path == requested)
        {
            return Some(start + from..start + to);
        }
        index = next;
    }
    None
}

/// The path in a deck kept as text that a file `owner`, a file of the deck
/// by its path in it, reads as `requested`: from the folder `owner` is in,
/// with `..` stopping at the deck's own folder as the deck reads it; or,
/// for a path `outside` the deck, the file by its name beside `owner`.
#[must_use]
pub fn kept_path(owner: &str, requested: &str, outside: bool) -> String {
    let mut parts = owner
        .rsplit_once('/')
        .map(|(folder, _)| folder.split('/').collect::<Vec<_>>())
        .unwrap_or_default();
    if outside {
        let name = requested.rsplit(['/', '\\']).next().unwrap_or(requested);
        parts.push(name.rsplit(':').next().unwrap_or(name));
    } else {
        for part in requested.split(['/', '\\']) {
            match part {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                part => parts.push(part),
            }
        }
    }
    parts.join("/")
}

/// Add file…: keep `text`, the file the reader picked, in `encoding`, with
/// the open deck the project keeps as text, under the path `request` asks
/// for, so the line that names it reads it; a line naming an absolute path
/// is rewritten to name the file by its name, an edit Undo takes back. The
/// deck is checked again. What was added, or why nothing was.
pub fn add_file_to_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    request: &crate::state::netlist_diagnostics::DeckFileRequest,
    text: String,
    encoding: TextEncoding,
    messages: crate::workbench::MessageCatalog,
) -> Result<String, String> {
    let deck = netlist
        .open_deck
        .as_ref()
        .filter(|deck| deck.is_stored())
        .ok_or_else(|| messages.text(MessageId::NetlistAddFileNeedsStoredDeck))?;
    let name = deck.root.name();
    let keep_as = request.keep_as.as_str();
    let said = |id| messages.format(id, &[("name", &name), ("path", keep_as)]);
    if keep_as.is_empty() || keep_as == name {
        return Err(said(MessageId::NetlistAddFileBadName));
    }
    let kept = deck
        .stored_files
        .iter()
        .flatten()
        .find(|file| file.path == keep_as);
    if kept.is_some_and(|file| file.text != text) {
        return Err(said(MessageId::NetlistAddFileClash));
    }
    let added = kept.is_none();
    if let Some(to) = &request.rewrite_to {
        rewrite_named_path(model, netlist, request, to, messages)?;
    }
    let Some(deck) = netlist.open_deck.as_mut() else {
        return Err(messages.text(MessageId::NetlistNoDeckOpen));
    };
    let file = StoredDeckFile {
        path: keep_as.to_owned(),
        text: text.into(),
        encoding,
    };
    if added {
        if let Some(files) = deck.stored_files.as_mut() {
            files.push(file.clone());
            files.sort_by(|left, right| left.path.cmp(&right.path));
        }
        let content = &mut *model.workspace.content;
        if let Some(stored) = content
            .decks
            .iter_mut()
            .find(|entry| entry.id == deck.id)
            .and_then(crate::state::ProjectDeck::stored_deck_mut)
        {
            stored.files.push(file);
            stored
                .files
                .sort_by(|left, right| left.path.cmp(&right.path));
        }
        content.decks_dirty = true;
    }
    // The deck reads the file it now keeps.
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    Ok(said(if added {
        MessageId::NetlistAddFileAdded
    } else {
        MessageId::NetlistAddFileAlreadyKept
    }))
}

/// Name the file `request` asks for as `to` on the line that names it,
/// through the editor where the file is open, so Undo takes it back, else
/// in the text the project keeps.
fn rewrite_named_path(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    request: &crate::state::netlist_diagnostics::DeckFileRequest,
    to: &str,
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    let deck = netlist
        .open_deck
        .as_ref()
        .ok_or_else(|| messages.text(MessageId::NetlistNoDeckOpen))?;
    let id = deck.id;
    let include = (request.owner != deck.root.name()).then_some(request.owner.as_str());
    let open = deck.file(include).map(|file| file.text.as_str().to_owned());
    let text = match &open {
        Some(text) => text.clone(),
        None => deck
            .stored_files
            .iter()
            .flatten()
            .find(|file| file.path == request.owner)
            .map(|file| file.text.as_str().to_owned())
            .ok_or_else(|| {
                messages.format(
                    MessageId::NetlistAddFileOwnerGone,
                    &[("file", &request.owner)],
                )
            })?,
    };
    let span = named_path_span(&text, request.line, &request.requested).ok_or_else(|| {
        messages.format(
            MessageId::NetlistAddFileLineMoved,
            &[
                ("line", &(request.line + 1).to_string()),
                ("file", &request.owner),
                ("path", &request.requested),
            ],
        )
    })?;
    let mut changed = text.clone();
    changed.replace_range(span, to);
    if open.is_some() {
        let edit = match include {
            None => crate::workbench::documents::netlist_document::OwnedNetlistReplacement::root(
                &text, changed, 1,
            ),
            Some(key) => {
                crate::workbench::documents::netlist_document::OwnedNetlistReplacement::dependency(
                    key, &text, changed, 1,
                )
            }
        };
        crate::workbench::documents::netlist_document::replace_deck_files(
            model,
            netlist,
            vec![edit],
            messages,
        )?;
        return Ok(());
    }
    if let Some(file) = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.stored_files.as_mut())
        .and_then(|files| files.iter_mut().find(|file| file.path == request.owner))
    {
        file.text = changed.clone().into();
    }
    let content = &mut *model.workspace.content;
    if let Some(file) = content
        .decks
        .iter_mut()
        .find(|entry| entry.id == id)
        .and_then(crate::state::ProjectDeck::stored_deck_mut)
        .and_then(|stored| {
            stored
                .files
                .iter_mut()
                .find(|file| file.path == request.owner)
        })
    {
        file.text = changed.into();
    }
    content.decks_dirty = true;
    Ok(())
}

/// Export deck…: the name and bytes of the open deck the project keeps as
/// text, alone in its own encoding, or with `with_files` with the files
/// kept with it as one ZIP named after it.
pub fn export_stored_deck(
    model: &Model,
    netlist: &NetlistDocumentState,
    with_files: bool,
    messages: MessageCatalog,
) -> Result<(String, Vec<u8>), String> {
    let id = netlist
        .open_deck
        .as_ref()
        .filter(|deck| deck.is_stored())
        .map(|deck| deck.id)
        .ok_or_else(|| messages.text(MessageId::NetlistExportNeedsStoredDeck))?;
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .and_then(crate::state::ProjectDeck::stored_deck)
        .ok_or_else(|| messages.text(MessageId::NetlistDeckNotInProject))?;
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

/// Before a run of the open deck: look at the disk ([`check_disk`]) when
/// `look`, and take each of its files that holds edits as it stands, to be
/// saved before the deck is prepared ([`save_edited_files`]), so the run
/// reads what its tabs show. Until that save is taken in
/// ([`land_saved_before_run`]) a file being saved is not looked at on disk.
/// What the look found, and what the run waits for on disk, when it waits,
/// with no file taken; or why it does not run.
pub fn sync_before_run(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    looks: &mut DiskLooks,
    look: bool,
    messages: crate::workbench::MessageCatalog,
) -> Result<BeforeRun, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (model, netlist, looks, look, messages);
        Ok(BeforeRun::default())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(missing) = missing_deck(model, netlist) {
            return Err(messages.format(
                MessageId::NetlistDeckMissing,
                &[
                    ("name", &missing.name),
                    ("path", &missing.path.display().to_string()),
                ],
            ));
        }
        let notices = if look {
            check_disk(model, netlist, looks, messages)
        } else {
            Vec::new()
        };
        let waits = if looks.is_empty() {
            changed_under_edits(netlist).map(|changed| {
                WaitsOnDisk::Answer(messages.format(
                    MessageId::NetlistRunWaitsOnDisk,
                    &[("name", &changed.name())],
                ))
            })
        } else {
            Some(WaitsOnDisk::Reading)
        };
        let Some(deck) = netlist
            .open_deck
            .as_mut()
            .filter(|deck| waits.is_none() && !deck.is_stored())
        else {
            return Ok(BeforeRun {
                notices,
                edited: Vec::new(),
                waits,
            });
        };
        let edited = std::iter::once((None, &mut deck.root))
            .chain(
                deck.includes
                    .iter_mut()
                    .map(|file| (Some(file.key()), file)),
            )
            .filter(|(_, file)| file.dirty)
            .map(|(include, file)| EditedFile {
                include,
                save: ask_to_save(&file.path),
                path: file.path.clone(),
                text: file.text.clone(),
                encoding: file.encoding,
                line_ending: file.line_ending,
                disk: file.disk.take(),
            })
            .collect();
        Ok(BeforeRun {
            notices,
            edited,
            waits: None,
        })
    }
}

/// What a run or check of the open deck finds before it starts: the notices
/// looking at the disk gave, and the files holding edits, as the press took
/// them, which are saved before the deck is prepared; or what it waits for
/// on disk first.
#[derive(Debug, Default)]
pub struct BeforeRun {
    pub notices: Vec<String>,
    pub edited: Vec<EditedFile>,
    /// What the run waits for on the deck's files on disk before the deck
    /// is prepared, when it waits; no file is taken then.
    pub waits: Option<WaitsOnDisk>,
}

/// What a run waits for on the deck's files on disk before the deck is
/// prepared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitsOnDisk {
    /// The read, away from the frame, of files whose length or time moved
    /// since they were read.
    Reading,
    /// The reader's answer for a file changed on disk while its tab held
    /// edits, to reload it or keep theirs: why the run waits, in words.
    Answer(String),
}

/// A file of the open deck holding edits, as a press took it: what saving it
/// before its deck is prepared writes, away from the frame.
#[derive(Debug, Clone)]
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub struct EditedFile {
    /// The include it is, or `None` for the deck.
    include: Option<String>,
    /// Which save of the file the press asked for ([`ask_to_save`]).
    save: u64,
    path: PathBuf,
    text: RevisedText,
    encoding: TextEncoding,
    line_ending: LineEnding,
    /// What was last read from or written to disk before the press: the
    /// file's stamp again should the save not write it.
    disk: Option<DiskStamp>,
}

/// What saving a deck's edited files came to: each file, written or not,
/// and why the save stopped, when it did. A failed write stops the save
/// there, as it stops a run.
#[derive(Debug, Default)]
pub struct SavedBeforeRun {
    files: Vec<(EditedFile, Option<(DiskStamp, LineEnding)>)>,
    refusal: Option<String>,
}

impl SavedBeforeRun {
    /// Why the save stopped, when it did.
    #[must_use]
    pub fn refusal(&self) -> Option<&str> {
        self.refusal.as_deref()
    }
}

/// One save of a deck's files at a time, and none while a run's preparation
/// reads them: a deck's files written away from the frame are written whole
/// before anything else writes or reads them.
#[cfg(not(target_arch = "wasm32"))]
static DECK_FILE_WRITES: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The saves asked of each deck file, counted as they are asked: a save a
/// press asked for writes only while no later save of the same file was
/// asked, so the text a press took never overwrites a newer one the reader
/// saved meanwhile.
#[cfg(not(target_arch = "wasm32"))]
static DECK_FILE_SAVES_ASKED: std::sync::Mutex<std::collections::BTreeMap<PathBuf, u64>> =
    std::sync::Mutex::new(std::collections::BTreeMap::new());

/// Ask to save the file at `path`: which save of it this is.
#[cfg(not(target_arch = "wasm32"))]
fn ask_to_save(path: &Path) -> u64 {
    let mut asked = DECK_FILE_SAVES_ASKED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let count = asked.entry(path.to_path_buf()).or_default();
    *count += 1;
    *count
}

/// Whether `save`, a save of the file at `path`, is the last asked of it.
#[cfg(not(target_arch = "wasm32"))]
fn last_save_asked(path: &Path, save: u64) -> bool {
    DECK_FILE_SAVES_ASKED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(path)
        == Some(&save)
}

/// Hold the deck's files for writing, or for reading as a run reads them,
/// until the guard drops.
#[cfg(not(target_arch = "wasm32"))]
fn hold_deck_files() -> std::sync::MutexGuard<'static, ()> {
    DECK_FILE_WRITES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Save `edited`, the files of a deck a press took with edits, in order,
/// away from the frame; the first that does not write stops the save. Holds
/// the deck's files while it writes, so a run pressed again meanwhile reads
/// them once they are written. A file saved again since the press, as the
/// reader's Save saves it, keeps that save: the press's older text is not
/// written over it.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_edited_files(edited: Vec<EditedFile>, messages: MessageCatalog) -> SavedBeforeRun {
    let _files = hold_deck_files();
    let mut saved = SavedBeforeRun::default();
    for file in edited {
        let written = if saved.refusal.is_none() && last_save_asked(&file.path, file.save) {
            match write_edited_file(&file, messages) {
                Ok(written) => Some(written),
                Err(why) => {
                    saved.refusal = Some(why);
                    None
                }
            }
        } else {
            None
        };
        saved.files.push((file, written));
    }
    saved
}

/// Write `file` as [`save_file`] writes a file: its stamp and the line
/// ending it was written with.
#[cfg(not(target_arch = "wasm32"))]
fn write_edited_file(
    file: &EditedFile,
    messages: MessageCatalog,
) -> Result<(DiskStamp, LineEnding), String> {
    let name = file_name(&file.path);
    // A file that had one line and no separator takes the editor's.
    let line_ending = match file.line_ending {
        LineEnding::None => LineEnding::detect(&file.text),
        line_ending => line_ending,
    };
    let bytes = file
        .encoding
        .encode(&line_ending.write(&file.text))
        .map_err(|error| not_saved(messages, &name, &error))?;
    write_through_temporary(&file.path, &name, &bytes, messages)?;
    Ok((disk_stamp(&file.path, &file.text), line_ending))
}

/// Take what saving the deck's edited files came to into the open deck:
/// each file written is held to what was written, and holds no edits unless
/// it was edited since the press; a file not written is as it was. Why the
/// save stopped, when it did.
pub fn land_saved_before_run(
    netlist: &mut NetlistDocumentState,
    saved: SavedBeforeRun,
) -> Option<String> {
    let SavedBeforeRun { files, refusal } = saved;
    let Some(deck) = netlist.open_deck.as_mut() else {
        return refusal;
    };
    for (edited, written) in files {
        let Some(file) = deck
            .file_mut(edited.include.as_deref())
            .filter(|file| file.path == edited.path)
        else {
            continue;
        };
        // A save the reader made meanwhile holds the file to what it wrote.
        if file.disk.is_some() {
            continue;
        }
        match written {
            Some((stamp, line_ending)) => {
                file.disk = Some(stamp);
                if file.line_ending == LineEnding::None {
                    file.line_ending = line_ending;
                }
                if file.text.revision() == edited.text.revision() {
                    file.dirty = false;
                }
            }
            None => file.disk = edited.disk,
        }
    }
    refusal
}

/// The saves of decks' files made before their preparation, in flight; the
/// frame loop takes each in once it is made, whether or not the preparation
/// went on.
#[derive(Debug, Clone, Default)]
pub struct DeckSaves(Vec<Arc<Mutex<mpsc::Receiver<SavedBeforeRun>>>>);

impl DeckSaves {
    /// Wait for the save `save` will say it made.
    pub fn push(&mut self, save: mpsc::Receiver<SavedBeforeRun>) {
        self.0.push(Arc::new(Mutex::new(save)));
    }

    /// The saves made since the last call, in the order they were asked for.
    pub fn take_landed(&mut self) -> Vec<SavedBeforeRun> {
        let mut landed = Vec::new();
        self.0.retain(|save| {
            let Ok(save) = save.lock() else {
                return false;
            };
            match save.try_recv() {
                Ok(saved) => {
                    landed.push(saved);
                    false
                }
                Err(mpsc::TryRecvError::Empty) => true,
                Err(mpsc::TryRecvError::Disconnected) => false,
            }
        });
        landed
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

/// The identity the tabs of the deck `id` go by, open or missing.
#[must_use]
pub fn deck_tab_id(id: DeckId) -> NetlistDocumentId {
    NetlistDocumentId::try_from_uuid(id.as_uuid()).unwrap_or_default()
}

/// The missing deck, when it is the one in front: its tab is the page's,
/// and the page stands in its missing state.
#[must_use]
pub fn missing_deck_in_front(model: &Model, netlist: &NetlistDocumentState) -> Option<MissingDeck> {
    missing_deck(model, netlist).filter(|missing| netlist.front == NetlistTab::Deck(missing.id))
}

/// Bring the deck `id`, which could not be read, to the front when its file
/// is missing: the page stands on its tab in its missing state, and Run
/// names it, unavailable until the file is found. Whether it did.
pub fn show_missing_deck(model: &Model, netlist: &mut NetlistDocumentState, id: DeckId) -> bool {
    if missing_deck(model, netlist).is_none_or(|missing| missing.id != id) {
        return false;
    }
    netlist.front = NetlistTab::Deck(id);
    netlist.run_target = super::netlist_tabs::RunTarget::Deck(id);
    netlist.deck_tab_closed = false;
    netlist.active_document_initialized = true;
    true
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
    messages: MessageCatalog,
) -> Result<String, String> {
    let name = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .map(crate::state::ProjectDeck::name)
        .ok_or_else(|| messages.text(MessageId::NetlistDeckNotInProject))?;
    if let Some(open) = netlist.open_deck.as_ref().filter(|deck| deck.id == id) {
        if let Some(unsaved) = open.unsaved() {
            return Err(messages.format(
                MessageId::NetlistSaveBeforeRemoving,
                &[("file", &unsaved.name()), ("name", &name)],
            ));
        }
        close_open_deck(model, netlist, open_documents);
    }
    let content = &mut *model.workspace.content;
    content.decks.retain(|deck| deck.id != id);
    content.decks_dirty = true;
    let reading = netlist
        .reading_deck
        .take_if(|reading| reading.id == id)
        .is_some();
    if reading || netlist.unreadable_deck == Some(id) {
        netlist
            .unreadable_deck
            .take_if(|unreadable| *unreadable == id);
        netlist.run_the_plan();
        if netlist.front == NetlistTab::Deck(id)
            && !super::netlist_document::open_generated_primary_in(model, netlist)
        {
            netlist.front = NetlistTab::Generated;
        }
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
    model
        .simulation
        .source
        .netlist_content
        .set(model.netlist_artifacts.generated_source.clone());
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
}

/// Locate: the deck `id` is read from `path` from now on, and opens;
/// whether it did now. A deck read away from the frame is read from `path`
/// once it reads, and the frame that opens it says so; a file that does not
/// read leaves the deck where it was.
#[cfg(not(target_arch = "wasm32"))]
pub fn locate_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    id: DeckId,
    path: &Path,
    messages: MessageCatalog,
) -> Result<bool, String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    if !model
        .workspace
        .content
        .decks
        .iter()
        .any(|deck| deck.id == id)
    {
        return Err(messages.text(MessageId::NetlistDeckNotInProject));
    }
    if reads_away() {
        std::fs::File::open(&path)
            .map_err(|error| unreadable(messages, &path.display().to_string(), &error))?;
        let file = path.clone();
        start_reading(
            model,
            netlist,
            open_documents,
            id,
            (true, Some(path)),
            messages,
            move || {
                read_file(&file, messages).map(|root| OpenDeck {
                    id,
                    root,
                    includes: Vec::new(),
                    stored_files: None,
                })
            },
        )?;
        return Ok(false);
    }
    let root = read_file(&path, messages)?;
    relocate(model, id, &path);
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
    Ok(true)
}

/// The project's deck `id` is read from the file at `path` from now on.
fn relocate(model: &mut Model, id: DeckId, path: &Path) {
    let folder = model
        .workspace
        .content
        .project
        .data_root()
        .map(Path::to_path_buf);
    let content = &mut *model.workspace.content;
    if let Some(deck) = content.decks.iter_mut().find(|deck| deck.id == id) {
        deck.source = crate::state::ProjectDeck::for_file(path, folder.as_deref()).source;
        content.decks_dirty = true;
    }
}

/// Save the file in front of the Netlist workspace: the deck, or the
/// include on its tab.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_shown_file(
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Result<(), String> {
    if !netlist.front.shows_deck() {
        return Err(messages.text(MessageId::NetlistSaveNotADeckFile));
    }
    let include = netlist.front.include().map(str::to_owned);
    let file = netlist
        .open_deck
        .as_mut()
        .filter(|deck| !deck.is_stored())
        .ok_or_else(|| messages.text(MessageId::NetlistSaveNotADeckFile))?
        .file_mut(include.as_deref())
        .ok_or_else(|| not_open(messages, include.as_deref()))?;
    save_file(file, messages)
}

/// What a check of a file of a deck the project keeps as text reads beside
/// the file itself: the deck's name, which of its files is in front, the
/// deck as the editor holds it while an include is in front, and the files
/// kept with it. Each is shared with the open deck, so taking it on the
/// frame reads no text; [`StoredCheck::sources`] puts the files together
/// on the check's worker.
#[derive(Debug, Clone)]
pub struct StoredCheck {
    name: String,
    include: Option<String>,
    root: Option<(RevisedText, LineEnding)>,
    files: Vec<StoredDeckFile>,
}

impl StoredCheck {
    /// Where the engine reads the file in front, and every file of the deck
    /// where the engine reads it, the one in front with `shown`.
    #[must_use]
    pub fn sources(&self, shown: &str) -> (PathBuf, Vec<(PathBuf, String)>) {
        let include = self.include.as_deref();
        let root_text = match &self.root {
            None => shown.to_owned(),
            Some((text, line_ending)) => line_ending.write(text).into_owned(),
        };
        let files = self
            .files
            .iter()
            .map(|file| StoredDeckFile {
                text: if include == Some(file.path.as_str()) {
                    shown.into()
                } else {
                    file.text.clone()
                },
                ..file.clone()
            })
            .collect::<Vec<_>>();
        let (root, sources) =
            rspice_design::deck::stored_deck_sources(&self.name, &root_text, &files);
        let at = include.map_or(root, rspice_design::deck::stored_file_identity);
        (at, sources)
    }
}

/// For a deck the project keeps as text, in front of the page: what a check
/// of the file in front reads beside it. `None` for a deck that is a file.
#[must_use]
pub fn stored_check(netlist: &NetlistDocumentState) -> Option<StoredCheck> {
    let deck = netlist.open_deck.as_ref()?;
    let files = deck.stored_files.as_ref()?;
    let include = netlist.front.include();
    Some(StoredCheck {
        name: deck.root.name(),
        include: include.map(str::to_owned),
        root: include.map(|_| (deck.root.text.clone(), deck.root.line_ending)),
        files: files.clone(),
    })
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
    let text = file.kept_text();
    if let Some(key) = include
        && let Some(stored) = files.iter_mut().find(|stored| stored.path == key)
    {
        stored.text = text.clone();
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

/// Why another deck cannot be opened while `unsaved`, a file of the open
/// deck, holds edits: its edits are never dropped for another deck.
pub fn save_first(messages: crate::workbench::MessageCatalog, unsaved: &OpenFile) -> String {
    messages.format(
        MessageId::NetlistSaveBeforeAnotherDeck,
        &[("name", &unsaved.name())],
    )
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
    messages: crate::workbench::MessageCatalog,
) -> Result<String, String> {
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(save_first(messages, unsaved));
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
        stored.text = text.into();
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
    messages: crate::workbench::MessageCatalog,
) -> Result<Vec<String>, String> {
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(save_first(messages, unsaved));
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
        return Err(messages.text(MessageId::NetlistNoDeckToAdd));
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

/// Open the deck at `path`, read on this frame: add it to the project
/// unless it is one of the project's decks already, and bring it to the
/// front. The deck open before it goes, so one with unsaved edits stops
/// this. The application reads the file away from the frame first
/// ([`read_deck_file_away`]) and opens what it read ([`open_read_deck`]).
#[cfg(not(target_arch = "wasm32"))]
pub fn open_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    path: &Path,
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    if shown_if_open(model, netlist, &path, messages)? {
        return Ok(());
    }
    let root = read_file(&path, messages)?;
    open_read_deck(model, netlist, open_documents, runs, root, messages)
}

/// Whether the deck file at `path` is the open deck, now brought to the
/// front; why no other deck can open in its place, its edits unsaved.
#[cfg(not(target_arch = "wasm32"))]
fn shown_if_open(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    path: &Path,
    messages: MessageCatalog,
) -> Result<bool, String> {
    if let Some(open) = &netlist.open_deck {
        if open.root.path == path {
            show_deck(model, netlist);
            return Ok(true);
        }
        if let Some(unsaved) = open.unsaved() {
            return Err(save_first(messages, unsaved));
        }
    }
    Ok(false)
}

/// Open `root`, a deck file already read, as [`open_deck`] opens one.
#[cfg(not(target_arch = "wasm32"))]
pub fn open_read_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    root: OpenFile,
    messages: MessageCatalog,
) -> Result<(), String> {
    if shown_if_open(model, netlist, &root.path, messages)? {
        return Ok(());
    }
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
        .find(|deck| deck.is_file(&root.path, folder.as_deref()))
    {
        Some(deck) => deck.id,
        None => {
            let deck = crate::state::ProjectDeck::for_file(&root.path, folder.as_deref());
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
    messages: MessageCatalog,
) -> Result<OpenDeck, String> {
    if let Some(stored) = deck.stored_deck() {
        return Ok(OpenDeck::from_stored(deck.id, stored));
    }
    let path = deck.file_path(project_folder).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    {
        read_file(&path, messages).map(|root| OpenDeck {
            id: deck.id,
            root,
            includes: Vec::new(),
            stored_files: None,
        })
    }
    #[cfg(target_arch = "wasm32")]
    Err(messages.format(
        MessageId::NetlistDeckOnDesktop,
        &[
            ("name", &deck.name()),
            ("path", &path.display().to_string()),
        ],
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
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    open_project_deck_by_id(model, netlist, open_documents, runs, id, true, messages)
}

/// Reopen the project's deck `id`, as a restored session does: as
/// [`open_deck_by_id`] opens it, except that the log says why a deck read
/// away from the frame did not read.
pub fn restore_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    id: DeckId,
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    open_project_deck_by_id(model, netlist, open_documents, runs, id, false, messages)
}

/// [`open_deck_by_id`], the page saying why a deck read away from the frame
/// did not read when `prompt`, else the log.
fn open_project_deck_by_id(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    id: DeckId,
    prompt: bool,
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    if netlist.open_deck.as_ref().is_some_and(|deck| deck.id == id) {
        show_deck(model, netlist);
        return Ok(());
    }
    // The deck being read comes to the front, still reading.
    if netlist
        .reading_deck
        .as_ref()
        .is_some_and(|reading| reading.id == id)
    {
        show_deck_being_read(netlist);
        return Ok(());
    }
    if let Some(unsaved) = netlist.open_deck.as_ref().and_then(OpenDeck::unsaved) {
        return Err(save_first(messages, unsaved));
    }
    let deck = model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .ok_or_else(|| messages.text(MessageId::NetlistDeckNotInProject))?;
    if reads_away() {
        let deck = deck.clone();
        let folder = model
            .workspace
            .content
            .project
            .data_root()
            .map(Path::to_path_buf);
        return start_reading(
            model,
            netlist,
            open_documents,
            id,
            (prompt, None),
            messages,
            move || read_project_deck(&deck, folder.as_deref(), messages),
        );
    }
    match read_project_deck(deck, model.workspace.content.project.data_root(), messages) {
        Ok(open) => {
            install_open_deck(model, netlist, open_documents, runs, open);
            Ok(())
        }
        Err(error) => {
            close_open_deck(model, netlist, open_documents);
            netlist.unreadable_deck = Some(id);
            show_missing_deck(model, netlist, id);
            Err(error)
        }
    }
}

/// Bring the project's deck to the front: the open deck; else the deck
/// whose file could not be read, in its missing state; else the project's
/// first deck, read now.
pub fn show_project_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    messages: MessageCatalog,
) {
    if show_deck(model, netlist) || show_deck_being_read(netlist) {
        return;
    }
    if let Some(id) = netlist.unreadable_deck {
        show_missing_deck(model, netlist, id);
        return;
    }
    open_project_deck(model, netlist, open_documents, runs, messages);
}

/// The deck Run runs away from the Netlist workspace, while the project's
/// simulation is its deck (a project opened from a deck, with no design):
/// the deck [`show_project_deck`] brings to the front, which Run opens
/// first when it is not open yet. `None` for a project with a design, whose
/// Run runs the plan.
#[must_use]
pub fn project_run_deck(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<crate::workbench::framework::feature::ProjectRunDeck> {
    use crate::workbench::framework::feature::ProjectRunDeck;

    if !model.is_netlist_first_without_schematic() {
        return None;
    }
    if let Some(deck) = &netlist.open_deck {
        return Some(ProjectRunDeck {
            id: deck.id,
            missing: false,
        });
    }
    if let Some(id) = netlist.unreadable_deck {
        return Some(ProjectRunDeck { id, missing: true });
    }
    model
        .workspace
        .content
        .decks
        .first()
        .map(|deck| ProjectRunDeck {
            id: deck.id,
            missing: false,
        })
}

/// Open the project's first deck when the project has decks and none is
/// open, once per project: a deck that cannot be read is reported once.
pub fn open_project_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    messages: MessageCatalog,
) {
    if netlist.open_deck.is_some() || netlist.reading_deck.is_some() {
        return;
    }
    let Some(deck) = model.workspace.content.decks.first() else {
        return;
    };
    if netlist.unreadable_deck == Some(deck.id) {
        return;
    }
    let id = deck.id;
    if reads_away() {
        let deck = deck.clone();
        let folder = model
            .workspace
            .content
            .project
            .data_root()
            .map(Path::to_path_buf);
        if let Err(error) = start_reading(
            model,
            netlist,
            open_documents,
            id,
            (false, None),
            messages,
            move || read_project_deck(&deck, folder.as_deref(), messages),
        ) {
            netlist.unreadable_deck = Some(id);
            model.record(
                crate::diagnostics::LogSeverity::Error,
                crate::diagnostics::LogSource::Netlist,
                error,
            );
        }
        return;
    }
    match read_project_deck(deck, model.workspace.content.project.data_root(), messages) {
        Ok(open) => install_open_deck(model, netlist, open_documents, runs, open),
        Err(error) => {
            netlist.unreadable_deck = Some(id);
            show_missing_deck(model, netlist, id);
            model.record(
                crate::diagnostics::LogSeverity::Error,
                crate::diagnostics::LogSource::Netlist,
                error,
            );
        }
    }
}

/// What asks for a frame once a deck read away from the frame is done. The
/// application sets it as it starts, and decks are read away from the frame
/// from then on; a process that sets none, a test or a command-line run,
/// reads a deck on the frame that opens it.
static DECK_READ_WAKER: OnceLock<Box<dyn Fn() + Send + Sync>> = OnceLock::new();

/// Read the decks opened from now on away from the frame, `waker` asking
/// for the frame that takes each read: what the application does as it
/// starts. Where there are no threads, decks are read on the frame.
pub fn read_decks_away_from_frame(waker: impl Fn() + Send + Sync + 'static) {
    let _ = DECK_READ_WAKER.set(Box::new(waker));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Whether the decks this thread opens are read away from the frame.
    static READS_AWAY_HERE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Read the decks this thread opens from now on away from the frame, as the
/// application does, with no frame asked for: what a test of the frames
/// meanwhile does.
#[cfg(any(test, feature = "test-support"))]
pub fn read_decks_away_here() {
    READS_AWAY_HERE.set(true);
}

/// Whether a deck opened now is read away from the frame.
fn reads_away() -> bool {
    if cfg!(target_arch = "wasm32") {
        return false;
    }
    #[cfg(any(test, feature = "test-support"))]
    if READS_AWAY_HERE.get() {
        return true;
    }
    DECK_READ_WAKER.get().is_some()
}

/// A project deck being read away from the frame, which a later frame takes
/// ([`land_deck_read`]).
#[derive(Debug, Clone)]
pub struct DeckRead {
    /// The deck read.
    pub id: DeckId,
    /// Whether the page says why, should the deck not read, as it does for a
    /// deck the reader opened; else the log does.
    prompt: bool,
    /// The file a located deck is read from once it reads.
    located: Option<PathBuf>,
    read: ReadAway<Result<OpenDeck, String>>,
}

/// What a thread of its own reads, until a frame takes it.
#[derive(Debug)]
struct ReadAway<T>(Arc<Mutex<mpsc::Receiver<T>>>);

impl<T> Clone for ReadAway<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// Where a read away from the frame stands.
enum Landed<T> {
    Reading,
    Read(T),
    /// The read ended with nothing to take.
    Lost,
}

impl<T: Send + 'static> ReadAway<T> {
    /// Read with `read` on a thread of its own, which asks for the frame
    /// that takes it.
    fn start(read: impl FnOnce() -> T + Send + 'static) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::channel();
        #[cfg(any(test, feature = "test-support"))]
        let memory = super::deck_memory::assumed_machine_memory();
        std::thread::Builder::new()
            .name("deck-read".to_owned())
            .spawn(move || {
                #[cfg(any(test, feature = "test-support"))]
                super::deck_memory::assume_machine_memory_here(memory);
                if sender.send(read()).is_ok()
                    && let Some(wake) = DECK_READ_WAKER.get()
                {
                    wake();
                }
            })?;
        Ok(Self(Arc::new(Mutex::new(receiver))))
    }

    /// Where the read stands, waiting for it to end when `wait`.
    fn landed(&self, wait: bool) -> Landed<T> {
        let Ok(receiver) = self.0.lock() else {
            return Landed::Lost;
        };
        let received = if wait {
            receiver
                .recv()
                .map_err(|_| mpsc::TryRecvError::Disconnected)
        } else {
            receiver.try_recv()
        };
        match received {
            Ok(read) => Landed::Read(read),
            Err(mpsc::TryRecvError::Empty) => Landed::Reading,
            Err(mpsc::TryRecvError::Disconnected) => Landed::Lost,
        }
    }
}

impl DeckRead {
    /// Read a deck with `read` on a thread of its own, which also makes its
    /// text ready for the editor, so the frame that shows it reads nothing
    /// in proportion to it.
    fn start(
        id: DeckId,
        (prompt, located): (bool, Option<PathBuf>),
        read: impl FnOnce() -> Result<OpenDeck, String> + Send + 'static,
    ) -> std::io::Result<Self> {
        let read =
            ReadAway::start(move || read().inspect(|deck| deck.root.text.ready_for_editor()))?;
        Ok(Self {
            id,
            prompt,
            located,
            read,
        })
    }
}

/// A deck file read away from the frame before anything opens it, so a
/// file that does not read as text is refused before a project is made
/// around it or lists it. [`open_read_deck`] opens what it read.
#[derive(Debug, Clone)]
pub struct DeckFileRead {
    path: PathBuf,
    read: ReadAway<Result<OpenFile, String>>,
}

impl DeckFileRead {
    /// The file being read.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The file once it is read, waiting for its read when `wait`: `None`
    /// while it is read.
    #[must_use]
    pub fn landed(&self, wait: bool, messages: MessageCatalog) -> Option<Result<OpenFile, String>> {
        match self.read.landed(wait) {
            Landed::Reading => None,
            Landed::Read(read) => Some(read),
            Landed::Lost => Some(Err(unreadable(
                messages,
                &self.path.display().to_string(),
                &std::io::Error::from(std::io::ErrorKind::Interrupted),
            ))),
        }
    }
}

/// Read the deck file at `path` away from the frame, as the application
/// reads one before it opens it: `None` where decks are read on the frame
/// that opens them. A file that does not open is refused at once.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_deck_file_away(
    path: &Path,
    messages: MessageCatalog,
) -> Option<Result<DeckFileRead, String>> {
    if !reads_away() {
        return None;
    }
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let refused =
        |error: &dyn std::fmt::Display| unreadable(messages, &path.display().to_string(), error);
    if let Err(error) = std::fs::File::open(&path) {
        return Some(Err(refused(&error)));
    }
    let file = path.clone();
    Some(
        ReadAway::start(move || {
            read_file(&file, messages).inspect(|file| file.text.ready_for_editor())
        })
        .map_err(|error| refused(&error))
        .map(|read| DeckFileRead {
            path: path.clone(),
            read,
        }),
    )
}

/// Read the project's deck `id` away from the frame with `read`. The deck
/// open before goes, as it does when a deck opens; the deck's tab comes to
/// the front at once, saying it is reading, and the frame after the read
/// takes it. Why it was not read, when no thread could read it. `said` is
/// whether the page says why the deck did not read, and the file a located
/// deck is read from once it reads.
fn start_reading(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    id: DeckId,
    said: (bool, Option<PathBuf>),
    messages: MessageCatalog,
    read: impl FnOnce() -> Result<OpenDeck, String> + Send + 'static,
) -> Result<(), String> {
    let reading = DeckRead::start(id, said, read)
        .map_err(|error| unreadable(messages, &deck_name(model, id), &error))?;
    close_open_deck(model, netlist, open_documents);
    netlist.unreadable_deck = None;
    netlist.reading_deck = Some(reading);
    show_deck_being_read(netlist);
    Ok(())
}

/// The name of the project's deck `id`.
fn deck_name(model: &Model, id: DeckId) -> String {
    model
        .workspace
        .content
        .decks
        .iter()
        .find(|deck| deck.id == id)
        .map(crate::state::ProjectDeck::name)
        .unwrap_or_default()
}

/// The deck being read away from the frame, by its id and its name: what
/// its tab names.
#[must_use]
pub fn deck_being_read(model: &Model, netlist: &NetlistDocumentState) -> Option<(DeckId, String)> {
    let id = netlist.reading_deck.as_ref()?.id;
    Some((id, deck_name(model, id)))
}

/// Bring the deck being read to the front, on its tab, which says it is
/// reading; whether there is one.
pub fn show_deck_being_read(netlist: &mut NetlistDocumentState) -> bool {
    let Some(id) = netlist.reading_deck.as_ref().map(|reading| reading.id) else {
        return false;
    };
    netlist.front = NetlistTab::Deck(id);
    netlist.run_target = super::netlist_tabs::RunTarget::Deck(id);
    netlist.deck_tab_closed = false;
    netlist.active_document_initialized = true;
    true
}

/// Whether the deck in front is being read, so the page says so where its
/// text will be.
#[must_use]
pub fn reading_deck_in_front(netlist: &NetlistDocumentState) -> bool {
    netlist
        .reading_deck
        .as_ref()
        .is_some_and(|reading| netlist.front == NetlistTab::Deck(reading.id))
}

/// Take the deck read away from the frame once its read is done, waiting
/// for it when `wait`: it becomes the open deck, in front unless its tab
/// was closed meanwhile, and a located deck is read from its new file from
/// then on, which is returned for the page to say. A deck that did not read
/// is left unreadable, as one read on the frame is: why is returned, as an
/// error, for the page to say when the reader opened the deck, and the log
/// says it otherwise.
pub fn land_deck_read(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    wait: bool,
    messages: MessageCatalog,
) -> Option<Result<String, String>> {
    let read = match netlist.reading_deck.as_ref()?.read.landed(wait) {
        Landed::Reading => return None,
        Landed::Read(read) => read,
        Landed::Lost => {
            let id = netlist.reading_deck.as_ref()?.id;
            Err(unreadable(
                messages,
                &deck_name(model, id),
                &std::io::Error::from(std::io::ErrorKind::Interrupted),
            ))
        }
    };
    let reading = netlist.reading_deck.take()?;
    match read {
        Ok(deck) => {
            let located = reading.located.as_deref().map(|path| {
                let said = messages.format(
                    MessageId::NetlistDeckLocated,
                    &[
                        ("name", &deck_name(model, reading.id)),
                        ("path", &path.display().to_string()),
                    ],
                );
                relocate(model, reading.id, path);
                said
            });
            if netlist.deck_tab_closed {
                // Its tab closed while it was read: the deck is open, as a
                // deck whose tab was closed is, until it is shown again.
                netlist.language_off = super::deck_memory::LanguageOff::as_opened(&deck);
                netlist.open_deck = Some(deck);
                runs.manual_run.edited_lines.clear();
            } else {
                install_open_deck(model, netlist, open_documents, runs, deck);
            }
            located.map(Ok)
        }
        Err(error) => {
            let id = reading.id;
            let in_front = netlist.front == NetlistTab::Deck(id);
            netlist.unreadable_deck = Some(id);
            netlist.run_the_plan();
            if in_front && !show_missing_deck(model, netlist, id) {
                netlist.front = NetlistTab::Generated;
            }
            if reading.prompt {
                Some(Err(error))
            } else {
                model.record(
                    crate::diagnostics::LogSeverity::Error,
                    crate::diagnostics::LogSource::Netlist,
                    error,
                );
                None
            }
        }
    }
}

/// Make `deck` the open deck and put it in front: the tabs of the deck it
/// replaces close with it, and a deck being read is let go.
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
    netlist.reading_deck = None;
    netlist.language_off = super::deck_memory::LanguageOff::as_opened(&deck);
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
    messages: crate::workbench::MessageCatalog,
) -> Result<(), String> {
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| messages.text(MessageId::NetlistNoDeckOpen))?;
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
            let stored = files.iter().find(|file| file.path == key).ok_or_else(|| {
                messages.format(
                    MessageId::NetlistIncludeNotKept,
                    &[("file", &file_name(path))],
                )
            })?;
            let (text, line_ending) = editor_text(&stored.text);
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
            deck.includes.push(read_file(path, messages)?);
            #[cfg(target_arch = "wasm32")]
            return Err(messages.format(
                MessageId::NetlistIncludeNotOpen,
                &[("file", &file_name(path))],
            ));
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
        messages,
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
