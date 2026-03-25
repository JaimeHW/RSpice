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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
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
    file: File,
}

impl PsfWriter {
    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file })
    }

    pub fn write_header(&mut self, header: &PsfHeader) -> std::io::Result<()> {
        write_header(&mut self.file, header)
    }

    pub fn write_trace(&mut self, data: &[f64]) -> std::io::Result<()> {
        let encoded = encode_f64_slice_le(data);
        self.file.write_all(&encoded)
    }
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

fn write_header(file: &mut File, header: &PsfHeader) -> std::io::Result<()> {
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

fn encode_f64_slice_le(data: &[f64]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len() * F64_SIZE_BYTES);
    for value in data {
        encoded.extend_from_slice(&value.to_le_bytes());
    }
    encoded
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
    use std::io::SeekFrom;
    use tempfile::tempdir;

    #[test]
    fn test_psf_binary_roundtrip() -> std::io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.psf");

        let num_traces = 2;
        let num_points = 100;
        let header = PsfHeader::new(num_traces, num_points);

        // Write
        {
            let mut writer = PsfWriter::create(&file_path)?;
            writer.write_header(&header)?;
            writer.write_trace(&vec![1.0; num_points as usize])?;
            writer.write_trace(&vec![2.0; num_points as usize])?;
        }

        // Read
        {
            let mut reader = PsfReader::open(&file_path)?;
            let t1 = reader.read_trace(0)?;
            let t2 = reader.read_trace(1)?;

            assert_eq!(t1[0], 1.0);
            assert_eq!(t2[0], 2.0);
            assert_eq!(t1.len(), num_points as usize);
        }

        Ok(())
    }

    #[test]
    fn test_read_trace_rejects_out_of_bounds_index() -> std::io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("oob.psf");

        let mut writer = PsfWriter::create(&file_path)?;
        writer.write_header(&PsfHeader::new(1, 4))?;
        writer.write_trace(&[1.0, 2.0, 3.0, 4.0])?;
        drop(writer);

        let mut reader = PsfReader::open(&file_path)?;
        let err = reader
            .read_trace(1)
            .expect_err("index 1 must be out of bounds");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        Ok(())
    }

    #[test]
    fn test_trace_payload_is_written_in_little_endian() -> std::io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("le.psf");

        let samples = [1.0f64, -2.5, std::f64::consts::PI];

        let mut writer = PsfWriter::create(&file_path)?;
        writer.write_header(&PsfHeader::new(1, samples.len() as u32))?;
        writer.write_trace(&samples)?;
        drop(writer);

        let mut file = File::open(file_path)?;
        file.seek(SeekFrom::Start(HEADER_SIZE_BYTES))?;
        let mut payload = vec![0u8; samples.len() * F64_SIZE_BYTES];
        file.read_exact(&mut payload)?;

        let expected = encode_f64_slice_le(&samples);
        assert_eq!(payload, expected);
        Ok(())
    }

    #[test]
    fn test_open_rejects_truncated_header() -> std::io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("truncated_header.psf");

        let mut file = File::create(&file_path)?;
        file.write_all(&[0u8; 8])?;
        drop(file);

        let err = match PsfReader::open(&file_path) {
            Ok(_) => return Err(Error::other("header must be rejected")),
            Err(err) => err,
        };
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
        Ok(())
    }

    #[test]
    fn test_read_trace_rejects_truncated_payload() -> std::io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("truncated_payload.psf");

        let header = PsfHeader::new(1, 2);
        let mut file = File::create(&file_path)?;
        write_header(&mut file, &header)?;
        file.write_all(&[0u8; 8])?;
        drop(file);

        let mut reader = PsfReader::open(&file_path)?;
        let err = reader
            .read_trace(0)
            .expect_err("truncated payload must be rejected");
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
        Ok(())
    }

    #[test]
    fn test_decode_f64_slice_rejects_non_multiple_of_eight() {
        let err = decode_f64_slice_le(&[1u8, 2u8, 3u8]).expect_err("must reject");
        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }
}
