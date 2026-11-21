//! A binary rawfile decoded as its bytes arrive, a block of rows at a time, for an import that
//! keeps a source in a results store as it reads it instead of holding it whole.
//!
//! The reader is pure: bytes in, blocks of rows out to the caller's sink, with no file, store or
//! window of its own, so the desktop import and a browser import worker read through the same
//! code. It reads one ordinary binary plot, the layout a simulator writes for one analysis. For
//! any other file it says so before a row is decoded, and the whole-file reader
//! ([`super::decode_spice_raw_plots_with_log`]) reads it instead. A plot it reads decodes to the
//! values the whole-file reader gives, bit for bit, and is refused for what that reader refuses.

use super::{RawReadError, coordinate_domain, plot_units};
use rspice_core::ResourceLimits;
use rspice_core::io::RawParseError;
use rspice_core::io::ltspice_raw::{
    RawWaveformData, parse_raw_plot_header, raw_table_has_coordinate,
    raw_time_carries_compression_marks,
};

/// About how many bytes of values one block of rows holds.
const BLOCK_BYTES: usize = 4 << 20;

/// What a streamed plot declares, read from its header before any row.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamedRawPlot {
    /// The plot's own `Plotname:` title.
    pub plot_name: String,
    pub domain: crate::WaveformDomain,
    pub coordinate_name: String,
    pub coordinate_unit: Option<String>,
    /// Each signal's name and unit, in file order.
    pub signals: Vec<(String, Option<String>)>,
    /// Whether every value is complex, as an AC plot's are.
    pub complex: bool,
    /// The rows the header declares.
    pub points: usize,
}

/// One block of decoded rows, column by column: the coordinate, each signal's real parts and, in
/// a complex plot, each signal's imaginary parts (empty otherwise).
#[derive(Debug, Clone, Copy)]
pub struct RawRowBlock<'a> {
    pub coordinate: &'a [f64],
    pub real: &'a [Vec<f64>],
    pub imag: &'a [Vec<f64>],
}

/// How a rawfile is read.
#[derive(Debug)]
pub enum RawStreamStart {
    /// As it arrives: feed the reader the bytes from [`RawStreamReader::data_offset`] on.
    Streamed(Box<RawStreamReader>),
    /// Whole, by the whole-file reader, for the reason given.
    WholeFile(&'static str),
}

/// A binary plot's rows, decoded from its bytes as they arrive.
#[derive(Debug)]
pub struct RawStreamReader {
    plot: StreamedRawPlot,
    /// The title the plot's header spells, untrimmed, for the reader's own words.
    header_plot_name: String,
    data_offset: usize,
    /// Every value a double; otherwise the coordinate is a double and the rest are singles.
    all_f64: bool,
    compression_marks: bool,
    row_bytes: usize,
    block_rows: usize,
    /// The bytes of a row split between two feeds.
    carry: Vec<u8>,
    rows: usize,
    coordinate: Vec<f64>,
    real: Vec<Vec<f64>>,
    imag: Vec<Vec<f64>>,
}

impl RawStreamReader {
    /// How the rawfile whose first bytes are `head`, `length` bytes in all, is read. A header
    /// that does not end inside `head` is read by the whole-file reader, as is any file that is
    /// not one ordinary binary plot whose rows end the file.
    pub fn start(
        head: &[u8],
        length: u64,
        limits: ResourceLimits,
    ) -> Result<RawStreamStart, RawReadError> {
        use RawStreamStart::WholeFile;
        // A header that does not read here is the whole-file reader's to refuse, in its words.
        let Ok((header, variables, data_offset)) = parse_raw_plot_header(head, limits) else {
            return Ok(WholeFile(
                "its header does not read in the bytes read first",
            ));
        };
        let flagged = |flag: &str| {
            header
                .flags
                .iter()
                .any(|declared| declared.eq_ignore_ascii_case(flag))
        };
        if !header.is_binary {
            return Ok(WholeFile("its values are text"));
        }
        if flagged("fastaccess") {
            return Ok(WholeFile("its values are stored column by column"));
        }
        if flagged("stepped") {
            return Ok(WholeFile("it holds every run of a .step sweep"));
        }
        if header.command.starts_with("RSpiceTable") {
            return Ok(WholeFile("it carries RSpice table metadata"));
        }
        if header.no_points == 0 || variables.len() < 2 || header.no_variables != variables.len() {
            return Ok(WholeFile("its header does not state its rows and signals"));
        }
        if variables.iter().any(|variable| {
            crate::numeric::nullable::nullable_value_type(&variable.var_type).is_some()
                || variable.var_type.starts_with("nullable_validity:")
        }) {
            return Ok(WholeFile("it marks unavailable samples"));
        }
        if !raw_table_has_coordinate(&header).map_err(RawReadError::Parse)?
            && rspice_core::io::raw_export::RawEventKind::from_plot_name(&header.plotname).is_some()
        {
            return Ok(WholeFile("it is an event plot"));
        }
        let declared = RawWaveformData {
            header,
            variables,
            waveforms: Vec::new(),
        };
        if declared.has_ordinal_axis().map_err(RawReadError::Parse)? {
            return Ok(WholeFile("it is a scalar result"));
        }
        let header = &declared.header;
        let columns = header.no_variables;
        let width = if header.is_complex { 2 } else { 1 };
        // The coordinate is always a double; the rest are doubles or, as LTspice writes them,
        // singles. The boundary decides, as it does for the whole-file reader: this plot's rows
        // must end the file, and the `Flags:` line breaks a tie.
        let row_bytes = |single: usize| (8 + (columns - 1) * single) * width;
        let payload = length.saturating_sub(data_offset as u64);
        let fits = |row: usize| {
            u64::try_from(row)
                .ok()
                .and_then(|row| row.checked_mul(header.no_points as u64))
                == Some(payload)
        };
        let all_f64 = match (fits(row_bytes(8)), fits(row_bytes(4))) {
            (true, true) => header.is_double,
            (true, false) => true,
            (false, true) => false,
            (false, false) => return Ok(WholeFile("its rows do not end the file")),
        };
        let domain = coordinate_domain(&declared, false)?;
        let mut units = plot_units(&declared, false, domain)?.into_iter();
        let coordinate_unit = units.next().flatten();
        let mut unit_check = crate::numeric::DecodedNumericDataset {
            domain,
            coordinate_name: declared.variables[0].name.clone(),
            coordinate_unit,
            coordinate: Vec::new(),
            signals: Vec::new(),
        };
        unit_check
            .normalize_coordinate_unit()
            .map_err(RawReadError::Coordinate)?;
        let signals = declared.variables[1..]
            .iter()
            .zip(units)
            .map(|(variable, unit)| (variable.name.clone(), unit))
            .collect::<Vec<_>>();
        let complex = header.is_complex;
        let row_bytes = row_bytes(if all_f64 { 8 } else { 4 });
        let block_rows = (BLOCK_BYTES / (8 * columns * width)).max(1);
        Ok(RawStreamStart::Streamed(Box::new(Self {
            compression_marks: raw_time_carries_compression_marks(header, &declared.variables),
            header_plot_name: header.plotname.clone(),
            plot: StreamedRawPlot {
                plot_name: header.plotname.trim().to_owned(),
                domain,
                coordinate_name: unit_check.coordinate_name,
                coordinate_unit: unit_check.coordinate_unit,
                signals,
                complex,
                points: header.no_points,
            },
            data_offset,
            all_f64,
            row_bytes,
            block_rows,
            carry: Vec::with_capacity(row_bytes),
            rows: 0,
            coordinate: Vec::with_capacity(block_rows),
            real: vec![Vec::with_capacity(block_rows); columns - 1],
            imag: if complex {
                vec![Vec::with_capacity(block_rows); columns - 1]
            } else {
                Vec::new()
            },
        })))
    }

    /// What the plot declares.
    #[must_use]
    pub fn plot(&self) -> &StreamedRawPlot {
        &self.plot
    }

    /// Where in the file the plot's rows start: the first byte to feed.
    #[must_use]
    pub fn data_offset(&self) -> usize {
        self.data_offset
    }

    /// The coordinate's first two values, read from `head`, the file's first bytes, when they
    /// hold two rows: which way the coordinate runs, before a row is fed.
    #[must_use]
    pub fn first_coordinates(&self, head: &[u8]) -> Option<(f64, f64)> {
        let at = |row: usize| {
            let start = self
                .data_offset
                .checked_add(row.checked_mul(self.row_bytes)?)?;
            let bytes = head.get(start..start.checked_add(8)?)?;
            let value = f64::from_le_bytes(bytes.try_into().ok()?);
            Some(if self.compression_marks {
                value.abs()
            } else {
                value
            })
        };
        Some((at(0)?, at(1)?))
    }

    /// Decode `bytes`, the next of the plot's data, handing `sink` each block of rows as it
    /// fills. A row split between two feeds is decoded once its last byte arrives.
    pub fn feed(
        &mut self,
        mut bytes: &[u8],
        sink: &mut dyn FnMut(RawRowBlock<'_>) -> Result<(), String>,
    ) -> Result<(), String> {
        if !self.carry.is_empty() {
            let take = (self.row_bytes - self.carry.len()).min(bytes.len());
            self.carry.extend_from_slice(&bytes[..take]);
            bytes = &bytes[take..];
            if self.carry.len() < self.row_bytes {
                return Ok(());
            }
            let row = std::mem::take(&mut self.carry);
            let decoded = self.row(&row, sink);
            self.carry = row;
            self.carry.clear();
            decoded?;
        }
        let whole = bytes.len() / self.row_bytes * self.row_bytes;
        for row in bytes[..whole].chunks_exact(self.row_bytes) {
            self.row(row, sink)?;
        }
        self.carry.extend_from_slice(&bytes[whole..]);
        Ok(())
    }

    /// Hand `sink` the last block once every byte has been fed, and say how many rows the plot
    /// held. A plot that ended before its declared rows is refused.
    pub fn finish(
        &mut self,
        sink: &mut dyn FnMut(RawRowBlock<'_>) -> Result<(), String>,
    ) -> Result<usize, String> {
        if !self.carry.is_empty() || self.rows != self.plot.points {
            return Err(data_error(format!(
                "the binary data ends after {} of the {} rows its header declares",
                self.rows, self.plot.points
            )));
        }
        self.flush(sink)?;
        Ok(self.rows)
    }

    fn row(
        &mut self,
        bytes: &[u8],
        sink: &mut dyn FnMut(RawRowBlock<'_>) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.rows == self.plot.points {
            return Err(data_error(format!(
                "the binary data holds more than the {} rows its header declares",
                self.plot.points
            )));
        }
        let mut at = 0;
        let mut value = |single: bool| {
            let value = if single {
                let bytes: [u8; 4] = bytes[at..at + 4].try_into().expect("a whole row");
                at += 4;
                f64::from(f32::from_le_bytes(bytes))
            } else {
                let bytes: [u8; 8] = bytes[at..at + 8].try_into().expect("a whole row");
                at += 8;
                f64::from_le_bytes(bytes)
            };
            if value.is_finite() {
                Ok(value)
            } else {
                Err(data_error(format!("Non-finite binary raw value: {value}")))
            }
        };
        let mut coordinate = value(false)?;
        if self.plot.complex {
            let imaginary = value(false)?;
            if imaginary != 0.0 {
                // The whole-file reader checks the coordinate once the plot is read, outside it.
                return Err(RawReadError::Parse(RawParseError::DataError(format!(
                    "RAW plot '{}': independent coordinate '{}' has a nonzero imaginary \
                     component at point {}; table coordinates must be real",
                    self.header_plot_name, self.plot.coordinate_name, self.rows
                )))
                .to_string());
            }
        }
        if self.compression_marks {
            // LTspice marks the points its waveform compression kept by setting the sign bit of
            // their time; the time itself is the magnitude.
            coordinate = coordinate.abs();
        }
        self.coordinate.push(coordinate);
        let single = !self.all_f64;
        for signal in 0..self.real.len() {
            self.real[signal].push(value(single)?);
            if self.plot.complex {
                self.imag[signal].push(value(single)?);
            }
        }
        self.rows += 1;
        if self.coordinate.len() == self.block_rows {
            self.flush(sink)?;
        }
        Ok(())
    }

    fn flush(
        &mut self,
        sink: &mut dyn FnMut(RawRowBlock<'_>) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.coordinate.is_empty() {
            return Ok(());
        }
        sink(RawRowBlock {
            coordinate: &self.coordinate,
            real: &self.real,
            imag: &self.imag,
        })?;
        self.coordinate.clear();
        self.real.iter_mut().for_each(Vec::clear);
        self.imag.iter_mut().for_each(Vec::clear);
        Ok(())
    }
}

/// A data error in the whole-file reader's words: the plot streamed is always the file's first.
fn data_error(detail: String) -> String {
    RawReadError::Parse(RawParseError::DataError(format!("plot 1: {detail}"))).to_string()
}

#[cfg(test)]
mod tests;
