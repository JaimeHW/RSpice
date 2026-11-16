//! The disk watch around a run's segment: a preflight that refuses a run whose rows cannot fit
//! before it solves, and a watch that stops it cleanly, keeping every whole chunk, when the
//! volume fills while it runs.

use std::path::Path;
use std::sync::Arc;

use rspice_core_base::result_sink::{
    EventChange, ResultSchema, ResultSink, RunMeta, RunStatus, SinkReceipt, SinkStop,
};

use crate::io::SegmentIo;
use crate::run_writer::RunIo;
use crate::sink::StoreSink;

/// Free bytes on the store's volume, asked when the run begins and before each write that grows
/// its segment; `None` when the host cannot say.
pub type FreeSpace = Arc<dyn Fn() -> Option<u64> + Send + Sync>;

/// Free space a segment keeps for closing once a write was refused: its last index and `END`.
pub const CLOSING_RESERVE: u64 = 1 << 20;

/// The free space of the volume holding `dir`, as the operating system reports it to an
/// unprivileged writer.
#[cfg(any(unix, windows))]
pub fn volume_free_space(dir: &Path) -> FreeSpace {
    let dir = dir.to_path_buf();
    Arc::new(move || available_bytes(&dir))
}

/// Bytes an unprivileged writer may still use on the volume holding `path`.
#[cfg(unix)]
pub fn available_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    // SAFETY: `path` is NUL-terminated and outlives the call, and `stat` is a writable `statvfs`
    // that the call fills when it returns 0, the only case in which it is read.
    let stat = unsafe {
        if libc::statvfs(path.as_ptr(), stat.as_mut_ptr()) != 0 {
            return None;
        }
        stat.assume_init()
    };
    #[allow(clippy::unnecessary_cast)]
    Some((stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64))
}

/// Bytes the calling user may still use on the volume holding `path`, quotas included.
#[cfg(windows)]
pub fn available_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    // A share's root is named with its trailing separator.
    if wide.last() != Some(&u16::from(b'\\')) {
        wide.push(u16::from(b'\\'));
    }
    wide.push(0);
    let mut available = 0u64;
    // SAFETY: `wide` is NUL-terminated and outlives the call, `available` is a writable `u64` the
    // call fills when it succeeds, and the two totals it is not asked for are null.
    let succeeded = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut available,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    (succeeded != 0).then_some(available)
}

/// Without a free-space query, the operating system's own refusal of a write is the stop.
#[cfg(not(any(unix, windows)))]
pub fn available_bytes(_path: &Path) -> Option<u64> {
    None
}

/// A segment that keeps room on its volume for the segment to close: a write that grows it past
/// the free space less [`CLOSING_RESERVE`] fails as storage full. Every write after that is the
/// writer closing the segment, which the reserve is for.
pub struct WatchedIo<I> {
    io: I,
    free: FreeSpace,
    full: bool,
}

impl<I> WatchedIo<I> {
    pub fn new(io: I, free: FreeSpace) -> Self {
        Self {
            io,
            free,
            full: false,
        }
    }

    /// The segment's own I/O.
    pub fn into_inner(self) -> I {
        self.io
    }
}

impl<I: SegmentIo> SegmentIo for WatchedIo<I> {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.io.read_at(offset, buf)
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        if !self.full {
            let grows = (offset + data.len() as u64).saturating_sub(self.io.size()?);
            if grows > 0
                && (self.free)().is_some_and(|free| free < grows.saturating_add(CLOSING_RESERVE))
            {
                self.full = true;
                return Err(std::io::ErrorKind::StorageFull.into());
            }
        }
        let written = self.io.write_at(offset, data);
        // The volume filled before the watch saw it: the writes closing the segment still try.
        if written
            .as_ref()
            .is_err_and(|error| error.kind() == std::io::ErrorKind::StorageFull)
        {
            self.full = true;
        }
        written
    }

    fn truncate(&mut self, len: u64) -> std::io::Result<()> {
        self.io.truncate(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }

    fn size(&self) -> std::io::Result<u64> {
        self.io.size()
    }
}

/// Told the bytes a run likely writes and the bytes free, when the first exceed the second but the
/// rows the run cannot avoid writing fit.
pub type SpaceWarning = Box<dyn FnMut(u64, u64) + Send>;

/// A store's sink with the disk preflight: when the run begins, the rows it cannot avoid writing
/// must fit in the free space, or the run is refused before it solves.
pub struct WatchedSink<I: RunIo> {
    inner: StoreSink<WatchedIo<I>>,
    free: FreeSpace,
    /// The fewest rows the run can write: no step is longer than its largest.
    rows_needed: f64,
    /// The rows it likely writes.
    rows_likely: f64,
    /// `(bytes needed, bytes free)` when the preflight refused the run.
    refused: Option<(u64, u64)>,
    warning: Option<SpaceWarning>,
}

impl<I: RunIo> WatchedSink<I> {
    /// `inner` with the preflight for a run that writes at least `rows_needed` rows and likely
    /// `rows_likely`.
    pub fn new(
        inner: StoreSink<WatchedIo<I>>,
        free: FreeSpace,
        rows_needed: f64,
        rows_likely: f64,
    ) -> Self {
        Self {
            inner,
            free,
            rows_needed,
            rows_likely,
            refused: None,
            warning: None,
        }
    }

    /// Tell `warning` when the run likely writes more than the volume holds.
    #[must_use]
    pub fn with_space_warning(mut self, warning: SpaceWarning) -> Self {
        self.warning = Some(warning);
        self
    }

    /// Bytes `rows` rows of `slots` values take in a segment at most: every value uncompressed,
    /// plus 3.5 % for frames, indexes and summaries.
    pub fn bytes(rows: f64, slots: usize) -> u64 {
        let bytes = (rows.max(1.0) * (slots as f64 + 1.0) * 8.0 * 1.035).ceil();
        if bytes.is_finite() && bytes < u64::MAX as f64 {
            bytes as u64
        } else {
            u64::MAX
        }
    }

    /// `(bytes needed, bytes free)` when the preflight refused the run before it began.
    pub fn refused(&self) -> Option<(u64, u64)> {
        self.refused
    }

    /// The fewest rows the run can write.
    pub fn rows_needed(&self) -> f64 {
        self.rows_needed
    }

    /// The store's sink and the free space it was watched against.
    pub fn into_parts(self) -> (StoreSink<WatchedIo<I>>, FreeSpace) {
        (self.inner, self.free)
    }
}

impl<I: RunIo> ResultSink for WatchedSink<I> {
    fn begin(&mut self, schema: &ResultSchema, meta: &RunMeta) -> Result<(), SinkStop> {
        if let Some(free) = (self.free)() {
            let needed = Self::bytes(self.rows_needed, schema.slots());
            if needed > free {
                self.refused = Some((needed, free));
                return Err(SinkStop::DiskFull {
                    needed: Some(needed),
                    available: Some(free),
                });
            }
            let likely = Self::bytes(self.rows_likely, schema.slots());
            if likely > free
                && let Some(warning) = &mut self.warning
            {
                warning(likely, free);
            }
        }
        self.inner.begin(schema, meta)
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        self.inner.row(x, values)
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        self.inner.amend_open_row(row, column, value)
    }

    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop> {
        self.inner.events(changes)
    }

    fn end(&mut self, status: RunStatus) -> Result<SinkReceipt, SinkStop> {
        self.inner.end(status)
    }
}
