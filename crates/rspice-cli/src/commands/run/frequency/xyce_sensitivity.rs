//! Xyce's `.SENS` table: one row per `.DC` point (one at an operating
//! point), holding the `.PRINT SENS` columns, then each objective and its
//! derivative columns (N_IO_OutputMgr.C:2611-2660, N_IO_OpBuilders.C:1689-1708).

use rspice_core::analysis::SensitivityResult;
use rspice_core::netlist::AnalysisCommand;

use super::RunContext;
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ScalarSignal, SignalKind, sensitivity_print_signals};

/// Whether a card is a DC objective of a Xyce `.SENS` study.
pub(in crate::commands::run) fn is_xyce_sensitivity(
    ctx: &RunContext<'_>,
    card: &AnalysisCommand,
) -> bool {
    ctx.engine.config().spice_dialect == rspice_core::config::SpiceDialect::Xyce
        && matches!(card, AnalysisCommand::Sensitivity { ac_sweep: None, .. })
}

/// Run every objective of the deck's Xyce `.SENS` study at its first
/// objective card, and publish the one table Xyce writes. Each later
/// objective card only takes its planned identity.
pub(in crate::commands::run) fn run_xyce_sensitivity(
    ctx: &RunContext<'_>,
    card: &AnalysisCommand,
) -> Result<(), CliError> {
    let cards = ctx
        .netlist
        .analyses
        .iter()
        .filter(|candidate| is_xyce_sensitivity(ctx, candidate))
        .collect::<Vec<_>>();
    let output = ctx.resolve_output("sens");
    if !cards
        .first()
        .is_some_and(|first| std::ptr::eq(*first, card))
    {
        return Ok(());
    }
    // Xyce differentiates the deck's one analysis: a transient or AC deck asks
    // for transient or AC sensitivities, which are not computed here.
    if let Some(analysis) = ctx
        .netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tran { .. } => Some("transient"),
            AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => Some("AC"),
            AnalysisCommand::Hb { .. } => Some("harmonic-balance"),
            AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => Some("noise"),
            _ => None,
        })
    {
        crate::console::diagnostic_line(format_args!(
            "Warning: .SENS in a {analysis} deck asks for {analysis} sensitivities, which are \
             not computed; the .SENS card is skipped"
        ));
        return Ok(());
    }
    if !ctx.quiet {
        crate::console::line(format_args!("Running Sensitivity analysis"))?;
    }
    let points = ctx
        .engine
        .run_sensitivity_dc_sweep_with_abort(ctx.netlist, &cards, &crate::abort::ProcessAbort)
        .map_err(|source| super::map_frequency_error(ctx, "Sensitivity", source))?;
    let operating_points = points
        .iter()
        .map(|point| (point.sweep_value, point.result.clone()))
        .collect::<Vec<_>>();
    let mut signals = sensitivity_print_signals(
        ctx.netlist,
        &operating_points,
        ctx.engine.config().resource_limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| super::map_frequency_error(ctx, "Sensitivity output projection", source))?;
    let options = &ctx.netlist.options;
    let direct = options.sensitivity_direct.unwrap_or(false);
    let adjoint = options.sensitivity_adjoint.unwrap_or(true);
    let scaled = options.sensitivity_output_scaled.unwrap_or(false);
    let unscaled = options.sensitivity_output_unscaled.unwrap_or(true);
    let column = |name: String, value: &dyn Fn(&SensitivityResult) -> Option<f64>, index| {
        points
            .iter()
            .map(|point| value(&point.objectives[index]))
            .collect::<Option<Vec<_>>>()
            .map(|values| ScalarSignal {
                raw_name: name.clone(),
                display_name: name,
                kind: SignalKind::Scalar,
                values,
                units: None,
            })
    };
    for (index, card) in cards.iter().enumerate() {
        let objective = objective_label(card);
        signals.extend(column(
            objective.clone(),
            &|study: &SensitivityResult| Some(study.output_value),
            index,
        ));
        let Some(first) = points.first() else {
            continue;
        };
        for (row, parameter) in first.objectives[index].sensitivities.iter().enumerate() {
            let name = format!("d_{objective}/d_{}", parameter.vector_name);
            let unscaled_value =
                move |study: &SensitivityResult| study.sensitivities.get(row).map(|r| r.absolute);
            // N_NLS_Sensitivity.C:703: the scaled derivative is dO/dp * p / 100.
            let scaled_value = move |study: &SensitivityResult| {
                study
                    .sensitivities
                    .get(row)
                    .map(|r| r.absolute * r.nominal_value / 100.0)
            };
            for (enabled, method) in [(direct, "dir"), (adjoint, "adj")] {
                if !enabled {
                    continue;
                }
                if unscaled {
                    signals.extend(column(format!("{name}_{method}"), &unscaled_value, index));
                }
                if scaled {
                    signals.extend(column(
                        format!("{name}_{method}_scaled"),
                        &scaled_value,
                        index,
                    ));
                }
            }
        }
    }
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Sensitivity: {} point(s), {} objective(s)",
            points.len(),
            cards.len()
        ))?;
    }

    let Some(output) = output else {
        return Ok(());
    };
    let single = matches!((points.as_slice(), cards.len()), ([_], 1));
    if ctx.format == OutputFormat::Json && !single {
        return Err(super::map_frequency_error(
            ctx,
            "Sensitivity",
            rspice_core::SimulationError::Circuit(
                "a .SENS table with more than one point or objective has no JSON result \
                 document; write it as CSV, raw or HDF5"
                    .to_owned(),
            ),
        ));
    }
    let analysis_id = output.analysis("sens")?;
    let index = (0..points.len()).map(|row| row as f64).collect::<Vec<_>>();
    let table = crate::commands::export_table::scalar_table(
        "sens",
        "Sensitivity",
        "Index",
        "notype",
        index,
        &signals,
    );
    super::super::document::publish_table_result(
        ctx,
        &output.path,
        analysis_id,
        super::super::document::scalar_schema(&signals)?,
        &table,
        // Only a one-point, one-objective table reaches the typed document.
        || {
            rspice_core::execution::AnalysisResultDocument::from_sensitivity(
                analysis_id,
                &points[0].objectives[0],
            )
        },
    )?;
    if !ctx.quiet {
        crate::console::line(format_args!(
            "  Sensitivities exported to: {}",
            output.path.display()
        ))?;
    }
    Ok(())
}

/// An objective as Xyce prints it: its `OBJFUNC` expression, uppercased.
fn objective_label(card: &AnalysisCommand) -> String {
    match card {
        AnalysisCommand::Sensitivity {
            output_node,
            output_is_current: true,
            ..
        } => format!("{{I({output_node})}}"),
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node: Some(reference),
            ..
        } => format!("{{V({output_node},{reference})}}"),
        AnalysisCommand::Sensitivity { output_node, .. } => format!("{{V({output_node})}}"),
        _ => String::new(),
    }
}
