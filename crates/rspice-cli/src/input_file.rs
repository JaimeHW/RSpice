//! Admit regular result files and read one bounded, owned byte snapshot.

use std::io::{self, Read};
use std::path::Path;

use rspice_core::{ResourceKind, ResourceLimitError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
}

impl ReadError {
    pub(crate) fn into_cli_error(self, path: &Path) -> crate::cli::CliError {
        match self {
            Self::Io(source) => crate::cli::CliError::InputReadError {
                path: path.to_owned(),
                source,
            },
            Self::ResourceLimit(source) => crate::cli::CliError::ResourceLimit {
                path: path.to_owned(),
                source,
            },
        }
    }
}

/// Check both the path and the opened handle. Nonblocking open on Unix closes
/// the replacement race with a FIFO; symlinks to regular files remain valid.
pub(crate) fn read(path: &Path, limit: usize) -> Result<Vec<u8>, ReadError> {
    ensure_regular(&std::fs::metadata(path)?)?;
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.custom_flags(libc::O_NONBLOCK | libc::O_NOCTTY);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    ensure_regular(&metadata)?;
    let length = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    read_limited(file, length, limit)
}

fn ensure_regular(metadata: &std::fs::Metadata) -> io::Result<()> {
    if metadata.is_file() {
        Ok(())
    } else {
        Err(io::Error::other("result input must be a regular file"))
    }
}

fn read_limited(
    mut reader: impl Read,
    initial_length: usize,
    limit: usize,
) -> Result<Vec<u8>, ReadError> {
    admit_bytes(initial_length, limit)?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(initial_length)
        .map_err(reserve_error)?;
    let mut chunk = [0_u8; 64 * 1024];
    loop {
        // One extra byte proves that the source grew past its budget. Do not
        // allocate or read the rest of a growing input before reporting it.
        let read_size = chunk.len().min((limit - bytes.len()).saturating_add(1));
        let count = match reader.read(&mut chunk[..read_size]) {
            Ok(0) => return Ok(bytes),
            Ok(count) => count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error.into()),
        };
        let requested = bytes.len().saturating_add(count);
        admit_bytes(requested, limit)?;
        if bytes.capacity() - bytes.len() < count {
            let capacity = bytes.capacity().saturating_mul(2).max(requested).min(limit);
            bytes
                .try_reserve_exact(capacity - bytes.len())
                .map_err(reserve_error)?;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

fn admit_bytes(requested: usize, limit: usize) -> Result<(), ReadError> {
    if requested > limit {
        Err(ResourceLimitError {
            resource: ResourceKind::ExternalDataBytes,
            requested,
            limit,
        }
        .into())
    } else {
        Ok(())
    }
}

fn reserve_error(error: std::collections::TryReserveError) -> io::Error {
    io::Error::other(format!(
        "unable to reserve memory for result input: {error}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn growth_beyond_metadata_stops_after_one_byte_past_the_limit() {
        let mut source = Cursor::new(vec![7; 256 * 1024]);
        let limit = 70_000;
        let error = read_limited(&mut source, 4, limit).unwrap_err();
        assert!(matches!(error, ReadError::ResourceLimit(error)
            if error.resource == ResourceKind::ExternalDataBytes
                && error.limit == limit && error.requested == limit + 1));
        assert_eq!(source.position(), (limit + 1) as u64);
    }

    #[test]
    fn admission_precedes_reads_and_exact_limits_do_not_truncate() {
        let mut source = Cursor::new(b"data");
        assert!(matches!(
            read_limited(&mut source, 5, 4),
            Err(ReadError::ResourceLimit(_))
        ));
        assert_eq!(source.position(), 0);
        assert_eq!(read_limited(&mut source, 1, 4).unwrap(), b"data");
        assert!(read_limited(Cursor::new([]), 0, 0).unwrap().is_empty());
    }

    #[test]
    fn interrupted_reads_retry_but_other_io_errors_are_preserved() {
        struct InterruptedOnce(Option<Cursor<&'static [u8]>>);
        impl Read for InterruptedOnce {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                match &mut self.0 {
                    Some(source) => source.read(buffer),
                    None => {
                        self.0 = Some(Cursor::new(b"x"));
                        Err(io::ErrorKind::Interrupted.into())
                    }
                }
            }
        }
        assert_eq!(read_limited(InterruptedOnce(None), 0, 1).unwrap(), b"x");
        struct Failed;
        impl Read for Failed {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::ErrorKind::PermissionDenied.into())
            }
        }
        assert!(
            matches!(read_limited(Failed, 0, 1), Err(ReadError::Io(error))
            if error.kind() == io::ErrorKind::PermissionDenied)
        );
    }
}
