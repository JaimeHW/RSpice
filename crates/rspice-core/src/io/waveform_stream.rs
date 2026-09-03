//! Writing a SPICE RAW file without holding the run in memory.
//!
//! [`super::raw_export`] takes a finished result and writes it; this takes one
//! point at a time and writes as it goes, so peak memory is the buffer rather
//! than the waveform. For a multi-hour transient that is the difference
//! between a file and an OOM.
//!
//! The format is the same SPICE RAW either way, which is what makes streaming
//! possible at all: `No. Points` is unknown until the run ends, so it is
//! written as a fixed-width field of zeroes and overwritten by [`
//! StreamingWaveformWriter::finalize`] seeking back to it. Anything that reads
//! RAW reads this, including this crate's own [`super::ltspice_raw`] parser —
//! which is why there is no reader here. There was one, and it was a
//! whole-file loader for the same bytes that required the caller to already
//! know the channel count.
//!
//! Nothing calls this yet. It is kept rather than deleted because it is not a
//! duplicate of `raw_export` but the constant-memory version of it, and the
//! hard part — the seek-and-backfill — is written and tested.
//!
//! # Usage
//! ```ignore
//! let mut writer = StreamingWaveformWriter::new(
//!     "output.raw",
//!     &["V(out)", "I(R1)"],
//!     4096,  // buffer size
//! )?;
//!
//! // During simulation
//! writer.write_point(time, &values)?;
//!
//! // End of simulation
//! writer.finalize()?;
//! ```

use crate::Value;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits, ResourceReadError};
use rspice_output::{AtomicArtifactError, AtomicArtifactFile};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use thiserror::Error;

/// Errors from bounded streaming waveform construction and loading.
#[derive(Debug, Error)]
pub enum WaveformStreamError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
    #[error("invalid streaming waveform configuration: {0}")]
    InvalidConfiguration(String),
    #[error("invalid streaming waveform data: {0}")]
    InvalidFormat(String),
}

impl From<ResourceReadError> for WaveformStreamError {
    fn from(error: ResourceReadError) -> Self {
        match error {
            ResourceReadError::Io(error) => Self::Io(error),
            ResourceReadError::ResourceLimit(error) => Self::ResourceLimit(error),
        }
    }
}

impl WaveformStreamError {
    fn into_io(self) -> io::Error {
        match self {
            Self::Io(error) => error,
            other => io::Error::other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamEncoding {
    Binary,
    Ascii,
}

const HEADER_POINT_COUNT_WIDTH: usize = 20;

//=============================================================================
// Streaming Writer
//=============================================================================

/// Disk-backed waveform writer for long simulations
///
/// Buffers data in memory and flushes to disk when the buffer is full.
/// Uses binary format for efficient storage: [time, v0, v1, ...] as f64s.
#[derive(Debug)]
pub struct StreamingWaveformWriter {
    /// Buffered writer backed by a same-directory staging transaction.
    writer: Option<BufWriter<AtomicArtifactFile>>,
    /// Number of channels (not including time)
    num_channels: usize,
    /// Channel names for header
    channel_names: Vec<String>,
    /// In-memory buffer before disk write
    buffer: Vec<Value>,
    /// Maximum buffer size (flush when exceeded)
    buffer_capacity: usize,
    /// Total points written
    points_written: usize,
    /// Byte offset of the fixed-width point count in the header.
    point_count_offset: u64,
    /// Points already flushed to the file.
    points_flushed: usize,
    /// Maximum points accepted by this writer.
    max_points: usize,
    /// Maximum scalar values written across the stream.
    max_values: usize,
    /// File format (binary or ASCII)
    binary: bool,
}

impl StreamingWaveformWriter {
    /// Create a new streaming writer
    ///
    /// # Arguments
    /// * `path` - Output file path
    /// * `channel_names` - Names of signal channels
    /// * `buffer_size` - Number of points to buffer before flushing
    ///
    /// # Returns
    /// The writer, or an IO error
    pub fn new<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
    ) -> io::Result<Self> {
        Self::new_with_limits(path, channel_names, buffer_size, ResourceLimits::default())
            .map_err(WaveformStreamError::into_io)
    }

    /// Create a binary streaming writer with explicit resource limits.
    pub fn new_with_limits<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
        resource_limits: ResourceLimits,
    ) -> Result<Self, WaveformStreamError> {
        Self::new_with_encoding(
            path,
            channel_names,
            buffer_size,
            resource_limits,
            StreamEncoding::Binary,
        )
    }

    /// Create a new ASCII format writer (human-readable but larger)
    pub fn new_ascii<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
    ) -> io::Result<Self> {
        Self::new_ascii_with_limits(path, channel_names, buffer_size, ResourceLimits::default())
            .map_err(WaveformStreamError::into_io)
    }

    /// Create an ASCII streaming writer with explicit resource limits.
    pub fn new_ascii_with_limits<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
        resource_limits: ResourceLimits,
    ) -> Result<Self, WaveformStreamError> {
        Self::new_with_encoding(
            path,
            channel_names,
            buffer_size,
            resource_limits,
            StreamEncoding::Ascii,
        )
    }

    fn new_with_encoding<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
        resource_limits: ResourceLimits,
        encoding: StreamEncoding,
    ) -> Result<Self, WaveformStreamError> {
        if buffer_size == 0 {
            return Err(WaveformStreamError::InvalidConfiguration(
                "buffer_size must be greater than zero".to_string(),
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataValues,
            channel_names.len(),
            resource_limits.max_external_data_values,
        )?;
        let channel_name_bytes = channel_names
            .iter()
            .fold(0usize, |total, name| total.saturating_add(name.len()));
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataBytes,
            channel_name_bytes,
            resource_limits.max_external_data_bytes,
        )?;
        for name in channel_names {
            if name.is_empty() || name.chars().any(char::is_whitespace) {
                return Err(WaveformStreamError::InvalidConfiguration(format!(
                    "channel name {name:?} must be non-empty and contain no whitespace"
                )));
            }
        }
        let num_channels = channel_names.len();
        let row_size = num_channels.checked_add(1).ok_or_else(|| {
            WaveformStreamError::InvalidConfiguration(
                "channel count overflows this platform".to_string(),
            )
        })?;
        let buffer_capacity = row_size.checked_mul(buffer_size).ok_or_else(|| {
            WaveformStreamError::InvalidConfiguration(
                "channel count and buffer size overflow this platform".to_string(),
            )
        })?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            buffer_capacity,
            resource_limits.max_result_values,
        )?;

        let mut names = Vec::new();
        names
            .try_reserve_exact(channel_names.len())
            .map_err(|error| {
                WaveformStreamError::InvalidConfiguration(format!(
                    "unable to allocate channel metadata: {error}"
                ))
            })?;
        for name in channel_names {
            let mut owned = String::new();
            owned.try_reserve_exact(name.len()).map_err(|error| {
                WaveformStreamError::InvalidConfiguration(format!(
                    "unable to allocate channel name {name:?}: {error}"
                ))
            })?;
            owned.push_str(name);
            names.push(owned);
        }
        let mut buffer = Vec::new();
        buffer.try_reserve_exact(buffer_capacity).map_err(|error| {
            WaveformStreamError::InvalidConfiguration(format!(
                "unable to allocate a {buffer_capacity}-value waveform buffer: {error}"
            ))
        })?;

        // Validation and memory reservation intentionally precede staging-file
        // creation. The destination is not changed until `finalize` publishes
        // a complete, synchronized artifact.
        let artifact =
            AtomicArtifactFile::prepare(path.as_ref()).map_err(atomic_artifact_io_error)?;
        let writer = BufWriter::with_capacity(65_536, artifact);
        let mut this = Self {
            writer: Some(writer),
            num_channels,
            channel_names: names,
            buffer,
            buffer_capacity,
            points_written: 0,
            point_count_offset: 0,
            points_flushed: 0,
            max_points: resource_limits.max_analysis_points,
            max_values: resource_limits.max_result_values,
            binary: encoding == StreamEncoding::Binary,
        };
        this.write_header()?;
        Ok(this)
    }

    /// Write file header
    fn write_header(&mut self) -> io::Result<()> {
        let writer = self.writer.as_mut().ok_or_else(inactive_writer_error)?;
        // Write a simple header format
        writeln!(writer, "Title: RSpice Streaming Waveform")?;
        writeln!(writer, "Date: {}", chrono_lite_now())?;
        writeln!(writer, "Plotname: Transient Analysis")?;
        writeln!(writer, "Flags: real double")?;
        writeln!(writer, "No. Variables: {}", self.num_channels + 1)?;
        write!(writer, "No. Points: ")?;
        self.point_count_offset = writer.stream_position()?;
        writeln!(writer, "{0:01$}", 0, HEADER_POINT_COUNT_WIDTH)?;
        writeln!(writer, "Variables:")?;
        writeln!(writer, "  0 time seconds")?;
        for (i, name) in self.channel_names.iter().enumerate() {
            writeln!(writer, "  {} {} voltage", i + 1, name)?;
        }
        if self.binary {
            writeln!(writer, "Binary:")?;
        } else {
            writeln!(writer, "Values:")?;
        }

        Ok(())
    }

    /// Write a time point with all channel values
    ///
    /// Values are buffered; use `flush()` to force disk write.
    #[inline]
    pub fn write_point(&mut self, time: Value, values: &[Value]) -> io::Result<()> {
        self.write_point_checked(time, values)
            .map_err(WaveformStreamError::into_io)
    }

    /// Write one point while preserving typed resource-limit failures.
    #[inline]
    pub fn write_point_checked(
        &mut self,
        time: Value,
        values: &[Value],
    ) -> Result<(), WaveformStreamError> {
        if self.writer.is_none() {
            return Err(WaveformStreamError::Io(inactive_writer_error()));
        }
        if values.len() != self.num_channels {
            let error = WaveformStreamError::InvalidFormat(format!(
                "waveform point has {} channel value(s), expected {}",
                values.len(),
                self.num_channels
            ));
            self.abort();
            return Err(error);
        }
        if !time.is_finite() || values.iter().any(|value| !value.is_finite()) {
            let error = WaveformStreamError::InvalidFormat(
                "waveform points must contain only finite values".to_string(),
            );
            self.abort();
            return Err(error);
        }
        let Some(next_points) = self.points_written.checked_add(1) else {
            let error = WaveformStreamError::InvalidFormat(
                "streaming waveform point count overflowed this platform".to_string(),
            );
            self.abort();
            return Err(error);
        };
        if let Err(error) =
            ResourceLimitError::ensure(ResourceKind::AnalysisPoints, next_points, self.max_points)
        {
            self.abort();
            return Err(error.into());
        }
        let row_size = self.num_channels.saturating_add(1);
        let next_values = next_points.saturating_mul(row_size);
        if let Err(error) =
            ResourceLimitError::ensure(ResourceKind::ResultValues, next_values, self.max_values)
        {
            self.abort();
            return Err(error.into());
        }

        self.buffer.push(time);
        self.buffer.extend_from_slice(values);

        // Check if flush needed
        if self.buffer.len() >= self.buffer_capacity
            && let Err(error) = self.flush_buffer()
        {
            self.abort();
            return Err(error.into());
        }

        self.points_written = next_points;
        Ok(())
    }

    /// Flush the internal buffer to disk
    fn flush_buffer(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let row_size = self.num_channels + 1;
        let buffered_points = self.buffer.len() / row_size;
        let writer = self.writer.as_mut().ok_or_else(inactive_writer_error)?;

        if self.binary {
            // Write as binary f64 values
            for chunk in self.buffer.chunks(row_size) {
                for &val in chunk {
                    writer.write_all(&val.to_le_bytes())?;
                }
            }
        } else {
            // Write as ASCII
            for (row, chunk) in self.buffer.chunks(row_size).enumerate() {
                write!(writer, "{}", self.points_flushed.saturating_add(row))?;
                for &val in chunk {
                    write!(writer, "\t{:.17e}", val)?;
                }
                writeln!(writer)?;
            }
        }

        self.points_flushed = self
            .points_flushed
            .checked_add(buffered_points)
            .ok_or_else(|| io::Error::other("flushed waveform point count overflowed"))?;
        self.buffer.clear();
        Ok(())
    }

    /// Force flush to disk
    pub fn flush(&mut self) -> io::Result<()> {
        let result = self.flush_buffer().and_then(|()| {
            self.writer
                .as_mut()
                .ok_or_else(inactive_writer_error)?
                .flush()
        });
        if result.is_err() {
            self.abort();
        }
        result
    }

    /// Finalize the file (flush and update header)
    pub fn finalize(mut self) -> io::Result<usize> {
        self.flush_buffer()?;
        let writer = self.writer.as_mut().ok_or_else(inactive_writer_error)?;
        writer.flush()?;

        let end_position = writer.stream_position()?;
        writer.seek(SeekFrom::Start(self.point_count_offset))?;
        write!(
            writer,
            "{0:01$}",
            self.points_written, HEADER_POINT_COUNT_WIDTH
        )?;
        writer.flush()?;
        writer.seek(SeekFrom::Start(end_position))?;

        let writer = self.writer.take().ok_or_else(inactive_writer_error)?;
        let artifact = writer.into_inner().map_err(|error| error.into_error())?;
        artifact.commit().map_err(atomic_artifact_io_error)?;

        Ok(self.points_written)
    }

    /// Get number of points written so far
    pub fn points_written(&self) -> usize {
        self.points_written
    }

    /// Get number of channels
    pub fn num_channels(&self) -> usize {
        self.num_channels
    }

    fn abort(&mut self) {
        self.writer.take();
        self.buffer.clear();
    }
}

fn inactive_writer_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::BrokenPipe,
        "streaming waveform transaction is no longer active",
    )
}

fn atomic_artifact_io_error(error: AtomicArtifactError<io::Error>) -> io::Error {
    let kind = match &error {
        AtomicArtifactError::Prepare(source) | AtomicArtifactError::Write(source) => source.kind(),
        AtomicArtifactError::Flush { source, .. } | AtomicArtifactError::Commit { source, .. } => {
            source.kind()
        }
    };
    io::Error::new(kind, error)
}

/// Simple timestamp without chrono dependency
fn chrono_lite_now() -> String {
    // Could use std::time::SystemTime if needed
    "Unknown".to_string()
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use rspice_output::stale_artifacts;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn temporary_path(label: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rspice-waveform-{label}-{}-{unique}.raw",
            std::process::id()
        ))
    }

    fn seed_destination(path: &Path, preexisting: bool) {
        if preexisting {
            std::fs::write(path, b"old complete waveform").expect("seed existing waveform");
        }
    }

    fn assert_old_or_absent(path: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(path).expect("read preserved waveform"),
                b"old complete waveform"
            );
        } else {
            assert!(!path.exists(), "failed stream published a destination");
        }
    }

    fn assert_no_stages(path: &Path) {
        assert!(
            stale_artifacts(path)
                .expect("inspect streaming staging artifacts")
                .is_empty(),
            "streaming writer left a staging artifact"
        );
    }

    #[test]
    fn ascii_writer_emits_ascii_header_and_global_row_indices() {
        let path = temporary_path("ascii-rows");
        let mut writer =
            StreamingWaveformWriter::new_ascii(&path, &["V(out)"], 2).expect("create ASCII writer");
        for index in 0..5 {
            writer
                .write_point(index as Value, &[index as Value + 0.5])
                .expect("write point");
        }
        assert_eq!(writer.finalize().expect("finalize"), 5);

        let contents = std::fs::read_to_string(&path).expect("read ASCII waveform");
        assert!(contents.contains("Values:\n"));
        assert!(!contents.contains("Binary:\n"));
        assert!(contents.contains("No. Points: 00000000000000000005\n"));
        let parsed = crate::io::ltspice_raw::parse_raw_file(&path)
            .expect("ASCII output must round-trip through the raw reader");
        assert_eq!(parsed.header.no_points, 5);
        assert_eq!(parsed.waveforms[1].y, vec![0.5, 1.5, 2.5, 3.5, 4.5]);

        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn invalid_writer_policy_does_not_truncate_destination() {
        let path = temporary_path("no-truncate");
        std::fs::write(&path, b"existing").expect("seed destination");
        let limits = ResourceLimits {
            max_result_values: 3,
            ..Default::default()
        };

        let error = StreamingWaveformWriter::new_with_limits(&path, &["V(out)"], 2, limits)
            .expect_err("four-value buffer must exceed the policy");

        assert!(matches!(
            error,
            WaveformStreamError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 4,
                limit: 3,
            })
        ));
        assert_eq!(std::fs::read(&path).expect("read destination"), b"existing");
        assert_no_stages(&path);
        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn writer_preserves_typed_point_limit_errors() {
        for preexisting in [false, true] {
            let path = temporary_path("point-limit");
            seed_destination(&path, preexisting);
            let limits = ResourceLimits {
                max_analysis_points: 1,
                ..Default::default()
            };
            let mut writer =
                StreamingWaveformWriter::new_with_limits(&path, &["V(out)"], 1, limits)
                    .expect("create limited writer");
            writer
                .write_point_checked(0.0, &[1.0])
                .expect("first point fits");

            let error = writer
                .write_point_checked(1.0, &[2.0])
                .expect_err("second point exceeds the limit");
            assert!(matches!(
                error,
                WaveformStreamError::ResourceLimit(ResourceLimitError {
                    resource: ResourceKind::AnalysisPoints,
                    requested: 2,
                    limit: 1,
                })
            ));
            assert_old_or_absent(&path, preexisting);
            assert_no_stages(&path);
            writer
                .finalize()
                .expect_err("a resource-limit error permanently aborts publication");
            assert_old_or_absent(&path, preexisting);
            assert_no_stages(&path);
        }
    }

    #[test]
    fn point_validation_error_aborts_publication_immediately() {
        for preexisting in [false, true] {
            let path = temporary_path("point-error");
            seed_destination(&path, preexisting);
            let mut writer = StreamingWaveformWriter::new(&path, &["V(out)"], 1)
                .expect("create streaming writer");

            writer
                .write_point_checked(0.0, &[])
                .expect_err("missing channel value must fail");

            assert_old_or_absent(&path, preexisting);
            assert_no_stages(&path);
        }
    }

    #[test]
    fn dropping_unfinalized_writer_aborts_publication() {
        for preexisting in [false, true] {
            let path = temporary_path("drop");
            seed_destination(&path, preexisting);
            let mut writer = StreamingWaveformWriter::new(&path, &["V(out)"], 1)
                .expect("create streaming writer");
            writer.write_point(0.0, &[1.0]).expect("write point");
            writer.flush().expect("flush staged waveform");
            assert_eq!(
                stale_artifacts(&path)
                    .expect("inspect active streaming stage")
                    .len(),
                1
            );

            drop(writer);

            assert_old_or_absent(&path, preexisting);
            assert_no_stages(&path);
        }
    }

    #[test]
    fn successful_finalize_publishes_complete_waveform() {
        for preexisting in [false, true] {
            let path = temporary_path("transaction-success");
            seed_destination(&path, preexisting);
            let mut writer = StreamingWaveformWriter::new(&path, &["V(out)"], 1)
                .expect("create streaming writer");
            writer.write_point(0.0, &[1.0]).expect("write point");
            writer.write_point(1.0, &[2.0]).expect("write point");

            assert_eq!(writer.finalize().expect("publish waveform"), 2);

            let parsed = crate::io::ltspice_raw::parse_raw_file(&path)
                .expect("published streaming RAW must parse");
            assert_eq!(parsed.header.no_points, 2);
            assert_eq!(parsed.waveforms[1].y, vec![1.0, 2.0]);
            assert_no_stages(&path);
            std::fs::remove_file(path).expect("remove test waveform");
        }
    }

    /// What this writer emits is a SPICE RAW file, not a private format, and
    /// the crate's own RAW reader is what proves it. The backfilled point
    /// count is the part worth pinning: it is written as zeroes, seeked back
    /// to and overwritten by `finalize`, and a reader that trusted the header
    /// would silently return nothing if that seek were ever dropped.
    #[test]
    fn finalized_binary_output_round_trips_through_the_raw_reader() {
        let path = temporary_path("raw-round-trip");
        let mut writer =
            StreamingWaveformWriter::new(&path, &["V(out)"], 2).expect("create binary writer");
        writer.write_point(0.0, &[1.0]).expect("write point");
        writer.write_point(1.0e-9, &[2.0]).expect("write point");
        writer.finalize().expect("finalize");

        let parsed = crate::io::ltspice_raw::parse_raw_file(&path)
            .expect("binary output must round-trip through the raw reader");

        assert!(parsed.header.is_double);
        assert_eq!(parsed.header.no_points, 2);
        assert_eq!(parsed.waveforms[0].y, vec![0.0, 1.0e-9]);
        assert_eq!(parsed.waveforms[1].y, vec![1.0, 2.0]);
        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn raw_reader_rejects_file_byte_limit_before_decoding() {
        let path = temporary_path("reader-limit");
        let mut writer =
            StreamingWaveformWriter::new(&path, &["V(out)"], 2).expect("create binary writer");
        writer.write_point(0.0, &[1.0]).expect("write point");
        writer.finalize().expect("finalize");
        let file_bytes = usize::try_from(std::fs::metadata(&path).expect("metadata").len())
            .expect("test file fits usize");
        let limits = ResourceLimits {
            max_external_data_bytes: file_bytes - 1,
            ..Default::default()
        };

        let error = crate::io::ltspice_raw::parse_raw_file_with_limits(path.as_path(), limits)
            .expect_err("file byte policy must reject before decoding");

        assert!(matches!(
            error,
            crate::io::ltspice_raw::RawParseError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ExternalDataBytes,
                requested,
                limit,
            }) if requested == file_bytes && limit == file_bytes - 1
        ));
        std::fs::remove_file(path).expect("remove test waveform");
    }
}
