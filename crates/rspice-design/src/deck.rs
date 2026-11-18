//! Decks: SPICE text files the project refers to.
//!
//! A deck on the desktop is a file. The project keeps where it is and
//! nothing of its text; the file is read when the deck opens or runs and
//! written when it is saved. The browser and the tablet have no file system,
//! so there the project keeps the deck's text, and the text of the files its
//! include lines read.

use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
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
    pub text: String,
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
    pub text: String,
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
            text: format!("* {title}\n\n.end\n"),
            encoding: TextEncoding::Utf8,
            files: Vec::new(),
        }
    }
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
        self.file_path(project_folder)
            .is_some_and(|file| plain_path(&file) == plain_path(path))
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
/// separators when it is under that folder, else as given.
#[must_use]
pub fn stored_deck_path(path: &Path, project_folder: Option<&Path>) -> PathBuf {
    let plain = plain_path(path);
    let Some(relative) =
        project_folder.and_then(|folder| plain.strip_prefix(plain_path(folder)).ok())
    else {
        return plain;
    };
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            _ => return plain,
        }
    }
    if parts.is_empty() {
        return plain;
    }
    PathBuf::from(parts.join("/"))
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
            text: ".model qn npn\r\n".to_owned(),
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
