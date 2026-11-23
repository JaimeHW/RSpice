//! Decks: SPICE text files the project refers to.
//!
//! A deck on the desktop is a file. The project keeps where it is and
//! nothing of its text; the file is read when the deck opens or runs and
//! written when it is saved. The browser and the tablet have no file system,
//! so there the project keeps the deck's text, and the text of the files its
//! include lines read.

use std::any::Any;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// A deck's identity in the project: stable when the file moves, new for
/// every deck added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeckId(Uuid);

impl DeckId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// The identity as a UUID, which a session names the deck's tabs by.
    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for DeckId {
    fn default() -> Self {
        Self::new()
    }
}

/// One deck of the project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDeck {
    pub id: DeckId,
    pub source: DeckSource,
}

/// Where a deck's text is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum DeckSource {
    /// A file. Relative to the project folder, written with `/`, when the
    /// file is under it; absolute otherwise.
    File { path: PathBuf },
    /// Text kept in the project, where there is no file system to keep a
    /// deck in.
    Stored(StoredDeck),
}

/// A deck the project keeps as text, with the files its include lines read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredDeck {
    /// The file name the deck is shown, run and exported under.
    pub name: String,
    /// The exact text, with the line separators it was written with.
    pub text: DeckText,
    /// The encoding the deck is exported in.
    pub encoding: TextEncoding,
    /// The files its include lines read.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<StoredDeckFile>,
}

/// A file a stored deck's include lines read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredDeckFile {
    /// Where the file is, relative to the deck, written with `/` and never
    /// above the deck's own folder.
    pub path: String,
    /// The exact text, with the line separators it was written with.
    pub text: DeckText,
    /// The encoding the file is exported in.
    pub encoding: TextEncoding,
}

impl StoredDeck {
    /// A new deck called `name`: a title line naming it, and `.end`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        let title = name.rsplit_once('.').map_or(name, |(stem, _)| stem).trim();
        Self {
            name: name.to_owned(),
            text: format!("* {title}\n\n.end\n").into(),
            encoding: TextEncoding::Utf8,
            files: Vec::new(),
        }
    }
}

/// The exact text of a deck the project keeps, or of a file kept with it,
/// with the line separators it was written with.
///
/// Cloning one shares the text, whatever its length: the project, a capture
/// of it, the baseline a save leaves and the open deck all hold the one
/// text. A text the Netlist editor gives is kept as the editor holds it and
/// put in one piece, with the file's own separators, only when something
/// reads it so; a key typed in a deck the project keeps then costs what one
/// in a deck that is a file costs. Its digest is taken once, the first time
/// it is asked for.
#[derive(Clone)]
pub struct DeckText(Arc<Shared<dyn DeckTextBody>>);

/// What a [`DeckText`] holds: an identity no other text has carried, the
/// text's digest once it is taken, and the text.
struct Shared<T: ?Sized> {
    identity: u64,
    digest: OnceLock<ContentDigest>,
    body: T,
}

/// The identity the next new text takes.
static NEXT_DECK_TEXT: AtomicU64 = AtomicU64::new(1);

/// A text a [`DeckText`] holds.
pub trait DeckTextBody: Any + Send + Sync {
    /// The exact text in one piece, made the first time it is asked for.
    fn text(&self) -> &str;
}

impl DeckTextBody for String {
    fn text(&self) -> &str {
        self
    }
}

impl DeckText {
    /// `body`, under an identity of its own.
    #[must_use]
    pub fn new(body: impl DeckTextBody) -> Self {
        Self(Arc::new(Shared {
            identity: NEXT_DECK_TEXT.fetch_add(1, Ordering::Relaxed),
            digest: OnceLock::new(),
            body,
        }))
    }

    /// The exact text in one piece.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.body.text()
    }

    /// An identity no other text has carried: equal for clones of one text,
    /// and two for the same characters held twice. A key that tells two
    /// texts apart without reading them reads it.
    #[must_use]
    pub fn identity(&self) -> u64 {
        self.0.identity
    }

    /// Whether `other` is this very text, not merely the same characters:
    /// told at once, whatever their length.
    #[must_use]
    pub fn is(&self, other: &Self) -> bool {
        self.0.identity == other.0.identity
    }

    /// What the text holds, as a `T`, when it is one.
    #[must_use]
    pub fn body<T: DeckTextBody>(&self) -> Option<&T> {
        (&self.0.body as &dyn Any).downcast_ref::<T>()
    }

    /// The SHA-256 digest of the exact text, taken the first time any clone
    /// of it asks.
    #[must_use]
    pub fn digest(&self) -> ContentDigest {
        *self.0.digest.get_or_init(|| {
            let mut hasher = Sha256::new();
            hasher.update(b"rspice-deck-text\0v1\0");
            hasher.update(self.as_str().as_bytes());
            ContentDigest::from_bytes(hasher.finalize().into())
        })
    }

    /// The digest, when it has been taken.
    #[must_use]
    pub fn digest_taken(&self) -> Option<ContentDigest> {
        self.0.digest.get().copied()
    }

    /// Whether `other` holds the same characters, as far as that can be told
    /// without putting a text the editor gave in one piece: at once for one
    /// text, by their digests when both are taken, and by the characters
    /// when neither came from the editor. A text the editor gave whose
    /// digest is not taken yet is an edit made since, and differs.
    #[must_use]
    pub fn kept_as(&self, other: &Self) -> bool {
        if self.is(other) {
            return true;
        }
        if let (Some(one), Some(two)) = (self.digest_taken(), other.digest_taken()) {
            return one == two;
        }
        match (self.body::<String>(), other.body::<String>()) {
            (Some(one), Some(two)) => one == two,
            _ => false,
        }
    }
}

impl Default for DeckText {
    fn default() -> Self {
        Self::from(String::new())
    }
}

impl From<String> for DeckText {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for DeckText {
    fn from(text: &str) -> Self {
        Self::new(text.to_owned())
    }
}

impl std::ops::Deref for DeckText {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for DeckText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Debug for DeckText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_str(), formatter)
    }
}

impl std::fmt::Display for DeckText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Two texts are equal when they hold the same characters: at once when
/// they are one text.
impl PartialEq for DeckText {
    fn eq(&self, other: &Self) -> bool {
        self.is(other) || self.as_str() == other.as_str()
    }
}

impl Eq for DeckText {}

impl PartialEq<str> for DeckText {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for DeckText {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for DeckText {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

/// Written as a string, as a deck's text always was.
impl Serialize for DeckText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DeckText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Self::from)
    }
}

/// Where the engine reads a file of a deck the project keeps as text: the
/// deck by its name, and each of its files by its path from the deck, all
/// under one folder, `/`, that no disk has. An include line's relative path
/// then names the file it names beside the deck, and nothing else.
#[must_use]
pub fn stored_file_identity(path: &str) -> PathBuf {
    PathBuf::from(format!("/{}", path.trim_start_matches(['/', '\\'])))
}

/// The path from the deck of the file the engine reads at `identity`: the
/// inverse of [`stored_file_identity`], whichever separators the identity
/// came back with. A path that is no identity is taken as a path already.
#[must_use]
pub fn stored_file_path(identity: &Path) -> String {
    identity
        .to_string_lossy()
        .replace('\\', "/")
        .trim_start_matches(['/', '\\'])
        .to_owned()
}

/// The deck `name`, written `text`, and the files kept with it, as the
/// engine reads them: the deck's identity, and every file, the deck first,
/// at its identity.
#[must_use]
pub fn stored_deck_sources(
    name: &str,
    text: &str,
    files: &[StoredDeckFile],
) -> (PathBuf, Vec<(PathBuf, String)>) {
    let root = stored_file_identity(name);
    let mut sources = Vec::with_capacity(files.len() + 1);
    sources.push((root.clone(), text.to_owned()));
    sources.extend(files.iter().map(|file| {
        (
            stored_file_identity(&file.path),
            file.text.as_str().to_owned(),
        )
    }));
    (root, sources)
}

/// The first `deck<N>.cir` that none of `taken` is called: the name a new
/// stored deck takes.
#[must_use]
pub fn new_stored_deck_name<'a, I>(taken: I) -> String
where
    I: IntoIterator<Item = &'a str> + Clone,
{
    (1_u32..)
        .map(|number| format!("deck{number}.cir"))
        .find(|name| {
            !taken
                .clone()
                .into_iter()
                .any(|other| other.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| "deck.cir".to_owned())
}

impl ProjectDeck {
    /// Whether `other` is this deck as it was: everything compared, and the
    /// texts as [`DeckText::kept_as`] compares them, so a deck the editor
    /// changed is told apart without its text being read.
    #[must_use]
    pub fn kept_as(&self, other: &Self) -> bool {
        self.id == other.id
            && match (&self.source, &other.source) {
                (DeckSource::Stored(one), DeckSource::Stored(two)) => {
                    one.name == two.name
                        && one.encoding == two.encoding
                        && one.text.kept_as(&two.text)
                        && one.files.len() == two.files.len()
                        && one.files.iter().zip(&two.files).all(|(one, two)| {
                            one.path == two.path
                                && one.encoding == two.encoding
                                && one.text.kept_as(&two.text)
                        })
                }
                _ => self.source == other.source,
            }
    }

    /// A new deck for the file at `path`, stored relative to `project_folder`
    /// when it is under that folder.
    #[must_use]
    pub fn for_file(path: &Path, project_folder: Option<&Path>) -> Self {
        Self {
            id: DeckId::new(),
            source: DeckSource::File {
                path: stored_deck_path(path, project_folder),
            },
        }
    }

    /// A new deck the project keeps as text.
    #[must_use]
    pub fn stored(deck: StoredDeck) -> Self {
        Self {
            id: DeckId::new(),
            source: DeckSource::Stored(deck),
        }
    }

    /// The deck's file, resolved against `project_folder` when it is stored
    /// relative; `None` for a deck the project keeps as text.
    #[must_use]
    pub fn file_path(&self, project_folder: Option<&Path>) -> Option<PathBuf> {
        let DeckSource::File { path } = &self.source else {
            return None;
        };
        Some(match project_folder {
            Some(folder) if path.is_relative() => plain_path(folder).join(path),
            _ => path.clone(),
        })
    }

    /// Keep the deck's file where it is when the project moves from the
    /// folder `from` to the folder `to`: by its path from `to` when it is
    /// under it, else by its whole path. A relative path with no folder to
    /// read it from stays as written.
    pub fn rebase(&mut self, from: Option<&Path>, to: Option<&Path>) {
        let DeckSource::File { path } = &self.source else {
            return;
        };
        if path.is_relative() && from.is_none() {
            return;
        }
        if let Some(file) = self.file_path(from) {
            self.source = DeckSource::File {
                path: stored_deck_path(&file, to),
            };
        }
    }

    /// Whether the deck is the file at `path`, however either is spelled.
    #[must_use]
    pub fn is_file(&self, path: &Path, project_folder: Option<&Path>) -> bool {
        self.file_path(project_folder).is_some_and(|file| {
            plain_path(&file) == plain_path(path)
                || resolved_path(&file).is_some_and(|file| Some(file) == resolved_path(path))
        })
    }

    /// The deck the project keeps as text, when it is one.
    #[must_use]
    pub const fn stored_deck(&self) -> Option<&StoredDeck> {
        match &self.source {
            DeckSource::Stored(deck) => Some(deck),
            DeckSource::File { .. } => None,
        }
    }

    /// The deck the project keeps as text, to change, when it is one.
    pub fn stored_deck_mut(&mut self) -> Option<&mut StoredDeck> {
        match &mut self.source {
            DeckSource::Stored(deck) => Some(deck),
            DeckSource::File { .. } => None,
        }
    }

    /// The name the deck is shown and run under: its file name.
    #[must_use]
    pub fn name(&self) -> String {
        match &self.source {
            DeckSource::File { path } => path.file_name().map_or_else(
                || path.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            ),
            DeckSource::Stored(deck) => deck.name.clone(),
        }
    }
}

/// `path` as the project stores it: relative to `project_folder` with `/`
/// separators when it is under that folder, else as given. The two are first
/// compared as written; when that does not place the file under the folder,
/// they are compared as the file system resolves them, so a folder reached
/// through a link, a `..`, a short name (`RUNNER~1`) or `/var` for
/// `/private/var` is still the folder the deck is under.
#[must_use]
pub fn stored_deck_path(path: &Path, project_folder: Option<&Path>) -> PathBuf {
    let plain = plain_path(path);
    let Some(folder) = project_folder else {
        return plain;
    };
    path_under(&plain, &plain_path(folder))
        .or_else(|| path_under(&resolved_path(path)?, &resolved_path(folder)?))
        .unwrap_or(plain)
}

/// `file`'s path from `folder` with `/` separators, when it is a file under
/// that folder.
fn path_under(file: &Path, folder: &Path) -> Option<PathBuf> {
    let relative = file.strip_prefix(folder).ok()?;
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| PathBuf::from(parts.join("/")))
}

/// `path` as the file system resolves it (links, `..`, short names and the
/// letter case it keeps), without the verbatim prefix; `None` when it does
/// not exist or there is no file system to ask, as in the browser.
fn resolved_path(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path)
        .ok()
        .map(|path| plain_path(&path))
}

/// `path` without the Windows verbatim prefix a canonical path carries
/// (`\\?\C:\...`, `\\?\UNC\server\share`), so the folder a project was saved
/// to and a deck picked in a file dialog compare as the same place. Any
/// other path is returned as it is.
#[must_use]
pub fn plain_path(path: &Path) -> PathBuf {
    let Some(text) = path.to_str() else {
        return path.to_path_buf();
    };
    if let Some(share) = text.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{share}"));
    }
    match text.strip_prefix(r"\\?\") {
        Some(drive)
            if drive
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
                && drive.get(1..3) == Some(":\\") =>
        {
            PathBuf::from(drive)
        }
        _ => path.to_path_buf(),
    }
}

/// How a text file's characters are stored as bytes. Read from the file's
/// bytes when it is opened and applied again when it is saved, so a file is
/// never converted behind its author's back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TextEncoding {
    #[default]
    Utf8,
    Utf8Bom,
    Utf16LeBom,
    Utf16BeBom,
    Latin1,
}

impl TextEncoding {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8 with BOM",
            Self::Utf16LeBom => "UTF-16 LE with BOM",
            Self::Utf16BeBom => "UTF-16 BE with BOM",
            Self::Latin1 => "ISO-8859-1",
        }
    }

    /// The text of `bytes` and the encoding it was stored in: a byte order
    /// mark names it, valid UTF-8 is UTF-8, and anything else is ISO-8859-1.
    pub fn decode(bytes: &[u8]) -> Result<(String, Self), String> {
        let encoding = if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
            Self::Utf8Bom
        } else if bytes.starts_with(&[0xff, 0xfe]) {
            Self::Utf16LeBom
        } else if bytes.starts_with(&[0xfe, 0xff]) {
            Self::Utf16BeBom
        } else if std::str::from_utf8(bytes).is_ok() {
            Self::Utf8
        } else {
            Self::Latin1
        };
        let text = rspice_core_netlist::netlist::decode_source_bytes(bytes)
            .map_err(|error| format!("the file could not be read as text: {error}"))?;
        Ok((text, encoding))
    }

    /// `source` as bytes in this encoding. A character ISO-8859-1 cannot
    /// hold is refused, naming its position, rather than replaced.
    pub fn encode(self, source: &str) -> Result<Vec<u8>, String> {
        match self {
            Self::Utf8 => Ok(source.as_bytes().to_vec()),
            Self::Utf8Bom => {
                let mut bytes = Vec::with_capacity(source.len().saturating_add(3));
                bytes.extend_from_slice(&[0xef, 0xbb, 0xbf]);
                bytes.extend_from_slice(source.as_bytes());
                Ok(bytes)
            }
            Self::Utf16LeBom | Self::Utf16BeBom => {
                let mut bytes =
                    Vec::with_capacity(source.len().saturating_mul(2).saturating_add(2));
                bytes.extend_from_slice(if self == Self::Utf16LeBom {
                    &[0xff, 0xfe]
                } else {
                    &[0xfe, 0xff]
                });
                for unit in source.encode_utf16() {
                    let encoded = if self == Self::Utf16LeBom {
                        unit.to_le_bytes()
                    } else {
                        unit.to_be_bytes()
                    };
                    bytes.extend_from_slice(&encoded);
                }
                Ok(bytes)
            }
            Self::Latin1 => {
                let mut bytes = Vec::with_capacity(source.chars().count());
                for (character_index, character) in source.chars().enumerate() {
                    let value = u32::from(character);
                    if value > u32::from(u8::MAX) {
                        return Err(format!(
                            "character {} (U+{value:04X}) cannot be represented in ISO-8859-1; use Save As with UTF-8 or remove the character",
                            character_index + 1
                        ));
                    }
                    bytes.push(value as u8);
                }
                Ok(bytes)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_deck_under_the_project_folder_is_stored_relative_with_forward_slashes() {
        let folder = Path::new("projects").join("amp");
        let file = folder.join("decks").join("amp.cir");
        let deck = ProjectDeck::for_file(&file, Some(&folder));
        let json = serde_json::to_value(&deck.source).expect("deck source serializes");
        assert_eq!(
            json,
            serde_json::json!({"kind": "file", "path": "decks/amp.cir"})
        );
        assert_eq!(
            deck.file_path(Some(&folder)),
            Some(folder.join("decks/amp.cir"))
        );
        assert_eq!(deck.name(), "amp.cir");
    }

    /// A project saved through a canonical path still keeps a deck under its
    /// folder by its relative path, and knows the deck by either spelling.
    #[cfg(windows)]
    #[test]
    fn a_verbatim_project_folder_still_stores_its_decks_relative() {
        let folder = PathBuf::from(r"\\?\C:\work\amp");
        let file = PathBuf::from(r"C:\work\amp\decks\amp.cir");
        let deck = ProjectDeck::for_file(&file, Some(&folder));
        let DeckSource::File { path } = &deck.source else {
            panic!("a deck opened from a file is a file");
        };
        assert_eq!(path, &PathBuf::from("decks/amp.cir"));
        assert!(deck.is_file(&file, Some(&folder)));
        assert!(deck.is_file(
            &PathBuf::from(r"\\?\C:\work\amp\decks\amp.cir"),
            Some(&folder)
        ));
    }

    /// A deck reached through a spelling of its folder the project does not
    /// share, here a `..`, is still stored by its path from the project
    /// folder, and the project knows it by either spelling.
    #[test]
    fn a_deck_under_another_spelling_of_the_project_folder_is_stored_relative() {
        let folder = std::env::temp_dir()
            .join(format!("rspice-deck-spelling-{}", std::process::id()))
            .join("amp");
        std::fs::create_dir_all(folder.join("decks")).expect("the folders");
        std::fs::create_dir_all(folder.join("x")).expect("the folders");
        let file = folder.join("decks").join("amp.cir");
        std::fs::write(&file, "* amp\n.end\n").expect("the deck");
        let roundabout = folder.join("x").join("..").join("decks").join("amp.cir");

        let deck = ProjectDeck::for_file(&roundabout, Some(&folder));

        let DeckSource::File { path } = &deck.source else {
            panic!("a deck opened from a file is a file");
        };
        assert_eq!(path, &PathBuf::from("decks/amp.cir"));
        assert!(deck.is_file(&roundabout, Some(&folder)));
        assert!(deck.is_file(&file, Some(&folder)));
        let _ = std::fs::remove_dir_all(folder.parent().expect("the test folder"));
    }

    #[test]
    fn a_deck_outside_the_project_folder_keeps_its_own_path() {
        // A deck is opened by its full path; one outside the project folder
        // is stored as that path and read back unchanged.
        let root = std::env::temp_dir();
        let folder = root.join("projects").join("amp");
        let file = root.join("elsewhere").join("tb.sp");
        let deck = ProjectDeck::for_file(&file, Some(&folder));
        let DeckSource::File { path } = &deck.source else {
            panic!("a deck opened from a file is a file");
        };
        assert_eq!(path, &file);
        assert_eq!(deck.file_path(Some(&folder)), Some(file));
    }

    #[test]
    fn a_stored_deck_keeps_its_text_and_files_in_the_project_file() {
        let mut stored = StoredDeck::new("amp_tb.cir");
        stored.files.push(StoredDeckFile {
            path: "models/amp.lib".to_owned(),
            text: ".model qn npn\r\n".into(),
            encoding: TextEncoding::Latin1,
        });
        let deck = ProjectDeck::stored(stored);
        let json = serde_json::to_value(&deck.source).expect("deck source serializes");
        assert_eq!(
            json,
            serde_json::json!({
                "kind": "stored",
                "name": "amp_tb.cir",
                "text": "* amp_tb\n\n.end\n",
                "encoding": "utf8",
                "files": [{
                    "path": "models/amp.lib",
                    "text": ".model qn npn\r\n",
                    "encoding": "latin1",
                }],
            })
        );
        let read: DeckSource = serde_json::from_value(json).expect("deck source reads back");
        assert_eq!(read, deck.source);
        assert_eq!(deck.name(), "amp_tb.cir");
        assert_eq!(deck.file_path(None), None);
        assert!(!deck.is_file(Path::new("amp_tb.cir"), None));
    }

    #[test]
    fn a_new_stored_deck_takes_the_first_name_no_deck_has() {
        assert_eq!(new_stored_deck_name(["deck2.cir"]), "deck1.cir");
        assert_eq!(
            new_stored_deck_name(["DECK1.cir", "deck2.cir"]),
            "deck3.cir"
        );
    }

    #[test]
    fn latin1_text_round_trips_and_an_unrepresentable_character_is_refused() {
        let bytes = b"* r\xe9sistance\r\n.end\r\n";
        let (text, encoding) = TextEncoding::decode(bytes).expect("decodes");
        assert_eq!(encoding, TextEncoding::Latin1);
        assert_eq!(text, "* r\u{e9}sistance\r\n.end\r\n");
        assert_eq!(encoding.encode(&text).expect("encodes"), bytes);
        assert!(encoding.encode("\u{3a9}").is_err());
    }
}
