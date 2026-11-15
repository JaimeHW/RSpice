//! Positioned I/O on one segment: the five operations an OPFS synchronous access handle offers.

use std::io;

/// Byte storage for one segment.
///
/// The writer only appends at its tail, overwrites its provisional tail, rewrites the superblock in
/// place and truncates; the reader only reads at offsets. Nothing needs a cursor, so the same code
/// runs on a file, in memory, and on a browser's origin-private file system.
pub trait SegmentIo {
    /// Fill `buf` from `offset`, or fail with `UnexpectedEof` when the segment is shorter.
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()>;
    /// Write all of `data` at `offset`, extending the segment when it ends before `offset + len`.
    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()>;
    /// Set the segment's length to `len`.
    fn truncate(&mut self, len: u64) -> io::Result<()>;
    /// Make every completed write durable on the storage device (a full sync). The writer calls it
    /// only when a run ends, at `END` or when it stops on an error; while a run goes, its writes
    /// are left to the operating system, which keeps them when the process dies.
    fn flush(&mut self) -> io::Result<()>;
    /// The segment's current length in bytes.
    fn size(&self) -> io::Result<u64>;
}

impl<T: SegmentIo + ?Sized> SegmentIo for Box<T> {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        (**self).read_at(offset, buf)
    }
    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        (**self).write_at(offset, data)
    }
    fn truncate(&mut self, len: u64) -> io::Result<()> {
        (**self).truncate(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        (**self).flush()
    }
    fn size(&self) -> io::Result<u64> {
        (**self).size()
    }
}

/// A segment held in memory.
///
/// Growth is fallible: when the allocator refuses, the write fails with `OutOfMemory`, which the
/// writer reports as [`crate::StoreError::MemoryFull`] with every earlier frame still readable.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MemoryIo {
    bytes: Vec<u8>,
}

impl MemoryIo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    fn grow_to(&mut self, len: usize) -> io::Result<()> {
        if len > self.bytes.len() {
            self.bytes
                .try_reserve(len - self.bytes.len())
                .map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
            self.bytes.resize(len, 0);
        }
        Ok(())
    }
}

fn span(offset: u64, len: usize) -> io::Result<(usize, usize)> {
    let start = usize::try_from(offset).map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
    let end = start
        .checked_add(len)
        .ok_or_else(|| io::Error::from(io::ErrorKind::OutOfMemory))?;
    Ok((start, end))
}

impl SegmentIo for MemoryIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        let (start, end) = span(offset, buf.len())?;
        let source = self
            .bytes
            .get(start..end)
            .ok_or_else(|| io::Error::from(io::ErrorKind::UnexpectedEof))?;
        buf.copy_from_slice(source);
        Ok(())
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        let (start, end) = span(offset, data.len())?;
        self.grow_to(end)?;
        self.bytes[start..end].copy_from_slice(data);
        Ok(())
    }

    fn truncate(&mut self, len: u64) -> io::Result<()> {
        let (len, _) = span(len, 0)?;
        if len < self.bytes.len() {
            self.bytes.truncate(len);
        } else {
            self.grow_to(len)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn size(&self) -> io::Result<u64> {
        Ok(self.bytes.len() as u64)
    }
}

/// A segment in a file, through positioned reads and writes.
#[cfg(any(unix, windows))]
#[derive(Debug)]
pub struct FileIo {
    file: std::fs::File,
}

#[cfg(any(unix, windows))]
impl FileIo {
    pub fn new(file: std::fs::File) -> Self {
        Self { file }
    }

    /// Create (or empty) the file at `path` for writing a new segment.
    pub fn create(path: &std::path::Path) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        Ok(Self { file })
    }

    /// Open an existing segment for reading.
    pub fn open(path: &std::path::Path) -> io::Result<Self> {
        Ok(Self {
            file: std::fs::File::open(path)?,
        })
    }

    pub fn into_inner(self) -> std::fs::File {
        self.file
    }
}

#[cfg(any(unix, windows))]
impl SegmentIo for FileIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        #[cfg(unix)]
        {
            std::os::unix::fs::FileExt::read_exact_at(&self.file, buf, offset)
        }
        #[cfg(windows)]
        {
            let (mut done, mut at) = (0usize, offset);
            while done < buf.len() {
                let n = std::os::windows::fs::FileExt::seek_read(&self.file, &mut buf[done..], at)?;
                if n == 0 {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
                }
                done += n;
                at += n as u64;
            }
            Ok(())
        }
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> io::Result<()> {
        #[cfg(unix)]
        {
            std::os::unix::fs::FileExt::write_all_at(&self.file, data, offset)
        }
        #[cfg(windows)]
        {
            let (mut done, mut at) = (0usize, offset);
            while done < data.len() {
                let n = std::os::windows::fs::FileExt::seek_write(&self.file, &data[done..], at)?;
                if n == 0 {
                    return Err(io::Error::from(io::ErrorKind::WriteZero));
                }
                done += n;
                at += n as u64;
            }
            Ok(())
        }
    }

    fn truncate(&mut self, len: u64) -> io::Result<()> {
        self.file.set_len(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.sync_all()
    }

    fn size(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }
}
