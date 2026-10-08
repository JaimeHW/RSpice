//! Flatten typed result documents without losing response identity or RF context.

use super::{conversion_error, enforce_table_value_limits};
use crate::cli::CliError;
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::commands::result_signal::qualified_name;
use std::path::Path;

/// Project signed or unsigned 64-bit integers only when binary64 is exact.
/// A wider round trip avoids saturating i64::MAX/u64::MAX casts, and permits
/// representable multiples above 2^53 instead of rejecting all large integers.
fn exact_integer_sample(value: impl Into<i128>) -> Option<f64> {
    let integer = value.into();
    let sample = integer as f64;
    (sample as i128 == integer).then_some(sample)
}

/// Append numeric payloads whose coordinates fit this table. Allocation is
/// admitted before constants and sparse matrix entries expand to full columns.
fn append_payload_columns(
    path: &Path,
    document: &rspice_core::execution::AnalysisResultDocument,
    scale: &[f64],
    columns: &mut Vec<ExportColumn>,
    limits: rspice_core::ResourceLimits,
    width: usize,
) -> Result<(), CliError> {
    use rspice_core::execution::SignalUnit;
    use rspice_core::execution::result_document::ResultPayload;
    let mut projection = PayloadProjection {
        path,
        points: scale.len(),
        columns,
        limits,
        width,
    };
    match document.payload() {
        ResultPayload::Sp(payload) => {
            for port in &payload.ports {
                projection.constant(
                    format!("Z0({})", port.number),
                    SignalUnit::Ohm,
                    port.reference_impedance,
                )?;
            }
        }
        ResultPayload::Pac(payload) => {
            projection.constant(
                "fundamental_frequency".into(),
                SignalUnit::Hertz,
                payload.fundamental_frequency,
            )?;
            for band in &payload.sidebands {
                if band.frequency_offsets != scale {
                    return Err(conversion_error(
                        path,
                        format!(
                            "PAC sideband {} offsets do not match the table coordinate",
                            band.sideband
                        ),
                    ));
                }
                projection.real(
                    format!("frequency(sb{})", band.sideband),
                    SignalUnit::Hertz,
                    &band.absolute_frequencies,
                )?;
            }
            if let Some(matrix) = &payload.conversion_matrix {
                projection.conversion_matrix(matrix)?;
            }
        }
        ResultPayload::Distortion(payload) => {
            if let Some(ratio) = payload.f2_over_f1 {
                projection.constant("f2_over_f1".into(), SignalUnit::Dimensionless, ratio)?;
                // F2 stays fixed at its ratio to the first swept F1 point.
                let first = scale
                    .first()
                    .copied()
                    .ok_or_else(|| conversion_error(path, "distortion has no F1 coordinate"))?;
                projection.constant("frequency(f2)".into(), SignalUnit::Hertz, first * ratio)?;
            }
            for product in &payload.products {
                projection.real(
                    format!("frequency({})", product.product.label()),
                    SignalUnit::Hertz,
                    &product.frequencies,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

struct PayloadProjection<'a> {
    path: &'a Path,
    points: usize,
    columns: &'a mut Vec<ExportColumn>,
    limits: rspice_core::ResourceLimits,
    width: usize,
}

impl PayloadProjection<'_> {
    fn push(
        &mut self,
        name: String,
        unit: rspice_core::execution::SignalUnit,
        components: usize,
        data: impl FnOnce() -> ColumnData,
    ) -> Result<(), CliError> {
        self.width = self.width.saturating_add(components);
        enforce_table_value_limits(
            self.path,
            self.points.saturating_mul(self.width),
            self.limits,
        )?;
        self.columns.push(ExportColumn {
            name,
            var_type: crate::commands::export_table::unit_type(&unit, "parameter").to_string(),
            unit: crate::commands::export_table::stated_unit(&unit),
            data: data(),
        });
        Ok(())
    }

    fn constant(
        &mut self,
        name: String,
        unit: rspice_core::execution::SignalUnit,
        value: f64,
    ) -> Result<(), CliError> {
        if !value.is_finite() {
            return Err(conversion_error(
                self.path,
                format!("payload quantity '{name}' is not finite"),
            ));
        }
        let points = self.points;
        self.push(name, unit, 1, || ColumnData::Real(vec![value; points]))
    }

    fn real(
        &mut self,
        name: String,
        unit: rspice_core::execution::SignalUnit,
        values: &[f64],
    ) -> Result<(), CliError> {
        if values.len() != self.points {
            return Err(conversion_error(
                self.path,
                format!(
                    "payload quantity '{name}' has {} points; expected {}",
                    values.len(),
                    self.points
                ),
            ));
        }
        self.push(name, unit, 1, || ColumnData::Real(values.to_vec()))
    }

    fn conversion_matrix(
        &mut self,
        matrix: &rspice_core::execution::result_document::PacConversionMatrixDocument,
    ) -> Result<(), CliError> {
        // Borrow entries while grouping. No dense response vectors exist until
        // their entire width passes admission, and duplicate samples never win
        // by insertion order.
        let mut paths = std::collections::BTreeMap::<_, std::collections::BTreeMap<_, _>>::new();
        for entry in &matrix.entries {
            if entry.frequency_index >= self.points {
                return Err(conversion_error(
                    self.path,
                    format!(
                        "PAC conversion matrix frequency index {} exceeds {} points",
                        entry.frequency_index, self.points
                    ),
                ));
            }
            let path = paths
                .entry((entry.input_sideband, entry.output_sideband))
                .or_default();
            if path.insert(entry.frequency_index, entry).is_some() {
                return Err(conversion_error(
                    self.path,
                    "PAC conversion matrix repeats a path at one frequency",
                ));
            }
        }
        for ((input, output), entries) in paths {
            let points = self.points;
            self.push(
                format!("conversion(sb{input}->sb{output})"),
                // The payload does not identify the input source's physical
                // kind. A voltage ratio and a transimpedance have different units.
                rspice_core::execution::SignalUnit::Unspecified,
                2,
                || {
                    let mut samples = vec![None; points];
                    for (index, entry) in entries {
                        samples[index] = Some(rspice_core::Complex64::new(
                            entry.value.real,
                            entry.value.imaginary,
                        ));
                    }
                    ColumnData::optional_complex(samples)
                },
            )?;
        }
        Ok(())
    }
}

/// Flatten one typed result document into the shared tabular model.
///
/// A document whose family carries no coordinate axis — the operating point,
/// the transfer function — flattens onto a single-point index axis, which is
/// what the flat writers already use for those families.
pub(in crate::commands) fn result_document_table(
    path: &Path,
    document: &rspice_core::execution::AnalysisResultDocument,
    resource_limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    use rspice_core::execution::result_document::{
        AxisValues, ResultPayload, ScalarValue, SeriesValues,
    };

    if document.axes().len() > 1 && document.frequency_table().is_none() {
        return Err(conversion_error(
            path,
            "a multi-axis result cannot be represented by a single flat table",
        ));
    }

    let (scale_name, scale): (String, Vec<f64>) = match document.axes().first() {
        Some(axis) => (
            axis.name().to_string(),
            match axis.values() {
                AxisValues::Real { values } => values.clone(),
                AxisValues::Integer { values } => {
                    values.iter().map(|value| {
                        exact_integer_sample(*value).ok_or_else(|| conversion_error(
                            path,
                            format!("axis '{}' coordinate {value} cannot be represented exactly by a numeric flat table", axis.name()),
                        ))
                    }).collect::<Result<Vec<_>, _>>()?
                }
            },
        ),
        None => ("point".to_string(), vec![0.0]),
    };
    // Admit both components of complex quantities before allocating copies.
    // Payload columns are admitted separately before each expansion below.
    let width = document
        .signals()
        .iter()
        .fold(document.axes().len().max(1), |width, signal| {
            width.saturating_add(if matches!(signal.values(), SeriesValues::Complex { .. }) {
                2
            } else {
                1
            })
        });
    let width = document.scalars().iter().fold(width, |width, scalar| {
        width.saturating_add(if matches!(scalar.value(), ScalarValue::Complex { .. }) {
            2
        } else {
            1
        })
    });
    enforce_table_value_limits(path, scale.len().saturating_mul(width), resource_limits)?;

    let mut columns = Vec::new();
    if document.frequency_table().is_some() {
        for axis in document.axes().iter().skip(1) {
            let AxisValues::Real { values } = axis.values() else {
                return Err(conversion_error(
                    path,
                    "table bindings require real coordinates",
                ));
            };
            columns.push(ExportColumn {
                name: axis.name().into(),
                var_type: "parameter".into(),
                unit: crate::commands::export_table::stated_unit(axis.unit()),
                data: ColumnData::Real(values.clone()),
            });
        }
    }
    for signal in document.signals() {
        let name = qualified_name(signal);
        let data = match signal.values() {
            SeriesValues::Real { samples } => ColumnData::optional_real(samples.clone()),
            SeriesValues::Complex { samples } => ColumnData::optional_complex(
                samples
                    .iter()
                    .map(|sample| {
                        sample.map(|value| rspice_core::Complex64::new(value.real, value.imaginary))
                    })
                    .collect(),
            ),
            other => {
                return Err(conversion_error(
                    path,
                    format!(
                        "series '{name}' has a representation no flat table carries: {other:?}"
                    ),
                ));
            }
        };
        columns.push(ExportColumn {
            unit: crate::commands::export_table::stated_unit(signal.descriptor().unit()),
            var_type: crate::commands::export_table::unit_type(
                signal.descriptor().unit(),
                rspice_core::execution::raw_variable_type(signal.descriptor().kind()),
            )
            .to_string(),
            name,
            data,
        });
    }

    for scalar in document.scalars() {
        let unsupported = || {
            conversion_error(
                path,
                format!(
                    "scalar '{}' cannot be represented exactly by a numeric flat table",
                    scalar.name()
                ),
            )
        };
        let data = match scalar.value() {
            ScalarValue::Real { value: Some(value) } => ColumnData::Real(vec![*value; scale.len()]),
            ScalarValue::Complex { value: Some(value) } => ColumnData::Complex {
                real: vec![value.real; scale.len()],
                imag: vec![value.imaginary; scale.len()],
            },
            ScalarValue::Integer { value } => ColumnData::Real(vec![
                exact_integer_sample(*value)
                    .ok_or_else(unsupported)?;
                scale.len()
            ]),
            ScalarValue::Count { value } => ColumnData::Real(vec![
                exact_integer_sample(*value)
                    .ok_or_else(unsupported)?;
                scale.len()
            ]),
            ScalarValue::Boolean { value } => {
                ColumnData::Real(vec![u8::from(*value) as f64; scale.len()])
            }
            _ => return Err(unsupported()),
        };
        // Keep the established .TF table names so formats emitted by the same
        // analysis also compare against one another.
        let name = match document.payload() {
            ResultPayload::Tf(payload) => match scalar.name() {
                "transfer_gain" => "transfer_function".to_string(),
                "input_impedance" => format!("{}#input_impedance", payload.input.to_lowercase()),
                "output_impedance" => {
                    format!("output_impedance_at_{}", payload.output.to_lowercase())
                }
                _ => scalar.name().to_string(),
            },
            _ => scalar.name().to_string(),
        };
        columns.push(ExportColumn {
            unit: scalar
                .unit()
                .and_then(crate::commands::export_table::stated_unit),
            name,
            var_type: scalar
                .unit()
                .map_or("value", |unit| {
                    crate::commands::export_table::unit_type(unit, "value")
                })
                .to_string(),
            data,
        });
    }
    append_payload_columns(path, document, &scale, &mut columns, resource_limits, width)?;
    if columns.is_empty() {
        return Err(conversion_error(
            path,
            "result contains no retained numeric quantities that a flat table can represent",
        ));
    }

    // Literal names can collide with qualified responses or scalars. Refuse
    // an ambiguous projection before replacing a destination artifact.
    let mut names = std::collections::HashSet::new();
    names.insert(scale_name.to_ascii_lowercase());
    for column in &columns {
        if !names.insert(column.name.to_ascii_lowercase()) {
            return Err(conversion_error(
                path,
                format!(
                    "typed quantities project to duplicate column '{}'",
                    column.name
                ),
            ));
        }
    }

    Ok(ExportTable {
        scale_unit: document
            .axes()
            .first()
            .and_then(|axis| crate::commands::export_table::stated_unit(axis.unit())),
        analysis: document.result_kind().tag().to_string(),
        plot_name: format!("{} ({})", document.result_kind().tag(), document.analysis()),
        scale_type: document
            .axes()
            .first()
            .map_or("index", |axis| {
                crate::commands::export_table::unit_type(axis.unit(), "parameter")
            })
            .to_string(),
        scale_name,
        scale,
        columns,
    })
}
