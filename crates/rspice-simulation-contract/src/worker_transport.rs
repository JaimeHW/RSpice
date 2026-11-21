//! Portable numeric-series representation for the worker protocol.
//!
//! Every numeric array crosses the worker boundary as its own transferable
//! `Float64Array`. A response costs one copy out of the worker's linear
//! memory and one copy into the receiver's; `postMessage` moves the buffers
//! between the threads without copying them. Neither side caps how many
//! values or buffers cross: a run needs the memory its result needs, and an
//! allocation the memory cannot give is refused as that allocation, with the
//! size it asked for ([`try_f64_buffer`]).
//!
//! The worker that keeps a browser's results stores is asked through [`ResultsStoreRequest`].

use serde::{Deserialize, Serialize};

use crate::resource_failure::ResourceFailure;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerF64Series {
    Inline(Vec<f64>),
    Buffer { buffer: usize, len: usize },
}

/// A zeroed buffer of `len` values for `object`, or the allocation failure
/// naming the bytes it needed.
///
/// A receiver copies each transferred array into its own memory with this,
/// so a result larger than the memory left to it is refused by name instead
/// of aborting the program. In 32-bit WebAssembly no allocation can exceed
/// `isize::MAX` bytes (2 GiB) and the whole memory is at most 4 GiB; both
/// arrive here as the same refusal.
pub fn try_f64_buffer(len: usize, object: &str) -> Result<Vec<f64>, ResourceFailure> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|error| f64_allocation_failure(object, len, &error.to_string()))?;
    values.resize(len, 0.0);
    Ok(values)
}

/// The refusal for `len` values of `object` that could not be allocated.
pub fn f64_allocation_failure(object: &str, len: usize, reason: &str) -> ResourceFailure {
    ResourceFailure::Allocation {
        object: object.to_owned(),
        bytes: u64::try_from(len.saturating_mul(std::mem::size_of::<f64>())).unwrap_or(u64::MAX),
        detail: reason.to_owned(),
    }
}

/// The total value count of a request's transfer buffers, or the overflow
/// that makes it unrepresentable on this platform.
pub fn checked_worker_request_numeric_total(
    current_total: usize,
    buffer_len: usize,
) -> Result<usize, String> {
    current_total
        .checked_add(buffer_len)
        .ok_or_else(|| "worker request numeric size overflows this platform".to_owned())
}

impl WorkerF64Series {
    pub fn into_convergence_values(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        if matches!(self, Self::Inline(_)) {
            return Err("Convergence arrays must use dedicated transfer buffers".to_owned());
        }
        self.into_vec(buffers)
    }
    pub fn from_vec(values: Vec<f64>, buffers: &mut Vec<Vec<f64>>) -> Self {
        let len = values.len();
        let buffer = buffers.len();
        buffers.push(values);
        Self::Buffer { buffer, len }
    }

    pub fn into_vec(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        match self {
            Self::Inline(values) => Ok(values),
            Self::Buffer { buffer, len } => {
                let values = buffers
                    .get(buffer)
                    .ok_or_else(|| format!("missing transferable buffer {buffer}"))?;
                if values.len() != len {
                    return Err(format!(
                        "transferable buffer {buffer} has length {}, expected {len}",
                        values.len()
                    ));
                }
                Ok(values.clone())
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Inline(values) => values.len(),
            Self::Buffer { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The worker that keeps this browser's results stores: what a page asks of it.
///
/// A browser keeps a project's results store in its origin-private file system, which a page
/// cannot read or write synchronously; a dedicated worker can, through synchronous access
/// handles. The page writes a segment it holds by sending its rows, and loads a kept segment by
/// asking for its bytes. Each request crosses as JSON beside its numeric buffers: [`Self::Rows`]
/// carries x in buffer 0 and the value slots, row-major, in buffer 1; [`Self::Read`] is answered
/// with the bytes in buffer 0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ResultsStoreRequest {
    /// Hold the store `store` for this tab, unless another tab holds it, and list what it keeps.
    Open { store: String },
    /// Let go of the store this tab holds.
    Close,
    /// Start writing a segment of `schema`, which the page holds under the logical `digest`.
    Begin {
        schema: WorkerResultsSchema,
        digest: String,
    },
    /// Rows for writing `write`.
    Rows { write: u32 },
    /// Close writing `write` on the rows it took.
    Finish { write: u32 },
    /// Drop writing `write` and what it wrote.
    Abandon { write: u32 },
    /// Check that the kept segment `digest` holds what its name states, and say its length.
    Check { digest: String },
    /// Bytes `offset .. offset + len` of the kept segment `digest`.
    Read {
        digest: String,
        offset: u64,
        len: u64,
    },
    /// Delete every kept segment but `digests`, and every file no write finished.
    Keep { digests: Vec<String> },
    /// Copy into the held store the segments `digests` that another project's store keeps, as a
    /// project copied under a new identity reads them: each copy is checked against its name.
    Adopt { digests: Vec<String> },
    /// Delete every project's store but the `live` projects', the one this tab holds and any
    /// another tab holds: what a project never saved, or since deleted, left behind.
    Sweep { live: Vec<String> },
    /// `read` of the window `values` of the kept segment `digest`, a series whose run's axis is
    /// the window `x` of the same segment: what a view draws, read here so the page never reads
    /// the segment.
    Series {
        digest: String,
        x: WorkerWindow,
        values: WorkerWindow,
        read: WorkerRead,
    },
}

/// A float as its bits, so every value, NaN and the infinities among them, crosses exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkerF64(pub u64);

impl WorkerF64 {
    pub fn get(self) -> f64 {
        f64::from_bits(self.0)
    }
}

impl From<f64> for WorkerF64 {
    fn from(value: f64) -> Self {
        Self(value.to_bits())
    }
}

/// The rows of a segment's column a waveform holds, as a page names them.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerWindow {
    /// The column: 0 for the run's x axis, `slot + 1` for a value slot.
    pub column: u64,
    pub first: u64,
    pub rows: u64,
    pub lead: WorkerLead,
}

/// What a window holds before its first row.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum WorkerLead {
    None,
    Value { value: WorkerF64 },
    Interpolated { fraction: WorkerF64 },
}

/// What a view reads of a series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum WorkerRead {
    Range {
        first: u64,
        count: u64,
    },
    Tile {
        x0: WorkerF64,
        x1: WorkerF64,
        pixels: u64,
    },
    LogTile {
        x0: WorkerF64,
        x1: WorkerF64,
        pixels: u64,
    },
    /// The value at `x`, interpolated as the series is drawn (`interpolation` names how).
    Point {
        x: WorkerF64,
        interpolation: String,
    },
}

/// What a series read gave. Samples cross in buffer 0; a tile's x and y, or its least and
/// greatest values, in buffers 0 and 1, and an envelope's touched pixels as bytes in buffer 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum WorkerReadValue {
    Samples,
    TileSamples {
        before: Option<(WorkerF64, WorkerF64)>,
        after: Option<(WorkerF64, WorkerF64)>,
    },
    TileEnvelope,
    Point {
        value: Option<WorkerF64>,
    },
    /// Why it could not be read.
    Failed {
        reason: String,
    },
}

/// What the worker answered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ResultsStoreResponse {
    Opened {
        standing: ResultsStoreStanding,
        segments: Vec<KeptSegmentRecord>,
    },
    Closed,
    Begun {
        write: u32,
    },
    /// The store has no room to begin the segment: it keeps none of it.
    Full,
    /// The rows were taken, or the store filled while they were written: the write then takes
    /// no more, and closes on the rows it holds.
    Taken {
        full: bool,
    },
    /// The write closed: what the store keeps of it, and whether a full store stopped it.
    Finished {
        record: KeptSegmentRecord,
        full: bool,
    },
    Abandoned,
    Checked {
        len: u64,
    },
    /// Buffer 0 holds the bytes.
    Bytes,
    Kept {
        removed: u32,
    },
    /// The segments copied in, which the held store keeps now.
    Adopted {
        segments: Vec<KeptSegmentRecord>,
    },
    /// The stores deleted, by project.
    Swept {
        removed: Vec<String>,
    },
    Series {
        value: WorkerReadValue,
    },
    Failed {
        reason: String,
    },
}

/// Whether this tab holds a store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ResultsStoreStanding {
    /// This tab holds it, and only this tab writes it.
    Held,
    /// Another tab holds it.
    HeldElsewhere,
    /// This browser cannot keep a store, in its words.
    Unavailable { reason: String },
}

/// A segment a store keeps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeptSegmentRecord {
    /// Its logical digest, in hex.
    pub digest: String,
    /// Its length in bytes.
    pub bytes: u64,
    pub rows: u64,
}

/// The schema and run of a segment as the page sends it to be written: its x axis and analog
/// signals, and the run it records. Event signals do not cross: their changes are not rows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerResultsSchema {
    pub x: (String, String),
    pub signals: Vec<WorkerResultsSignal>,
    pub analysis: String,
    pub sweep: Vec<(String, f64)>,
    pub seed: Option<u64>,
    pub options: Vec<(String, String)>,
}

/// One analog signal: one value slot when real, two (real, imaginary) when complex.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerResultsSignal {
    pub name: String,
    pub unit: String,
    pub complex: bool,
    pub quantity: String,
    pub path: String,
}

impl WorkerResultsSchema {
    /// The schema and run of a segment, or why it cannot be sent as rows: an event signal.
    pub fn of(
        schema: &rspice_results_store::Schema,
        meta: &rspice_results_store::RunMeta,
    ) -> Result<Self, String> {
        use rspice_results_store::SignalKind;
        let signals = schema
            .signals
            .iter()
            .map(|signal| {
                let complex = match signal.kind {
                    SignalKind::Real => false,
                    SignalKind::Complex => true,
                    _ => {
                        return Err(format!(
                            "'{}' is an event signal, whose changes are not rows",
                            signal.name
                        ));
                    }
                };
                Ok(WorkerResultsSignal {
                    name: signal.name.clone(),
                    unit: signal.unit.clone(),
                    complex,
                    quantity: signal.quantity.clone(),
                    path: signal.path.clone(),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            x: (schema.x.name.clone(), schema.x.unit.clone()),
            signals,
            analysis: meta.analysis.clone(),
            sweep: meta.sweep.clone(),
            seed: meta.seed,
            options: meta.options.clone(),
        })
    }

    /// The schema and run the worker writes.
    pub fn store(&self) -> (rspice_results_store::Schema, rspice_results_store::RunMeta) {
        use rspice_results_store::{Axis, RunMeta, Schema, Signal, SignalKind};
        let signals = self
            .signals
            .iter()
            .map(|signal| Signal {
                name: signal.name.clone(),
                unit: signal.unit.clone(),
                kind: if signal.complex {
                    SignalKind::Complex
                } else {
                    SignalKind::Real
                },
                quantity: signal.quantity.clone(),
                path: signal.path.clone(),
            })
            .collect();
        let schema = Schema {
            x: Axis {
                name: self.x.0.clone(),
                unit: self.x.1.clone(),
            },
            signals,
        };
        let meta = RunMeta {
            analysis: self.analysis.clone(),
            sweep: self.sweep.clone(),
            seed: self.seed,
            options: self.options.clone(),
        };
        (schema, meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_buffer_the_memory_cannot_hold_is_refused_with_the_size_it_needed() {
        assert_eq!(
            try_f64_buffer(3, "x").expect("three values fit"),
            vec![0.0; 3]
        );
        let len = usize::MAX / 4;
        let failure = try_f64_buffer(len, "transfer buffer 7").expect_err("cannot be had");
        let ResourceFailure::Allocation { object, bytes, .. } = &failure else {
            panic!("an allocation failure, not {failure:?}")
        };
        assert_eq!(object, "transfer buffer 7");
        assert_eq!(*bytes, u64::try_from(usize::MAX).unwrap());
        assert!(
            failure.to_string().starts_with(&format!(
                "Unable to allocate transfer buffer 7: {} bytes were needed: ",
                usize::MAX
            )),
            "{failure}"
        );
        assert_eq!(failure.code(), "allocation_failed");
    }

    fn signal(name: &str, kind: rspice_results_store::SignalKind) -> rspice_results_store::Signal {
        rspice_results_store::Signal {
            quantity: "voltage".to_owned(),
            path: "x1".to_owned(),
            ..rspice_results_store::Signal::new(name, "V", kind)
        }
    }

    #[test]
    fn a_segment_crosses_to_the_store_worker_as_the_schema_it_was_written_with() {
        use rspice_results_store::{Axis, RunMeta, Schema, SignalKind};
        let schema = Schema {
            x: Axis {
                name: "time".to_owned(),
                unit: "s".to_owned(),
            },
            signals: vec![
                signal("v(out)", SignalKind::Real),
                signal("h", SignalKind::Complex),
            ],
        };
        let meta = RunMeta {
            analysis: ".tran 1u 1m".to_owned(),
            sweep: vec![("r1".to_owned(), 1e3)],
            seed: Some(7),
            options: vec![("reltol".to_owned(), "1e-4".to_owned())],
        };
        let sent = WorkerResultsSchema::of(&schema, &meta).expect("analog signals cross");
        let request = ResultsStoreRequest::Begin {
            schema: sent,
            digest: "ab".repeat(32),
        };
        let wire = serde_json::to_string(&request).expect("written");
        let ResultsStoreRequest::Begin { schema: read, .. } =
            serde_json::from_str(&wire).expect("read back")
        else {
            panic!("a begin request, not {wire}")
        };
        assert_eq!(read.store(), (schema, meta));
    }

    #[test]
    fn a_series_read_crosses_every_float_exactly() {
        let lead = f64::NAN;
        let request = ResultsStoreRequest::Series {
            digest: "cd".repeat(32),
            x: WorkerWindow {
                column: 0,
                first: 3,
                rows: 10,
                lead: WorkerLead::Value { value: lead.into() },
            },
            values: WorkerWindow {
                column: 2,
                first: 3,
                rows: 10,
                lead: WorkerLead::Interpolated {
                    fraction: 0.25.into(),
                },
            },
            read: WorkerRead::Tile {
                x0: f64::NEG_INFINITY.into(),
                x1: 1e-300.into(),
                pixels: 800,
            },
        };
        let wire = serde_json::to_string(&request).expect("written");
        let read: ResultsStoreRequest = serde_json::from_str(&wire).expect("read back");
        assert_eq!(read, request);
        let ResultsStoreRequest::Series { x, read, .. } = read else {
            panic!("a series request, not {wire}")
        };
        let WorkerLead::Value { value } = x.lead else {
            panic!("a value lead")
        };
        assert!(value.get().is_nan());
        assert_eq!(
            read,
            WorkerRead::Tile {
                x0: f64::NEG_INFINITY.into(),
                x1: 1e-300.into(),
                pixels: 800
            }
        );
    }

    #[test]
    fn an_event_signal_is_refused_as_rows() {
        use rspice_results_store::{RunMeta, Schema, SignalKind};
        let schema = Schema {
            signals: vec![signal("d", SignalKind::Logic)],
            ..Schema::default()
        };
        let refused = WorkerResultsSchema::of(&schema, &RunMeta::default())
            .expect_err("an event signal has no rows");
        assert!(refused.contains("'d' is an event signal"), "{refused}");
    }

    #[test]
    fn a_request_total_only_refuses_what_this_platform_cannot_count() {
        assert_eq!(checked_worker_request_numeric_total(10, 20), Ok(30));
        assert!(checked_worker_request_numeric_total(usize::MAX, 1).is_err());
    }
}
