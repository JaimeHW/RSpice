//! Bounded, cancellation-aware byte sinks for result serialization.
//!
//! Serializing a typed result is proportional to the result, and a result is
//! proportional to authored deck size. Every serializer in the workspace
//! therefore writes through [`BoundedAbortWriter`], which turns a cancelled
//! run or an oversized artifact into a typed failure at the byte that crosses
//! the boundary instead of after an unbounded allocation has already happened.
//!
//! The writer never returns a short write: it either accepts the whole buffer
//! or fails. `serde_json` treats the failure as an ordinary I/O error, so a
//! caller must read [`BoundedAbortWriter::failure`] to learn whether the error
//! was cancellation, the byte limit, or a real encoding fault.

use std::io;

use crate::abort_signal::AbortSignal;

/// Why a [`BoundedAbortWriter`] refused a write.
///
/// This is deliberately distinct from the serializer's own error: a cancelled
/// or oversized artifact is an execution outcome, not malformed data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedWriteFailure {
    /// The abort source was signalled before the write was accepted.
    Aborted,
    /// Accepting the write would exceed the declared byte limit.
    ByteLimitExceeded {
        /// The limit that was declared for this artifact.
        limit_bytes: u64,
    },
    /// The accumulated buffer could not be grown.
    AllocationFailed,
}

/// A `Vec<u8>` sink that polls an abort source and enforces a byte limit.
pub struct BoundedAbortWriter<'a> {
    abort: &'a dyn AbortSignal,
    byte_limit: u64,
    bytes: Vec<u8>,
    failure: Option<BoundedWriteFailure>,
}

impl<'a> BoundedAbortWriter<'a> {
    /// Create a writer that stops at `byte_limit` bytes or on abort.
    pub const fn new(abort: &'a dyn AbortSignal, byte_limit: u64) -> Self {
        Self {
            abort,
            byte_limit,
            bytes: Vec::new(),
            failure: None,
        }
    }

    /// The declared byte limit.
    pub const fn byte_limit(&self) -> u64 {
        self.byte_limit
    }

    /// Bytes accepted so far.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether nothing has been accepted yet.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The typed reason the last write failed, when one did.
    pub const fn failure(&self) -> Option<BoundedWriteFailure> {
        self.failure
    }

    /// Consume the writer and return the accepted bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Consume the writer and return the accepted bytes as UTF-8 text.
    ///
    /// The error carries the invalid bytes so a caller can report where the
    /// encoder produced them.
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes)
    }
}

impl io::Write for BoundedAbortWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.abort.is_aborted() {
            self.failure = Some(BoundedWriteFailure::Aborted);
            return Err(io::Error::other("serialization cancelled"));
        }
        let Some(new_len) = self.bytes.len().checked_add(buffer.len()) else {
            self.failure = Some(BoundedWriteFailure::AllocationFailed);
            return Err(io::Error::other("serialized length overflowed usize"));
        };
        if new_len as u128 > u128::from(self.byte_limit) {
            self.failure = Some(BoundedWriteFailure::ByteLimitExceeded {
                limit_bytes: self.byte_limit,
            });
            return Err(io::Error::other("serialized artifact byte limit exceeded"));
        }
        if let Err(error) = self.bytes.try_reserve(buffer.len()) {
            self.failure = Some(BoundedWriteFailure::AllocationFailed);
            return Err(io::Error::other(format!(
                "cannot allocate serialization buffer: {error}"
            )));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use super::*;
    use crate::abort_signal::{ImmediateAbort, NoAbort};

    #[test]
    fn accepted_writes_accumulate_exactly() {
        let mut writer = BoundedAbortWriter::new(&NoAbort, 16);
        assert!(writer.is_empty());
        assert_eq!(writer.write(b"abc").expect("write accepted"), 3);
        assert_eq!(writer.write(b"de").expect("write accepted"), 2);
        assert_eq!(writer.len(), 5);
        assert_eq!(writer.failure(), None);
        assert_eq!(writer.into_string().expect("utf-8"), "abcde");
    }

    #[test]
    fn byte_limit_is_reported_as_a_typed_failure() {
        let mut writer = BoundedAbortWriter::new(&NoAbort, 4);
        assert!(writer.write(b"abcd").is_ok());
        assert!(writer.write(b"e").is_err());
        assert_eq!(
            writer.failure(),
            Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes: 4 })
        );
        assert_eq!(writer.len(), 4);
    }

    #[test]
    fn abort_is_reported_before_any_byte_is_accepted() {
        let mut writer = BoundedAbortWriter::new(&ImmediateAbort, u64::MAX);
        assert!(writer.write(b"a").is_err());
        assert_eq!(writer.failure(), Some(BoundedWriteFailure::Aborted));
        assert!(writer.is_empty());
    }
}
