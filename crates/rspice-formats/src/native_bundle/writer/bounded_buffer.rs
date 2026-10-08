//! Seekable output whose length and reserved capacity stay within a byte budget.

use std::io::{self, Cursor, Seek, SeekFrom, Write};

pub(super) struct BoundedBuffer {
    inner: Cursor<Vec<u8>>,
    limit: u64,
}

impl BoundedBuffer {
    pub(super) fn new(limit: u64) -> Self {
        Self {
            inner: Cursor::new(Vec::new()),
            limit,
        }
    }

    pub(super) fn into_inner(self) -> Vec<u8> {
        self.inner.into_inner()
    }
}

impl Write for BoundedBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let end = self
            .inner
            .position()
            .checked_add(bytes.len() as u64)
            .filter(|end| *end <= self.limit)
            .and_then(|end| usize::try_from(end).ok())
            .ok_or_else(|| {
                io::Error::other(format!("output exceeds the {}-byte limit", self.limit))
            })?;
        let buffer = self.inner.get_mut();
        if end > buffer.capacity() {
            // Geometric growth avoids reallocating for every JSON token, but
            // never reserves beyond the caller's output budget.
            let capacity = end
                .max(buffer.capacity().saturating_mul(2))
                .min(usize::try_from(self.limit).unwrap_or(usize::MAX));
            buffer
                .try_reserve_exact(capacity - buffer.len())
                .map_err(|error| {
                    io::Error::other(format!("could not reserve output buffer: {error}"))
                })?;
        }
        self.inner.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for BoundedBuffer {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        // Seeking allocates nothing. The next write checks the resulting end
        // position, including any gap, before Cursor can grow its backing Vec.
        self.inner.seek(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::SerializeSeq;
    use std::cell::Cell;

    #[test]
    fn overwrites_gaps_and_growth_obey_the_same_output_budget() {
        let mut output = BoundedBuffer::new(505);
        output.write_all(&[1; 300]).unwrap();
        output.write_all(&[2; 100]).unwrap();
        assert!(output.inner.get_ref().capacity() <= 505);
        let before = output.inner.get_ref().clone();
        assert!(output.write_all(&[3; 106]).is_err());
        assert_eq!(output.inner.get_ref(), &before);
        assert_eq!(output.stream_position().unwrap(), 400);
        output.seek(SeekFrom::Start(10)).unwrap();
        output.write_all(&[4; 5]).unwrap();
        assert_eq!(output.inner.get_ref().len(), 400);
        output.seek(SeekFrom::End(100)).unwrap();
        output.write_all(&[5; 5]).unwrap();
        assert_eq!(output.inner.get_ref().len(), 505);
        assert_eq!(&output.inner.get_ref()[400..500], &[0; 100]);
        assert!(output.write_all(&[6]).is_err());
        output.seek(SeekFrom::Start(u64::MAX)).unwrap();
        assert!(output.write_all(&[7]).is_err());
        assert!(output.inner.get_ref().capacity() <= 505);
    }

    #[test]
    fn json_serialization_stops_before_materializing_an_oversized_payload() {
        struct Payload(Cell<usize>);
        impl serde::Serialize for Payload {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut sequence = serializer.serialize_seq(Some(10_000))?;
                for index in 0..10_000 {
                    self.0.set(index + 1);
                    sequence.serialize_element(&index)?;
                }
                sequence.end()
            }
        }
        let payload = Payload(Cell::new(0));
        let mut output = BoundedBuffer::new(64);
        let error = serde_json::to_writer(&mut output, &payload).unwrap_err();
        assert!(error.is_io());
        assert!(error.to_string().contains("64-byte limit"));
        assert!(payload.0.get() < 100);
        assert!(output.inner.get_ref().len() <= 64);
        assert!(output.inner.get_ref().capacity() <= 64);
    }
}
