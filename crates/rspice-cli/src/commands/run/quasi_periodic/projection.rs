//! Apply the shared authored-output contract without losing tone coordinates.
use super::*;
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use rspice_core::execution::result_document::{AxisValues, SeriesValues};
use rspice_core::execution::{
    ProjectionSource, ProjectionSourceSignal, ProjectionValues, SignalDescriptor, SignalSchema,
    SignalUnit,
};

fn variable_type(descriptor: &SignalDescriptor) -> &'static str {
    match descriptor.unit() {
        SignalUnit::Volt => "voltage",
        SignalUnit::Ampere => "current",
        SignalUnit::Hertz => "frequency",
        SignalUnit::Second => "time",
        _ => rspice_core::execution::raw_variable_type(descriptor.kind()),
    }
}

pub(super) fn project(
    ctx: &RunContext<'_>,
    document: &AnalysisResultDocument,
) -> Result<(SignalSchema, Option<ExportTable>), CliError> {
    let tag = document.analysis().tag();
    let axis = document
        .axes()
        .first()
        .ok_or_else(|| CliError::InternalError {
            message: format!("{tag} has no primary axis"),
        })?;
    let scale: Vec<f64> = match axis.values() {
        AxisValues::Real { values } => values.clone(),
        AxisValues::Integer { values } => values.iter().map(|value| *value as f64).collect(),
    };
    let mut sources = Vec::with_capacity(document.signals().len());
    for signal in document.signals() {
        let descriptor = signal.descriptor();
        let (values, validity) = match signal.values() {
            SeriesValues::Real { samples } => (
                ProjectionValues::Real(
                    samples
                        .iter()
                        .map(|value| value.unwrap_or(0.0))
                        .collect::<Vec<_>>()
                        .into(),
                ),
                samples.iter().map(Option::is_some).collect::<Vec<_>>(),
            ),
            SeriesValues::Complex { samples } => (
                ProjectionValues::Complex {
                    real: samples
                        .iter()
                        .map(|value| value.map_or(0.0, |value| value.real))
                        .collect::<Vec<_>>()
                        .into(),
                    imag: samples
                        .iter()
                        .map(|value| value.map_or(0.0, |value| value.imaginary))
                        .collect::<Vec<_>>()
                        .into(),
                },
                samples.iter().map(Option::is_some).collect(),
            ),
            _ => {
                return Err(CliError::InternalError {
                    message: format!("{tag} primary series must be real or complex"),
                });
            }
        };
        let registry = match descriptor.owner() {
            rspice_core::execution::SignalOwner::Node(name)
            | rspice_core::execution::SignalOwner::Branch(name) => name.as_str(),
            _ => descriptor.canonical_name(),
        };
        sources.push(
            ProjectionSourceSignal::new(
                descriptor.display_name(),
                registry,
                descriptor.kind(),
                values,
            )
            .map_err(|error| CliError::InternalError {
                message: error.to_string(),
            })?
            .with_validity(validity),
        );
    }
    let source = ProjectionSource::new(document.result_kind(), &tag)
        .with_axis(scale.as_slice())
        .with_signals(sources);
    let contract = crate::commands::run_signals::projection(ctx.netlist)
        .map_err(|error| engine_error(ctx, &tag, error))?;
    let projected = contract
        .project(&ctx.netlist.params, &source, &crate::abort::ProcessAbort)
        .map_err(|error| engine_error(ctx, &tag, error))?;
    let projected = projected.into_signals();
    let mut columns = Vec::with_capacity(projected.len());
    let mut descriptors = Vec::with_capacity(projected.len());
    for signal in projected {
        let descriptor = document
            .signals()
            .iter()
            .find(|original| {
                original.descriptor().canonical_name() == signal.descriptor().canonical_name()
            })
            .map_or_else(
                || signal.descriptor().clone(),
                |original| original.descriptor().clone(),
            );
        if ctx.format != OutputFormat::Json && signal.validity().iter().any(|present| !present) {
            return Err(CliError::ConversionError {
                message: format!(
                    "{tag} signal '{}' contains undefined samples; JSON retains their explicit availability and evidence",
                    descriptor.display_name()
                ),
            });
        }
        let data = match signal.values() {
            ProjectionValues::Real(values) => ColumnData::Real(values.to_vec()),
            ProjectionValues::Complex { real, imag } => ColumnData::Complex {
                real: real.to_vec(),
                imag: imag.to_vec(),
            },
        };
        columns.push(ExportColumn {
            unit: None,
            name: signal.descriptor().display_name().to_owned(),
            var_type: variable_type(&descriptor).into(),
            data,
        });
        descriptors.push(Ok(descriptor));
    }
    if document.result_kind() == rspice_core::execution::AnalysisResultKind::Qpss {
        // A physical frequency is not a replacement for an independent tone
        // tuple. These coordinate columns remain even when SAVE selects nodes.
        for signal in document.signals().iter().filter(|signal| {
            signal.descriptor().canonical_name() == "frequency"
                || signal
                    .descriptor()
                    .canonical_name()
                    .starts_with("tone_index(")
        }) {
            let descriptor = signal.descriptor();
            if columns
                .iter()
                .any(|column| column.name.eq_ignore_ascii_case(descriptor.display_name()))
            {
                continue;
            }
            let SeriesValues::Real { samples } = signal.values() else {
                unreachable!("validated QPSS coordinates")
            };
            columns.push(ExportColumn {
                unit: None,
                name: descriptor.display_name().into(),
                var_type: variable_type(descriptor).into(),
                data: ColumnData::Real(
                    samples
                        .iter()
                        .map(|value| value.expect("validated QPSS coordinate sample"))
                        .collect(),
                ),
            });
            descriptors.push(Ok(descriptor.clone()));
        }
    }
    let schema = super::super::document::distinct_schema(descriptors)?;
    let table = (ctx.format != OutputFormat::Json).then(|| ExportTable {
        scale_unit: None,
        analysis: document.result_kind().tag().into(),
        plot_name: format!("Quasiperiodic {}", document.result_kind().tag()),
        scale_name: axis.name().into(),
        scale_type: if axis.unit() == &SignalUnit::Hertz {
            "frequency"
        } else {
            "index"
        }
        .into(),
        scale,
        columns,
    });
    Ok((schema, table))
}
