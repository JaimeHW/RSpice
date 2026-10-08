//! Unified tabular result model shared by `run`, `convert`, and `compare`.
//!
//! Every analysis that honors `--output` routes its tabular results through
//! [`ExportTable`], which renders the same data in any requested
//! [`OutputFormat`]. HDF5 retains a typed table section; analyses with richer
//! HDF5 sections may use their own projections:
//!
//! - `raw` / `ascii`: SPICE rawfile (binary or ASCII values), with
//!   `Flags: complex` and interleaved real/imaginary pairs for AC data
//! - `csv` / `tsv`: one column for the scale plus one (real) or two
//!   (complex, `Re(..)`/`Im(..)`) columns per signal
//! - `json`: a `{"analysis", "scale", "signals"}` document
//!
//! The same structure is what `convert` and `compare` read files back into;
//! see [`crate::commands::waveform_io`]. `vcd` is not among them: a dump
//! carries irregular event timelines rather than grid columns, so it is built
//! by [`crate::commands::vcd_io`] and refused here.

use crate::cli::{CliError, OutputFormat};
use crate::commands::publish;
use crate::commands::run_signals::{ComplexSignal, ScalarSignal};
use rspice_formats::delimited::layout::{ColumnKind, RECORD_MARKER, complex_pair_name};
use std::io::Write;
use std::path::Path;

/// Data for one exported signal column.
#[derive(Clone)]
pub(crate) enum ColumnData {
    NullableReal(Vec<Option<f64>>),
    NullableComplex(Vec<Option<rspice_core::Complex64>>),
    Real(Vec<f64>),
    Complex { real: Vec<f64>, imag: Vec<f64> },
}

impl ColumnData {
    pub(crate) fn optional_real(values: Vec<Option<f64>>) -> Self {
        if values.iter().all(Option::is_some) {
            Self::Real(values.into_iter().flatten().collect())
        } else {
            Self::NullableReal(values)
        }
    }

    pub(crate) fn optional_complex(values: Vec<Option<rspice_core::Complex64>>) -> Self {
        if values.iter().all(Option::is_some) {
            let (real, imag) = values
                .into_iter()
                .flatten()
                .map(|value| (value.re, value.im))
                .unzip();
            Self::Complex { real, imag }
        } else {
            Self::NullableComplex(values)
        }
    }

    pub(crate) fn optional_complex_parts(
        real: Vec<Option<f64>>,
        imag: Vec<Option<f64>>,
    ) -> Result<Self, &'static str> {
        if real.len() != imag.len() {
            return Err("complex component lengths differ");
        }
        // Keep ordinary dense imports on their original two-vector path.
        if real.iter().chain(&imag).all(Option::is_some) {
            return Ok(Self::Complex {
                real: real.into_iter().flatten().collect(),
                imag: imag.into_iter().flatten().collect(),
            });
        }
        let values = real
            .into_iter()
            .zip(imag)
            .map(|(real, imag)| match (real, imag) {
                (Some(real), Some(imag)) => Ok(Some(rspice_core::Complex64::new(real, imag))),
                (None, None) => Ok(None),
                _ => Err("complex real and imaginary samples must have matching availability"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::NullableComplex(values))
    }
}

/// One exported signal column.
#[derive(Clone)]
pub(crate) struct ExportColumn {
    /// Explicit unit symbol, preserving case and prefixes; None is unstated.
    pub(crate) unit: Option<String>,
    /// Display name, e.g. `V(out)` or `onoise_spectrum`
    pub(crate) name: String,
    /// Rawfile variable type, e.g. `voltage`
    pub(crate) var_type: String,
    pub(crate) data: ColumnData,
}

/// A complete result table for one analysis.
#[derive(Clone)]
pub(crate) struct ExportTable {
    /// Explicit unit of the independent coordinate.
    pub(crate) scale_unit: Option<String>,
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

/// The base unit legacy rawfile types imply. Unknown types state no unit.
pub(crate) fn type_unit(var_type: &str) -> Option<&'static str> {
    if var_type.trim() == "S" {
        return Some("S");
    }
    Some(match var_type.trim().to_ascii_lowercase().as_str() {
        "v" | "volt" | "volts" | "voltage" => "V",
        "a" | "amp" | "ampere" | "amperes" | "current" => "A",
        "s" | "sec" | "second" | "seconds" | "time" => "s",
        "hz" | "hertz" | "frequency" => "Hz",
        "rad/s" | "radian_per_second" | "angular_frequency" => "rad/s",
        "pole" | "zero" => "rad/s",
        "ohm" | "ohms" | "resistance" | "impedance" => "ohm",
        "siemens" | "mho" | "conductance" => "S",
        "w" | "watt" | "power" => "W",
        "deg" | "degree" | "degrees" => "deg",
        "rad" | "radian" | "radians" => "rad",
        "1" | "scalar" | "ratio" | "dimensionless" | "index" => "1",
        "logic" | "digital" => "logic",
        _ => return None,
    })
}

pub(crate) fn stated_unit(unit: &rspice_core::execution::SignalUnit) -> Option<String> {
    (unit != &rspice_core::execution::SignalUnit::Unspecified).then(|| unit.symbol())
}

/// Preserve known physical quantities when flattening a typed result descriptor.
pub(crate) fn unit_type<'a>(
    unit: &rspice_core::execution::SignalUnit,
    fallback: &'a str,
) -> &'a str {
    use rspice_core::execution::SignalUnit;
    match unit {
        SignalUnit::Volt => "voltage",
        SignalUnit::Ampere => "current",
        SignalUnit::Ohm => "resistance",
        SignalUnit::Siemens => "conductance",
        SignalUnit::Watt => "power",
        SignalUnit::Hertz => "frequency",
        SignalUnit::Second => "time",
        SignalUnit::Degree => "degrees",
        SignalUnit::Radian => "radians",
        SignalUnit::RadianPerSecond => "angular_frequency",
        SignalUnit::Dimensionless => "dimensionless",
        SignalUnit::Logic => "digital",
        _ => fallback,
    }
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
        scale_unit: None,
        analysis: analysis.into(),
        plot_name: plot_name.into(),
        scale_name: scale_name.into(),
        scale_type: scale_type.into(),
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                unit: signal.unit_symbol(),
                name: signal.display_name.clone(),
                var_type: signal.raw_variable_type().to_string(),
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
        scale_unit: None,
        analysis: analysis.into(),
        plot_name: plot_name.into(),
        scale_name: "frequency".to_string(),
        scale_type: "frequency".to_string(),
        scale,
        columns: signals
            .iter()
            .map(|signal| ExportColumn {
                unit: signal.unit_symbol(),
                name: signal.display_name.clone(),
                var_type: signal.raw_variable_type().to_string(),
                data: ColumnData::Complex {
                    real: signal.real.clone(),
                    imag: signal.imag.clone(),
                },
            })
            .collect(),
    }
}

impl ExportTable {
    /// Dense formats carry nullable columns as an explicitly typed value/validity pair.
    /// Zero padding is meaningful only with the accompanying 0/1 validity column.
    pub(crate) fn dense_encoding(&self) -> std::borrow::Cow<'_, Self> {
        if !self.columns.iter().any(|c| {
            matches!(
                c.data,
                ColumnData::NullableReal(_) | ColumnData::NullableComplex(_)
            )
        }) {
            return std::borrow::Cow::Borrowed(self);
        }
        let mut table = self.clone();
        let mut names: std::collections::HashSet<_> =
            self.columns.iter().map(|c| c.name.clone()).collect();
        table.columns = table
            .columns
            .into_iter()
            .flat_map(|column| {
                let (data, validity, representation) = match &column.data {
                    ColumnData::NullableReal(values) => (
                        ColumnData::Real(values.iter().map(|v| v.unwrap_or(0.0)).collect()),
                        values.iter().map(|v| f64::from(v.is_some())).collect(),
                        "nullable_real",
                    ),
                    ColumnData::NullableComplex(values) => (
                        ColumnData::Complex {
                            real: values.iter().map(|v| v.map_or(0.0, |v| v.re)).collect(),
                            imag: values.iter().map(|v| v.map_or(0.0, |v| v.im)).collect(),
                        },
                        values.iter().map(|v| f64::from(v.is_some())).collect(),
                        "nullable_complex",
                    ),
                    _ => return vec![column],
                };
                let base = format!("Valid({})", column.name);
                let mut mask_name = base.clone();
                let mut suffix = 1;
                while !names.insert(mask_name.clone()) {
                    mask_name = format!("{base} [{suffix}]");
                    suffix += 1;
                }
                vec![
                    ExportColumn {
                        name: column.name,
                        var_type: format!("{representation}:{}", column.var_type),
                        unit: column.unit,
                        data,
                    },
                    ExportColumn {
                        name: mask_name,
                        var_type: format!("nullable_validity:{}", column.var_type),
                        unit: Some("1".into()),
                        data: ColumnData::Real(validity),
                    },
                ]
            })
            .collect();
        std::borrow::Cow::Owned(table)
    }

    pub(crate) fn restore_nullable_columns(&mut self) -> Result<(), String> {
        let mut columns = Vec::new();
        let mut encoded = std::mem::take(&mut self.columns).into_iter();
        while let Some(column) = encoded.next() {
            let representation = column
                .var_type
                .strip_prefix("nullable_real:")
                .map(|kind| (kind, false))
                .or_else(|| {
                    column
                        .var_type
                        .strip_prefix("nullable_complex:")
                        .map(|kind| (kind, true))
                });
            let Some((kind, complex)) = representation else {
                if column.var_type.starts_with("nullable_validity:") {
                    return Err("nullable validity column has no preceding value column".into());
                }
                columns.push(column);
                continue;
            };
            let name = column.name;
            let validity = encoded
                .next()
                .ok_or("nullable value column has no validity column")?;
            // Pairing is positional and explicitly typed; the mask label can
            // carry a suffix to avoid collisions with authored signal names.
            if validity.var_type != format!("nullable_validity:{kind}")
                || validity.unit.as_deref() != Some("1")
            {
                return Err("nullable validity column does not match its value column".into());
            }
            let ColumnData::Real(flags) = validity.data else {
                return Err("nullable validity columns must be real".into());
            };
            let restore = |values: Vec<f64>| -> Result<Vec<Option<f64>>, String> {
                if values.len() != flags.len() {
                    return Err("nullable value/validity lengths differ".into());
                }
                values
                    .into_iter()
                    .zip(&flags)
                    .map(|(value, &flag)| match flag {
                        0.0 if value == 0.0 => Ok(None),
                        1.0 if value.is_finite() => Ok(Some(value)),
                        _ => Err("invalid nullable value or validity flag".to_owned()),
                    })
                    .collect()
            };
            let data = match (complex, column.data) {
                (false, ColumnData::Real(values)) => ColumnData::optional_real(restore(values)?),
                (true, ColumnData::Complex { real, imag }) => {
                    ColumnData::optional_complex_parts(restore(real)?, restore(imag)?)?
                }
                _ => {
                    return Err(
                        "nullable dense column representation does not match its type".into(),
                    );
                }
            };
            columns.push(ExportColumn {
                name,
                var_type: kind.to_owned(),
                unit: column.unit,
                data,
            });
        }
        self.columns = columns;
        Ok(())
    }

    pub(crate) fn is_complex(&self) -> bool {
        self.columns.iter().any(|column| {
            matches!(
                column.data,
                ColumnData::Complex { .. } | ColumnData::NullableComplex(_)
            )
        })
    }

    /// Keep only the requested columns, matching names without case sensitivity.
    /// Qualified names select exactly that column; a bare alias must be unique.
    pub(crate) fn select_variables(&mut self, requested: &[String]) -> Result<(), CliError> {
        if requested.is_empty() {
            return Ok(());
        }

        let mut aliases = std::collections::HashMap::<String, Vec<usize>>::new();
        for (index, column) in self.columns.iter().enumerate() {
            let name = column.name.trim().to_ascii_lowercase();
            if let Some(inner) = name
                .split_once('(')
                .and_then(|(_, rest)| rest.strip_suffix(')'))
            {
                aliases
                    .entry(inner.trim().to_owned())
                    .or_default()
                    .push(index);
            }
            aliases.entry(name).or_default().push(index);
        }
        let mut keep = vec![false; self.columns.len()];
        for want in requested {
            let name = want.trim().to_ascii_lowercase();
            let qualified = name.contains('(') && name.ends_with(')');
            let matches: Vec<_> = aliases
                .get(&name)
                .into_iter()
                .flatten()
                .copied()
                .filter(|&index| {
                    !qualified || self.columns[index].name.trim().eq_ignore_ascii_case(&name)
                })
                .collect();
            match matches.as_slice() {
                [] => {
                    return Err(CliError::InvalidArgument {
                        message: format!("variable '{want}' not found in input"),
                        suggestion: Some(format!(
                            "available variables: {}",
                            self.columns
                                .iter()
                                .map(|column| column.name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )),
                    });
                }
                [index] => keep[*index] = true,
                _ => {
                    return Err(CliError::InvalidArgument {
                        message: format!("variable selector '{want}' is ambiguous"),
                        suggestion: Some(format!(
                            "use a full column name: {}",
                            matches
                                .iter()
                                .map(|&index| self.columns[index].name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )),
                    });
                }
            }
        }
        let mut keep = keep.into_iter();
        self.columns.retain(|_| keep.next().unwrap_or(false));
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

        fn filter<T>(values: &mut Vec<T>, keep: &[bool]) {
            let mut index = 0;
            values.retain(|_| {
                let kept = keep.get(index).copied().unwrap_or(false);
                index += 1;
                kept
            });
        }

        filter(&mut self.scale, &keep);
        for column in &mut self.columns {
            match &mut column.data {
                ColumnData::NullableReal(values) => filter(values, &keep),
                ColumnData::NullableComplex(values) => filter(values, &keep),
                ColumnData::Real(values) => filter(values, &keep),
                ColumnData::Complex { real, imag } => {
                    filter(real, &keep);
                    filter(imag, &keep);
                }
            }
        }
    }

    /// Write the table to `path` in the requested format.
    ///
    /// VCD is not a table at all — it carries event timelines, which only a
    /// transient captures — so it is refused here rather than flattened.
    pub(crate) fn write(&self, path: &Path, format: OutputFormat) -> Result<(), CliError> {
        self.validate_samples(path)?;
        if format == OutputFormat::Hdf5 {
            return crate::hdf5::write_table(path, self, None);
        }
        if format == OutputFormat::Vcd {
            return Err(crate::commands::vcd_io::unsupported_analysis(
                &self.analysis,
            ));
        }

        publish::artifact(path, |writer| self.write_to(writer, path, format))
            .map_err(|error| crate::cli::map_atomic_output_error(path, error))
    }

    /// Serialize this table into an already staged artifact.
    pub(crate) fn write_to(
        &self,
        writer: &mut dyn Write,
        path: &Path,
        format: OutputFormat,
    ) -> Result<(), CliError> {
        self.validate_samples(path)?;
        match format {
            OutputFormat::Raw => self.write_raw(writer, path, true),
            OutputFormat::RawAscii => self.write_raw(writer, path, false),
            OutputFormat::Csv => self.write_delimited(writer, path, ','),
            OutputFormat::Tsv => self.write_delimited(writer, path, '\t'),
            OutputFormat::Json => self.write_json(writer, path),
            OutputFormat::Hdf5 => {
                crate::hdf5::write_hdf5_to_writer(writer, &crate::hdf5::table_data(self, None))
                    .map_err(|error| crate::hdf5::map_output_error(path, error))
            }
            OutputFormat::Vcd => Err(crate::commands::vcd_io::unsupported_analysis(
                &self.analysis,
            )),
        }
    }

    fn validate_samples(&self, path: &Path) -> Result<(), CliError> {
        let expected = self.scale.len();
        let valid = self.scale.iter().all(|v| v.is_finite())
            && self.columns.iter().all(|column| match &column.data {
                ColumnData::Real(values) => {
                    values.len() == expected && values.iter().all(|v| v.is_finite())
                }
                ColumnData::NullableReal(values) => {
                    values.len() == expected && values.iter().flatten().all(|v| v.is_finite())
                }
                ColumnData::NullableComplex(values) => {
                    values.len() == expected
                        && values
                            .iter()
                            .flatten()
                            .all(|v| v.re.is_finite() && v.im.is_finite())
                }
                ColumnData::Complex { real, imag } => {
                    real.len() == expected
                        && imag.len() == expected
                        && real.iter().chain(imag).all(|v| v.is_finite())
                }
            });
        if valid {
            Ok(())
        } else {
            Err(CliError::output_error(
                path,
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "table columns must match the coordinate length and contain only finite present values",
                ),
            ))
        }
    }

    /// Value of `column` at row `row` as (real, imag).
    fn value_at(data: &ColumnData, row: usize) -> (f64, f64) {
        match data {
            ColumnData::NullableReal(values) => {
                (values.get(row).copied().flatten().unwrap_or(0.0), 0.0)
            }
            ColumnData::NullableComplex(values) => values
                .get(row)
                .copied()
                .flatten()
                .map_or((0.0, 0.0), |v| (v.re, v.im)),
            ColumnData::Real(values) => (values.get(row).copied().unwrap_or(0.0), 0.0),
            ColumnData::Complex { real, imag } => (
                real.get(row).copied().unwrap_or(0.0),
                imag.get(row).copied().unwrap_or(0.0),
            ),
        }
    }

    fn write_raw<W: Write + ?Sized>(
        &self,
        writer: &mut W,
        path: &Path,
        binary: bool,
    ) -> Result<(), CliError> {
        let encoded = self.dense_encoding();
        if let std::borrow::Cow::Owned(table) = encoded {
            return table.write_raw(writer, path, binary);
        }
        let complex = self.is_complex();
        let io_err = |e: std::io::Error| CliError::output_error(path, e);

        let escaped_plot = self.plot_name.is_empty()
            || self.plot_name.trim() != self.plot_name
            || self.plot_name.chars().any(char::is_control);
        let plot_name = if escaped_plot {
            raw_token(&self.plot_name)
        } else {
            std::borrow::Cow::Borrowed(self.plot_name.as_str())
        };
        let variables = || {
            std::iter::once((self.scale_name.as_str(), self.scale_type.as_str())).chain(
                self.columns
                    .iter()
                    .map(|column| (column.name.as_str(), column.var_type.as_str())),
            )
        };
        let text_metadata = escaped_plot
            || variables()
                .any(|(name, var_type)| needs_raw_escape(name) || needs_raw_escape(var_type));

        writeln!(writer, "Title: {plot_name}").map_err(io_err)?;
        writeln!(writer, "Date: Generated by RSpice").map_err(io_err)?;
        writeln!(writer, "Plotname: {plot_name}").map_err(io_err)?;
        let units: Vec<_> = std::iter::once(self.scale_unit.clone())
            .chain(self.columns.iter().map(|column| column.unit.clone()))
            .collect();
        let real_variables = if complex {
            std::iter::once(0)
                .chain(
                    self.columns
                        .iter()
                        .enumerate()
                        .filter_map(|(index, column)| {
                            matches!(column.data, ColumnData::Real(_)).then_some(index + 1)
                        }),
                )
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let labels = text_metadata.then(|| variables().collect::<Vec<_>>());
        rspice_core::io::ltspice_raw::write_raw_table_layout_metadata(
            writer,
            &real_variables,
            &units,
            labels
                .as_ref()
                .map(|variables| (self.plot_name.as_str(), variables.as_slice())),
        )
        .map_err(io_err)?;
        writeln!(
            writer,
            "Flags: {}",
            if complex { "complex" } else { "real" }
        )
        .map_err(io_err)?;
        writeln!(writer, "No. Variables: {}", self.columns.len() + 1).map_err(io_err)?;
        writeln!(writer, "No. Points: {}", self.scale.len()).map_err(io_err)?;
        writeln!(writer, "Variables:").map_err(io_err)?;
        for (index, (name, var_type)) in variables().enumerate() {
            writeln!(
                writer,
                "\t{}\t{}\t{}",
                index,
                raw_token(name),
                raw_token(var_type)
            )
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
                        write!(writer, "\t{:.17e},{:.17e}", re, im).map_err(io_err)
                    } else {
                        write!(writer, "\t{:.17e}", re).map_err(io_err)
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

    fn write_delimited<W: Write + ?Sized>(
        &self,
        writer: &mut W,
        path: &Path,
        delimiter: char,
    ) -> Result<(), CliError> {
        let io_err = |e: std::io::Error| CliError::output_error(path, e);

        write!(writer, "{}", delimited_cell(&self.scale_name, delimiter)).map_err(io_err)?;
        for column in &self.columns {
            match column.data {
                ColumnData::NullableReal(_) | ColumnData::Real(_) => {
                    write!(
                        writer,
                        "{}{}",
                        delimiter,
                        delimited_cell(&column.name, delimiter)
                    )
                    .map_err(io_err)?;
                }
                ColumnData::Complex { .. } | ColumnData::NullableComplex(_) => {
                    write!(
                        writer,
                        "{}{}{}{}",
                        delimiter,
                        delimited_cell(&format!("Re({})", column.name), delimiter),
                        delimiter,
                        delimited_cell(&format!("Im({})", column.name), delimiter)
                    )
                    .map_err(io_err)?;
                }
            }
        }
        writeln!(writer).map_err(io_err)?;

        for (row, scale_value) in self.scale.iter().enumerate() {
            write!(writer, "{:.17e}", scale_value).map_err(io_err)?;
            for column in &self.columns {
                let (re, im) = Self::value_at(&column.data, row);
                match column.data {
                    ColumnData::NullableReal(ref values) => {
                        write!(writer, "{delimiter}").map_err(io_err)?;
                        if let Some(value) = values[row] {
                            write!(writer, "{value:.17e}").map_err(io_err)?;
                        }
                    }
                    ColumnData::Real(_) => {
                        write!(writer, "{}{:.17e}", delimiter, re).map_err(io_err)?;
                    }
                    ColumnData::NullableComplex(ref values) if values[row].is_none() => {
                        write!(writer, "{delimiter}{delimiter}").map_err(io_err)?;
                    }
                    ColumnData::Complex { .. } | ColumnData::NullableComplex(_) => {
                        write!(writer, "{0}{1:.17e}{0}{2:.17e}", delimiter, re, im)
                            .map_err(io_err)?;
                    }
                }
            }
            writeln!(writer).map_err(io_err)?;
        }

        // Adjacent real signals can legitimately be named Re(x) and Im(x).
        // Declare their representation only when legacy inference would merge
        // them, keeping ordinary numeric CSV/TSV exports unchanged.
        let needs_layout = self.columns.windows(2).any(|pair| {
            pair.iter().all(|column| {
                matches!(
                    column.data,
                    ColumnData::Real(_) | ColumnData::NullableReal(_)
                )
            }) && complex_pair_name(&pair[0].name, &pair[1].name).is_some()
        });
        if needs_layout {
            write!(writer, "{RECORD_MARKER}").map_err(io_err)?;
            for column in &self.columns {
                match column.data {
                    ColumnData::Real(_) | ColumnData::NullableReal(_) => {
                        write!(writer, "{delimiter}{}", ColumnKind::Real.as_str())
                            .map_err(io_err)?;
                    }
                    ColumnData::Complex { .. } | ColumnData::NullableComplex(_) => {
                        write!(
                            writer,
                            "{delimiter}{}{delimiter}{}",
                            ColumnKind::ComplexReal.as_str(),
                            ColumnKind::ComplexImag.as_str()
                        )
                        .map_err(io_err)?;
                    }
                }
            }
            writeln!(writer).map_err(io_err)?;
        }

        Ok(())
    }

    fn write_json<W: Write + ?Sized>(&self, writer: &mut W, path: &Path) -> Result<(), CliError> {
        let signals: Vec<serde_json::Value> = self
            .columns
            .iter()
            .map(|column| {
                let mut value = match &column.data {
                    ColumnData::NullableReal(values) => serde_json::json!({
                        "name": column.name, "type": column.var_type, "values": values,
                    }),
                    ColumnData::NullableComplex(values) => serde_json::json!({
                        "name": column.name, "type": column.var_type,
                        "real": values.iter().map(|value| value.map(|value| value.re)).collect::<Vec<_>>(),
                        "imag": values.iter().map(|value| value.map(|value| value.im)).collect::<Vec<_>>(),
                    }),
                    ColumnData::Real(values) => serde_json::json!({
                        "name": column.name,
                        "type": column.var_type,
                        "values": values,
                    }),
                    ColumnData::Complex { real, imag } => serde_json::json!({
                        "name": column.name,
                        "type": column.var_type,
                        "real": real,
                        "imag": imag,
                    }),
                };
                if let Some(unit) = &column.unit {
                    value["unit"] = serde_json::Value::String(unit.clone());
                }
                value
            })
            .collect();

        let mut json = serde_json::json!({
            "analysis": self.analysis,
            "plot_name": self.plot_name,
            "scale": {
                "name": self.scale_name,
                "type": self.scale_type,
                "values": self.scale,
            },
            "signals": signals,
        });
        if let Some(unit) = &self.scale_unit {
            json["scale"]["unit"] = serde_json::Value::String(unit.clone());
        }

        serde_json::to_writer_pretty(&mut *writer, &json)
            .map_err(|e| CliError::output_json_error(path, e))?;
        writer
            .write_all(b"\n")
            .map_err(|e| CliError::output_error(path, e))?;
        Ok(())
    }
}

fn needs_raw_escape(value: &str) -> bool {
    value.is_empty()
        || value
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control() || ch == '%')
}

/// RAW declarations have no quoting convention. Escape unsafe UTF-8 bytes and
/// the escape introducer so distinct labels stay distinct even in readers that
/// ignore the exact text metadata. A lone '%' represents the empty label.
fn raw_token(value: &str) -> std::borrow::Cow<'_, str> {
    if !needs_raw_escape(value) {
        return std::borrow::Cow::Borrowed(value);
    }
    use std::fmt::Write as _;
    let mut token = String::new();
    for ch in value.chars() {
        if ch.is_whitespace() || ch.is_control() || ch == '%' {
            for byte in ch.encode_utf8(&mut [0; 4]).bytes() {
                write!(token, "%{byte:02X}").expect("writing to String cannot fail");
            }
        } else {
            token.push(ch);
        }
    }
    if token.is_empty() {
        token.push('%');
    }
    std::borrow::Cow::Owned(token)
}

pub(crate) fn delimited_cell(value: &str, delimiter: char) -> String {
    // An empty single-column header must remain a record. Quote authored
    // leading U+FEFF too so it cannot become a file-level signature.
    if value.is_empty()
        || value.starts_with('\u{feff}')
        || value.contains(delimiter)
        || value.contains('"')
        || value.contains('\n')
        || value.contains('\r')
        || value.trim() != value
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_row_table() -> ExportTable {
        ExportTable {
            scale_unit: None,
            analysis: "tran".to_string(),
            plot_name: "Transient Analysis".to_string(),
            scale_name: "time".to_string(),
            scale_type: "time".to_string(),
            scale: vec![0.0],
            columns: vec![ExportColumn {
                unit: None,
                name: "V(out)".to_string(),
                var_type: "voltage".to_string(),
                data: ColumnData::Real(vec![1.25]),
            }],
        }
    }

    #[test]
    fn csv_publication_replaces_only_with_the_complete_format() {
        for preexisting in [false, true] {
            let directory = std::env::temp_dir().join(format!(
                "rspice-export-table-csv-{}-{preexisting}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&directory);
            std::fs::create_dir(&directory).expect("create CSV export test directory");
            let destination = directory.join("result.csv");
            if preexisting {
                std::fs::write(&destination, b"old complete artifact")
                    .expect("seed CSV destination");
            }

            one_row_table()
                .write(&destination, OutputFormat::Csv)
                .expect("publish CSV table");

            let expected = format!("time,V(out)\n{:.17e},{:.17e}\n", 0.0_f64, 1.25_f64);
            assert_eq!(
                std::fs::read(&destination).expect("read published CSV"),
                expected.as_bytes()
            );
            assert!(
                rspice_output::stale_artifacts(&destination)
                    .expect("inspect CSV staging artifacts")
                    .is_empty()
            );
            std::fs::remove_dir_all(directory).expect("remove CSV export test directory");
        }
    }

    #[test]
    fn invalid_hdf5_table_rejection_does_not_touch_existing_destination() {
        let directory = std::env::temp_dir().join(format!(
            "rspice-export-table-hdf5-rejection-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create export table test directory");
        let destination = directory.join("result.h5");
        std::fs::write(&destination, b"old complete artifact")
            .expect("seed export table destination");
        let table = ExportTable {
            scale_unit: None,
            analysis: "test".to_string(),
            plot_name: "test".to_string(),
            scale_name: "time".to_string(),
            scale_type: "time".to_string(),
            scale: vec![f64::NAN],
            columns: Vec::new(),
        };

        let error = table
            .write(&destination, OutputFormat::Hdf5)
            .expect_err("non-finite HDF5 data must be rejected");
        assert!(error.to_string().contains("finite"), "{error}");
        assert_eq!(
            std::fs::read(&destination).expect("read preserved destination"),
            b"old complete artifact"
        );

        let _ = std::fs::remove_dir_all(directory);
    }
}
