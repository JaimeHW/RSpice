//! Execute independent-tone analyses against the carrier selected by the plan.
use super::context::PeriodicArtifact;
use super::{PublishedResult, RunContext};
use crate::cli::{CliError, OutputFormat};
use rspice_core::engine::{QpssConfig, QpssOperatingPoint};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisResultDocument, AnalysisResultDocumentBuilder, ResultDocumentError,
};
use rspice_core::netlist::{QpacCard, QpnoiseCard, QpssCard, QpxfCard};

mod projection;

fn engine_error(ctx: &RunContext<'_>, tag: &str, source: rspice_core::SimulationError) -> CliError {
    if matches!(source, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        CliError::CoreSimulationError {
            source,
            analysis: Some(tag.to_uppercase()),
        }
    }
}

pub(super) fn run_qpss(ctx: &RunContext<'_>, card: &QpssCard) -> Result<(), CliError> {
    let artifact = ctx.resolve_periodic_analysis("qpss")?;
    let config =
        QpssConfig::from_qpss_card(card).map_err(|error| engine_error(ctx, "qpss", error))?;
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Running {}: {} independent tones",
            artifact.analysis,
            config.grid.frequencies_hz.len()
        ))?;
    }
    let point = ctx
        .engine
        .run_qpss_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|error| engine_error(ctx, "qpss", error))?;
    if !ctx.quiet {
        crate::console::line(format_args!(
            "✓ QPSS converged in {} iterations (normalized residual {:.3e})",
            point.iterations(),
            point.normalized_residual()
        ))?;
    }
    if artifact.path.is_some() {
        publish(
            ctx,
            &artifact,
            AnalysisResultDocument::from_qpss(
                artifact.analysis,
                &point,
                &ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            ),
        )?;
    }
    ctx.retain_qpss(artifact.analysis, point);
    Ok(())
}

fn dependent<R>(
    ctx: &RunContext<'_>,
    tag: &'static str,
    run: impl FnOnce(&QpssOperatingPoint) -> Result<R, rspice_core::SimulationError>,
    project: impl FnOnce(
        AnalysisInstanceId,
        AnalysisInstanceId,
        &R,
    ) -> Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
) -> Result<(), CliError> {
    let artifact = ctx.resolve_periodic_analysis(tag)?;
    let upstream = ctx.planned_upstream(artifact.analysis, tag)?;
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Running {} around {upstream}",
            artifact.analysis
        ))?;
    }
    let result = {
        let periodic = ctx.periodic();
        let point = periodic.qpss(upstream).ok_or_else(|| {
            CliError::simulation_error_in(
                format!(
                    "{} requires retained {upstream}, but that QPSS operating point is unavailable",
                    artifact.analysis
                ),
                tag,
            )
        })?;
        run(point).map_err(|error| engine_error(ctx, tag, error))?
    };
    if artifact.path.is_some() {
        publish(
            ctx,
            &artifact,
            project(artifact.analysis, upstream, &result),
        )?;
    }
    if !ctx.quiet {
        crate::console::line(format_args!("✓ {} complete", tag.to_uppercase()))?;
    }
    Ok(())
}

pub(super) fn run_qpac(ctx: &RunContext<'_>, card: &QpacCard) -> Result<(), CliError> {
    dependent(
        ctx,
        "qpac",
        |point| {
            ctx.engine.run_qpac_card_from_qpss_with_abort(
                ctx.netlist,
                card,
                point,
                &crate::abort::ProcessAbort,
            )
        },
        |analysis, parent, result| {
            AnalysisResultDocument::from_qpac(
                analysis,
                parent,
                result,
                &ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
        },
    )
}
pub(super) fn run_qpxf(ctx: &RunContext<'_>, card: &QpxfCard) -> Result<(), CliError> {
    dependent(
        ctx,
        "qpxf",
        |point| {
            ctx.engine.run_qpxf_card_from_qpss_with_abort(
                ctx.netlist,
                card,
                point,
                &crate::abort::ProcessAbort,
            )
        },
        |analysis, parent, result| {
            AnalysisResultDocument::from_qpxf(
                analysis,
                parent,
                result,
                &ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
        },
    )
}
pub(super) fn run_qpnoise(ctx: &RunContext<'_>, card: &QpnoiseCard) -> Result<(), CliError> {
    dependent(
        ctx,
        "qpnoise",
        |point| {
            ctx.engine.run_qpnoise_card_from_qpss_with_abort(
                ctx.netlist,
                card,
                point,
                &crate::abort::ProcessAbort,
            )
        },
        |analysis, parent, result| {
            AnalysisResultDocument::from_qpnoise(
                analysis,
                parent,
                result,
                &ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
        },
    )
}

fn publish(
    ctx: &RunContext<'_>,
    artifact: &PeriodicArtifact,
    builder: Result<AnalysisResultDocumentBuilder, ResultDocumentError>,
) -> Result<(), CliError> {
    let Some(path) = &artifact.path else {
        return Ok(());
    };
    let builder =
        builder.map_err(|error| super::document::document_error(ctx, artifact.analysis, error))?;
    let document = super::document::finish(ctx, artifact.analysis, builder)?;
    let (schema, table) = projection::project(ctx, &document)?;
    match ctx.format {
        OutputFormat::Json => super::document::write_document(ctx, path, &document)?,
        OutputFormat::Hdf5 => {
            let table = table.expect("flat quasiperiodic projection");
            crate::hdf5::write_table(
                path,
                &table,
                Some(super::document::hdf5_identity(ctx, artifact.analysis)?),
            )?;
        }
        format => table
            .expect("flat quasiperiodic projection")
            .write(path, format)?,
    }
    ctx.record_published(PublishedResult {
        analysis_id: artifact.analysis.tag(),
        schema,
        artifact: path.clone(),
    });
    ctx.record_output(path.clone());
    Ok(())
}
