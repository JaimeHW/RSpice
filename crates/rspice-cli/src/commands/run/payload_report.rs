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
use rspice_core::io::raw_export::RawFormat;
use std::path::Path;

pub(super) fn publish(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis_id: AnalysisInstanceId,
    analysis: &str,
    title: &str,
    build: impl Fn() -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
) -> Result<(), CliError> {
    publish_with_rawfile(ctx, path, analysis_id, analysis, title, build, None)
}

/// A rawfile's own rendering of one result, where it is not the flat table.
pub(super) type Rawfile<'a> = &'a dyn Fn(&mut dyn std::io::Write, RawFormat) -> std::io::Result<()>;

/// [`publish`], with the binary and ASCII rawfiles written by `rawfile`.
pub(super) fn publish_with_rawfile(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis_id: AnalysisInstanceId,
    analysis: &str,
    title: &str,
    build: impl Fn() -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
    rawfile: Option<Rawfile<'_>>,
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
    let raw_format = match ctx.format {
        OutputFormat::Raw => Some(RawFormat::Binary),
        OutputFormat::RawAscii => Some(RawFormat::Ascii),
        _ => None,
    };
    match rawfile.zip(raw_format) {
        Some((rawfile, format)) => {
            document::publish_analysis_result(ctx, path, analysis_id, schema, build, |path, _| {
                crate::commands::publish::artifact(path, |writer| {
                    rawfile(writer, format).map_err(|error| CliError::output_error(path, error))
                })
                .map_err(|error| crate::cli::map_atomic_output_error(path, error))
            })
        }
        None => document::publish_table_result(ctx, path, analysis_id, schema, &table, build),
    }
}
