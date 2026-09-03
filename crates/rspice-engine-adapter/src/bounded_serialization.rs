//! Shared bounded, cancellation-aware serialization primitives.

use std::io::{self, Write};

use rspice_core::abort_signal::AbortSignal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BoundedWriteFailure {
    Aborted,
    TooLarge { limit_bytes: u64 },
    Allocation(String),
    LengthOverflow,
}

pub(crate) struct BoundedAbortWriter<'a> {
    abort: &'a dyn AbortSignal,
    byte_limit: u64,
    bytes: Vec<u8>,
    failure: Option<BoundedWriteFailure>,
}

impl<'a> BoundedAbortWriter<'a> {
    pub(crate) fn new(abort: &'a dyn AbortSignal, byte_limit: u64) -> Self {
        Self {
            abort,
            byte_limit,
            bytes: Vec::new(),
            failure: None,
        }
    }

    pub(crate) fn failure(&self) -> Option<&BoundedWriteFailure> {
        self.failure.as_ref()
    }

    pub(crate) fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes)
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    fn fail(&mut self, failure: BoundedWriteFailure, message: &'static str) -> io::Error {
        self.failure = Some(failure);
        io::Error::other(message)
    }
}

impl Write for BoundedAbortWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.abort.is_aborted() {
            return Err(self.fail(
                BoundedWriteFailure::Aborted,
                "result serialization cancelled",
            ));
        }
        let Some(new_len) = self.bytes.len().checked_add(buffer.len()) else {
            return Err(self.fail(
                BoundedWriteFailure::LengthOverflow,
                "result serialization length overflow",
            ));
        };
        if new_len as u128 > self.byte_limit as u128 {
            return Err(self.fail(
                BoundedWriteFailure::TooLarge {
                    limit_bytes: self.byte_limit,
                },
                "result serialization byte limit exceeded",
            ));
        }
        if let Err(error) = self.bytes.try_reserve(buffer.len()) {
            let detail = error.to_string();
            return Err(self.fail(
                BoundedWriteFailure::Allocation(detail),
                "result serialization allocation failed",
            ));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
