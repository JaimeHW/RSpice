//! A stored window written with the rows it reads, for a reader in another memory.
//!
//! A saved file names a stored waveform by the window of the kept segment it reads, and the
//! segment is read where the file is opened. A browser's hardcopy worker is another wasm instance:
//! the page's segments are not in its memory, so a snapshot for it carries each window's rows
//! beside the window it names. The worker reads those rows, and the result keeps the identity,
//! and so the digest, the window gives it where the segment is.

use std::cell::Cell;
use std::sync::Arc;

use rspice_results::store_reader::{
    ColumnReference, LeadReference, kept_segment_name, parse_digest,
};
use rspice_results::waveform::{ColumnSource, StoredWindow, WaveformValues, WindowLead};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::numeric::sample_serde;

thread_local! {
    static CARRYING_ROWS: Cell<bool> = const { Cell::new(false) };
}

/// Run `write` as a snapshot for a reader in another memory is written or read: each stored window
/// is written as the window it names together with the rows it reads, and read from those rows.
pub fn carrying_rows<T>(write: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            CARRYING_ROWS.with(|carrying| carrying.set(self.0));
        }
    }
    let _restore = Restore(CARRYING_ROWS.with(|carrying| carrying.replace(true)));
    write()
}

pub(super) fn carrying() -> bool {
    CARRYING_ROWS.with(Cell::get)
}

/// The rows a window reads from its column: the window's own, and the row before its first when
/// its lead is interpolated from that row.
fn held_rows(window: StoredWindow) -> Option<(u64, u64)> {
    let lead_row = u64::from(matches!(window.lead, WindowLead::Interpolated(_)));
    Some((window.first.checked_sub(lead_row)?, window.rows + lead_row))
}

/// A window of a stored column as a snapshot writes it: the window a saved file names, and, in a
/// snapshot for a reader in another memory, the rows it reads.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WindowWire {
    segment: String,
    column: u64,
    column_rows: u64,
    first: u64,
    rows: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lead: Option<LeadReference>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_rows",
        deserialize_with = "deserialize_rows"
    )]
    held: Option<Vec<f64>>,
}

fn serialize_rows<S: Serializer>(
    rows: &Option<Vec<f64>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match rows {
        Some(rows) => sample_serde::serialize(rows, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_rows<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<f64>>, D::Error> {
    sample_serde::deserialize(deserializer).map(Some)
}

impl WindowWire {
    /// `reference` with the rows `values`, the window it names, reads.
    pub(super) fn carrying(
        reference: ColumnReference,
        values: &WaveformValues,
    ) -> Result<Self, String> {
        let (source, window) = values
            .stored_window()
            .ok_or("a window written with its rows is a window of a stored column")?;
        let (from, count) = held_rows(window)
            .ok_or("the window interpolates from a row before its column's first")?;
        let count = usize::try_from(count)
            .map_err(|_| "the window holds more rows than this host can address")?;
        let mut held = vec![0.0; count];
        source
            .read(from, &mut held)
            .map_err(|cause| format!("{} of {}: {cause}", source.name(), source.store()))?;
        Ok(Self {
            held: Some(held),
            ..Self::from(reference)
        })
    }

    /// The window this names, and the rows it carries when it carries them.
    pub(super) fn into_parts(self) -> (ColumnReference, Option<Vec<f64>>) {
        let reference = ColumnReference {
            segment: self.segment,
            column: self.column,
            column_rows: self.column_rows,
            first: self.first,
            rows: self.rows,
            lead: self.lead,
        };
        (reference, self.held)
    }
}

impl From<ColumnReference> for WindowWire {
    fn from(reference: ColumnReference) -> Self {
        Self {
            segment: reference.segment,
            column: reference.column,
            column_rows: reference.column_rows,
            first: reference.first,
            rows: reference.rows,
            lead: reference.lead,
            held: None,
        }
    }
}

/// The window `reference` names, read from the rows a snapshot carried for it.
pub(super) fn carried_values(
    reference: &ColumnReference,
    held: Vec<f64>,
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
    let (from, count) =
        held_rows(window).ok_or("the window interpolates from a row before its column's first")?;
    if held.len() as u64 != count {
        return Err(format!(
            "a window of {count} rows carries {} of them",
            held.len()
        ));
    }
    let axis = reference.column == 0;
    let source = CarriedColumn {
        store: format!(
            "the rows a snapshot carries of {}",
            kept_segment_name(&digest)
        ),
        name: if axis {
            "x axis".to_owned()
        } else {
            format!("column {}", reference.column)
        },
        identity: (digest.0, reference.column),
        rows: reference.column_rows,
        from,
        ascending: axis
            && held.iter().all(|value| value.is_finite())
            && held.windows(2).all(|pair| pair[0] < pair[1]),
        held,
    };
    WaveformValues::stored(Arc::new(source), window)
}

/// The rows of a stored column a snapshot carried: the ones its window reads, under the identity
/// of the column they were read from.
struct CarriedColumn {
    name: String,
    store: String,
    identity: ([u8; 32], u64),
    rows: u64,
    /// The column row the first carried row is.
    from: u64,
    held: Vec<f64>,
    ascending: bool,
}

impl ColumnSource for CarriedColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.store.clone()
    }

    fn rows(&self) -> u64 {
        self.rows
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        let rows = first
            .checked_sub(self.from)
            .and_then(|start| usize::try_from(start).ok())
            .and_then(|start| self.held.get(start..start.checked_add(out.len())?));
        match rows {
            Some(rows) => {
                out.copy_from_slice(rows);
                Ok(())
            }
            None => Err(format!(
                "rows {first}..{} are not among the rows the snapshot carries",
                first + out.len() as u64
            )),
        }
    }

    fn ascending(&self) -> bool {
        self.ascending
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some(self.identity)
    }
}
