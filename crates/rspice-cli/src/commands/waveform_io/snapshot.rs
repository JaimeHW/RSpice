//! A bounded, owned source for operations that publish exactly what they validated.
use super::*;
use std::path::PathBuf;

enum Source {
    Bytes(Vec<u8>),
    // The reader owns the bytes, so even HDF5 needs only one source buffer.
    Hdf5(Box<rustyhdf5::File>),
}

pub(crate) struct ResultSnapshot {
    path: PathBuf,
    limits: rspice_core::ResourceLimits,
    source: Source,
}

impl ResultSnapshot {
    pub(crate) fn read(path: &Path, limits: rspice_core::ResourceLimits) -> Result<Self, CliError> {
        let bytes = read_input_bytes_limited(path, limits.max_external_data_bytes)?;
        let source = if detect_format(path) == InputFormat::Hdf5 {
            Source::Hdf5(Box::new(
                rustyhdf5::File::from_bytes(bytes)
                    .map_err(|error| conversion_error(path, error))?,
            ))
        } else {
            Source::Bytes(bytes)
        };
        Ok(Self {
            path: path.to_owned(),
            limits,
            source,
        })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        match &self.source {
            Source::Bytes(bytes) => bytes,
            Source::Hdf5(file) => file.as_bytes(),
        }
    }

    /// Touchstone filenames can supply the port count. Validate the same bytes
    /// under the proposed destination name before copying them there.
    pub(crate) fn touchstone_at(&self, path: &Path) -> Result<ImportedResult, CliError> {
        let table = touchstone::parse(path, self.bytes(), self.limits, None)?;
        validate_result(path, table.into(), self.limits)
    }

    pub(crate) fn load(&self, section: Option<&str>) -> Result<ImportedResult, CliError> {
        let path = self.path();
        let limits = self.limits;
        let format = detect_format(path);
        let section = section.filter(|_| supports_sections(format));
        let text = || {
            std::str::from_utf8(self.bytes()).map_err(|error| CliError::InputReadError {
                path: path.to_owned(),
                source: std::io::Error::new(std::io::ErrorKind::InvalidData, error),
            })
        };
        let result = match &self.source {
            Source::Hdf5(file) => {
                let readback = crate::hdf5::read_hdf5_sections_from_file_with_limits(file, limits)
                    .map_err(|error| hdf5_read_error(path, error))?;
                hdf5_result(path, readback, section)?
            }
            Source::Bytes(bytes) => match format {
                InputFormat::Raw | InputFormat::RawAscii => {
                    let file = rspice_core::io::ltspice_raw::parse_raw_plots_bytes_with_limits(
                        bytes, limits,
                    )
                    .map_err(|error| raw_read_error(path, error))?;
                    raw_result(path, file, section)?
                }
                InputFormat::Csv => parse_delimited(path, text()?, ',', limits)?,
                InputFormat::Tsv => parse_delimited(path, text()?, '\t', limits)?,
                InputFormat::Json => parse_json(path, text()?, limits)?,
                InputFormat::Vcd => {
                    let document = rspice_core::io::vcd::parse_vcd_bytes_with_limits(bytes, limits)
                        .map_err(|error| crate::commands::vcd_io::read_error(path, error))?;
                    crate::commands::vcd_io::vcd_table(path, document, limits)?.into()
                }
                InputFormat::Touchstone => touchstone::parse(path, bytes, limits, section)?.into(),
                InputFormat::Hdf5 => unreachable!("HDF5 snapshots own their reader"),
            },
        };
        validate_result(path, result, limits)
    }
}
