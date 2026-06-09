//! Unified text/raw output writers for `rspice run` analyses.
//!
//! Every analysis that honors `--output` routes its tabular results through
//! [`ExportTable`], which renders the same data in any requested
//! [`OutputFormat`] (HDF5 is handled separately by callers because its
//! sections are analysis-specific):
//!
//! - `raw` / `ascii`: SPICE rawfile (binary or ASCII values), with
//!   `Flags: complex` and interleaved real/imaginary pairs for AC data
//! - `csv` / `tsv`: one column for the scale plus one (real) or two
//!   (complex, `Re(..)`/`Im(..)`) columns per signal
//! - `json`: a `{"analysis", "scale", "signals"}` document

use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ComplexSignal, ScalarSignal};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Data for one exported signal column.
pub(super) enum ColumnData {
    Real(Vec<f64>),
    Complex { real: Vec<f64>, imag: Vec<f64> },
}

/// One exported signal column.
pub(super) struct ExportColumn {
    /// Display name, e.g. `V(out)` or `onoise_spectrum`
    pub(super) name: String,
    /// Rawfile variable type, e.g. `voltage`
    pub(super) var_type: &'static str,
    pub(super) data: ColumnData,
}

/// A complete result table for one analysis.
pub(super) struct ExportTable {
    /// JSON `analysis` tag, e.g. `ac`
    pub(super) analysis: &'static str,
    /// Rawfile `Plotname`, e.g. `AC Analysis`
    pub(super) plot_name: &'static str,
    /// Scale (independent variable) name, e.g. `time` or `frequency`
    pub(super) scale_name: String,
    /// Rawfile variable type of the scale
    pub(super) scale_type: &'static str,
    pub(super) scale: Vec<f64>,
    pub(super) columns: Vec<ExportColumn>,
}

/// Build a table from real-valued signals (transient, DC sweep, noise, ...).
pub(super) fn scalar_table(
    analysis: &'static str,
    plot_name: &'static str,
    scale_name: impl Into<String>,
    scale_type: &'static str,
    scale: Vec<f64>,
    signals: &[ScalarSignal],
) -> ExportTable {
    ExportTable {
        analysis,
        plot_name,
        scale_name: scale_name.into(),
        scale_type,
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                name: signal.display_name.clone(),
                var_type: signal.kind.raw_variable_type(),
                data: ColumnData::Real(signal.values.clone()),
            })
            .collect(),
    }
}

/// Build a table from complex-valued signals (AC and related sweeps).
pub(super) fn complex_table(
    analysis: &'static str,
    plot_name: &'static str,
    scale: Vec<f64>,
    signals: &[ComplexSignal],
) -> ExportTable {
    ExportTable {
        analysis,
        plot_name,
        scale_name: "frequency".to_string(),
        scale_type: "frequency",
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                name: signal.display_name.clone(),
                var_type: signal.kind.raw_variable_type(),
                data: ColumnData::Complex {
                    real: signal.real.clone(),
                    imag: signal.imag.clone(),
                },
            })
            .collect(),
    }
}

impl ExportTable {
    fn is_complex(&self) -> bool {
        self.columns
            .iter()
            .any(|column| matches!(column.data, ColumnData::Complex { .. }))
    }

    /// Write the table to `path` in the requested format.
    ///
    /// HDF5 has analysis-specific sections and must be handled by the caller.
    pub(super) fn write(&self, path: &Path, format: OutputFormat) -> Result<(), CliError> {
        let file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
        let mut writer = BufWriter::new(file);

        match format {
            OutputFormat::Raw => self.write_raw(&mut writer, path, true),
            OutputFormat::RawAscii => self.write_raw(&mut writer, path, false),
            OutputFormat::Csv => self.write_delimited(&mut writer, path, ','),
            OutputFormat::Tsv => self.write_delimited(&mut writer, path, '\t'),
            OutputFormat::Json => self.write_json(&mut writer, path),
            OutputFormat::Hdf5 => Err(CliError::InternalError {
                message: "HDF5 export must be handled by the analysis-specific writer".to_string(),
            }),
        }
    }

    /// Value of `column` at row `row` as (real, imag).
    fn value_at(data: &ColumnData, row: usize) -> (f64, f64) {
        match data {
            ColumnData::Real(values) => (values.get(row).copied().unwrap_or(0.0), 0.0),
            ColumnData::Complex { real, imag } => (
                real.get(row).copied().unwrap_or(0.0),
                imag.get(row).copied().unwrap_or(0.0),
            ),
        }
    }

    fn write_raw<W: Write>(&self, writer: &mut W, path: &Path, binary: bool) -> Result<(), CliError> {
        let complex = self.is_complex();
        let io_err = |e: std::io::Error| CliError::output_error(path, e);

        writeln!(writer, "Title: {}", self.plot_name).map_err(io_err)?;
        writeln!(writer, "Date: Generated by RSpice").map_err(io_err)?;
        writeln!(writer, "Plotname: {}", self.plot_name).map_err(io_err)?;
        writeln!(writer, "Flags: {}", if complex { "complex" } else { "real" }).map_err(io_err)?;
        writeln!(writer, "No. Variables: {}", self.columns.len() + 1).map_err(io_err)?;
        writeln!(writer, "No. Points: {}", self.scale.len()).map_err(io_err)?;
        writeln!(writer, "Variables:").map_err(io_err)?;
        writeln!(writer, "\t0\t{}\t{}", self.scale_name, self.scale_type).map_err(io_err)?;
        for (index, column) in self.columns.iter().enumerate() {
            writeln!(writer, "\t{}\t{}\t{}", index + 1, column.name, column.var_type)
                .map_err(io_err)?;
        }

        if binary {
            writeln!(writer, "Binary:").map_err(io_err)?;
            for row in 0..self.scale.len() {
                let mut emit = |re: f64, im: f64| -> Result<(), CliError> {
                    writer.write_all(&re.to_le_bytes()).map_err(io_err)?;
                    if complex {
                        writer.write_all(&im.to_le_bytes()).map_err(io_err)?;
                    }
                    Ok(())
                };
                emit(self.scale[row], 0.0)?;
                for column in &self.columns {
                    let (re, im) = Self::value_at(&column.data, row);
                    emit(re, im)?;
                }
            }
        } else {
            writeln!(writer, "Values:").map_err(io_err)?;
            for row in 0..self.scale.len() {
                write!(writer, "{}", row).map_err(io_err)?;
                let emit = |writer: &mut W, re: f64, im: f64| -> Result<(), CliError> {
                    if complex {
                        write!(writer, "\t{:.15e},{:.15e}", re, im).map_err(io_err)
                    } else {
                        write!(writer, "\t{:.15e}", re).map_err(io_err)
                    }
                };
                emit(writer, self.scale[row], 0.0)?;
                for column in &self.columns {
                    let (re, im) = Self::value_at(&column.data, row);
                    emit(writer, re, im)?;
                }
                writeln!(writer).map_err(io_err)?;
            }
        }

        Ok(())
    }

    fn write_delimited<W: Write>(
        &self,
        writer: &mut W,
        path: &Path,
        delimiter: char,
    ) -> Result<(), CliError> {
        let io_err = |e: std::io::Error| CliError::output_error(path, e);

        write!(writer, "{}", self.scale_name).map_err(io_err)?;
        for column in &self.columns {
            match column.data {
                ColumnData::Real(_) => {
                    write!(writer, "{}{}", delimiter, column.name).map_err(io_err)?;
                }
                ColumnData::Complex { .. } => {
                    write!(
                        writer,
                        "{0}Re({1}){0}Im({1})",
                        delimiter, column.name
                    )
                    .map_err(io_err)?;
                }
            }
        }
        writeln!(writer).map_err(io_err)?;

        for (row, scale_value) in self.scale.iter().enumerate() {
            write!(writer, "{:.9e}", scale_value).map_err(io_err)?;
            for column in &self.columns {
                let (re, im) = Self::value_at(&column.data, row);
                match column.data {
                    ColumnData::Real(_) => {
                        write!(writer, "{}{:.9e}", delimiter, re).map_err(io_err)?;
                    }
                    ColumnData::Complex { .. } => {
                        write!(writer, "{0}{1:.9e}{0}{2:.9e}", delimiter, re, im)
                            .map_err(io_err)?;
                    }
                }
            }
            writeln!(writer).map_err(io_err)?;
        }

        Ok(())
    }

    fn write_json<W: Write>(&self, writer: &mut W, path: &Path) -> Result<(), CliError> {
        let signals: Vec<serde_json::Value> = self
            .columns
            .iter()
            .map(|column| match &column.data {
                ColumnData::Real(values) => serde_json::json!({
                    "name": column.name,
                    "values": values,
                }),
                ColumnData::Complex { real, imag } => serde_json::json!({
                    "name": column.name,
                    "real": real,
                    "imag": imag,
                }),
            })
            .collect();

        let json = serde_json::json!({
            "analysis": self.analysis,
            "scale": {
                "name": self.scale_name,
                "values": self.scale,
            },
            "signals": signals,
        });

        serde_json::to_writer_pretty(&mut *writer, &json)
            .map_err(|e| CliError::output_json_error(path, e))?;
        writer
            .write_all(b"\n")
            .map_err(|e| CliError::output_error(path, e))?;
        Ok(())
    }
}
