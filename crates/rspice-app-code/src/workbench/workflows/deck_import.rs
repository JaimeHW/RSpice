//! Decks brought into a project that keeps them as text, where there is no
//! file system to keep a deck in: SPICE files picked together, or one ZIP of
//! them, stored or deflated. Each SPICE file no other picked file includes
//! is a deck; the engine says which files include which, by reading each
//! one among the others. The rest are kept with each deck, by their paths
//! from it, for its include lines to read. Out again, a deck and its files
//! are one ZIP.

use std::collections::BTreeSet;
use std::path::Path;

use crate::state::{StoredDeck, StoredDeckFile, TextEncoding};

/// The extensions of a file that can be a deck: a library or an include is
/// read by one, never run alone.
const DECK_EXTENSIONS: &[&str] = &[
    "cir", "sp", "spi", "spice", "net", "ckt", "cki", "scs", "txt",
];

/// The decks among `files`, each a path among the picked files and its
/// bytes: one ZIP is read for the files in it. Every file no other includes
/// and whose name says it is a deck is one; the text files besides are kept
/// with each.
pub fn import_files(files: Vec<(String, Vec<u8>)>) -> Result<Vec<StoredDeck>, String> {
    if let [(name, bytes)] = files.as_slice()
        && bytes.starts_with(b"PK\x03\x04")
    {
        return import_zip(name, bytes);
    }
    if files
        .iter()
        .any(|(_, bytes)| bytes.starts_with(b"PK\x03\x04"))
    {
        return Err("Pick one ZIP alone, or SPICE files.".to_owned());
    }
    let mut picked = Vec::with_capacity(files.len());
    let mut paths = BTreeSet::new();
    for (path, bytes) in files {
        let path = plain_relative(&path);
        if path.is_empty() || is_binary(&bytes) {
            continue;
        }
        if !paths.insert(path.clone()) {
            return Err(format!("{path} is picked twice."));
        }
        let (text, encoding) = TextEncoding::decode(&bytes)?;
        picked.push(Picked {
            path,
            text,
            encoding,
        });
    }
    decks_among(&picked)
}

/// The decks in the ZIP `bytes`, called `name`: its files, by their paths
/// in it, read as [`import_files`] reads picked files.
pub fn import_zip(name: &str, bytes: &[u8]) -> Result<Vec<StoredDeck>, String> {
    let files = rspice_formats::zip::read_zip_files(
        bytes,
        crate::state::MAX_PROJECT_SOURCE_FILES,
        crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    )
    .map_err(|error| format!("{name}: {error}"))?;
    if files
        .iter()
        .any(|(_, bytes)| bytes.starts_with(b"PK\x03\x04"))
    {
        return Err(format!("{name} holds another ZIP; pick the files in it."));
    }
    if files.is_empty() {
        return Err(format!("{name} holds no files."));
    }
    import_files(files)
}

/// The text of `bytes`, the one file `name` picked to keep with a deck, and
/// its encoding: a ZIP, or a file that is not text, is refused.
pub fn picked_text(name: &str, bytes: &[u8]) -> Result<(String, TextEncoding), String> {
    if bytes.starts_with(b"PK\x03\x04") || is_binary(bytes) {
        return Err(format!(
            "{name} is not a text file: pick the file the line names."
        ));
    }
    TextEncoding::decode(bytes)
}

/// The deck and the files kept with it as one ZIP: the deck by its name,
/// each file by its path from the deck, each in its own encoding.
pub fn export_zip(deck: &StoredDeck) -> Result<Vec<u8>, String> {
    let mut entries = vec![(deck.name.clone(), deck.encoding.encode(&deck.text)?)];
    for file in &deck.files {
        entries.push((file.path.clone(), file.encoding.encode(&file.text)?));
    }
    let entries = entries
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect::<Vec<_>>();
    rspice_formats::zip::deterministic_stored_zip(&entries).map_err(|error| error.to_string())
}

/// A text file picked, by its path among the picked files.
struct Picked {
    path: String,
    text: String,
    encoding: TextEncoding,
}

/// The decks among `picked`: each deck file no other picked file includes,
/// with the rest kept by their paths from it.
fn decks_among(picked: &[Picked]) -> Result<Vec<StoredDeck>, String> {
    let sources = picked
        .iter()
        .map(|file| {
            (
                rspice_design::deck::stored_file_identity(&file.path),
                file.text.clone(),
            )
        })
        .collect::<Vec<_>>();
    let bundle = rspice_core::netlist::SealedSourceBundle::try_new_relative(sources)
        .map_err(|error| error.to_string())?;
    // A file is a deck's own unless something picked reads it. A file the
    // engine cannot read through still says what it read before it stopped.
    let mut included = BTreeSet::new();
    for file in picked {
        let identity = rspice_design::deck::stored_file_identity(&file.path);
        let mut processor =
            rspice_core::netlist::IncludeProcessor::new_sealed(&identity, bundle.clone());
        let _ = processor.expand_content_with_abort(
            &file.text,
            &identity,
            &rspice_core::abort_signal::NoAbort,
        );
        included.extend(
            processor.resolved_dependencies().iter().map(|dependency| {
                rspice_design::deck::stored_file_path(dependency.resolved_path())
            }),
        );
    }
    let roots = picked
        .iter()
        .filter(|file| is_deck_name(&file.path) && !included.contains(&file.path))
        .collect::<Vec<_>>();
    let rest = picked
        .iter()
        .filter(|file| !roots.iter().any(|root| root.path == file.path))
        .collect::<Vec<_>>();
    let decks = roots
        .iter()
        .map(|deck| stored_deck(deck, &rest))
        .collect::<Result<Vec<_>, _>>()?;
    if decks.is_empty() {
        return Err(
            "No deck among the files: a deck is a .cir, .sp, .net or similar file that no other \
             picked file includes."
                .to_owned(),
        );
    }
    Ok(decks)
}

/// `deck`, with each of `rest`, the picked files that are no deck, kept by
/// its path from the deck's folder. A file above that folder is kept where
/// the deck's `..` reaches, which stops at the deck's folder as the deck
/// reads it.
fn stored_deck(deck: &Picked, rest: &[&Picked]) -> Result<StoredDeck, String> {
    let folder = deck.path.rsplit_once('/').map_or("", |(folder, _)| folder);
    let name = deck
        .path
        .rsplit('/')
        .next()
        .unwrap_or(&deck.path)
        .to_owned();
    let mut files = Vec::new();
    let mut kept = BTreeSet::new();
    for file in rest {
        let path = from_folder(folder, &file.path);
        if path == name || !kept.insert(path.clone()) {
            return Err(format!(
                "{} cannot be kept with {name}: another of its files is kept as {path}.",
                file.path
            ));
        }
        files.push(StoredDeckFile {
            path,
            text: file.text.clone(),
            encoding: file.encoding,
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(StoredDeck {
        name,
        text: deck.text.clone(),
        encoding: deck.encoding,
        files,
    })
}

/// `path`, a path among the picked files, from `folder`, another: under it
/// by the rest, else by what follows the folders the two share.
fn from_folder(folder: &str, path: &str) -> String {
    let folder = folder
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    let parts = path.split('/').collect::<Vec<_>>();
    let shared = folder
        .iter()
        .zip(&parts)
        .take_while(|(left, right)| left == right)
        .count();
    parts[shared..].join("/")
}

/// `path` with `/` between its folders, and no `.`, no leading `/` and no
/// `..` that leaves the picked files.
fn plain_relative(path: &str) -> String {
    let mut parts = Vec::new();
    for part in path.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    parts.join("/")
}

/// Whether `bytes` are not text: a NUL among the first 8 KiB, as version
/// control tells a binary file. A UTF-16 file names itself first.
fn is_binary(bytes: &[u8]) -> bool {
    if bytes.starts_with(&[0xff, 0xfe]) || bytes.starts_with(&[0xfe, 0xff]) {
        return false;
    }
    bytes.iter().take(8 * 1024).any(|byte| *byte == 0)
}

/// Whether a file called `path` can be a deck, by its extension.
fn is_deck_name(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| {
            DECK_EXTENSIONS
                .iter()
                .any(|deck| deck.eq_ignore_ascii_case(extension))
        })
}
