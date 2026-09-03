//! Publication of one analysis result in the representation its format
//! declares.
//!
//! Two representations, one result:
//!
//! - `json` publishes the shared [`AnalysisResultDocument`]. Its shape, its
//!   per-family payload, and every unit and descriptor in it are decided by
//!   `rspice-core`; this module only supplies the identity the run assigned —
//!   the canonical analysis instance, the coordinate, the topology
//!   fingerprint, and the artifact namespaces — and serializes the result.
//!   The document is the complete typed result: a channel the authored output
//!   projection did not retain keeps its descriptor and declares that it was
//!   not retained, instead of vanishing from the artifact.
//! - `csv`, `tsv`, `raw`, and `ascii` publish the flat, authored tabular
//!   projection, because those formats have no representation for absence,
//!   for a per-family payload, or for a nested identity.
//!
//! Both register the coordinate-local [`SignalSchema`] of the artifact they
//! wrote, so an axis deck can union the schemas of its coordinates and publish
//! a validity manifest naming what each coordinate did and did not carry.

use std::path::Path;

use rspice_core::execution::{
    AnalysisInstanceId, AnalysisResultDocument, AnalysisResultDocumentBuilder, ResultCoordinate,
    ResultDocumentError, ResultNamespaces, SignalSchema,
};

use super::RunContext;
use crate::cli::{CliError, OutputFormat, map_atomic_output_error};
use crate::commands::export_table::ExportTable;
use crate::commands::publish;

/// A result the run published, recorded as it was written.
#[derive(Debug, Clone)]
pub(crate) struct PublishedResult {
    /// Canonical analysis identity the artifact was namespaced by.
    pub(crate) analysis_id: String,
    /// Complete signal schema of the coordinate-local artifact.
    pub(crate) schema: SignalSchema,
    /// Path the artifact was staged at.
    pub(crate) artifact: std::path::PathBuf,
}

/// Publish one analysis result and record its coordinate-local schema.
///
/// `document` is only invoked for a format that publishes a typed document, so
/// a flat run never pays to project a result it will not serialize; `flat` is
/// only invoked for the formats that have no typed representation.
pub(super) fn publish_analysis_result(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis_id: AnalysisInstanceId,
    schema: SignalSchema,
    document: impl FnOnce() -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
    flat: impl FnOnce(&Path, OutputFormat) -> Result<(), CliError>,
) -> Result<(), CliError> {
    match ctx.format {
        OutputFormat::Json => {
            let builder = document().map_err(|error| document_error(ctx, analysis_id, error))?;
            let built = finish(ctx, analysis_id, builder)?;
            write_document(ctx, path, &built)?;
        }
        format => flat(path, format)?,
    }
    ctx.record_published(PublishedResult {
        analysis_id: analysis_id.tag(),
        schema,
        artifact: path.to_path_buf(),
    });
    Ok(())
}

/// Publish one analysis result whose flat rendering is an [`ExportTable`].
pub(super) fn publish_table_result(
    ctx: &RunContext<'_>,
    path: &Path,
    analysis_id: AnalysisInstanceId,
    schema: SignalSchema,
    table: &ExportTable,
    document: impl FnOnce() -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
) -> Result<(), CliError> {
    publish_analysis_result(ctx, path, analysis_id, schema, document, |path, format| {
        table.write(path, format)
    })
}

/// Attach the identity this run assigned and validate the finished document.
///
/// The coordinate, topology fingerprint, and namespaces are the run's own
/// contribution; everything else in the document came from the core builder
/// for that result family.
pub(super) fn finish(
    ctx: &RunContext<'_>,
    analysis_id: AnalysisInstanceId,
    mut builder: AnalysisResultDocumentBuilder,
) -> Result<AnalysisResultDocument, CliError> {
    if let Some(coordinate) = ctx.run_coordinate() {
        builder = builder.coordinate(ResultCoordinate::from_run_coordinate(coordinate));
    }
    if let Some(topology) = ctx.topology_fingerprint()? {
        builder = builder.topology_fingerprint(topology);
    }
    finalize(ctx, analysis_id, builder)
}

/// Attach an identity the caller resolved per coordinate, for an executor that
/// publishes several coordinates through one context.
pub(super) fn finish_at_coordinate(
    ctx: &RunContext<'_>,
    analysis_id: AnalysisInstanceId,
    coordinate: &rspice_core::execution::RunCoordinate,
    topology: rspice_core::execution::TopologyFingerprint,
    builder: AnalysisResultDocumentBuilder,
) -> Result<AnalysisResultDocument, CliError> {
    finalize(
        ctx,
        analysis_id,
        builder
            .coordinate(ResultCoordinate::from_run_coordinate(coordinate))
            .topology_fingerprint(topology),
    )
}

fn finalize(
    ctx: &RunContext<'_>,
    analysis_id: AnalysisInstanceId,
    builder: AnalysisResultDocumentBuilder,
) -> Result<AnalysisResultDocument, CliError> {
    builder
        .namespaces(ResultNamespaces {
            output: analysis_id.tag(),
            checkpoint: analysis_id.tag(),
        })
        .build_with_abort(&crate::abort::ProcessAbort)
        .map_err(|error| document_error(ctx, analysis_id, error))
}

/// Byte budget for one serialized document.
///
/// The engine's own artifact policy bounds it, so a pathological result fails
/// before it fills a disk.
pub(super) fn json_byte_limit(ctx: &RunContext<'_>) -> u64 {
    u64::try_from(ctx.engine.config().resource_limits.max_external_data_bytes).unwrap_or(u64::MAX)
}

/// Serialize one finished document into a staged artifact.
pub(super) fn write_document(
    ctx: &RunContext<'_>,
    path: &Path,
    document: &AnalysisResultDocument,
) -> Result<(), CliError> {
    let json = document
        .to_json_with_abort(&crate::abort::ProcessAbort, json_byte_limit(ctx))
        .map_err(|error| document_error(ctx, document.analysis(), error))?;
    publish::artifact(path, |writer: &mut dyn std::io::Write| {
        writer
            .write_all(json.as_bytes())
            .and_then(|()| writer.write_all(b"\n"))
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

/// The coordinate-local schema of a real-valued flat artifact.
pub(super) fn scalar_schema(
    signals: &[crate::commands::run_signals::ScalarSignal],
) -> Result<SignalSchema, CliError> {
    distinct_schema(
        signals
            .iter()
            .map(crate::commands::run_signals::scalar_descriptor),
    )
}

/// The coordinate-local schema of a complex-valued flat artifact.
pub(super) fn complex_schema(
    signals: &[crate::commands::run_signals::ComplexSignal],
) -> Result<SignalSchema, CliError> {
    distinct_schema(
        signals
            .iter()
            .map(crate::commands::run_signals::complex_descriptor),
    )
}

/// Build the schema of one artifact from its exported columns.
///
/// A schema names the distinct signals a coordinate carries. An authored
/// `.PRINT` card may legitimately repeat a column — `.print dc V(out) V(out)`
/// prints it twice — and the flat artifact reproduces that faithfully, but the
/// schema records the signal once. Repetition is a display contract; the
/// schema is what a union across coordinates is taken over.
pub(super) fn distinct_schema(
    descriptors: impl IntoIterator<
        Item = Result<
            rspice_core::execution::SignalDescriptor,
            rspice_core::execution::SignalSchemaError,
        >,
    >,
) -> Result<SignalSchema, CliError> {
    let mut seen = std::collections::HashSet::new();
    let mut distinct = Vec::new();
    for descriptor in descriptors {
        let descriptor = descriptor.map_err(schema_error)?;
        if seen.insert((descriptor.kind(), descriptor.canonical_name().to_string())) {
            distinct.push(descriptor);
        }
    }
    SignalSchema::new(distinct).map_err(schema_error)
}

/// The schema of a result that exports no per-signal series — a scalar-only
/// family such as `.TF` or pole-zero, whose values live in the document's
/// typed payload rather than in named columns.
pub(super) fn empty_schema() -> SignalSchema {
    SignalSchema::default()
}

fn schema_error(error: rspice_core::execution::SignalSchemaError) -> CliError {
    CliError::CoreSimulationError {
        source: rspice_core::SimulationError::Circuit(format!(
            "published artifact has an invalid signal schema: {error}"
        )),
        analysis: Some("Result schema".to_string()),
    }
}

/// A typed result document could not be built or serialized.
///
/// This is never a reason to publish the untyped table instead: a consumer that
/// asked for the typed representation would silently receive something else.
pub(super) fn document_error(
    ctx: &RunContext<'_>,
    analysis: AnalysisInstanceId,
    error: ResultDocumentError,
) -> CliError {
    if matches!(error, ResultDocumentError::Aborted) {
        return super::cancellation_cli_error(ctx.args.timeout);
    }
    CliError::CoreSimulationError {
        source: rspice_core::SimulationError::Circuit(format!(
            "{} cannot publish a typed result document: {error}",
            analysis.tag()
        )),
        analysis: Some(analysis.tag()),
    }
}
