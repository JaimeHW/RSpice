//! Execution of the periodic large-signal family: `.PSS`, `.PAC`, `.PNOISE`
//! and `.ENVELOPE`.
//!
//! `.PSS` is the carrier: it solves a periodic steady state and retains the
//! exact numerical operating point core exposes beside the result. `.PAC`,
//! `.PNOISE` and `.ENVELOPE` linearize or continue around a carrier, and the
//! canonical plan — not source proximity guessed here — says which instance
//! each one belongs to. That binding is read off the plan and the retained
//! state is consumed directly, so a deck with two carriers cannot attach a
//! dependent card to the wrong one, and the large-signal problem is solved
//! once per carrier rather than once per dependent card.
//!
//! Every family publishes the shared typed result document under `-f json`
//! and the authored flat projection otherwise, exactly as the other families
//! do.

use rspice_core::execution::{AnalysisInstanceId, AnalysisResultDocument, AnalysisResultKind};
use rspice_core::netlist::{EnvelopeCard, PacCard, PeriodicSweep, PnoiseCard, PssCard};

use super::RunContext;
use super::context::PeriodicArtifact;
use crate::cli::{CliError, OutputFormat};
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::commands::run_signals::{ComplexSignal, SignalKind};

/// Run one authored `.PSS` card and retain the carrier it converged on.
pub(super) fn run_pss_card(ctx: &RunContext<'_>, card: &PssCard) -> Result<(), CliError> {
    let config = rspice_core::analysis::PssConfig::from(card);
    let artifact = ctx.resolve_periodic_analysis("pss")?;
    super::advanced::announce_pss(ctx, &config);
    // The operating-point entry point is what retains the shooting state a
    // dependent `.PAC`/`.PNOISE` consumes; the plain PSS entry point returns
    // the result only and would force a second large-signal solve per card.
    let operating_point = ctx
        .engine
        .run_pss_operating_point_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|error| map_periodic_error(ctx, "PSS", error))?;
    ensure_not_cancelled(ctx)?;
    let analysis = operating_point.analysis();
    super::advanced::report_pss(ctx, analysis.iterations, analysis.period, &analysis.result);
    if let Some(path) = &artifact.path {
        super::advanced::export_pss(ctx, artifact.analysis, path, &analysis.result)?;
    }
    ctx.retain_pss(artifact.analysis, operating_point);
    Ok(())
}

/// Run one authored `.PAC` card against the carrier the plan bound it to.
pub(super) fn run_pac_card(ctx: &RunContext<'_>, card: &PacCard) -> Result<(), CliError> {
    let artifact = ctx.resolve_periodic_analysis("pac")?;
    preflight_periodic_sweep(ctx, &card.sweep, "PAC")?;
    let upstream = ctx.planned_upstream(artifact.analysis, "PAC")?;
    let config = rspice_core::analysis::PacConfig::from(card);
    if !ctx.quiet {
        println!(
            "Running PAC analysis around {upstream}: {} points, sidebands {}..={}",
            card.sweep.points, card.sideband_min, card.sideband_max
        );
    }

    let result = {
        let periodic = ctx.periodic();
        if let Some(operating_point) = periodic.pss(upstream) {
            ctx.engine.run_pac_from_pss_with_abort(
                ctx.netlist,
                config,
                operating_point,
                &crate::abort::ProcessAbort,
            )
        } else if let Some((operating_point, _)) = periodic.hb(upstream) {
            ctx.engine.run_pac_from_hb_with_abort(
                ctx.netlist,
                config,
                operating_point,
                &crate::abort::ProcessAbort,
            )
        } else {
            return Err(missing_carrier(".PAC", artifact.analysis, upstream));
        }
    }
    .map_err(|error| map_periodic_error(ctx, "PAC", error))?;
    ensure_not_cancelled(ctx)?;

    if !ctx.quiet {
        println!(
            "✓ PAC converged in {} iterations (residual {:.3e})",
            result.result.iterations, result.result.residual
        );
    }
    let Some(path) = &artifact.path else {
        return Ok(());
    };
    export_pac(ctx, &artifact, path, upstream, &result.result)
}

/// Run one authored `.PNOISE` card against the carrier the plan bound it to.
pub(super) fn run_pnoise_card(ctx: &RunContext<'_>, card: &PnoiseCard) -> Result<(), CliError> {
    let artifact = ctx.resolve_periodic_analysis("pnoise")?;
    preflight_periodic_sweep(ctx, &card.sweep, "PNoise")?;
    let upstream = ctx.planned_upstream(artifact.analysis, "PNoise")?;
    if !ctx.quiet {
        println!(
            "Running PNOISE analysis around {upstream}: {} offsets on {}",
            card.sweep.points, card.output_node
        );
    }

    let result = {
        let periodic = ctx.periodic();
        if let Some(operating_point) = periodic.pss(upstream) {
            ctx.engine.run_pnoise_card_from_pss_with_abort(
                ctx.netlist,
                card,
                operating_point,
                &crate::abort::ProcessAbort,
            )
        } else if let Some((operating_point, _)) = periodic.hb(upstream) {
            ctx.engine.run_pnoise_card_from_hb_with_abort(
                ctx.netlist,
                card,
                operating_point,
                &crate::abort::ProcessAbort,
            )
        } else {
            return Err(missing_carrier(".PNOISE", artifact.analysis, upstream));
        }
    }
    .map_err(|error| map_periodic_error(ctx, "PNoise", error))?;
    ensure_not_cancelled(ctx)?;

    if !ctx.quiet {
        println!(
            "✓ PNOISE complete: {} offsets on {}",
            result.offset_frequencies().len(),
            result.output()
        );
    }
    let Some(path) = &artifact.path else {
        return Ok(());
    };
    export_pnoise(ctx, &artifact, path, upstream, &result)
}

/// Run one authored `.ENVELOPE` card, continuing the carrier the plan bound
/// it to.
///
/// Envelope following continues an exact harmonic-balance carrier, so the
/// configuration retained beside that carrier's operating point is what the
/// continuation uses. A shooting `.PSS` carrier is refused rather than
/// converted into one.
pub(super) fn run_envelope_card(ctx: &RunContext<'_>, card: &EnvelopeCard) -> Result<(), CliError> {
    let artifact = ctx.resolve_periodic_analysis("env")?;
    let upstream = ctx.planned_upstream(artifact.analysis, "Envelope")?;
    if !ctx.quiet {
        println!(
            "Running ENVELOPE continuation of {upstream}: {:.3e} s slow time",
            card.duration
        );
    }

    let config = {
        let periodic = ctx.periodic();
        let Some((_, config)) = periodic.hb(upstream) else {
            return Err(CliError::simulation_error_in(
                format!(
                    ".ENVELOPE continues a harmonic-balance carrier; the canonical plan bound {} to {upstream}, which retained no harmonic-balance operating point",
                    artifact.analysis
                ),
                "Envelope",
            ));
        };
        config.clone()
    };
    let result = ctx
        .engine
        .run_envelope_with_abort(
            ctx.netlist,
            config,
            &card.frozen_sources,
            card.duration,
            card.max_step,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_periodic_error(ctx, "Envelope", error))?;
    ensure_not_cancelled(ctx)?;

    if !ctx.quiet {
        println!(
            "✓ ENVELOPE complete: {} slow-time points",
            result.continued_transient().time.len()
        );
    }
    let Some(path) = &artifact.path else {
        return Ok(());
    };
    export_envelope(ctx, &artifact, path, upstream, &result)
}

//=============================================================================
// Publication
//=============================================================================

/// Publish one periodic AC sweep: one complex column per node and branch, per
/// sideband, over the offset-frequency axis.
fn export_pac(
    ctx: &RunContext<'_>,
    artifact: &PeriodicArtifact,
    path: &std::path::Path,
    upstream: AnalysisInstanceId,
    result: &rspice_core::analysis::PacResult,
) -> Result<(), CliError> {
    let analysis_id = artifact.analysis;
    let frequencies = result.frequencies.clone();
    let sidebands = pac_projected_sidebands(ctx, result)?;
    let table = pac_export_table(&frequencies, &sidebands);
    let schema = super::document::distinct_schema(table.columns.iter().map(|column| {
        rspice_core::execution::signal_descriptor(
            &column.name,
            &column.name,
            rspice_core::execution::SignalKind::Scalar,
            rspice_core::execution::SignalValueType::Complex,
        )
    }))?;

    super::document::publish_analysis_result(
        ctx,
        path,
        analysis_id,
        schema,
        || {
            AnalysisResultDocument::from_pac(analysis_id, result)
                .map(|builder| builder.parent_analysis(upstream))
        },
        |path, format| {
            if matches!(format, OutputFormat::Hdf5) {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Periodic AC".to_string();
                data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                let mut section = crate::hdf5::Hdf5AcSection::new(frequencies.clone());
                for column in &table.columns {
                    let ColumnData::Complex { real, imag } = &column.data else {
                        return Err(CliError::InternalError {
                            message: "a PAC sideband published a real response column".to_string(),
                        });
                    };
                    section.add_signal(column.name.clone(), real.clone(), imag.clone());
                }
                data.ac = Some(section);
                crate::hdf5::write_hdf5(path, &data)
                    .map_err(|error| super::shared::map_hdf5_output_error(path, error))
            } else {
                table.write(path, format)
            }
        },
    )?;
    ctx.record_output(path.to_path_buf());
    if !ctx.quiet {
        println!("  PAC sidebands exported to: {}", path.display());
    }
    Ok(())
}

/// One sideband's projected response columns.
struct PacSideband {
    sideband: i32,
    signals: Vec<ComplexSignal>,
}

/// Each sideband's node and branch responses, run through the deck's authored
/// output contract.
///
/// The projection is applied per sideband against the plain `V(node)`/`I(dev)`
/// spellings, because that is what a `.PRINT`/`.SAVE` card names; the sideband
/// index decorates the exported column afterwards. Composing it into the name
/// first would make every authored probe unresolvable, and leaving it off
/// entirely would publish columns that differ only in a number the artifact no
/// longer records. `.DISTO` composes its product label the same way.
fn pac_projected_sidebands(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::PacResult,
) -> Result<Vec<PacSideband>, CliError> {
    let point_count = result.frequencies.len();
    let mut sidebands = Vec::with_capacity(result.sideband_indices().len());
    for sideband in result.sideband_indices() {
        let mut node_columns = vec![(Vec::new(), Vec::new()); result.node_names.len()];
        let mut branch_columns = vec![(Vec::new(), Vec::new()); result.branch_names.len()];
        for index in 0..point_count {
            let data = result.get_sideband_data(index, sideband).ok_or_else(|| {
                CliError::simulation_error_in(
                    format!("PAC sideband {sideband} is missing at frequency index {index}"),
                    "PAC",
                )
            })?;
            if data.node_voltages.len() != result.node_names.len()
                || data.branch_currents.len() != result.branch_names.len()
            {
                return Err(CliError::simulation_error_in(
                    "a PAC sideband record disagrees with the result schema".to_string(),
                    "PAC",
                ));
            }
            for (column, value) in node_columns.iter_mut().zip(&data.node_voltages) {
                column.0.push(value.re);
                column.1.push(value.im);
            }
            for (column, value) in branch_columns.iter_mut().zip(&data.branch_currents) {
                column.0.push(value.re);
                column.1.push(value.im);
            }
        }

        let mut signals = Vec::with_capacity(node_columns.len() + branch_columns.len());
        for (name, (real, imag)) in result.node_names.iter().zip(node_columns) {
            signals.push(ComplexSignal {
                display_name: format!("V({name})"),
                raw_name: name.clone(),
                kind: SignalKind::Voltage,
                real,
                imag,
            });
        }
        for (name, (real, imag)) in result.branch_names.iter().zip(branch_columns) {
            signals.push(ComplexSignal {
                display_name: format!("I({name})"),
                raw_name: name.clone(),
                kind: SignalKind::Current,
                real,
                imag,
            });
        }
        let signals = crate::commands::run_signals::complex_export_signals(
            ctx.netlist,
            AnalysisResultKind::Pac,
            "PAC",
            &result.frequencies,
            &signals,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some(format!("PAC sideband {sideband} output projection")),
        })?;
        sidebands.push(PacSideband { sideband, signals });
    }
    Ok(sidebands)
}

/// The flat PAC table: the offset-frequency scale and one complex column per
/// projected signal per sideband, named `<signal>:sb<index>`.
fn pac_export_table(frequencies: &[f64], sidebands: &[PacSideband]) -> ExportTable {
    ExportTable {
        analysis: "pac".to_string(),
        plot_name: "Periodic AC".to_string(),
        scale_name: "offset_frequency".to_string(),
        scale_type: "frequency".to_string(),
        scale: frequencies.to_vec(),
        columns: sidebands
            .iter()
            .flat_map(|entry| {
                entry.signals.iter().map(|signal| ExportColumn {
                    name: format!("{}:sb{}", signal.display_name, entry.sideband),
                    var_type: signal.raw_variable_type().to_string(),
                    data: ColumnData::Complex {
                        real: signal.real.clone(),
                        imag: signal.imag.clone(),
                    },
                })
            })
            .collect(),
    }
}

/// Publish one periodic-noise sweep over its offset-frequency axis.
fn export_pnoise(
    ctx: &RunContext<'_>,
    artifact: &PeriodicArtifact,
    path: &std::path::Path,
    upstream: AnalysisInstanceId,
    result: &rspice_core::engine::PeriodicNoiseResult,
) -> Result<(), CliError> {
    let analysis_id = artifact.analysis;
    let offsets = result.offset_frequencies();
    let table = ExportTable {
        analysis: "pnoise".to_string(),
        plot_name: "Periodic Noise".to_string(),
        scale_name: "offset_frequency".to_string(),
        scale_type: "frequency".to_string(),
        scale: offsets.clone(),
        columns: pnoise_columns(result),
    };
    let schema = super::document::distinct_schema(table.columns.iter().map(|column| {
        rspice_core::execution::signal_descriptor(
            &column.name,
            &column.name,
            rspice_core::execution::SignalKind::Scalar,
            rspice_core::execution::SignalValueType::Real,
        )
    }))?;

    super::document::publish_analysis_result(
        ctx,
        path,
        analysis_id,
        schema,
        || {
            AnalysisResultDocument::from_pnoise(analysis_id, result)
                .map(|builder| builder.parent_analysis(upstream))
        },
        |path, format| {
            if matches!(format, OutputFormat::Hdf5) {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Periodic Noise".to_string();
                data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                let mut section =
                    crate::hdf5::Hdf5WaveformSection::new("offset_frequency", offsets.clone());
                for column in &table.columns {
                    let ColumnData::Real(values) = &column.data else {
                        return Err(CliError::InternalError {
                            message: "periodic noise published a complex density column"
                                .to_string(),
                        });
                    };
                    section.add_signal(column.name.clone(), values.clone());
                }
                data.noise = Some(section);
                crate::hdf5::write_hdf5(path, &data)
                    .map_err(|error| super::shared::map_hdf5_output_error(path, error))
            } else {
                table.write(path, format)
            }
        },
    )?;
    ctx.record_output(path.to_path_buf());
    if !ctx.quiet {
        println!("  PNOISE spectrum exported to: {}", path.display());
    }
    Ok(())
}

/// One spectral phase-noise column the standalone analyzer may or may not
/// have decomposed: its published name and how to read it off a point.
type OptionalPhaseNoiseColumn = (
    &'static str,
    fn(&rspice_core::analysis::pnoise::PhaseNoisePoint) -> Option<f64>,
);

/// The real density columns of one periodic-noise result.
///
/// The three forms measure different physical quantities — an absolute output
/// PSD, a carrier-relative phase-noise density, and the standalone analyzer's
/// spectral decomposition — so each publishes the columns it actually has
/// rather than a common shape padded with invented values.
fn pnoise_columns(result: &rspice_core::engine::PeriodicNoiseResult) -> Vec<ExportColumn> {
    use rspice_core::engine::PeriodicNoiseResult;

    let real = |name: &str, values: Vec<f64>| ExportColumn {
        name: name.to_string(),
        var_type: "noise".to_string(),
        data: ColumnData::Real(values),
    };
    match result {
        PeriodicNoiseResult::Driven { result, .. } => {
            let mut columns = vec![real("output_noise", result.output_noise.clone())];
            if let Some(input_noise) = &result.input_noise {
                columns.push(real("input_referred_noise", input_noise.clone()));
            }
            for (label, density) in &result.contributors {
                columns.push(real(
                    &format!("contribution:{}", label.to_ascii_lowercase()),
                    density.clone(),
                ));
            }
            columns
        }
        PeriodicNoiseResult::Oscillator { result, .. } => {
            vec![real("phase_noise", result.phase_noise_dbc.clone())]
        }
        PeriodicNoiseResult::Spectral(result) => {
            let mut columns = vec![real(
                "phase_noise",
                result
                    .spectral_points
                    .iter()
                    .map(|point| point.pn_dbc_hz)
                    .collect(),
            )];
            let optional: [OptionalPhaseNoiseColumn; 3] = [
                ("am_noise", |point| point.am_noise),
                ("upper_sideband_noise", |point| point.upper_sideband),
                ("lower_sideband_noise", |point| point.lower_sideband),
            ];
            for (name, extract) in optional {
                let column = result
                    .spectral_points
                    .iter()
                    .map(extract)
                    .collect::<Vec<_>>();
                // A column no point carries is absent from the artifact, not
                // published as a row of zeros. The typed document is where a
                // partially populated column keeps its per-sample absence.
                if let Some(values) = column.iter().copied().collect::<Option<Vec<_>>>() {
                    columns.push(real(name, values));
                }
            }
            columns
        }
    }
}

/// Publish one envelope-following run: the continued slow-time trajectory,
/// the same table shape as a transient export.
fn export_envelope(
    ctx: &RunContext<'_>,
    artifact: &PeriodicArtifact,
    path: &std::path::Path,
    upstream: AnalysisInstanceId,
    result: &rspice_core::engine::EnvelopeResult,
) -> Result<(), CliError> {
    let analysis_id = artifact.analysis;
    let transient = result.continued_transient();
    let times = transient.time.clone();
    let signals = crate::commands::run_signals::transient_export_signals(
        ctx.netlist,
        transient,
        ctx.engine.config().resource_limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("Envelope output projection".to_string()),
    })?;

    super::document::publish_analysis_result(
        ctx,
        path,
        analysis_id,
        super::document::scalar_schema(&signals)?,
        || {
            AnalysisResultDocument::from_envelope(analysis_id, result)
                .map(|builder| builder.parent_analysis(upstream))
        },
        |path, format| {
            if matches!(format, OutputFormat::Hdf5) {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Envelope Following".to_string();
                data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                let mut section = crate::hdf5::Hdf5WaveformSection::new("time", times.clone());
                for signal in &signals {
                    section.add_typed_signal(
                        signal.display_name.clone(),
                        signal.raw_variable_type(),
                        signal.values.clone(),
                    );
                }
                data.transient = Some(section);
                crate::hdf5::write_hdf5(path, &data)
                    .map_err(|error| super::shared::map_hdf5_output_error(path, error))
            } else {
                crate::commands::export_table::scalar_table(
                    "envelope",
                    "Envelope Following",
                    "time",
                    "time",
                    times.clone(),
                    &signals,
                )
                .write(path, format)
            }
        },
    )?;
    ctx.record_output(path.to_path_buf());
    if !ctx.quiet {
        println!("  Envelope trajectory exported to: {}", path.display());
    }
    Ok(())
}

//=============================================================================
// Shared refusals
//=============================================================================

/// Reject a sweep whose requested grid cannot fit the analysis budget before
/// the solver allocates anything.
fn preflight_periodic_sweep(
    ctx: &RunContext<'_>,
    sweep: &PeriodicSweep,
    analysis: &str,
) -> Result<(), CliError> {
    let limit = ctx.engine.config().resource_limits.max_analysis_points;
    if sweep.points > limit {
        return Err(CliError::CoreSimulationError {
            source: rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::AnalysisPoints,
                requested: sweep.points,
                limit,
            }),
            analysis: Some(analysis.to_string()),
        });
    }
    Ok(())
}

/// The plan named a carrier this run never retained.
fn missing_carrier(
    card: &str,
    analysis: AnalysisInstanceId,
    upstream: AnalysisInstanceId,
) -> CliError {
    CliError::simulation_error_in(
        format!(
            "the canonical plan bound {analysis} to {upstream}, which retained no periodic operating point for {card} to linearize around"
        ),
        card.trim_start_matches('.'),
    )
}

fn ensure_not_cancelled(ctx: &RunContext<'_>) -> Result<(), CliError> {
    if crate::abort::reason().is_some() {
        Err(super::cancellation_cli_error(ctx.args.timeout))
    } else {
        Ok(())
    }
}

fn map_periodic_error(
    ctx: &RunContext<'_>,
    analysis: &str,
    error: rspice_core::SimulationError,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        // Carry the engine's typed failure rather than its text. All four
        // cards of this family - .PSS, .PAC, .PNOISE and .ENVELOPE - report
        // through here, so stringifying re-decided every one of their failures
        // as the simulation category: a device the periodic solver refuses
        // left this process with the same status as one that would not
        // converge.
        CliError::CoreSimulationError {
            source: error,
            analysis: Some(analysis.to_string()),
        }
    }
}
