//! Publish payload-based reports through the same projection as conversion.

use super::{
    RunContext, document,
    export::{ColumnData, ExportTable},
};
use crate::cli::{CliError, OutputFormat};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisResultDocumentBuilder, ResultDocumentError, SignalDescriptor,
    SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
};
use std::path::Path;

pub(super) fn publish(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis_id: AnalysisInstanceId,
    analysis: &str,
    title: &str,
    build: impl Fn() -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
) -> Result<(), CliError> {
    let mut table = if ctx.format == OutputFormat::Json {
        // Typed publication invokes the builder below and never serializes this
        // placeholder or pays for a dense tabular payload expansion.
        ExportTable {
            analysis: analysis.into(),
            plot_name: title.into(),
            scale_unit: None,
            scale_name: "point".into(),
            scale_type: "index".into(),
            scale: vec![0.0],
            columns: Vec::new(),
        }
    } else {
        let builder = build().map_err(|error| document::document_error(ctx, analysis_id, error))?;
        let built = document::finish(ctx, analysis_id, builder)?;
        crate::commands::waveform_io::result_document_table(
            path,
            &built,
            ctx.engine.config().resource_limits,
        )?
    };
    table.analysis = analysis.into();
    table.plot_name = title.into();
    let schema = document::distinct_schema(table.columns.iter().map(|column| {
        SignalDescriptor::new(
            &column.name,
            &column.name,
            SignalKind::Scalar,
            match column.unit.as_deref() {
                Some("V") => SignalUnit::Volt,
                Some("A") => SignalUnit::Ampere,
                Some("rad/s") => SignalUnit::RadianPerSecond,
                Some("ohm") => SignalUnit::Ohm,
                Some("1") => SignalUnit::Dimensionless,
                Some(unit) => SignalUnit::Custom(unit.into()),
                None => SignalUnit::Unspecified,
            },
            if matches!(
                column.data,
                ColumnData::Complex { .. } | ColumnData::NullableComplex(_)
            ) {
                SignalValueType::Complex
            } else {
                SignalValueType::Real
            },
            SignalShape::Scalar,
            SignalOwner::Analysis,
        )
    }))?;
    document::publish_table_result(ctx, path, analysis_id, schema, &table, build)
}
