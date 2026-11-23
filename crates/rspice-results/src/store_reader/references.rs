//! How a saved result names what a kept segment holds for it, in place of the samples.
//!
//! A project, a recovery or a session writes a stored waveform as the window of a column of a
//! segment named by its logical digest, and a stored event history as a signal of one, and
//! keeps the segments beside the file. Reading the file back finds each segment by its digest
//! ([`super::kept_segment`]); one still waiting for its first check reads as being checked until
//! it has had it, and one no folder holds, or that failed its check, reads as missing and names
//! its file.

use std::cell::Cell;
use std::sync::Arc;

use rspice_results_store::Digest;

use super::pending_sources::{CheckingColumn, CheckingEvents};
use super::{MissingSegment, Segment, SegmentColumn, SegmentEvents, kept_segment};
use crate::events::points::{EventHistorySource, EventPoint, EventPoints};
use crate::waveform::{ColumnSource, StoredWindow, WaveformValues, WindowLead};

thread_local! {
    static WRITING_REFERENCES: Cell<bool> = const { Cell::new(false) };
}

/// Run `write` with stored event histories serialized as [`EventsReference`]s rather than
/// their points: what a saved result writes.
pub fn writing_references<T>(write: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            WRITING_REFERENCES.with(|writing| writing.set(self.0));
        }
    }
    let _restore = Restore(WRITING_REFERENCES.with(|writing| writing.replace(true)));
    write()
}

/// Whether [`writing_references`] is running here.
pub(crate) fn references_written() -> bool {
    WRITING_REFERENCES.with(Cell::get)
}

/// The digest `hex` writes, as a reference names a segment.
pub fn parse_digest(hex: &str) -> Result<Digest, String> {
    digest_of(hex)
}

fn digest_of(hex: &str) -> Result<Digest, String> {
    let bytes = hex.as_bytes();
    if bytes.len() != 64 {
        return Err(format!("'{hex}' is not a results store's digest"));
    }
    let mut digest = [0_u8; 32];
    for (byte, pair) in digest.iter_mut().zip(bytes.chunks(2)) {
        let pair = std::str::from_utf8(pair).map_err(|error| error.to_string())?;
        *byte = u8::from_str_radix(pair, 16)
            .map_err(|_| format!("'{hex}' is not a results store's digest"))?;
    }
    Ok(Digest(digest))
}

/// What a stored window holds before its first row, as a saved result names it.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum LeadReference {
    Value(f64),
    Interpolated(f64),
}

/// A window of a column of a kept segment, as a saved result names it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnReference {
    /// The segment's logical digest.
    pub segment: String,
    /// The column: 0 for the run's x axis, else its value slot plus one.
    pub column: u64,
    /// The rows the column holds.
    pub column_rows: u64,
    pub first: u64,
    pub rows: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lead: Option<LeadReference>,
    /// Whether the column's rows ascend strictly, as its run checked them: what a reader knows
    /// of its run's axis while the segment waits for its first check.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ascending: bool,
    /// The digest of the whole column ([`rspice_results_store::ColumnDigest`]), as its store
    /// stated it: the result keeps its identity where the segment is missing or still being
    /// checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_digest: Option<String>,
}

impl ColumnReference {
    /// The reference to `values`, when they are a window of a column whose store states its
    /// digest.
    pub fn of(values: &WaveformValues) -> Option<Self> {
        let (source, window) = values.stored_window()?;
        let (digest, column) = source.identity()?;
        Some(Self {
            segment: Digest(digest).to_string(),
            column,
            column_rows: source.rows(),
            first: window.first,
            rows: window.rows,
            lead: match window.lead {
                WindowLead::None => None,
                WindowLead::Value(value) => Some(LeadReference::Value(value)),
                WindowLead::Interpolated(fraction) => Some(LeadReference::Interpolated(fraction)),
            },
            ascending: source.ascending(),
            column_digest: source
                .column_digest()
                .map(|digest| Digest(digest).to_string()),
        })
    }

    /// The column digest the reference states.
    pub fn stated_column_digest(&self) -> Result<Option<[u8; 32]>, String> {
        self.column_digest
            .as_deref()
            .map(|hex| digest_of(hex).map(|digest| digest.0))
            .transpose()
    }

    /// The window, read from the kept segment it names, or showing as unavailable where no
    /// folder holds the segment. A segment still waiting for its first check is read once the
    /// check ends.
    pub fn values(&self) -> Result<WaveformValues, String> {
        let digest = digest_of(&self.segment)?;
        let window = StoredWindow {
            first: self.first,
            rows: self.rows,
            lead: match self.lead {
                None => WindowLead::None,
                Some(LeadReference::Value(value)) => WindowLead::Value(value),
                Some(LeadReference::Interpolated(fraction)) => WindowLead::Interpolated(fraction),
            },
        };
        let column_digest = self.stated_column_digest()?;
        let source: Arc<dyn ColumnSource> = match kept_segment(digest) {
            Some(segment) => segment_column(segment, digest, self.column, self.column_rows)?,
            None if super::checking(digest.0) => CheckingColumn::new(
                digest,
                self.column,
                self.column_rows,
                self.ascending,
                column_digest,
                format!("column {} of a results store being checked", self.column),
            ),
            None => MissingSegment::column(
                digest,
                self.column,
                self.column_rows,
                self.ascending,
                column_digest,
                format!("column {} of a missing results store", self.column),
            ),
        };
        WaveformValues::stored(source, window)
    }
}

/// Column `column` of `segment`, whose logical digest is `digest`, as a result that names
/// `column_rows` rows reads it. A segment of other rows is refused.
pub(super) fn segment_column(
    segment: Arc<Segment>,
    digest: Digest,
    column: u64,
    column_rows: u64,
) -> Result<Arc<dyn ColumnSource>, String> {
    if column_rows != segment.rows() {
        return Err(format!(
            "the results store {} does not hold the {column_rows} rows its result names",
            rspice_results_store::kept_segment_name(&digest),
        ));
    }
    let slot = match column.checked_sub(1) {
        None => None,
        Some(slot) => {
            Some(usize::try_from(slot).map_err(|_| format!("column {column} is out of range"))?)
        }
    };
    let name = column_name(&segment, slot);
    Ok(SegmentColumn::new(segment, slot, name))
}

/// Event signal `signal` of `segment`.
pub(super) fn segment_events(
    segment: Arc<Segment>,
    signal: u64,
) -> Result<Arc<dyn EventHistorySource>, String> {
    let signal = usize::try_from(signal).map_err(|_| format!("signal {signal} is out of range"))?;
    let name = segment
        .with_reader(|reader| Ok(reader.schema().signals.get(signal).map(|s| s.name.clone())))
        .ok()
        .flatten()
        .unwrap_or_else(|| format!("signal {signal}"));
    Ok(SegmentEvents::new(segment, signal, name))
}

/// The name of the column at `slot`, read from the segment's slot index: a result naming
/// every column of a wide segment once looked each one up by summing the slots of every
/// signal before it, for every signal, from a fresh copy of the schema.
fn column_name(segment: &Segment, slot: Option<usize>) -> String {
    let Some(slot) = slot else {
        return "x axis".to_owned();
    };
    segment
        .slot_name(slot)
        .unwrap_or_else(|| format!("slot {slot}"))
}

/// An event signal of a kept segment from a time on, as a saved result names it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventsReference {
    /// The segment's logical digest.
    pub segment: String,
    pub signal: u64,
    /// The time the history starts from; none for the whole signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    /// The changes the history holds from then on.
    pub changes: u64,
}

impl EventsReference {
    /// The reference to `points`, when they are a stored signal whose store states its digest.
    pub fn of<P: EventPoint>(points: &EventPoints<P>) -> Option<Self> {
        let ((digest, signal), from) = points.stored_identity()?;
        Some(Self {
            segment: Digest(digest).to_string(),
            signal,
            from: (from > f64::NEG_INFINITY).then_some(from),
            changes: points.len() as u64,
        })
    }

    /// The history, read from the kept segment it names, or showing no points where no folder
    /// holds the segment. A segment still waiting for its first check is read once the check
    /// ends.
    pub fn points<P: EventPoint>(&self) -> Result<EventPoints<P>, String> {
        let digest = digest_of(&self.segment)?;
        let from = self.from.unwrap_or(f64::NEG_INFINITY);
        let source: Arc<dyn EventHistorySource> = match kept_segment(digest) {
            Some(segment) => segment_events(segment, self.signal)?,
            None if super::checking(digest.0) => CheckingEvents::new(
                digest,
                self.signal,
                self.changes,
                format!("signal {} of a results store being checked", self.signal),
            ),
            None => MissingSegment::new(
                digest,
                self.signal,
                self.changes,
                format!("signal {} of a missing results store", self.signal),
            ),
        };
        EventPoints::stored_holding(source, from, self.changes)
    }
}
