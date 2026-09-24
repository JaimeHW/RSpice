//! Where a file-backed source's table is read from.
//!
//! A `PWL FILE=` card names a file, and the engine opens what it is given: its
//! loader takes a path, never bytes. A stimulus definition also retains the
//! table's text in the project document, so a project that has been zipped,
//! mailed and opened where the original share is not mounted still holds the
//! data its run needs — but only if something turns those bytes back into a
//! file the engine can be pointed at.
//!
//! This module is that something, and it is the one owner of the rule every
//! reader follows:
//!
//! 1. the file the card names, when it is there to be read;
//! 2. otherwise the definition's retained copy, written once to a
//!    content-addressed file in the user's cache;
//! 3. otherwise the named file again, so the refusal names what the user typed.
//!
//! The deck writer and every preview route through [`route`], so a waveform
//! cannot draw in the instrument and then be refused by the run, or the other
//! way round.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::product::ContentDigest;
use crate::state::stimulus_library::definition::RetainedPwlFile;

/// What a reader has to find a table with.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TableSources<'a> {
    /// The folder project-relative references resolve against, once the
    /// project has one.
    pub data_root: Option<&'a Path>,
    /// The retained copy as a file, from [`materialized`].
    pub retained: Option<&'a Path>,
}

/// The file a card's table is read from, and whose it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableRoute {
    /// The file the card names, resolved against the project's data folder.
    Named(String),
    /// The definition's retained copy, because the named file is not there.
    Retained(String),
}

impl TableRoute {
    /// The path the engine is given.
    #[must_use]
    pub fn path(&self) -> &str {
        match self {
            Self::Named(path) | Self::Retained(path) => path,
        }
    }
}

/// A stored reference as the engine has to be given it.
///
/// A relative reference is relative to the project's data folder, which is what
/// lets a project be moved; the engine resolves nothing against the deck, so
/// the reference is made absolute here. An absolute path is the user's own
/// choice of a file outside the project and is left alone.
#[must_use]
pub(crate) fn resolved(stored: &str, data_root: Option<&Path>) -> String {
    let trimmed = stored.trim();
    match data_root {
        Some(root) if !trimmed.is_empty() && !Path::new(trimmed).is_absolute() => {
            root.join(trimmed).to_string_lossy().into_owned()
        }
        _ => trimmed.to_owned(),
    }
}

/// Which file this stored reference is read from here.
#[must_use]
pub(crate) fn route(stored: &str, sources: TableSources<'_>) -> TableRoute {
    let named = resolved(stored, sources.data_root);
    if Path::new(&named).is_file() {
        return TableRoute::Named(named);
    }
    match sources.retained {
        Some(retained) => TableRoute::Retained(retained.to_string_lossy().into_owned()),
        None => TableRoute::Named(named),
    }
}

/// [`route`] for a reader holding the retained table itself rather than a file
/// of it: the copy is written only once the named file has turned out not to
/// be there, so a project whose shares are all mounted writes nothing.
///
/// The second half is why the retained copy could not be used, when it could
/// not — a cache folder that cannot be written is the user's to hear about,
/// beside the missing file it failed to stand in for.
#[must_use]
pub(crate) fn route_retaining(
    stored: &str,
    data_root: Option<&Path>,
    table: Option<&RetainedPwlFile>,
) -> (TableRoute, Option<String>) {
    let named = route(
        stored,
        TableSources {
            data_root,
            retained: None,
        },
    );
    let Some(table) = table else {
        return (named, None);
    };
    if Path::new(named.path()).is_file() {
        return (named, None);
    }
    match materialized(table) {
        Ok(retained) => (
            route(
                stored,
                TableSources {
                    data_root,
                    retained: Some(&retained),
                },
            ),
            None,
        ),
        Err(failure) => (named, Some(failure)),
    }
}

/// The retained copy as a file the engine can open.
///
/// The file is named by the digest of the bytes in it, so the same table always
/// lands at the same path — which is what lets the engine's waveform cache, and
/// a deck read back later, find it again — and two definitions retaining the
/// same measurement share one copy. It is written through a temporary sibling
/// and renamed into place, so a reader never opens half a table.
///
/// A digest this process has already written or verified is not checked again:
/// previews ask every frame.
pub(crate) fn materialized(table: &RetainedPwlFile) -> Result<PathBuf, String> {
    let path = cache_directory()
        .join(short_digest(&table.digest))
        .join(file_name(&table.file_name));
    let mut verified = verified_digests()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if verified.contains(&table.digest) && path.is_file() {
        return Ok(path);
    }
    let current = std::fs::read(&path).is_ok_and(|bytes| bytes == table.contents.as_bytes());
    if !current {
        write_atomically(&path, table.contents.as_bytes()).map_err(|error| {
            format!(
                "the retained copy of '{}' could not be written to '{}': {error}",
                table.file_name,
                path.display()
            )
        })?;
    }
    verified.insert(table.digest);
    Ok(path)
}

/// Whether the file a card names still holds the bytes the definition retains.
///
/// `None` when the named file cannot be read, which is the route's business
/// rather than a disagreement.
#[must_use]
pub(crate) fn named_file_matches(named: &str, table: &RetainedPwlFile) -> Option<bool> {
    let bytes = std::fs::read(named).ok()?;
    Some(bytes == table.contents.as_bytes())
}

/// Why the engine cannot load the table in this file, or `None` when it can.
///
/// A file that is there and is not a table is the case nothing else catches:
/// the transient evaluator logs the load failure and returns the source's
/// offset, which draws as a flat line that looks like a waveform, and a run is
/// refused only after it has been dispatched. The answer is the engine's own
/// loader's, so this states no grammar of its own.
///
/// Remembered per file as it stands — its length and modification time — so a
/// preview asking every frame reads the file once and an edited file is read
/// again.
#[must_use]
pub(crate) fn engine_refusal(path: &str) -> Option<String> {
    /// More files than this at once is not a working set; start over.
    const REMEMBERED: usize = 64;
    static CHECKED: OnceLock<Mutex<HashMap<TableStamp, Option<String>>>> = OnceLock::new();

    let metadata = std::fs::metadata(path).ok()?;
    let stamp = (
        PathBuf::from(path),
        metadata.len(),
        metadata.modified().ok(),
    );
    let mut checked = CHECKED
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(answer) = checked.get(&stamp) {
        return answer.clone();
    }
    let answer = rspice_core::device::pwl_file::load_pwl_file(path)
        .err()
        .map(|error| error.to_string());
    if checked.len() >= REMEMBERED {
        checked.clear();
    }
    checked.insert(stamp, answer.clone());
    answer
}

/// One file as it stood when it was checked.
type TableStamp = (PathBuf, u64, Option<std::time::SystemTime>);

fn verified_digests() -> &'static Mutex<HashSet<ContentDigest>> {
    static VERIFIED: OnceLock<Mutex<HashSet<ContentDigest>>> = OnceLock::new();
    VERIFIED.get_or_init(Mutex::default)
}

/// The user's cache, or the system's temporary folder where there is none.
fn cache_directory() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("rspice")
        .join("stimulus-tables")
}

/// Sixteen hex digits of the digest: enough that two tables never share a
/// folder, short enough to leave the deck line readable and the path well
/// inside what Windows will open.
fn short_digest(digest: &ContentDigest) -> String {
    let mut text = digest.to_string();
    text.truncate(16);
    text
}

/// The retained file's own name, reduced to what is safe as one path component.
///
/// The name comes out of a project document, so it is not trusted to be a bare
/// file name. A retained table is always text, and the engine picks its loader
/// by extension, so a name that would select the WAV loader is given the CSV
/// one instead.
fn file_name(retained: &str) -> String {
    let base = retained
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default()
        .trim_matches('.');
    let mut name: String = base
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if name.is_empty() {
        name.push_str("table.csv");
    }
    let selects_wav_loader = Path::new(&name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"));
    if selects_wav_loader {
        name.push_str(".csv");
    }
    name
}

fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let directory = path
        .parent()
        .ok_or_else(|| std::io::Error::other("the cache path has no parent folder"))?;
    std::fs::create_dir_all(directory)?;
    // Named apart from every other writer: two windows opening the same
    // project materialize the same digest at the same moment.
    let staging = directory.join(format!(".{}.partial", uuid::Uuid::new_v4().simple()));
    std::fs::write(&staging, bytes)?;
    std::fs::rename(&staging, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&staging);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(name: &str, contents: &str) -> RetainedPwlFile {
        RetainedPwlFile::new(name, contents, 0)
    }

    #[test]
    fn a_retained_table_becomes_a_file_holding_exactly_its_bytes() {
        let retained = table("route-bytes.csv", "0 0\n1e-9 1.25\n2e-9 0\n");
        let path = materialized(&retained).expect("the cache is writable");
        assert_eq!(
            std::fs::read_to_string(&path).expect("the copy is readable"),
            retained.contents
        );
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("route-bytes.csv")
        );
        // The same bytes land at the same path, whoever asks.
        assert_eq!(materialized(&retained.clone()), Ok(path));
    }

    #[test]
    fn a_copy_someone_damaged_is_written_again() {
        let retained = table("route-damaged.csv", "0 0\n5e-9 3.3\n");
        let path = materialized(&retained).expect("the cache is writable");
        std::fs::remove_file(&path).expect("the copy can be removed");
        let again = materialized(&retained).expect("the cache is writable");
        assert_eq!(again, path);
        assert_eq!(
            std::fs::read_to_string(&again).expect("the copy is readable"),
            retained.contents
        );
    }

    #[test]
    fn a_name_from_a_document_is_reduced_to_one_safe_component() {
        assert_eq!(file_name("step.csv"), "step.csv");
        assert_eq!(file_name("..\\..\\windows\\step 1.csv"), "step_1.csv");
        assert_eq!(file_name("/etc/passwd"), "passwd");
        assert_eq!(file_name(".."), "table.csv");
        assert_eq!(file_name(""), "table.csv");
        // Text is never handed to the WAV loader.
        assert_eq!(file_name("capture.WAV"), "capture.WAV.csv");
    }

    #[test]
    fn the_named_file_wins_while_it_is_there() {
        let folder = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-table-route-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        std::fs::write(folder.join("named.csv"), "0 0\n1 1\n").expect("a named table");
        let retained = folder.join("retained.csv");
        let sources = TableSources {
            data_root: Some(&folder),
            retained: Some(&retained),
        };
        assert_eq!(
            route("named.csv", sources),
            TableRoute::Named(folder.join("named.csv").to_string_lossy().into_owned())
        );
        assert_eq!(
            route("absent.csv", sources),
            TableRoute::Retained(retained.to_string_lossy().into_owned())
        );
        // With nothing retained the refusal names the file the card does.
        assert_eq!(
            route(
                "absent.csv",
                TableSources {
                    data_root: Some(&folder),
                    retained: None
                }
            ),
            TableRoute::Named(folder.join("absent.csv").to_string_lossy().into_owned())
        );
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// A file that is there and is not a table is refused in the loader's own
    /// words, and the answer follows the file when it is rewritten.
    #[test]
    fn a_file_the_engine_cannot_load_is_refused_in_the_loaders_words() {
        let folder = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-table-refusal-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let path = folder.join("step.csv");
        let text = path.to_string_lossy().into_owned();

        std::fs::write(&path, "0 0\n200u 1\n").expect("a table with a SPICE suffix");
        let refusal = engine_refusal(&text).expect("the loader reads decimal text only");
        assert!(refusal.contains("line 2"), "{refusal}");
        assert_eq!(engine_refusal(&text), Some(refusal), "remembered");

        std::fs::write(&path, "0 0\n200e-6 1\n2e-3 1\n").expect("a plain table");
        assert_eq!(engine_refusal(&text), None);
        assert_eq!(
            engine_refusal(&folder.join("absent.csv").to_string_lossy()),
            None,
            "a missing file is the route's to report"
        );
        let _ = std::fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_named_file_is_compared_with_the_retained_bytes() {
        let folder = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-table-match-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let named = folder.join("step.csv");
        std::fs::write(&named, "0 0\n1 1\n").expect("a named table");
        let named = named.to_string_lossy().into_owned();
        assert_eq!(
            named_file_matches(&named, &table("step.csv", "0 0\n1 1\n")),
            Some(true)
        );
        assert_eq!(
            named_file_matches(&named, &table("step.csv", "0 0\n1 2\n")),
            Some(false)
        );
        assert_eq!(
            named_file_matches(
                &folder.join("absent.csv").to_string_lossy(),
                &table("step.csv", "0 0\n")
            ),
            None
        );
        let _ = std::fs::remove_dir_all(&folder);
    }
}
