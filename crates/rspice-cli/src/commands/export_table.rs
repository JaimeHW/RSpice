//! Unified tabular result model shared by `run`, `convert`, and `compare`.
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
//!
//! The same structure is what `convert` and `compare` read files back into;
//! see [`crate::commands::waveform_io`].

use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ComplexSignal, ScalarSignal};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Data for one exported signal column.
pub(crate) enum ColumnData {
    Real(Vec<f64>),
    Complex { real: Vec<f64>, imag: Vec<f64> },
}

/// One exported signal column.
pub(crate) struct ExportColumn {
    /// Display name, e.g. `V(out)` or `onoise_spectrum`
    pub(crate) name: String,
    /// Rawfile variable type, e.g. `voltage`
    pub(crate) var_type: String,
    pub(crate) data: ColumnData,
}

/// A complete result table for one analysis.
pub(crate) struct ExportTable {
    /// JSON `analysis` tag, e.g. `ac`
    pub(crate) analysis: String,
    /// Rawfile `Plotname`, e.g. `AC Analysis`
    pub(crate) plot_name: String,
    /// Scale (independent variable) name, e.g. `time` or `frequency`
    pub(crate) scale_name: String,
    /// Rawfile variable type of the scale
    pub(crate) scale_type: String,
    pub(crate) scale: Vec<f64>,
    pub(crate) columns: Vec<ExportColumn>,
}

/// Build a table from real-valued signals (transient, DC sweep, noise, ...).
pub(crate) fn scalar_table(
    analysis: impl Into<String>,
    plot_name: impl Into<String>,
    scale_name: impl Into<String>,
    scale_type: impl Into<String>,
    scale: Vec<f64>,
    signals: &[ScalarSignal],
) -> ExportTable {
    ExportTable {
        analysis: analysis.into(),
        plot_name: plot_name.into(),
        scale_name: scale_name.into(),
        scale_type: scale_type.into(),
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                name: signal.display_name.clone(),
                var_type: signal.kind.raw_variable_type().to_string(),
                data: ColumnData::Real(signal.values.clone()),
            })
            .collect(),
    }
}

/// Build a table from complex-valued signals (AC and related sweeps).
pub(crate) fn complex_table(
    analysis: impl Into<String>,
    plot_name: impl Into<String>,
    scale: Vec<f64>,
    signals: &[ComplexSignal],
) -> ExportTable {
    ExportTable {
        analysis: analysis.into(),
        plot_name: plot_name.into(),
        scale_name: "frequency".to_string(),
        scale_type: "frequency".to_string(),
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                name: signal.display_name.clone(),
                var_type: signal.kind.raw_variable_type().to_string(),
                data: ColumnData::Complex {
                    real: signal.real.clone(),
                    imag: signal.imag.clone(),
                },
            })
            .collect(),
    }
}

impl ExportTable {
    pub(crate) fn is_complex(&self) -> bool {
        self.columns
            .iter()
            .any(|column| matches!(column.data, ColumnData::Complex { .. }))
    }

    /// Keep only the requested columns (case-insensitive name match).
    ///
    /// Names match either the full column name (`V(out)`) or the bare inner
    /// name (`out`). Unknown names are an error listing what is available.
    pub(crate) fn select_variables(&mut self, requested: &[String]) -> Result<(), CliError> {
        if requested.is_empty() {
            return Ok(());
        }

        let matches = |column: &ExportColumn, want: &str| {
            if column.name.eq_ignore_ascii_case(want) {
                return true;
            }
            // Match `out` against `V(out)` / `I(out)`
            let inner = column
                .name
                .split_once('(')
                .and_then(|(_, rest)| rest.strip_suffix(')'));
            inner.is_some_and(|inner| inner.eq_ignore_ascii_case(want))
        };

        for want in requested {
            if !self.columns.iter().any(|column| matches(column, want)) {
                let available: Vec<&str> =
                    self.columns.iter().map(|c| c.name.as_str()).collect();
                return Err(CliError::InvalidArgument {
                    message: format!("variable '{}' not found in input", want),
                    suggestion: Some(format!("available variables: {}", available.join(", "))),
                });
            }
        }

        self.columns
            .retain(|column| requested.iter().any(|want| matches(column, want)));
        Ok(())
    }

    /// Keep only rows whose scale value lies within `[start, stop]`.
    pub(crate) fn clip_scale_range(&mut self, start: Option<f64>, stop: Option<f64>) {
        if start.is_none() && stop.is_none() {
            return;
        }
        let lo = start.unwrap_or(f64::NEG_INFINITY);
        let hi = stop.unwrap_or(f64::INFINITY);

        let keep: Vec<bool> = self
            .scale
            .iter()
            .map(|&value| value >= lo && value <= hi)
            .collect();

        let filter = |values: &mut Vec<f64>| {
            let mut index = 0;
            values.retain(|_| {
                let kept = keep.get(index).copied().unwrap_or(false);
                index += 1;
                kept
            });
        };

        filter(&mut self.scale);
        for column in &mut self.columns {
            match &mut column.data {
                ColumnData::Real(values) => filter(values),
                ColumnData::Complex { real, imag } => {
                    filter(real);
                    filter(imag);
                }
            }
        }
    }

    /// Flatten to real-valued named series, scale first; complex columns
    /// expand to `Re(name)` / `Im(name)`. Used for value-wise comparison.
    pub(crate) fn to_real_series(&self) -> Vec<(String, Vec<f64>)> {
        let mut series = Vec::with_capacity(self.columns.len() + 1);
        series.push((self.scale_name.clone(), self.scale.clone()));
        for column in &self.columns {
            match &column.data {
                ColumnData::Real(values) => series.push((column.name.clone(), values.clone())),
                ColumnData::Complex { real, imag } => {
                    series.push((format!("Re({})", column.name), real.clone()));
                    series.push((format!("Im({})", column.name), imag.clone()));
                }
            }
        }
        series
    }

    /// Write the table to `path` in the requested format.
    ///
    /// HDF5 has analysis-specific sections and must be handled by the caller.
    pub(crate) fn write(&self, path: &Path, format: OutputFormat) -> Result<(), CliError> {
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
