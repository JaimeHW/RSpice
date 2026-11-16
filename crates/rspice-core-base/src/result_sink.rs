//! Where an analysis writes the points it accepts.
//!
//! An analysis hands each accepted point to a [`ResultSink`] as one row: its x and the saved
//! values in schema order, gathered by the engine into a buffer it reuses. The sink decides where
//! the rows go. [`VecSink`] keeps them in memory, column by column, which is what every in-memory
//! result is rebuilt from; a results store streams them to disk.
//!
//! Only accepted points reach a sink. Rejected trials, Newton retries and restored states never
//! do, so a sink has no rollback. The one retroactive write is [`ResultSink::amend_open_row`],
//! which corrects a value of a row that is still open (row 0's initial-condition capacitor
//! currents, known only once the first step is accepted).
//!
//! The module is a layer-0 leaf: it names nothing else in the crate, so an analysis at any rank
//! can write through it and a store crate can implement it without reaching above it. Its types
//! mirror the store format's own; an adapter maps one onto the other.

use std::fmt;

/// The x axis of a run's rows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResultAxis {
    pub name: String,
    pub unit: String,
}

/// What one saved signal records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultSignalKind {
    /// One value slot per row.
    Real,
    /// Two value slots per row: real part, then imaginary part.
    Complex,
    /// A digital net: event changes of 0, 1, X or Z with strength. No value slot.
    Logic,
    /// A real-valued event net. No value slot.
    RealEvent,
    /// A digital bus of `width` bits. No value slot.
    Bus { width: u32 },
}

impl ResultSignalKind {
    /// Value slots this signal takes in each row; event signals take none.
    pub fn slots(self) -> usize {
        match self {
            Self::Real => 1,
            Self::Complex => 2,
            Self::Logic | Self::RealEvent | Self::Bus { .. } => 0,
        }
    }
}

/// One saved signal. Its id is its position in [`ResultSchema::signals`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultSignal {
    pub name: String,
    pub unit: String,
    pub kind: ResultSignalKind,
    /// What the value measures (`voltage`, `current`, ...).
    pub quantity: String,
    /// Hierarchical instance path, empty at the top level.
    pub path: String,
}

impl ResultSignal {
    pub fn new(name: impl Into<String>, unit: impl Into<String>, kind: ResultSignalKind) -> Self {
        Self {
            name: name.into(),
            unit: unit.into(),
            kind,
            quantity: String::new(),
            path: String::new(),
        }
    }
}

/// The column table of a run: a row is x plus, in signal order, each signal's value slots.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResultSchema {
    pub x: ResultAxis,
    pub signals: Vec<ResultSignal>,
}

impl ResultSchema {
    /// Value slots in each row.
    pub fn slots(&self) -> usize {
        self.signals.iter().map(|signal| signal.kind.slots()).sum()
    }
}

/// What the run was: its analysis card, sweep coordinates, seed and options.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RunMeta {
    pub analysis: String,
    pub sweep: Vec<(String, f64)>,
    pub seed: Option<u64>,
    pub options: Vec<(String, String)>,
}

/// A four-state logic value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Logic {
    Zero,
    One,
    X,
    Z,
}

/// The value an event signal changes to.
#[derive(Debug, Clone, PartialEq)]
pub enum EventValue {
    Logic(Logic),
    Real(f64),
    /// A bus as three bit strings of `width.div_ceil(8)` bytes each, least significant byte first:
    /// bit `i` is X when `x` has it set, else Z when `z` has it set, else the bit of `value`.
    Bus {
        value: Vec<u8>,
        x: Vec<u8>,
        z: Vec<u8>,
    },
}

/// One change of one event signal.
#[derive(Debug, Clone, PartialEq)]
pub struct EventChange {
    /// The signal's id: its position in the schema.
    pub signal: u32,
    pub time: f64,
    /// Commit order among changes at the same time.
    pub sequence: u32,
    pub value: EventValue,
    /// Drive strength.
    pub strength: u8,
}

/// Why a run stopped before completing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopReason {
    DiskFull,
    MemoryFull,
    Cancelled,
    Timeout,
    Failed(String),
}

/// How a run ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    Completed,
    Stopped(StopReason),
}

/// What a sink kept, once the run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SinkReceipt {
    pub rows: u64,
    pub events: u64,
    /// Bytes written to storage; 0 for a sink that keeps rows in memory.
    pub bytes: u64,
    /// The logical digest of what was kept, from a sink that computes one.
    pub digest: Option<[u8; 32]>,
}

/// Why a sink refused a write. Everything accepted before it stays readable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkStop {
    /// The storage is full. The bytes are known when a preflight measured them; the error the
    /// operating system returns carries neither.
    DiskFull {
        needed: Option<u64>,
        available: Option<u64>,
    },
    /// Memory could not grow to hold what the sink buffers.
    MemoryFull { bytes: Option<u64> },
    /// Any other failure, with its message.
    Io(String),
}

impl fmt::Display for SinkStop {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DiskFull { needed, available } => {
                formatter.write_str("the results storage is full")?;
                match (needed, available) {
                    (Some(needed), Some(available)) => write!(
                        formatter,
                        " ({needed} bytes needed, {available} bytes available)"
                    ),
                    (Some(needed), None) => write!(formatter, " ({needed} bytes needed)"),
                    (None, Some(available)) => {
                        write!(formatter, " ({available} bytes available)")
                    }
                    (None, None) => Ok(()),
                }
            }
            Self::MemoryFull { bytes: Some(bytes) } => {
                write!(formatter, "results memory is full ({bytes} bytes)")
            }
            Self::MemoryFull { bytes: None } => formatter.write_str("results memory is full"),
            Self::Io(message) => write!(formatter, "results could not be written: {message}"),
        }
    }
}

impl std::error::Error for SinkStop {}

/// Where an analysis writes its accepted points.
///
/// The analysis calls [`begin`](Self::begin) once, then [`row`](Self::row) for every accepted
/// point in order, with [`amend_open_row`](Self::amend_open_row) and [`events`](Self::events)
/// between them. Whoever handed the sink to the run calls [`end`](Self::end) with how the run
/// ended, which may come without `begin` when the run stopped before its first row. A refusal
/// stops the run; the rows already accepted stay.
///
/// `row` is in the solve loop: an implementation copies the values and returns, with no
/// allocation beyond amortised growth and no lookup per value.
pub trait ResultSink {
    /// The run's columns and what it is, before its first row.
    fn begin(&mut self, schema: &ResultSchema, meta: &RunMeta) -> Result<(), SinkStop>;

    /// One accepted point: x and the saved values in schema order, gathered by the engine.
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop>;

    /// Overwrite value slot `column` of row `row`. Only a row still open can be amended: in
    /// practice row 0, before row 1 arrives.
    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop>;

    /// Accepted event changes, in commit order.
    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop>;

    /// The run ended with `status`: close what was written and say what was kept.
    fn end(&mut self, status: RunStatus) -> Result<SinkReceipt, SinkStop>;
}

/// A borrowed sink is a sink: a run can write through a sink its caller keeps.
impl<S: ResultSink + ?Sized> ResultSink for &mut S {
    fn begin(&mut self, schema: &ResultSchema, meta: &RunMeta) -> Result<(), SinkStop> {
        (**self).begin(schema, meta)
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        (**self).row(x, values)
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        (**self).amend_open_row(row, column, value)
    }

    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop> {
        (**self).events(changes)
    }

    fn end(&mut self, status: RunStatus) -> Result<SinkReceipt, SinkStop> {
        (**self).end(status)
    }
}

/// A sink that keeps every row in memory, x and each value slot in its own column.
///
/// This is what an in-memory result is rebuilt from: the columns are the values the engine
/// gathered, bit for bit, in the order it accepted them.
#[derive(Debug, Default)]
pub struct VecSink {
    capacity: usize,
    x: Vec<f64>,
    columns: Vec<Vec<f64>>,
    events: Vec<EventChange>,
}

impl VecSink {
    pub fn new() -> Self {
        Self::default()
    }

    /// A sink that reserves room for `rows` rows in every column when the run begins.
    pub fn with_capacity(rows: usize) -> Self {
        Self {
            capacity: rows,
            ..Self::default()
        }
    }

    /// The x of every row so far.
    pub fn x(&self) -> &[f64] {
        &self.x
    }

    /// One column per value slot, in schema order.
    pub fn columns(&self) -> &[Vec<f64>] {
        &self.columns
    }

    /// Every event change so far, in commit order.
    pub fn event_changes(&self) -> &[EventChange] {
        &self.events
    }

    /// The x column, the value columns and the event changes.
    pub fn into_parts(self) -> (Vec<f64>, Vec<Vec<f64>>, Vec<EventChange>) {
        (self.x, self.columns, self.events)
    }
}

impl ResultSink for VecSink {
    fn begin(&mut self, schema: &ResultSchema, _meta: &RunMeta) -> Result<(), SinkStop> {
        self.x = Vec::with_capacity(self.capacity);
        self.columns = (0..schema.slots())
            .map(|_| Vec::with_capacity(self.capacity))
            .collect();
        self.events.clear();
        Ok(())
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        if values.len() != self.columns.len() {
            return Err(row_width(values.len(), self.columns.len()));
        }
        self.x.push(x);
        for (column, &value) in self.columns.iter_mut().zip(values) {
            column.push(value);
        }
        Ok(())
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        let slot = usize::try_from(row).ok().and_then(|row| {
            self.columns
                .get_mut(column as usize)
                .and_then(|values| values.get_mut(row))
        });
        match slot {
            Some(slot) => {
                *slot = value;
                Ok(())
            }
            None => Err(SinkStop::Io(format!(
                "value slot {column} of row {row} is not open (rows: {}, slots: {})",
                self.x.len(),
                self.columns.len()
            ))),
        }
    }

    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop> {
        self.events.extend_from_slice(changes);
        Ok(())
    }

    fn end(&mut self, _status: RunStatus) -> Result<SinkReceipt, SinkStop> {
        Ok(SinkReceipt {
            rows: self.x.len() as u64,
            events: self.events.len() as u64,
            bytes: 0,
            digest: None,
        })
    }
}

#[cold]
fn row_width(found: usize, expected: usize) -> SinkStop {
    SinkStop::Io(format!(
        "a row of {found} values for a schema of {expected} value slots"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> ResultSchema {
        ResultSchema {
            x: ResultAxis {
                name: "time".into(),
                unit: "s".into(),
            },
            signals: vec![
                ResultSignal::new("out", "V", ResultSignalKind::Real),
                ResultSignal::new("d", "", ResultSignalKind::Logic),
                ResultSignal::new("i(v1)", "A", ResultSignalKind::Real),
            ],
        }
    }

    #[test]
    fn a_vec_sink_keeps_each_slot_as_a_column_bit_for_bit() {
        let mut sink = VecSink::with_capacity(4);
        sink.begin(&schema(), &RunMeta::default()).unwrap();
        sink.row(0.0, &[-0.0, 1.0]).unwrap();
        sink.row(1e-9, &[f64::NAN, f64::from_bits(0x7ff8_0000_dead_beef)])
            .unwrap();
        sink.amend_open_row(0, 1, 2.5).unwrap();
        let receipt = sink.end(RunStatus::Completed).unwrap();
        assert_eq!(receipt.rows, 2);
        let (x, columns, events) = sink.into_parts();
        assert_eq!(x, [0.0, 1e-9]);
        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0][0].to_bits(), (-0.0f64).to_bits());
        assert!(columns[0][1].is_nan());
        assert_eq!(columns[1][0], 2.5);
        assert_eq!(columns[1][1].to_bits(), 0x7ff8_0000_dead_beef);
        assert!(events.is_empty());
    }

    #[test]
    fn a_vec_sink_refuses_a_row_of_the_wrong_width_and_an_amendment_outside_its_rows() {
        let mut sink = VecSink::new();
        sink.begin(&schema(), &RunMeta::default()).unwrap();
        assert!(matches!(sink.row(0.0, &[1.0]), Err(SinkStop::Io(_))));
        assert!(sink.x().is_empty());
        sink.row(0.0, &[1.0, 2.0]).unwrap();
        assert!(matches!(
            sink.amend_open_row(1, 0, 0.0),
            Err(SinkStop::Io(_))
        ));
        assert!(matches!(
            sink.amend_open_row(0, 2, 0.0),
            Err(SinkStop::Io(_))
        ));
    }

    #[test]
    fn a_sink_stop_says_what_ran_out() {
        assert_eq!(
            SinkStop::DiskFull {
                needed: Some(10),
                available: Some(4)
            }
            .to_string(),
            "the results storage is full (10 bytes needed, 4 bytes available)"
        );
        assert_eq!(
            SinkStop::DiskFull {
                needed: None,
                available: None
            }
            .to_string(),
            "the results storage is full"
        );
        assert_eq!(
            SinkStop::MemoryFull { bytes: None }.to_string(),
            "results memory is full"
        );
    }

    #[test]
    fn the_sink_trait_is_object_safe() {
        let mut sink = VecSink::new();
        let dynamic: &mut dyn ResultSink = &mut sink;
        dynamic.begin(&schema(), &RunMeta::default()).unwrap();
        dynamic.row(0.0, &[1.0, 2.0]).unwrap();
        assert_eq!(dynamic.end(RunStatus::Completed).unwrap().rows, 1);
    }
}
