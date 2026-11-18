//! Decks: SPICE text files the project refers to.
//!
//! A deck on the desktop is a file. The project keeps where it is and
//! nothing of its text; the file is read when the deck opens or runs and
//! written when it is saved.

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

    /// The deck's file, resolved against `project_folder` when it is stored
    /// relative.
    #[must_use]
    pub fn file_path(&self, project_folder: Option<&Path>) -> PathBuf {
        let DeckSource::File { path } = &self.source;
        match project_folder {
            Some(folder) if path.is_relative() => folder.join(path),
            _ => path.clone(),
        }
    }

    /// The name the deck is shown and run under: its file name.
    #[must_use]
    pub fn name(&self) -> String {
        let DeckSource::File { path } = &self.source;
        path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        )
    }
}

/// `path` as the project stores it: relative to `project_folder` with `/`
/// separators when it is under that folder, else as given.
#[must_use]
pub fn stored_deck_path(path: &Path, project_folder: Option<&Path>) -> PathBuf {
    let Some(relative) = project_folder.and_then(|folder| path.strip_prefix(folder).ok()) else {
        return path.to_path_buf();
    };
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            _ => return path.to_path_buf(),
        }
    }
    if parts.is_empty() {
        return path.to_path_buf();
    }
    PathBuf::from(parts.join("/"))
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
        assert_eq!(deck.file_path(Some(&folder)), folder.join("decks/amp.cir"));
        assert_eq!(deck.name(), "amp.cir");
    }

    #[test]
    fn a_deck_outside_the_project_folder_keeps_its_own_path() {
        let folder = Path::new("projects").join("amp");
        let file = Path::new("elsewhere").join("tb.sp");
        let deck = ProjectDeck::for_file(&file, Some(&folder));
        assert_eq!(deck.file_path(Some(&folder)), file);
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
