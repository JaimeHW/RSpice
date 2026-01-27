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
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

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
                .unwrap()
                .as_secs(),
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
        let mut buffer = [0u8; 24];
        file.read_exact(&mut buffer)?;

        let header: PsfHeader = bincode::deserialize(&buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if header.magic != PsfHeader::MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid PSF magic",
            ));
        }

        Ok(Self { file, header })
    }

    /// Read a specific trace by index
    pub fn read_trace(&mut self, trace_idx: u32) -> std::io::Result<Vec<f64>> {
        if trace_idx >= self.header.num_traces {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Trace index out of bounds",
            ));
        }

        let offset = 24 + (trace_idx as u64 * self.header.num_points as u64 * 8);
        self.file.seek(SeekFrom::Start(offset))?;

        let mut data = vec![0.0f64; self.header.num_points as usize];
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, data.len() * 8) };
        self.file.read_exact(byte_slice)?;

        Ok(data)
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
        let buffer = bincode::serialize(header)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        self.file.write_all(&buffer)
    }

    pub fn write_trace(&mut self, data: &[f64]) -> std::io::Result<()> {
        let byte_slice =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 8) };
        self.file.write_all(byte_slice)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
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
}
