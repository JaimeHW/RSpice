//! Streaming Waveform Storage
//!
//! Provides disk-backed waveform storage for long transient simulations.
//! Waveform data is buffered in memory and periodically flushed to disk
//! to avoid memory exhaustion on multi-hour simulations.
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
use crate::resource::{
    ResourceKind, ResourceLimitError, ResourceLimits, ResourceReadError, read_file_bytes_limited,
};
use std::fs::File;
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
    /// Buffered file writer
    writer: BufWriter<File>,
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

        // Validation and memory reservation intentionally precede file
        // creation so bad input never truncates an existing destination.
        let file = File::create(path)?;
        let writer = BufWriter::with_capacity(65_536, file);
        let mut this = Self {
            writer,
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
        // Write a simple header format
        writeln!(self.writer, "Title: RSpice Streaming Waveform")?;
        writeln!(self.writer, "Date: {}", chrono_lite_now())?;
        writeln!(self.writer, "Plotname: Transient Analysis")?;
        writeln!(self.writer, "Flags: real double")?;
        writeln!(self.writer, "No. Variables: {}", self.num_channels + 1)?;
        write!(self.writer, "No. Points: ")?;
        self.point_count_offset = self.writer.stream_position()?;
        writeln!(self.writer, "{0:01$}", 0, HEADER_POINT_COUNT_WIDTH)?;
        writeln!(self.writer, "Variables:")?;
        writeln!(self.writer, "  0 time seconds")?;
        for (i, name) in self.channel_names.iter().enumerate() {
            writeln!(self.writer, "  {} {} voltage", i + 1, name)?;
        }
        if self.binary {
            writeln!(self.writer, "Binary:")?;
        } else {
            writeln!(self.writer, "Values:")?;
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
        if values.len() != self.num_channels {
            return Err(WaveformStreamError::InvalidFormat(format!(
                "waveform point has {} channel value(s), expected {}",
                values.len(),
                self.num_channels
            )));
        }
        if !time.is_finite() || values.iter().any(|value| !value.is_finite()) {
            return Err(WaveformStreamError::InvalidFormat(
                "waveform points must contain only finite values".to_string(),
            ));
        }
        let next_points = self.points_written.checked_add(1).ok_or_else(|| {
            WaveformStreamError::InvalidFormat(
                "streaming waveform point count overflowed this platform".to_string(),
            )
        })?;
        ResourceLimitError::ensure(ResourceKind::AnalysisPoints, next_points, self.max_points)?;
        let row_size = self.num_channels.saturating_add(1);
        let next_values = next_points.saturating_mul(row_size);
        ResourceLimitError::ensure(ResourceKind::ResultValues, next_values, self.max_values)?;

        self.buffer.push(time);
        self.buffer.extend_from_slice(values);

        // Check if flush needed
        if self.buffer.len() >= self.buffer_capacity {
            self.flush_buffer()?;
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

        if self.binary {
            // Write as binary f64 values
            for chunk in self.buffer.chunks(row_size) {
                for &val in chunk {
                    self.writer.write_all(&val.to_le_bytes())?;
                }
            }
        } else {
            // Write as ASCII
            for (row, chunk) in self.buffer.chunks(row_size).enumerate() {
                write!(self.writer, "{}", self.points_flushed.saturating_add(row))?;
                for &val in chunk {
                    write!(self.writer, "\t{:.17e}", val)?;
                }
                writeln!(self.writer)?;
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
        self.flush_buffer()?;
        self.writer.flush()
    }

    /// Finalize the file (flush and update header)
    pub fn finalize(mut self) -> io::Result<usize> {
        self.flush_buffer()?;
        self.writer.flush()?;

        let end_position = self.writer.stream_position()?;
        self.writer.seek(SeekFrom::Start(self.point_count_offset))?;
        write!(
            self.writer,
            "{0:01$}",
            self.points_written, HEADER_POINT_COUNT_WIDTH
        )?;
        self.writer.flush()?;
        self.writer.seek(SeekFrom::Start(end_position))?;

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
}

/// Simple timestamp without chrono dependency
fn chrono_lite_now() -> String {
    // Could use std::time::SystemTime if needed
    "Unknown".to_string()
}

//=============================================================================
// Streaming Reader (for loading waveform files)
//=============================================================================

/// Reader for streaming waveform files
#[derive(Debug)]
pub struct StreamingWaveformReader {
    /// Memory-mapped or file-backed data
    data: Vec<Value>,
    /// Number of channels (including time)
    num_columns: usize,
    /// Total number of points
    num_points: usize,
}

impl StreamingWaveformReader {
    /// Load a binary waveform file
    pub fn load_binary<P: AsRef<Path>>(path: P, num_channels: usize) -> io::Result<Self> {
        Self::load_binary_with_limits(path, num_channels, ResourceLimits::default())
            .map_err(WaveformStreamError::into_io)
    }

    /// Load a binary waveform with explicit external-data and result limits.
    pub fn load_binary_with_limits<P: AsRef<Path>>(
        path: P,
        num_channels: usize,
        resource_limits: ResourceLimits,
    ) -> Result<Self, WaveformStreamError> {
        let num_columns = num_channels.checked_add(1).ok_or_else(|| {
            WaveformStreamError::InvalidConfiguration(
                "channel count overflows this platform".to_string(),
            )
        })?;
        let bytes = read_file_bytes_limited(
            path.as_ref(),
            ResourceKind::ExternalDataBytes,
            resource_limits.max_external_data_bytes,
        )?;
        let header = parse_binary_stream_header(&bytes)?;
        if header.num_columns != num_columns {
            return Err(WaveformStreamError::InvalidFormat(format!(
                "binary waveform header declares {} column(s), but the caller supplied {} channel(s) ({} column(s))",
                header.num_columns, num_channels, num_columns
            )));
        }

        let data_bytes = &bytes[header.payload_offset..];
        if !data_bytes.len().is_multiple_of(8) {
            return Err(WaveformStreamError::InvalidFormat(format!(
                "binary waveform payload has {} trailing byte(s)",
                data_bytes.len() % 8
            )));
        }
        let num_values = data_bytes.len() / 8;
        if !num_values.is_multiple_of(num_columns) {
            return Err(WaveformStreamError::InvalidFormat(format!(
                "binary waveform contains {num_values} values, which is not divisible by {num_columns} columns"
            )));
        }
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataValues,
            num_values,
            resource_limits.max_external_data_values,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            num_values,
            resource_limits.max_result_values,
        )?;
        let num_points = num_values / num_columns;
        if header.num_points != 0 && header.num_points != num_points {
            return Err(WaveformStreamError::InvalidFormat(format!(
                "binary waveform header declares {} point(s), but the payload contains {}",
                header.num_points, num_points
            )));
        }
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            num_points,
            resource_limits.max_analysis_points,
        )?;

        let mut data = Vec::new();
        data.try_reserve_exact(num_values).map_err(|error| {
            WaveformStreamError::InvalidFormat(format!(
                "unable to allocate {num_values} waveform values: {error}"
            ))
        })?;
        for chunk in data_bytes.chunks_exact(8) {
            let value = f64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
            if !value.is_finite() {
                return Err(WaveformStreamError::InvalidFormat(format!(
                    "binary waveform contains non-finite value {value}"
                )));
            }
            data.push(value);
        }

        Ok(Self {
            data,
            num_columns,
            num_points,
        })
    }

    /// Get time array
    pub fn times(&self) -> Vec<Value> {
        self.data
            .chunks(self.num_columns)
            .map(|row| row[0])
            .collect()
    }

    /// Get channel data by index (0-based, not including time)
    pub fn channel(&self, index: usize) -> Vec<Value> {
        self.data
            .chunks(self.num_columns)
            .map(|row| row.get(index + 1).copied().unwrap_or(0.0))
            .collect()
    }

    /// Get number of points
    pub fn num_points(&self) -> usize {
        self.num_points
    }
}

#[derive(Debug, Clone, Copy)]
struct BinaryStreamHeader {
    payload_offset: usize,
    num_columns: usize,
    num_points: usize,
}

fn parse_binary_stream_header(bytes: &[u8]) -> Result<BinaryStreamHeader, WaveformStreamError> {
    let mut line_start = 0usize;
    let mut num_columns = None;
    let mut num_points = None;

    while line_start < bytes.len() {
        let relative_end = bytes[line_start..]
            .iter()
            .position(|byte| *byte == b'\n')
            .ok_or_else(|| {
                WaveformStreamError::InvalidFormat(
                    "binary waveform header line has no terminating newline".to_string(),
                )
            })?;
        let newline = line_start + relative_end;
        let content_end = if newline > line_start && bytes[newline - 1] == b'\r' {
            newline - 1
        } else {
            newline
        };
        let line = std::str::from_utf8(&bytes[line_start..content_end]).map_err(|error| {
            WaveformStreamError::InvalidFormat(format!(
                "binary waveform header is not UTF-8: {error}"
            ))
        })?;
        let line = line.trim();

        if line == "Binary:" {
            let num_columns = num_columns.ok_or_else(|| {
                WaveformStreamError::InvalidFormat(
                    "binary waveform header has no No. Variables field".to_string(),
                )
            })?;
            let num_points = num_points.ok_or_else(|| {
                WaveformStreamError::InvalidFormat(
                    "binary waveform header has no No. Points field".to_string(),
                )
            })?;
            return Ok(BinaryStreamHeader {
                payload_offset: newline + 1,
                num_columns,
                num_points,
            });
        }
        if line == "Values:" {
            return Err(WaveformStreamError::InvalidFormat(
                "ASCII waveform data cannot be loaded by the binary reader".to_string(),
            ));
        }
        if let Some(value) = line.strip_prefix("No. Variables:") {
            num_columns = Some(parse_header_count("No. Variables", value)?);
        } else if let Some(value) = line.strip_prefix("No. Points:") {
            num_points = Some(parse_header_count("No. Points", value)?);
        }

        line_start = newline + 1;
    }

    Err(WaveformStreamError::InvalidFormat(
        "binary waveform header has no Binary: marker".to_string(),
    ))
}

fn parse_header_count(field: &str, value: &str) -> Result<usize, WaveformStreamError> {
    value.trim().parse::<usize>().map_err(|_| {
        WaveformStreamError::InvalidFormat(format!(
            "binary waveform header has invalid {field} value {value:?}"
        ))
    })
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
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
        let parsed = crate::compat::parse_raw_file(&path)
            .expect("ASCII output must round-trip through the raw reader");
        assert_eq!(parsed.header.no_points, 5);
        assert_eq!(parsed.waveforms[1].y, vec![0.5, 1.5, 2.5, 3.5, 4.5]);

        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn invalid_writer_policy_does_not_truncate_destination() {
        let path = temporary_path("no-truncate");
        std::fs::write(&path, b"existing").expect("seed destination");
        let mut limits = ResourceLimits::default();
        limits.max_result_values = 3;

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
        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn writer_preserves_typed_point_limit_errors() {
        let path = temporary_path("point-limit");
        let mut limits = ResourceLimits::default();
        limits.max_analysis_points = 1;
        let mut writer = StreamingWaveformWriter::new_with_limits(&path, &["V(out)"], 1, limits)
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
        drop(writer);
        std::fs::remove_file(path).expect("remove test waveform");
    }

    #[test]
    fn binary_reader_rejects_file_byte_limit_before_decoding() {
        let path = temporary_path("reader-limit");
        let mut writer =
            StreamingWaveformWriter::new(&path, &["V(out)"], 2).expect("create binary writer");
        writer.write_point(0.0, &[1.0]).expect("write point");
        writer.finalize().expect("finalize");
        let streamed =
            StreamingWaveformReader::load_binary(&path, 1).expect("load finalized binary waveform");
        assert_eq!(streamed.num_points(), 1);
        assert_eq!(streamed.times(), vec![0.0]);
        assert_eq!(streamed.channel(0), vec![1.0]);
        let parsed = crate::compat::parse_raw_file(&path)
            .expect("binary output must round-trip through the raw reader");
        assert!(parsed.header.is_double);
        assert_eq!(parsed.header.no_points, 1);
        assert_eq!(parsed.waveforms[1].y, vec![1.0]);
        let file_bytes = usize::try_from(std::fs::metadata(&path).expect("metadata").len())
            .expect("test file fits usize");
        let mut limits = ResourceLimits::default();
        limits.max_external_data_bytes = file_bytes - 1;

        let error = StreamingWaveformReader::load_binary_with_limits(&path, 1, limits)
            .expect_err("file byte policy must reject before decoding");

        assert!(matches!(
            error,
            WaveformStreamError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ExternalDataBytes,
                requested,
                limit,
            }) if requested == file_bytes && limit == file_bytes - 1
        ));
        std::fs::remove_file(path).expect("remove test waveform");
    }
}
