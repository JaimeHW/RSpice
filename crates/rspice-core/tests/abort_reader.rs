//! Cooperative cancellation must escape standard readers and JSON decoders.
use rspice_core::abort_signal::{AbortReader, AbortSignal, NoAbort};
use std::io::{self, Cursor, Read};
use std::sync::atomic::{AtomicUsize, Ordering};

struct StopAfter {
    allowed: usize,
    polls: AtomicUsize,
}

impl StopAfter {
    fn new(allowed: usize) -> Self {
        Self {
            allowed,
            polls: AtomicUsize::new(0),
        }
    }
}

impl AbortSignal for StopAfter {
    fn is_aborted(&self) -> bool {
        let poll = self.polls.fetch_add(1, Ordering::Relaxed);
        // Fail a regression deterministically instead of allowing the test
        // to spin forever if a decoder retries the cancellation error.
        assert!(
            poll <= self.allowed,
            "reader retried cooperative cancellation"
        );
        poll == self.allowed
    }
}

struct Chunked<'a>(&'a mut Cursor<&'static [u8]>);

impl Read for Chunked<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = buffer.len().min(3);
        self.0.read(&mut buffer[..count])
    }
}

#[test]
fn standard_readers_stop_after_partial_progress_without_retrying_cancellation() {
    for exact in [false, true] {
        let abort = StopAfter::new(1);
        let mut source = Cursor::new(b"abcdef".as_slice());
        let mut reader = AbortReader::new(Chunked(&mut source), &abort, "cancelled test input");
        let error = if exact {
            reader.read_exact(&mut [0; 6]).unwrap_err()
        } else {
            let mut bytes = Vec::new();
            let error = reader.read_to_end(&mut bytes).unwrap_err();
            assert_eq!(bytes, b"abc");
            error
        };
        assert_ne!(error.kind(), io::ErrorKind::Interrupted);
        assert!(reader.was_cancelled());
        assert!(!reader.exceeded_cap());
        assert_eq!(source.position(), 3);
    }
}

#[test]
fn json_cache_decoder_propagates_cancellation_after_partial_input() {
    let abort = StopAfter::new(3);
    let mut reader =
        AbortReader::with_byte_cap(b"[1,2,3]".as_slice(), &abort, "cache cancelled", 64);
    let error = serde_json::from_reader::<_, Vec<u32>>(&mut reader).unwrap_err();
    assert!(error.is_io(), "{error}");
    assert!(reader.was_cancelled());
    assert!(!reader.exceeded_cap());
    assert_eq!(reader.bytes_read(), 3);
}

#[test]
fn uncapped_reads_report_the_bytes_delivered_to_the_caller() {
    let mut reader = AbortReader::new(b"abcdef".as_slice(), &NoAbort, "test input");
    let mut bytes = [0; 3];
    reader.read_exact(&mut bytes).unwrap();
    assert_eq!(reader.bytes_read(), 3);
    reader.read_exact(&mut bytes).unwrap();
    assert_eq!(reader.bytes_read(), 6);
}

#[test]
fn empty_reads_at_the_cap_do_not_consume_or_probe_the_source() {
    let mut source = Cursor::new(b"ab".as_slice());
    let mut reader = AbortReader::with_byte_cap(&mut source, &NoAbort, "test input", 1);
    reader.read_exact(&mut [0; 1]).unwrap();
    assert_eq!(reader.read(&mut []).unwrap(), 0);
    assert_eq!(reader.bytes_read(), 1);
    assert!(!reader.exceeded_cap());
    assert_eq!(source.position(), 1);
}

#[test]
fn failed_cap_probe_is_terminal_without_consuming_more_input() {
    let mut source = Cursor::new(b"abc".as_slice());
    let mut reader = AbortReader::with_byte_cap(&mut source, &NoAbort, "test input", 1);
    reader.read_exact(&mut [0; 1]).unwrap();
    for _ in 0..3 {
        assert!(reader.read(&mut [0; 1]).is_err());
    }
    assert!(reader.exceeded_cap());
    assert!(!reader.was_cancelled());
    assert_eq!(reader.bytes_read(), 1);
    assert_eq!(source.position(), 2);
}
