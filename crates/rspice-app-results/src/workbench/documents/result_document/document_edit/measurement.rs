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
pub fn open_measurement(state: &mut ResultsParts<'_>) {
    *state.result_document_dialog = Some(ResultDocumentDialog::Measurement(
        MeasurementDraft::default(),
    ));
}

/// What the form shows for `draft` on this frame.
pub fn preview(state: ResultsReads<'_>, draft: &MeasurementDraft) -> MeasurementPreview {
    let (run, analysis) = match reading_analysis(state) {
        Ok(found) => found,
        Err(refusal) => {
            return MeasurementPreview {
                pane: String::new(),
                signals: Vec::new(),
                signal_units: Vec::new(),
                x_unit: String::new(),
                value: Ok(None),
                refusal: Some(refusal),
            };
        }
    };
    let pane = active_document_id(state).and_then(|document_id| {
        let (_, pane_id) = super::super::persistent_document::reading_position(state, document_id)?;
        let document = state
            .model
            .workspace
            .content
            .visualization_document(document_id)?;
        let pane = document.panes().iter().find(|pane| pane.id == pane_id)?;
        // A signal argument picks from the traces this pane's strip shows.
        let signals = document
            .traces()
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .map(|trace| trace.label.clone())
            .collect::<Vec<_>>();
        Some((format!("{} · {}", document.title(), pane.title), signals))
    });
    let expression = draft.expression.trim();
    let digits = usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let value = if expression.is_empty() {
        Ok(None)
    } else {
        evaluate(analysis, expression).map(|value| Some(fmt_si_significant(value, "", digits)))
    };
    let refusal = pane.is_none().then(|| QUICK_VIEW.to_owned());
    // A level is typed in its signal's unit, as the strip reads the trace.
    let unit_of = |name: &str| {
        let retained = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == name)
            .and_then(|waveform| waveform.unit.as_deref());
        super::super::browser_signal_unit(
            name,
            retained,
            super::super::analysis_default_unit(analysis.analysis_type),
        )
        .to_owned()
    };
    // A run's label already names it ("Run 1 (8:31:04 PM)"), as the
    // navigator shows it; a number in front would name it twice. A quick
    // view's strip can show any of the analysis's signals, offered by the
    // names an expression reads them by: `|V(out)|` is `V(out)`, whose
    // phase comes with it.
    let (pane, signals) = pane.unwrap_or_else(|| {
        let mut signals: Vec<String> = Vec::new();
        for waveform in &analysis.waveforms {
            let name = waveform.name.as_str();
            let name = name
                .strip_prefix('|')
                .and_then(|inner| inner.strip_suffix('|'))
                .unwrap_or(name);
            if !name.starts_with("phase(") && !signals.iter().any(|known| known == name) {
                signals.push(name.to_owned());
            }
        }
        (format!("{} · {}", run.label, analysis.label), signals)
    });
    let signal_units = signals.iter().map(|name| unit_of(name)).collect();
    MeasurementPreview {
        pane,
        signals,
        signal_units,
        x_unit: x_unit(state, run, analysis),
        value,
        refusal,
    }
}

/// The unit the strip reads the analysis's x-axis in, which a point on it is
/// typed in; before the strip has drawn it, the analysis's own.
fn x_unit(state: ResultsReads<'_>, run: &SimulationRun, analysis: &AnalysisResult) -> String {
    let key = super::super::AnalysisPresentationKey::new(run.dataset_id, analysis);
    state
        .results
        .models
        .last_built()
        .and_then(|models| models.iter().find(|model| model.analysis_key == key))
        .map_or_else(
            || analysis.analysis_type.axis_info().1.to_owned(),
            |model| model.x_unit.clone(),
        )
}

/// Keep the draft's measurement on the selected pane. The form closes once
/// it lands.
pub(super) fn commit(
    state: &mut ResultsParts<'_>,
    draft: &MeasurementDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    let document_id =
        active_document_id(state.results_reads()).ok_or_else(|| QUICK_VIEW.to_owned())?;
    let expression = draft.expression.trim().to_owned();
    if expression.is_empty() {
        return Err("Enter an expression to measure.".to_owned());
    }
    let (_, analysis) = reading_analysis(state.results_reads())?;
    let value = evaluate(analysis, &expression)?;
    let (_, pane_id) =
        super::super::persistent_document::reading_position(state.results_reads(), document_id)
            .ok_or_else(|| "The result document has no pane.".to_owned())?;
    let trace_ids = pane_traces(
        project_document(state.results_reads(), document_id)?,
        pane_id,
        &expression,
    )?;
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
        CalculatorExpr::Number(_) | CalculatorExpr::Constant(_) | CalculatorExpr::Keyword(_) => {}
    }
}
