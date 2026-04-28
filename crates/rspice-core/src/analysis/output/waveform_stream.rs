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
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

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
        let file = File::create(path)?;
        let writer = BufWriter::with_capacity(65536, file);

        let names: Vec<String> = channel_names.iter().map(|s| s.to_string()).collect();
        let num_channels = names.len();

        // Buffer holds (time + channels) * buffer_size values
        let buffer_capacity = (num_channels + 1) * buffer_size;

        let mut this = Self {
            writer,
            num_channels,
            channel_names: names,
            buffer: Vec::with_capacity(buffer_capacity),
            buffer_capacity,
            points_written: 0,
            binary: true,
        };

        // Write header
        this.write_header()?;

        Ok(this)
    }

    /// Create a new ASCII format writer (human-readable but larger)
    pub fn new_ascii<P: AsRef<Path>>(
        path: P,
        channel_names: &[&str],
        buffer_size: usize,
    ) -> io::Result<Self> {
        let mut writer = Self::new(path, channel_names, buffer_size)?;
        writer.binary = false;
        Ok(writer)
    }

    /// Write file header
    fn write_header(&mut self) -> io::Result<()> {
        // Write a simple header format
        writeln!(self.writer, "Title: RSpice Streaming Waveform")?;
        writeln!(self.writer, "Date: {}", chrono_lite_now())?;
        writeln!(self.writer, "Plotname: Transient Analysis")?;
        writeln!(self.writer, "Flags: real")?;
        writeln!(self.writer, "No. Variables: {}", self.num_channels + 1)?;
        writeln!(self.writer, "No. Points: 0")?; // Will update at finalize
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
        debug_assert_eq!(values.len(), self.num_channels);

        self.buffer.push(time);
        self.buffer.extend_from_slice(values);

        // Check if flush needed
        if self.buffer.len() >= self.buffer_capacity {
            self.flush_buffer()?;
        }

        self.points_written += 1;
        Ok(())
    }

    /// Flush the internal buffer to disk
    fn flush_buffer(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let row_size = self.num_channels + 1;

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
                write!(self.writer, "{}", row)?;
                for &val in chunk {
                    write!(self.writer, "\t{:.9e}", val)?;
                }
                writeln!(self.writer)?;
            }
        }

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

        // Note: Updating the header with final point count would require
        // seeking back, which complicates things. For simplicity, we leave
        // the header as-is and the reader can count points.

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
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        // Skip header (find "Binary:" marker)
        let header_end = bytes
            .windows(7)
            .position(|w| w == b"Binary:")
            .map(|p| p + 8) // Skip past "Binary:\n"
            .unwrap_or(0);

        let data_bytes = &bytes[header_end..];
        let num_columns = num_channels + 1;
        let num_values = data_bytes.len() / 8;
        let num_points = num_values / num_columns;

        let mut data = Vec::with_capacity(num_values);
        for chunk in data_bytes.chunks(8) {
            if chunk.len() == 8 {
                let val = f64::from_le_bytes([
                    chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
                ]);
                data.push(val);
            }
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

//=============================================================================
// Tests
//=============================================================================

