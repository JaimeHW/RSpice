//! The one error type every store operation returns.

use std::fmt;
use std::io;

/// Why a store operation stopped.
///
/// Storage exhaustion is typed so a frontend can report "stopped: storage full, partial results kept"
/// rather than a generic I/O failure; everything already written stays readable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    /// The backing store reported that it is out of space.
    DiskFull,
    /// The in-memory backend could not grow.
    MemoryFull,
    /// Any other I/O failure, with the operating system's message.
    Io {
        kind: io::ErrorKind,
        message: String,
    },
    /// Bytes that do not decode as the format says they must.
    Corrupt { offset: u64, reason: String },
    /// A row carried a different number of values than the schema has value slots.
    RowWidth { expected: usize, found: usize },
    /// An amendment named a row that is no longer in the open chunk, or a slot outside the row.
    RowNotOpen { row: u64, slot: usize },
    /// An event named a signal that is not an event signal, or carried a value of the wrong kind.
    EventSignal { signal: u32, reason: &'static str },
    /// The schema cannot be stored as given.
    Schema(String),
    /// A read named a signal, slot or row range the segment does not have.
    OutOfRange(String),
    /// The writer was used after it stopped on an earlier error.
    WriterStopped,
}

impl StoreError {
    pub(crate) fn corrupt(offset: u64, reason: impl Into<String>) -> Self {
        Self::Corrupt {
            offset,
            reason: reason.into(),
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::StorageFull => Self::DiskFull,
            io::ErrorKind::OutOfMemory => Self::MemoryFull,
            kind => Self::Io {
                kind,
                message: error.to_string(),
            },
        }
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DiskFull => f.write_str("the results store is out of disk space"),
            Self::MemoryFull => f.write_str("the in-memory results store could not grow"),
            Self::Io { message, .. } => write!(f, "results store I/O failed: {message}"),
            Self::Corrupt { offset, reason } => {
                write!(f, "results segment is damaged at byte {offset}: {reason}")
            }
            Self::RowWidth { expected, found } => {
                write!(
                    f,
                    "a result row carries {found} values; the schema has {expected} value slots"
                )
            }
            Self::RowNotOpen { row, slot } => {
                write!(
                    f,
                    "row {row}, slot {slot} is not in the open chunk and can no longer be amended"
                )
            }
            Self::EventSignal { signal, reason } => {
                write!(f, "event for signal {signal}: {reason}")
            }
            Self::Schema(reason) => write!(f, "results schema: {reason}"),
            Self::OutOfRange(reason) => write!(f, "results read out of range: {reason}"),
            Self::WriterStopped => {
                f.write_str("the results writer already stopped on an earlier error")
            }
        }
    }
}

impl std::error::Error for StoreError {}
