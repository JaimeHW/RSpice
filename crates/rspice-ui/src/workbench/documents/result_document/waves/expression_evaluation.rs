//! Resolve the input family before evaluating and retain each output's axis.

use super::*;
use crate::state::ComplexExpressionPolicy;

pub(super) fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionSeriesResult {
    evaluate_expression_with_policy(
        simulation,
        analysis_index,
        text,
        selection,
        ComplexExpressionPolicy::Rectangular,
    )
}

pub(super) fn evaluate_expression_with_policy(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
    complex_policy: ComplexExpressionPolicy,
) -> ExpressionSeriesResult {
    let run = simulation.active_run().ok_or("analysis no longer exists")?;
    let analysis = run
        .analyses
        .get(analysis_index)
        .ok_or("analysis no longer exists")?;
    // A Waves document may contain several analysis strips. A pane selection
    // applies only to the exact dataset and analysis it identifies.
    let selection = selection.filter(|selection| {
        selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
    });
    let expr =
        calculator::parser::try_parse(text).map_err(|error| format!("parse error: {error}"))?;
    if let Some(selection) = selection {
        if selection.source_indices.is_empty() {
            return Err("the expression selection contains no samples".to_owned());
        }
        if let Some(plan) = selection.family_render_plan() {
            return plan
                .groups()
                .iter()
                .map(|group| {
                    evaluate_scope(
                        analysis,
                        text,
                        &expr,
                        complex_policy,
                        ExpressionSource::FamilyMember {
                            ordinal: group.ordinal,
                        },
                        Some(&group.source_indices),
                        Some(&group.x_values),
                    )
                    .map_err(|error| format!("{}: {error}", group.label))
                })
                .collect();
        }
        return evaluate_scope(
            analysis,
            text,
            &expr,
            complex_policy,
            ExpressionSource::SelectedSamples,
            Some(&selection.source_indices),
            None,
        )
        .map(|output| vec![output]);
    }
    evaluate_scope(
        analysis,
        text,
        &expr,
        complex_policy,
        ExpressionSource::Analysis,
        None,
        None,
    )
    .map(|output| vec![output])
}

fn evaluate_scope(
    analysis: &AnalysisResult,
    text: &str,
    expr: &calculator::ast::CalculatorExpr,
    policy: ComplexExpressionPolicy,
    source: ExpressionSource,
    indices: Option<&[usize]>,
    axis: Option<&[f64]>,
) -> Result<ExpressionWaveform, String> {
    let mut ctx = calculator::WaveformsContext::with_policy(&analysis.waveforms, policy);
    if let Some(indices) = indices {
        ctx = ctx
            .with_sample_projection(indices, axis)
            .map_err(|error| error.to_string())?;
    }
    let scalar_axis =
        if let Some(axis) = axis {
            axis.to_vec()
        } else if let Some(waveform) = analysis.waveforms.first() {
            if let Some(indices) = indices {
                indices
                    .iter()
                    .map(|index| {
                        waveform.x.get(*index).copied().ok_or_else(|| {
                            "selected rows exceed the retained analysis axis".to_owned()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                waveform
                    .x
                    .first()
                    .zip(waveform.x.last())
                    .map(|(&first, &last)| vec![first, last])
                    .unwrap_or_default()
            }
        } else {
            Vec::new()
        };
    let value = calculator::evaluator::evaluate(expr, &ctx).map_err(|error| error.to_string())?;
    let waveform = calculator::evaluated_waveform(value, text, Some(&scalar_axis))?;
    if waveform.x.is_empty() {
        return Err("expression produced no samples".to_owned());
    }
    Ok(ExpressionWaveform { source, waveform })
}
