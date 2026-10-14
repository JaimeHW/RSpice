//! Add a scalar measurement to the pane the reader is working in.
//!
//! The expression is evaluated on the selected pane's bound analysis and
//! kept with the document as `AddScalarMeasurement`, referencing the pane's
//! traces it reads. Over a quick view the form still evaluates, so the value
//! can be read, but nothing is kept: a quick view has no document to keep it.

use rspice_results::number_format::fmt_si_significant;
use rspice_results_ui::document_dialogs::measurement::{MeasurementDraft, MeasurementPreview};

use super::*;
use crate::analysis::calculator;
use crate::results::visualization_document::TraceId;

const QUICK_VIEW: &str = "Create a result document to keep measurements.";

/// Open Add measurement on the result the reader is looking at.
pub(crate) fn open_measurement(state: &mut AppState) {
    state.workbench.result_document_dialog = Some(ResultDocumentDialog::Measurement(
        MeasurementDraft::default(),
    ));
}

/// What the form shows for `draft` on this frame.
pub(super) fn preview(state: &AppState, draft: &MeasurementDraft) -> MeasurementPreview {
    let (run, analysis) = match reading_analysis(state) {
        Ok(found) => found,
        Err(refusal) => {
            return MeasurementPreview {
                pane: String::new(),
                value: Ok(None),
                refusal: Some(refusal),
            };
        }
    };
    let document = active_document_id(state).and_then(|document_id| {
        let (_, pane_id) = super::super::persistent_document::reading_position(state, document_id)?;
        let document = state
            .workspace
            .content
            .visualization_document(document_id)?;
        let pane = document.panes().iter().find(|pane| pane.id == pane_id)?;
        Some((
            format!("{} · {}", document.title(), pane.title),
            usize::from(document.presentation().significant_digits),
        ))
    });
    let expression = draft.expression.trim();
    let digits = document.as_ref().map_or_else(
        || {
            usize::from(
                state
                    .ui
                    .preferences
                    .result_presentation_policy()
                    .displayed_significant_digits()
                    .get(),
            )
        },
        |(_, digits)| *digits,
    );
    let value = if expression.is_empty() {
        Ok(None)
    } else {
        evaluate(analysis, expression).map(|value| Some(fmt_si_significant(value, "", digits)))
    };
    let refusal = document.is_none().then(|| QUICK_VIEW.to_owned());
    MeasurementPreview {
        pane: document.map_or_else(
            || format!("Run {} · {} · {}", run.id, run.label, analysis.label),
            |(pane, _)| pane,
        ),
        value,
        refusal,
    }
}

/// Keep the draft's measurement on the selected pane. The form closes once
/// it lands.
pub(super) fn commit(
    state: &mut AppState,
    draft: &MeasurementDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    let document_id = active_document_id(state).ok_or_else(|| QUICK_VIEW.to_owned())?;
    let expression = draft.expression.trim().to_owned();
    if expression.is_empty() {
        return Err("Enter an expression to measure.".to_owned());
    }
    let (_, analysis) = reading_analysis(state)?;
    let value = evaluate(analysis, &expression)?;
    let (_, pane_id) = super::super::persistent_document::reading_position(state, document_id)
        .ok_or_else(|| "The result document has no pane.".to_owned())?;
    let trace_ids = pane_traces(project_document(state, document_id)?, pane_id, &expression)?;
    transact(
        state,
        document_id,
        vec![DocumentEdit::AddScalarMeasurement {
            pane_id,
            trace_ids,
            expression,
            value,
        }],
    )?;
    Ok(None)
}

/// Reduce `expression` to one finite number over the analysis's waveforms.
fn evaluate(analysis: &AnalysisResult, expression: &str) -> Result<f64, String> {
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let context = calculator::WaveformsContext::new(&analysis.waveforms);
    match calculator::evaluator::evaluate(&parsed, &context)
        .and_then(calculator::CalcValue::into_real)
        .map_err(|error| error.to_string())?
    {
        calculator::RealValue::Scalar(value) if value.is_finite() => Ok(value),
        calculator::RealValue::Scalar(_) => Err("The expression has no finite value.".to_owned()),
        calculator::RealValue::Waveform(..) => Err(
            "The expression is a trace; reduce it with avg(), rms() or another scalar function."
                .to_owned(),
        ),
    }
}

/// The pane's traces the measurement reads: each signal the expression
/// names, or the pane's first trace when it names none.
fn pane_traces(
    document: &VisualizationDocument,
    pane_id: PaneId,
    expression: &str,
) -> Result<Vec<TraceId>, String> {
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let mut signals = Vec::new();
    collect_signals(&parsed, &mut signals);
    if signals.is_empty() {
        return document
            .traces()
            .iter()
            .find(|trace| trace.pane_id == pane_id)
            .map(|trace| vec![trace.id])
            .ok_or_else(|| "The selected pane has no trace to keep a measurement on.".to_owned());
    }
    signals
        .iter()
        .map(|signal| {
            document
                .traces()
                .iter()
                .find(|trace| trace.pane_id == pane_id && trace.label == *signal)
                .map(|trace| trace.id)
                .ok_or_else(|| format!("{signal} is not a trace of the selected pane."))
        })
        .collect()
}

fn collect_signals(expression: &calculator::ast::CalculatorExpr, signals: &mut Vec<String>) {
    use calculator::ast::CalculatorExpr;
    match expression {
        CalculatorExpr::WaveformRef { signal, .. } => {
            if !signals.contains(signal) {
                signals.push(signal.clone());
            }
        }
        CalculatorExpr::BinaryOp { left, right, .. } => {
            collect_signals(left, signals);
            collect_signals(right, signals);
        }
        CalculatorExpr::UnaryOp { operand, .. } => collect_signals(operand, signals),
        CalculatorExpr::FunctionCall { args, .. } => {
            for argument in args {
                collect_signals(argument, signals);
            }
        }
        CalculatorExpr::Number(_) | CalculatorExpr::Constant(_) => {}
    }
}
