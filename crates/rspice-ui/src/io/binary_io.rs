//! Binary Waveform I/O (PSF-Lite)
//!
//! High-performance binary format for massive simulation datasets.
//! Optimized for fast random access and low memory overhead.
//!
//! # Specification
//!
//! - **Header**: Magic, Version, NumTraces, PointsPerTrace.
//! - **Trace metadata**: Name, Unit, Type (Real/Complex).
//! - **Data**: IEEE 754 Doubles, Chunked for memory-mapped access.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

const HEADER_SIZE_BYTES: u64 = 24;
const F64_SIZE_BYTES: usize = std::mem::size_of::<f64>();

/// PSF-Lite Binary Waveform Header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsfHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub num_traces: u32,
    pub num_points: u32,
    pub timestamp: u64,
}

impl PsfHeader {
    pub const MAGIC: [u8; 4] = *b"PSFL";
    pub const VERSION: u32 = 1;

    pub fn new(num_traces: u32, num_points: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            num_traces,
            num_points,
            timestamp: crate::common::time_compat::unix_epoch().as_secs(),
        }
    }
}

/// A reader for PSF-Lite binary files
pub struct PsfReader {
    file: File,
    header: PsfHeader,
}

impl PsfReader {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let header = read_header(&mut file)?;

        if header.magic != PsfHeader::MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid PSF magic",
            ));
        }

        Ok(Self { file, header })
    }

    pub fn header(&self) -> &PsfHeader {
        &self.header
    }

    /// Read a specific trace by index
    pub fn read_trace(&mut self, trace_idx: u32) -> std::io::Result<Vec<f64>> {
        if trace_idx >= self.header.num_traces {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Trace index out of bounds",
            ));
        }

        let trace_len = self.header.num_points as usize;
        let trace_bytes = trace_len
            .checked_mul(F64_SIZE_BYTES)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "trace length overflow"))?;
        let offset = HEADER_SIZE_BYTES
            + (trace_idx as u64)
                .checked_mul(trace_bytes as u64)
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "trace offset overflow"))?;
        self.file.seek(SeekFrom::Start(offset))?;

        let mut encoded = vec![0u8; trace_bytes];
        self.file.read_exact(&mut encoded)?;
        decode_f64_slice_le(&encoded)
    }
}

/// A writer for PSF-Lite binary files
pub struct PsfWriter {
    #[cfg(not(target_arch = "wasm32"))]
    path: PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    expected: crate::io::durable_file::ExpectedContent,
    encoded: Vec<u8>,
    header: Option<PsfHeader>,
    traces_written: u32,
    published: bool,
}

impl PsfWriter {
    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = path;
            Err(Error::new(
                ErrorKind::Unsupported,
                "PSF path publication is unavailable in the browser; serialize waveform data for a browser download instead",
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        let path = path.as_ref().to_path_buf();
        #[cfg(not(target_arch = "wasm32"))]
        let expected = crate::io::durable_file::observe_expected_content(&path)
            .map_err(publication_io_error)?;
        #[cfg(not(target_arch = "wasm32"))]
        Ok(Self {
            path,
            expected,
            encoded: Vec::new(),
            header: None,
            traces_written: 0,
            published: false,
        })
    }

    pub fn write_header(&mut self, header: &PsfHeader) -> std::io::Result<()> {
        if self.published || self.header.is_some() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "PSF header may be written exactly once",
            ));
        }
        if header.magic != PsfHeader::MAGIC || header.version != PsfHeader::VERSION {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "PSF header has an unsupported magic or version",
            ));
        }
        let trace_bytes = (header.num_points as usize)
            .checked_mul(F64_SIZE_BYTES)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "PSF trace size overflow"))?;
        let payload_bytes = (header.num_traces as usize)
            .checked_mul(trace_bytes)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "PSF payload size overflow"))?;
        let total_bytes = (HEADER_SIZE_BYTES as usize)
            .checked_add(payload_bytes)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "PSF file size overflow"))?;
        self.encoded
            .try_reserve_exact(total_bytes)
            .map_err(|error| {
                Error::other(format!(
                    "could not allocate {total_bytes} bytes for complete PSF serialization: {error}"
                ))
            })?;
        write_header(&mut self.encoded, header)?;
        self.header = Some(header.clone());
        self.publish_if_complete()
    }

    pub fn write_trace(&mut self, data: &[f64]) -> std::io::Result<()> {
        if self.published {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "PSF file is already complete and published",
            ));
        }
        let header = self.header.as_ref().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                "write the PSF header before traces",
            )
        })?;
        if self.traces_written >= header.num_traces {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "PSF contains more traces than declared by its header",
            ));
        }
        if data.len() != header.num_points as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "PSF trace has {} points; header declares {}",
                    data.len(),
                    header.num_points
                ),
            ));
        }
        for value in data {
            self.encoded.extend_from_slice(&value.to_le_bytes());
        }
        self.traces_written += 1;
        self.publish_if_complete()
    }

    /// Require a complete declared payload and durably publish it. The final
    /// `write_trace` already publishes automatically; this method makes
    /// completeness explicit for callers and reports incomplete writers.
    pub fn finish(mut self) -> std::io::Result<()> {
        if self.published {
            return Ok(());
        }
        let header = self
            .header
            .as_ref()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "PSF writer has no header"))?;
        if self.traces_written != header.num_traces {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "PSF writer has {} of {} declared traces",
                    self.traces_written, header.num_traces
                ),
            ));
        }
        self.publish_if_complete()
    }

    fn publish_if_complete(&mut self) -> std::io::Result<()> {
        let Some(header) = self.header.as_ref() else {
            return Ok(());
        };
        if self.traces_written != header.num_traces || self.published {
            return Ok(());
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err(Error::new(
                ErrorKind::Unsupported,
                "PSF path publication is unavailable in the browser",
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            crate::io::durable_file::compare_exchange_bytes(
                &self.path,
                self.expected,
                &self.encoded,
            )
            .map_err(publication_io_error)?;
            self.published = true;
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn publication_io_error(error: crate::io::durable_file::CompareExchangeError) -> Error {
    Error::other(format!("PSF publication failed: {error}"))
}

fn read_header(file: &mut File) -> std::io::Result<PsfHeader> {
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;

    let version = read_u32_le(file)?;
    let num_traces = read_u32_le(file)?;
    let num_points = read_u32_le(file)?;
    let timestamp = read_u64_le(file)?;

    Ok(PsfHeader {
        magic,
        version,
        num_traces,
        num_points,
        timestamp,
    })
}

fn write_header(file: &mut impl Write, header: &PsfHeader) -> std::io::Result<()> {
    file.write_all(&header.magic)?;
    file.write_all(&header.version.to_le_bytes())?;
    file.write_all(&header.num_traces.to_le_bytes())?;
    file.write_all(&header.num_points.to_le_bytes())?;
    file.write_all(&header.timestamp.to_le_bytes())
}

fn read_u32_le(file: &mut File) -> std::io::Result<u32> {
    let mut bytes = [0u8; 4];
    file.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_u64_le(file: &mut File) -> std::io::Result<u64> {
    let mut bytes = [0u8; 8];
    file.read_exact(&mut bytes)?;
    Ok(u64::from_le_bytes(bytes))
}

fn decode_f64_slice_le(bytes: &[u8]) -> std::io::Result<Vec<f64>> {
    if !crate::utils::numeric::is_multiple_of(bytes.len(), F64_SIZE_BYTES) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "f64 payload size is not a multiple of 8 bytes",
        ));
    }

    let mut decoded = Vec::with_capacity(bytes.len() / F64_SIZE_BYTES);
    for chunk in bytes.chunks_exact(F64_SIZE_BYTES) {
        let mut value_bytes = [0u8; F64_SIZE_BYTES];
        value_bytes.copy_from_slice(chunk);
        decoded.push(f64::from_le_bytes(value_bytes));
    }
    Ok(decoded)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_writer_publishes_readable_psf() {
        let root = unique_temp_dir("complete");
        let path = root.join("wave.psfl");
        let header = PsfHeader::new(2, 3);
        let mut writer = PsfWriter::create(&path).expect("create writer");
        writer.write_header(&header).expect("write header");
        writer.write_trace(&[1.0, 2.0, 3.0]).expect("trace one");
        writer.write_trace(&[4.0, 5.0, 6.0]).expect("trace two");
        writer.finish().expect("finish writer");

        let mut reader = PsfReader::open(&path).expect("open PSF");
        assert_eq!(reader.header().num_traces, 2);
        assert_eq!(reader.read_trace(0).unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(reader.read_trace(1).unwrap(), vec![4.0, 5.0, 6.0]);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn invalid_or_incomplete_payload_never_touches_predecessor() {
        let root = unique_temp_dir("incomplete");
        let path = root.join("wave.psfl");
        std::fs::write(&path, b"predecessor").expect("write predecessor");
        let mut writer = PsfWriter::create(&path).expect("create writer");
        writer
            .write_header(&PsfHeader::new(1, 3))
            .expect("write header");

        assert!(writer.write_trace(&[1.0, 2.0]).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"predecessor");
        assert!(writer.finish().is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"predecessor");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn complete_writer_rejects_late_external_change() {
        let root = unique_temp_dir("late-change");
        let path = root.join("wave.psfl");
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let mut writer = PsfWriter::create(&path).expect("create writer");
        writer
            .write_header(&PsfHeader::new(1, 2))
            .expect("write header");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = writer.write_trace(&[1.0, 2.0]);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-psf-writer-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
