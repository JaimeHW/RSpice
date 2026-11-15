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

/// The analysis whose sensitivities a Xyce `.SENS` deck asks for and that
/// are not computed here: Xyce differentiates the deck's one analysis, and a
/// transient or AC deck asks for transient or AC sensitivities.
fn uncomputed_sensitivity_analysis(ctx: &RunContext<'_>) -> Option<&'static str> {
    ctx.netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tran { .. } => Some("transient"),
            AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => Some("AC"),
            AnalysisCommand::Hb { .. } => Some("harmonic-balance"),
            AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => Some("noise"),
            _ => None,
        })
}

/// The deck's Xyce `.SENS` objective cards when the study will read its
/// derivatives from `dc_card`'s sweep and has not run yet: `dc_card` is the
/// deck's first `.DC` card, the one the study sweeps, and comes before the
/// study's first card.
pub(in crate::commands::run) fn xyce_sensitivity_cards_reading<'n>(
    ctx: &RunContext<'n>,
    dc_card: &AnalysisCommand,
) -> Option<Vec<&'n AnalysisCommand>> {
    let analyses = &ctx.netlist.analyses;
    let first_dc = analyses
        .iter()
        .position(|card| matches!(card, AnalysisCommand::Dc { .. }))?;
    let first_study = analyses
        .iter()
        .position(|card| is_xyce_sensitivity(ctx, card))?;
    (std::ptr::eq(&analyses[first_dc], dc_card)
        && first_dc < first_study
        && uncomputed_sensitivity_analysis(ctx).is_none())
    .then(|| {
        analyses
            .iter()
            .filter(|card| is_xyce_sensitivity(ctx, card))
            .collect()
    })
}

/// Run every objective of the deck's Xyce `.SENS` study at its first
/// objective card, and publish the one table Xyce writes. Each later
/// objective card only takes its planned identity. When the deck's `.DC`
/// card already read the study from its sweep, that study is published.
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
    if let Some(analysis) = uncomputed_sensitivity_analysis(ctx) {
        crate::console::diagnostic_line(format_args!(
            "Warning: .SENS in a {analysis} deck asks for {analysis} sensitivities, which are \
             not computed; the .SENS card is skipped"
        ));
        return Ok(());
    }
    if !ctx.quiet {
        crate::console::line(format_args!("Running Sensitivity analysis"))?;
    }
    let points = match ctx.take_xyce_dc_sensitivity() {
        Some(study) => study,
        None => ctx.engine.run_sensitivity_dc_sweep_with_abort(
            ctx.netlist,
            &cards,
            &crate::abort::ProcessAbort,
        ),
    }
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
    let schema = super::super::document::scalar_schema(&signals)?;
    // Xyce's result is this table, whatever its points and objectives, so
    // every format writes the table: JSON carries the CSV's columns the way a
    // .STEP sweep's JSON does.
    if ctx.format == OutputFormat::Hdf5 {
        crate::hdf5::write_table(
            &output.path,
            &table,
            Some(super::super::document::hdf5_identity(ctx, analysis_id)?),
        )?;
    } else {
        table.write(&output.path, ctx.format)?;
    }
    ctx.record_published(super::super::PublishedResult {
        analysis_id: analysis_id.tag(),
        result_kind: None,
        schema,
        artifact: output.path.clone(),
        source_sample_presence: None,
    });
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
