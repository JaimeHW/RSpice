//! Exact byte identity across short reads, size admission and interrupted input.

use super::*;

struct ShortReader<'a> {
    bytes: &'a [u8],
    reads: usize,
    fail_after: Option<usize>,
}

impl Read for ShortReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.reads += 1;
        if self.fail_after.is_some_and(|limit| self.reads > limit) {
            return Err(std::io::Error::other("interrupted project read"));
        }
        let len = 2.min(self.bytes.len()).min(buffer.len());
        buffer[..len].copy_from_slice(&self.bytes[..len]);
        self.bytes = &self.bytes[len..];
        Ok(len)
    }
}

#[test]
fn bounded_snapshot_authenticates_actual_bytes_and_never_returns_partial_input() {
    let input = b"short reads retained exactly\n";
    let mut reader = ShortReader {
        bytes: input,
        reads: 0,
        fail_after: None,
    };
    let rejected = ProjectBytes::read(&mut reader, MAX_PROJECT_FILE_BYTES + 1);
    assert!(
        matches!(rejected, Err(ProjectIoError::InvalidData(message)) if message.contains("supported maximum"))
    );
    assert_eq!(reader.reads, 0);

    let captured = ProjectBytes::read(&mut reader, 2).unwrap();
    assert_eq!(captured.bytes, input);
    assert_eq!(captured.digest(), crate::persistence::digest_bytes(input));
    assert!(reader.reads > 2);

    let mut reader = ShortReader {
        bytes: input,
        reads: 0,
        fail_after: Some(1),
    };
    assert!(
        matches!(ProjectBytes::read(&mut reader, input.len() as u64),
        Err(ProjectIoError::Io(message)) if message == "interrupted project read")
    );
    assert_eq!(reader.reads, 2);
}
