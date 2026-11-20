//! Compare the result the reader is looking at with a baseline dataset.
//!
//! The candidate is the selected pane's binding over a project result
//! document, or the active analysis over a quick view. Over a document the
//! receipt is recorded with it, both datasets attached in the same
//! transaction; over a quick view the outcome is shown and not kept.

use rspice_results::number_format::fmt_si_significant;
use rspice_results::waveform_comparison::{
    ComparisonAlignmentDraft, ComparisonOptions, ComparisonSide, comparison_signal_names,
    execute_comparison, matching_comparison_analysis,
};
use rspice_results_ui::document_dialogs::compare::{
    CompareChoices, CompareDraft, CompareOutcome, SignalOutcome, Verdict,
};

use super::super::AnalysisPresentationKey;
use super::*;
use crate::results::visualization_document::ComparisonReceipt;

/// The retained runs that hold the candidate's analysis on the same X axis,
/// each with its matching analysis and the signals the two share.
fn baselines<'a>(
    state: ResultsReads<'a>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Vec<(&'a SimulationRun, Vec<String>)> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|other| other.dataset_id != run.dataset_id)
        .filter_map(|other| {
            let baseline = matching_comparison_analysis(analysis, other)?;
            let signals = comparison_signal_names(analysis, baseline).ok()?;
            Some((other, signals))
        })
        .collect()
}

fn run_label(run: &SimulationRun) -> String {
    format!("Run {} · {}", run.id, run.label)
}

/// Whether Compare datasets has a candidate and a baseline to compare it with.
///
/// The toolbar asks on every frame, so the answer is kept per candidate until
/// the retained runs change, and it reads no sample: see [`comparable`].
pub fn compare_available(state: ResultsReads<'_>) -> bool {
    let Ok((run, analysis)) = reading_analysis(state) else {
        return false;
    };
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    state
        .results
        .compare_availability
        .get_or_insert_with(&state.model.simulation, key, || {
            state
                .model
                .simulation
                .retained
                .runs
                .iter()
                .filter(|other| other.dataset_id != run.dataset_id)
                .filter_map(|other| matching_comparison_analysis(analysis, other))
                .any(|baseline| comparable(analysis, baseline))
        })
}

/// Whether `baseline` can be compared with `candidate` as far as their
/// waveforms' names, units, lengths and axes say: the same coordinate unit,
/// and a signal on the candidate's axis that the baseline holds once, in the
/// same unit, with a sample for every coordinate. No sample is read; the
/// samples are checked when the comparison opens ([`baselines`]).
fn comparable(candidate: &AnalysisResult, baseline: &AnalysisResult) -> bool {
    use rspice_results::waveform::WaveformValues;

    if candidate.waveform_coordinate_unit() != baseline.waveform_coordinate_unit() {
        return false;
    }
    let Some(reference) = candidate.waveforms.first() else {
        return false;
    };
    // A run's waveforms share its axis; an axis held twice is the same axis
    // when its length and ends agree.
    let on_axis = |x: &WaveformValues| {
        WaveformValues::ptr_eq(x, &reference.x)
            || (x.len() == reference.x.len() && x.ends() == reference.x.ends())
    };
    candidate
        .waveforms
        .iter()
        .filter(|waveform| on_axis(&waveform.x) && waveform.y.len() == waveform.x.len())
        .any(|waveform| {
            let mut matching = baseline
                .waveforms
                .iter()
                .filter(|other| other.name == waveform.name);
            match (matching.next(), matching.next()) {
                (Some(other), None) => {
                    other.unit == waveform.unit && other.y.len() == other.x.len()
                }
                _ => false,
            }
        })
}

/// Whether Compare datasets can run here, and why not: away from Results
/// there is no active result document, and in it a second compatible run is
/// missing.
pub fn compare_availability(
    state: ResultsReads<'_>,
) -> crate::workbench::framework::command_registry::CommandAvailability {
    use crate::workbench::framework::command_registry::CommandAvailability;

    if compare_available(state) {
        CommandAvailability::Available
    } else if state.workbench.workspace != Workspace::Results {
        CommandAvailability::Disabled("open an active result document")
    } else {
        CommandAvailability::Disabled("a second compatible run is required")
    }
}

/// Open Compare datasets on the result the reader is looking at, against the
/// first compatible baseline.
pub fn open_compare(state: &mut ResultsParts<'_>) {
    // Compare datasets is disabled while either is missing.
    if !compare_available(state.results_reads()) {
        return;
    }
    let baseline = reading_analysis(state.results_reads())
        .ok()
        .and_then(|(run, analysis)| {
            baselines(state.results_reads(), run, analysis)
                .first()
                .map(|(baseline, _)| baseline.dataset_id)
        });
    // It is offered from the runs' names and axes alone; a baseline whose
    // samples turn out not to compare opens the form with its refusal.
    *state.result_document_dialog = Some(ResultDocumentDialog::Compare(CompareDraft {
        baseline,
        ..CompareDraft::default()
    }));
}

const NO_BASELINE: &str = "No other retained dataset holds this analysis on the same X axis.";

/// The parsed numbers of a draft.
struct CompareNumbers {
    threshold: f64,
    maximum_lag: u32,
    absolute_tolerance: f64,
    relative_tolerance: f64,
}

fn numbers(draft: &CompareDraft) -> Result<CompareNumbers, String> {
    let number = |label: &str, text: &str| {
        crate::quantity::parse_engineering_value(text)
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| format!("{label} is not a number."))
    };
    let tolerance = |label: &str, text: &str| {
        number(label, text).and_then(|value| {
            (value >= 0.0)
                .then_some(value)
                .ok_or_else(|| format!("{label} cannot be negative."))
        })
    };
    Ok(CompareNumbers {
        threshold: if draft.alignment == ComparisonAlignmentDraft::FirstThresholdCrossing {
            number("Threshold", &draft.threshold)?
        } else {
            0.0
        },
        maximum_lag: if draft.alignment == ComparisonAlignmentDraft::CrossCorrelation {
            draft
                .maximum_lag
                .trim()
                .parse()
                .map_err(|_| "Max lag is a whole number of samples.".to_owned())?
        } else {
            0
        },
        absolute_tolerance: tolerance("Abs tol", &draft.absolute_tolerance)?,
        relative_tolerance: tolerance("Rel tol", &draft.relative_tolerance)?,
    })
}

/// Resolve the form's choices against the retained datasets, moving a
/// baseline or signal that no longer exists onto the first one that does.
pub fn choices(state: ResultsReads<'_>, draft: &mut CompareDraft) -> CompareChoices {
    let (run, analysis) = match reading_analysis(state) {
        Ok(found) => found,
        Err(refusal) => {
            return CompareChoices {
                candidate: String::new(),
                baselines: Vec::new(),
                signals: Vec::new(),
                refusal: Some(refusal),
            };
        }
    };
    let baselines = baselines(state, run, analysis);
    let selected = draft
        .baseline
        .and_then(|id| {
            baselines
                .iter()
                .find(|(baseline, _)| baseline.dataset_id == id)
        })
        .or_else(|| baselines.first());
    draft.baseline = selected.map(|(baseline, _)| baseline.dataset_id);
    let signals = selected.map_or_else(Vec::new, |(_, signals)| signals.clone());
    if !signals.contains(&draft.signal) {
        draft.signal = signals.first().cloned().unwrap_or_default();
    }
    let refusal = if baselines.is_empty() {
        Some(NO_BASELINE.to_owned())
    } else {
        numbers(draft).err()
    };
    CompareChoices {
        candidate: format!("{} · {}", run_label(run), analysis.label),
        baselines: baselines
            .iter()
            .map(|(baseline, _)| (baseline.dataset_id, run_label(baseline)))
            .collect(),
        signals,
        refusal,
    }
}

/// Run the comparison and keep its outcome in the form, recording the
/// receipt with the open document. Once the outcome shows, `Done` closes.
pub(super) fn commit(
    state: &mut ResultsParts<'_>,
    mut draft: CompareDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    if draft.outcome.is_some() {
        return Ok(None);
    }
    let numbers = numbers(&draft)?;
    let (outcome, edits) = {
        let (run, analysis) = reading_analysis(state.results_reads())?;
        let baseline = draft
            .baseline
            .and_then(|id| {
                state
                    .model
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .find(|run| run.dataset_id == id)
            })
            .ok_or_else(|| "The baseline dataset is no longer retained.".to_owned())?;
        let baseline_analysis = matching_comparison_analysis(analysis, baseline)
            .ok_or_else(|| "The baseline holds no matching analysis.".to_owned())?;
        let signal_names = comparison_signal_names(analysis, baseline_analysis)?;
        let side = |run: &SimulationRun, analysis| ComparisonSide {
            binding: DatasetBinding::new(
                run.dataset_id,
                super::super::retained_dataset_digest(state.results_reads(), run),
            ),
            analysis,
        };
        let receipt = execute_comparison(
            side(run, &analysis.data),
            side(baseline, &baseline_analysis.data),
            &signal_names,
            ComparisonOptions {
                alignment: draft.alignment,
                alignment_signal: &draft.signal,
                threshold: numbers.threshold,
                maximum_lag_samples: numbers.maximum_lag,
                absolute_tolerance: numbers.absolute_tolerance,
                relative_tolerance: numbers.relative_tolerance,
            },
            &mut |_, _| true,
        )?
        .receipt;
        let outcome = outcome(
            state.results_reads(),
            analysis,
            baseline,
            &receipt,
            &signal_names,
            &numbers,
        );
        let edits = match active_document_id(state.results_reads()) {
            Some(document_id) => {
                let document = project_document(state.results_reads(), document_id)?;
                let mut edits = Vec::new();
                for (source_run, source_analysis) in
                    [(baseline, baseline_analysis), (run, analysis)]
                {
                    let source = super::super::create_document::source_reference(
                        source_run,
                        source_analysis,
                    )
                    .map_err(|error| error.to_string())?;
                    let attached = document
                        .datasets()
                        .iter()
                        .any(|dataset| dataset.binding().dataset_id == source_run.dataset_id);
                    edits.push(if attached {
                        DocumentEdit::MergeDatasetProjection(source)
                    } else {
                        DocumentEdit::AttachDataset(source)
                    });
                }
                edits.push(DocumentEdit::RecordComparison(receipt));
                Some((document_id, edits))
            }
            None => None,
        };
        (outcome, edits)
    };
    if let Some((document_id, edits)) = edits {
        transact(state, document_id, edits)?;
    }
    draft.outcome = Some(outcome);
    Ok(Some(ResultDocumentDialog::Compare(draft)))
}

/// The receipt as the form shows it: each signal by name, its worst error in
/// the signal's own unit, and its verdict.
fn outcome(
    state: ResultsReads<'_>,
    analysis: &AnalysisResult,
    baseline: &SimulationRun,
    receipt: &ComparisonReceipt,
    signal_names: &[String],
    numbers: &CompareNumbers,
) -> CompareOutcome {
    let digits = usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let signals = receipt
        .signals
        .iter()
        .map(|signal| {
            // The receipt keys signals by their place in the compared list.
            let name = signal
                .signal_key
                .strip_prefix("signal:")
                .and_then(|index| index.parse::<usize>().ok())
                .and_then(|index| signal_names.get(index))
                .map_or("Unnamed signal", String::as_str);
            let unit = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)
                .map_or("", |waveform| {
                    super::super::browser_signal_unit(
                        name,
                        waveform.unit.as_deref(),
                        super::super::analysis_default_unit(analysis.analysis_type),
                    )
                });
            SignalOutcome {
                name: name.to_owned(),
                compared_rows: signal.compared_rows,
                failed_rows: signal.failed_rows,
                maximum_error: fmt_si_significant(signal.maximum_absolute_error, unit, digits),
                verdict: Verdict::of(
                    signal.failed_rows,
                    signal.maximum_absolute_error,
                    signal.maximum_relative_error,
                    numbers.absolute_tolerance,
                    numbers.relative_tolerance,
                ),
            }
        })
        .collect();
    CompareOutcome {
        baseline: run_label(baseline),
        rows_compared: receipt.rows_compared,
        signals,
    }
}

#[cfg(test)]
mod tests;
