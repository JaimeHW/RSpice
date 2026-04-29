use super::types::{TouchstoneDataFormat, TouchstoneMatrixFormat, TouchstoneOptions};
use super::*;

mod delimited;
mod nutmeg;
mod psf;
mod touchstone;

// =============================================================================
// Waveform Reader
// =============================================================================

/// Waveform file reader
pub struct WaveformReader {
    format: WaveformFormat,
}

impl WaveformReader {
    /// Create reader for format
    pub fn new(format: WaveformFormat) -> Self {
        Self { format }
    }

    /// Read from file
    pub fn read(&self, path: &Path) -> Result<WaveformDataset, String> {
        match self.format {
            WaveformFormat::Psf => self.read_psf(path),
            WaveformFormat::Csv => self.read_csv(path),
            WaveformFormat::Tsv => self.read_tsv(path),
            WaveformFormat::Nutmeg | WaveformFormat::AsciiRaw => self.read_nutmeg(path),
            WaveformFormat::Touchstone => self.read_touchstone(path),
            _ => Err(format!(
                "Format {:?} read is not implemented (supported: PSF-Lite, Csv, Tsv, Nutmeg/AsciiRaw, Touchstone)",
                self.format
            )),
        }
    }
}
