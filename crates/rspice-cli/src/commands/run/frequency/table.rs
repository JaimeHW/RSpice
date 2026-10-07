//! Physical DATA coordinates in flat exports share the ordinary signal table.
use super::*;
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use rspice_core::engine::{FrequencyDataColumn, FrequencyDataTarget};
use rspice_core::execution::AnalysisInstanceId;
use std::path::Path;

pub(super) fn write(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis: AnalysisInstanceId,
    format: OutputFormat,
    mut table: ExportTable,
    coordinates: &[FrequencyDataColumn],
) -> Result<(), CliError> {
    let mut columns = Vec::with_capacity(coordinates.len().saturating_add(table.columns.len()));
    for coordinate in coordinates {
        if coordinate.target == FrequencyDataTarget::Frequency {
            continue;
        }
        columns.push(ExportColumn {
            name: format!("data({})", coordinate.name.to_ascii_lowercase()),
            var_type: "parameter".into(),
            unit: crate::commands::export_table::stated_unit(&coordinate.target.unit()),
            data: ColumnData::Real(coordinate.values.clone()),
        });
    }
    columns.append(&mut table.columns);
    table.columns = columns;
    if format == OutputFormat::Hdf5 {
        crate::hdf5::write_table(
            path,
            &table,
            Some(super::super::document::hdf5_identity(ctx, analysis)?),
        )
    } else {
        table.write(path, format)
    }
}
