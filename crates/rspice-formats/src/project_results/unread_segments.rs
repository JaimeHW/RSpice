//! A kept segment a saved result names but cannot read: gone from every folder, or there and
//! damaged. A damaged one says what is wrong with it instead of reading as deleted, and only its
//! own run goes without samples: the rest of the history opens. The browser keeps its segments
//! in the page's memory, so a result read after the page was left says that is where they were.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rspice_results::events::points::{EventPoint, EventPoints};
use rspice_results::store_reader::{
    ColumnReference, EventsReference, LeadReference, kept_segment_name, parse_digest,
};
use rspice_results::waveform::{ColumnSource, StoredWindow, WaveformValues, WindowLead};

/// The folders kept segments are looked for in, newest first, as this process named them.
static FOLDERS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

/// Look for kept segments in `folder` too, before every folder named earlier.
pub fn keep_segments_in(folder: &Path) {
    {
        let mut folders = FOLDERS.lock().unwrap_or_else(|p| p.into_inner());
        folders.retain(|known| known != folder);
        folders.insert(0, folder.to_path_buf());
    }
    rspice_results::store_reader::keep_segments_in(folder);
}

/// Whether `values` can be read: false for a window of a segment no folder holds, or one a
/// folder holds damaged.
pub fn readable(values: &WaveformValues) -> bool {
    values
        .stored_window()
        .is_none_or(|(source, _)| source.read(0, &mut []).is_ok())
}

/// The waveform `reference` names: its segment's window, or, where a folder holds the segment
/// damaged, a window that says what is wrong with it. A malformed reference is refused.
pub(super) fn values(reference: &ColumnReference) -> Result<WaveformValues, String> {
    let name = kept_segment_name(&parse_digest(&reference.segment)?);
    match reference.values() {
        Ok(values) if readable(&values) => Ok(values),
        Ok(missing) => Ok(damage(&name)
            .and_then(|(path, reason)| damaged(reference, path, reason).ok())
            .unwrap_or(missing)),
        Err(reason) => {
            let path = damage(&name).map_or_else(|| PathBuf::from(&name), |(path, _)| path);
            damaged(reference, path, reason)
        }
    }
}

/// Why the browser cannot read a result a session or project names: it kept the samples in the
/// page's memory, which leaving or reloading the page empties.
#[cfg(not(any(unix, windows)))]
const LEFT_WITH_THE_PAGE: &str = "this browser kept these results in the page's memory, and reloading or leaving the page \
     emptied it";

/// The kept segment file `name` that a folder holds, and what is wrong with it; `None` when no
/// folder holds one.
#[cfg(any(unix, windows))]
fn damage(name: &str) -> Option<(PathBuf, String)> {
    let folders = FOLDERS.lock().unwrap_or_else(|p| p.into_inner()).clone();
    let path = folders
        .iter()
        .map(|folder| folder.join(name))
        .find(|path| path.is_file())?;
    let reason = match rspice_results::store_reader::StoreReader::open_file(&path) {
        Err(error) => error.to_string(),
        Ok(reader) if reader.digest().is_none() => {
            "it was never closed: the end its run writes is missing".to_owned()
        }
        Ok(_) => "its rows are not the ones its name states".to_owned(),
    };
    Some((path, reason))
}

/// A build without files has no folder to hold a segment: every segment it names was in the
/// memory of a page since left.
#[cfg(not(any(unix, windows)))]
fn damage(name: &str) -> Option<(PathBuf, String)> {
    Some((PathBuf::from(name), LEFT_WITH_THE_PAGE.to_owned()))
}

fn damaged(
    reference: &ColumnReference,
    path: PathBuf,
    reason: String,
) -> Result<WaveformValues, String> {
    let digest = parse_digest(&reference.segment)?;
    let window = StoredWindow {
        first: reference.first,
        rows: reference.rows,
        lead: match reference.lead {
            None => WindowLead::None,
            Some(LeadReference::Value(value)) => WindowLead::Value(value),
            Some(LeadReference::Interpolated(fraction)) => WindowLead::Interpolated(fraction),
        },
    };
    #[cfg(any(unix, windows))]
    let failure = format!("its results store {} is damaged: {reason}", path.display());
    #[cfg(not(any(unix, windows)))]
    let failure = reason;
    let source = DamagedSegment {
        name: format!("column {} of a results store", reference.column),
        path,
        failure,
        digest: digest.0,
        column: reference.column,
        rows: reference.column_rows,
    };
    WaveformValues::stored(Arc::new(source), window)
}

/// A column of a kept segment a folder holds but that cannot be read as the one a result names.
struct DamagedSegment {
    name: String,
    path: PathBuf,
    /// What reading it says.
    failure: String,
    digest: [u8; 32],
    column: u64,
    /// The rows the saved result stated.
    rows: u64,
}

impl ColumnSource for DamagedSegment {
    fn name(&self) -> &str {
        &self.name
    }

    /// No file holds the column as it was saved, so no save keeps this file beside a project.
    fn store(&self) -> String {
        self.path
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
    }

    fn rows(&self) -> u64 {
        self.rows
    }

    fn read(&self, _first: u64, _out: &mut [f64]) -> Result<(), String> {
        Err(self.failure.clone())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.digest, self.column))
    }
}

/// The waveform `values` reads now that more folders may hold its segment: `None` when it reads
/// already, or still cannot.
pub fn read_again(values: &WaveformValues) -> Option<WaveformValues> {
    if readable(values) {
        return None;
    }
    let again = self::values(&ColumnReference::of(values)?).ok()?;
    readable(&again).then_some(again)
}

/// The event history `points` reads now that more folders may hold its segment: `None` when it
/// reads already, or still cannot.
pub fn read_events_again<P: EventPoint>(points: &EventPoints<P>) -> Option<EventPoints<P>> {
    let unread = |points: &EventPoints<P>| {
        points
            .stored_source()
            .is_some_and(|source| source.before(f64::NEG_INFINITY).is_err())
    };
    if !unread(points) {
        return None;
    }
    let again = EventsReference::of(points)?.points().ok()?;
    (!unread(&again)).then_some(again)
}
