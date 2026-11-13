//! Resolve the input family before evaluating and retain each output's axis.

use super::*;
use crate::state::ComplexExpressionPolicy;
use rspice_results_ui::waves::lanes::EventLaneLayout;

#[cfg(test)]
pub(super) fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionSeriesResult {
    evaluate_expression_reading(
        simulation,
        analysis_index,
        text,
        selection,
        ComplexExpressionPolicy::Rectangular,
        None,
    )
}

/// Evaluate `text` with the strip's buses readable by `analog()`, each in
/// the signedness of the radix `lanes` reads it in, the reader's own buses
/// among them.
pub(super) fn evaluate_expression_reading(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
    complex_policy: ComplexExpressionPolicy,
    lanes: Option<&EventLaneLayout>,
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
                        (analysis, lanes),
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
            (analysis, lanes),
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
        (analysis, lanes),
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
    (analysis, lanes): (&AnalysisResult, Option<&EventLaneLayout>),
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
    let ctx = EventWords {
        values: ctx,
        analysis,
        lanes,
    };
    let value = calculator::evaluator::evaluate(expr, &ctx).map_err(|error| error.to_string())?;
    let waveform = calculator::evaluated_waveform(value, text, Some(&scalar_axis))?;
    if waveform.x.is_empty() {
        return Err("expression produced no samples".to_owned());
    }
    Ok(ExpressionWaveform { source, waveform })
}

/// An analysis' values, with its digital nets and buses read from their exact
/// event histories for `analog()`.
struct EventWords<'a, C> {
    values: C,
    analysis: &'a AnalysisResult,
    lanes: Option<&'a EventLaneLayout>,
}

impl<C: calculator::evaluator::EvaluationContext> calculator::evaluator::EvaluationContext
    for EventWords<'_, C>
{
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<calculator::evaluator::CalcValue, calculator::evaluator::EvaluationError> {
        self.values.get_waveform(signal, dataset)
    }

    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<calculator::evaluator::CalcValue, calculator::evaluator::EvaluationError> {
        self.values.get_magnitude(signal, dataset)
    }

    fn get_event_word(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<calculator::functions::EventWord, calculator::evaluator::EvaluationError> {
        event_word(self.analysis, self.lanes, signal)
    }
}

/// The exact history of the bus or logic net `D(name)` names: one of the
/// reader's buses, a bus the run declared (with or without its range), or a
/// single net.
fn event_word(
    analysis: &AnalysisResult,
    lanes: Option<&EventLaneLayout>,
    signal: &str,
) -> Result<calculator::functions::EventWord, calculator::evaluator::EvaluationError> {
    use calculator::evaluator::EvaluationError;
    use rspice_results::engine_evidence::{VcdBit, event_code_to_vcd_bit};
    use rspice_results_ui::waves::logic::{BusLane, BusOrigin, BusRadix};

    let missing = || EvaluationError::IdentifierNotFound(signal.to_owned());
    let name = signal
        .strip_prefix("D(")
        .and_then(|name| name.strip_suffix(')'))
        .unwrap_or(signal);
    let Some(crate::state::AnalysisResultPayload::TransientEvents {
        digital_traces,
        digital_buses,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return Err(missing());
    };
    let history = |net: &str| {
        digital_traces
            .iter()
            .find(|trace| trace.node_name == net)
            .map(|trace| {
                trace
                    .points
                    .iter()
                    .map(|point| (point.time_s, point.value_code))
                    .collect::<Vec<_>>()
            })
    };
    let user = lanes.and_then(|lanes| {
        lanes
            .user_buses
            .iter()
            .find(|bus| bus.name == name)
            .map(|bus| (bus.name.clone(), bus.members.clone()))
    });
    let declared = || {
        digital_buses.iter().find_map(|bus| {
            let label = format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb);
            (bus.name == name || label == name).then(|| {
                (
                    label,
                    bus.members
                        .iter()
                        .map(|member| format!("D({member})"))
                        .collect::<Vec<_>>(),
                )
            })
        })
    };
    let (label, members) = match user.or_else(declared) {
        Some(bus) => bus,
        None if history(name).is_some() => (signal.to_owned(), vec![format!("D({name})")]),
        None => return Err(missing()),
    };
    let nets: Vec<String> = members
        .iter()
        .map(|member| {
            member
                .strip_prefix("D(")
                .and_then(|net| net.strip_suffix(')'))
                .unwrap_or(member)
                .to_owned()
        })
        .collect();
    let histories = nets
        .iter()
        .map(|net| history(net))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(missing)?;
    let lane = BusLane::assemble(label.clone(), nets, BusOrigin::User, &histories);
    if let Some(refusal) = lane.refusal() {
        return Err(EvaluationError::MathError(format!("{signal}: {refusal}")));
    }
    let words = (0..lane.times().len())
        .filter_map(|index| lane.word(index))
        .map(|codes| {
            codes
                .iter()
                .map(|code| match event_code_to_vcd_bit(*code) {
                    Some(VcdBit::Zero) => Some(false),
                    Some(VcdBit::One) => Some(true),
                    _ => None,
                })
                .collect()
        })
        .collect();
    let end = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| waveform.x.last().copied())
        .chain(lane.times().last().copied())
        .fold(f64::NEG_INFINITY, f64::max);
    Ok(calculator::functions::EventWord {
        times: lane.times().to_vec(),
        words,
        signed: lanes.is_some_and(|lanes| lanes.bus_view(&label).radix == BusRadix::Signed),
        end,
    })
}
