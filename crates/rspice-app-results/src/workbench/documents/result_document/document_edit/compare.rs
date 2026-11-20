//! Compare the result the reader is looking at with a baseline dataset.
//!
//! The candidate is the selected pane's binding over a project result
//! document, or the active analysis over a quick view. The comparison reads
//! the signals the reader chose, the plotted ones unless they chose others,
//! from wherever each run keeps them, on a worker of its own while the form
//! shows how far it has got. Over a document the receipt is recorded with it,
//! both datasets attached in the same transaction by reference; over a quick
//! view the outcome is shown and not kept.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rspice_results::number_format::fmt_si_significant;
use rspice_results::waveform_comparison::{
    ComparisonAlignmentDraft, ComparisonOutcome, comparison_signal_names,
    matching_comparison_analysis,
};
use rspice_results_ui::document_dialogs::compare::{
    CompareChoices, CompareDraft, CompareOutcome, SignalOutcome, Verdict,
};

use super::super::{AnalysisPresentationKey, SourceWaveformPresentationKey};
use super::*;
use crate::product::DatasetId;
use crate::results::visualization_document::ComparisonAlignmentMethod;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

mod difference;
mod worker;

pub(super) use difference::plot_difference;
pub(crate) use difference::{compared, comparison_marks, step_difference};

/// A run the candidate can be compared with, and the signals the two share.
#[derive(Debug, Clone)]
pub(crate) struct Baseline {
    dataset_id: DatasetId,
    signals: Arc<[String]>,
}

/// The retained runs that hold the candidate's analysis on the same X axis,
/// each with the signals the two share.
///
/// The form asks on every frame, so the answer is kept per candidate until
/// the retained runs change. It reads no sample: the signals are matched by
/// name, unit and axis.
fn baselines(
    state: ResultsReads<'_>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Arc<[Baseline]> {
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    state
        .results
        .compare_baselines
        .get_or_insert_with(&state.model.simulation, key, || {
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
                    Some(Baseline {
                        dataset_id: other.dataset_id,
                        signals: signals.into(),
                    })
                })
                .collect()
        })
}

fn retained_run(state: ResultsReads<'_>, dataset_id: DatasetId) -> Option<&SimulationRun> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
}

/// A run as the form names it: by its number, and its label unless the label
/// already starts with that number.
fn run_label(run: &SimulationRun) -> String {
    let name = format!("Run {}", run.id);
    if run
        .label
        .strip_prefix(&name)
        .is_some_and(|rest| !rest.starts_with(|next: char| next.is_ascii_digit()))
    {
        run.label.clone()
    } else {
        format!("{name} · {}", run.label)
    }
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
/// same unit, with a sample for every coordinate. No sample is read.
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

/// The signals of `shared` the reader has plotted, or every one of them when
/// none is plotted.
fn plotted(
    state: ResultsReads<'_>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
    shared: &[String],
) -> Vec<String> {
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let plotted = shared
        .iter()
        .filter(|name| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| &waveform.name == *name)
                .is_some_and(|waveform| {
                    state.results.session.waveform_visibility(
                        &SourceWaveformPresentationKey::new(key, name.as_str()),
                        waveform.visible,
                    )
                })
        })
        .cloned()
        .collect::<Vec<_>>();
    if plotted.is_empty() {
        shared.to_vec()
    } else {
        plotted
    }
}

/// Open Compare datasets on the result the reader is looking at, against the
/// first compatible baseline, with the signals the reader has plotted chosen.
pub fn open_compare(state: &mut ResultsParts<'_>) {
    // Compare datasets is disabled while either is missing.
    if !compare_available(state.results_reads()) {
        return;
    }
    let reads = state.results_reads();
    let (baseline, signals) = reading_analysis(reads)
        .ok()
        .and_then(|(run, analysis)| {
            let baselines = baselines(reads, run, analysis);
            let first = baselines.first()?;
            Some((
                Some(first.dataset_id),
                plotted(reads, run, analysis, &first.signals),
            ))
        })
        .unwrap_or_default();
    // It is offered from the runs' names and axes alone; a baseline whose
    // samples turn out not to compare refuses when the comparison runs.
    *state.result_document_dialog = Some(ResultDocumentDialog::Compare(CompareDraft {
        baseline,
        signals,
        ..CompareDraft::default()
    }));
}

const NO_BASELINE: &str = "No other retained dataset holds this analysis on the same X axis.";
const NO_SIGNAL: &str = "Choose at least one signal to compare.";

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

/// The chosen signals the baseline shares, in the order the runs hold them.
fn chosen(draft: &CompareDraft, shared: &[String]) -> Vec<String> {
    shared
        .iter()
        .filter(|signal| draft.signals.contains(signal))
        .cloned()
        .collect()
}

/// Resolve the form's choices against the retained datasets, moving a
/// baseline or alignment signal that no longer exists onto the first one that
/// does. It reads no sample.
pub fn choices(state: ResultsReads<'_>, draft: &mut CompareDraft) -> CompareChoices {
    let (run, analysis) = match reading_analysis(state) {
        Ok(found) => found,
        Err(refusal) => {
            return CompareChoices {
                candidate: String::new(),
                baselines: Vec::new(),
                signals: Vec::new(),
                refusal: Some(refusal),
                running: None,
            };
        }
    };
    let baselines = baselines(state, run, analysis);
    let selected = draft
        .baseline
        .and_then(|id| baselines.iter().find(|baseline| baseline.dataset_id == id))
        .or_else(|| baselines.first());
    draft.baseline = selected.map(|baseline| baseline.dataset_id);
    let signals = selected.map_or_else(Vec::new, |baseline| baseline.signals.to_vec());
    if !signals.contains(&draft.signal) {
        draft.signal = signals.first().cloned().unwrap_or_default();
    }
    let refusal = if baselines.is_empty() {
        Some(NO_BASELINE.to_owned())
    } else if chosen(draft, &signals).is_empty() {
        Some(NO_SIGNAL.to_owned())
    } else {
        numbers(draft).err()
    };
    CompareChoices {
        candidate: format!("{} · {}", run_label(run), analysis.label),
        baselines: baselines
            .iter()
            .filter_map(|baseline| {
                let run = retained_run(state, baseline.dataset_id)?;
                Some((baseline.dataset_id, run_label(run)))
            })
            .collect(),
        signals,
        refusal,
        running: None,
    }
}

/// What a running comparison was asked, to record its answer against.
#[derive(Debug)]
struct Request {
    /// Each run by its dataset and the analysis by its id in the run.
    candidate: (DatasetId, u64),
    baseline: (DatasetId, u64),
    /// The document the comparison is recorded with; `None` over a quick view.
    document: Option<ResultDocumentId>,
    alignment_signal: String,
    absolute_tolerance: f64,
    relative_tolerance: f64,
}

/// A comparison on its worker.
#[derive(Debug)]
pub(crate) struct Running {
    worker: worker::Worker,
    request: Request,
}

/// A difference's tolerance band, `None` where none can be drawn, with the
/// evaluation of the difference it was read from.
type DrawnBand = (
    u64,
    Option<Arc<rspice_results_ui::waves::pane::ToleranceBand>>,
);

/// The comparison that last answered, as its report states it.
#[derive(Debug)]
pub(crate) struct Finished {
    /// The compared runs, and the candidate's analysis, whose strip draws
    /// the differences and is stepped through them.
    candidate_id: DatasetId,
    baseline_id: DatasetId,
    analysis: AnalysisPresentationKey,
    absolute_tolerance: f64,
    relative_tolerance: f64,
    /// Each drawn difference's tolerance band, by the difference's text,
    /// with the evaluation it was read from.
    bands: Mutex<HashMap<String, DrawnBand>>,
    /// The X span of each place, a list per compared signal.
    place_spans: Vec<Arc<[(f64, f64)]>>,
    candidate: String,
    baseline: String,
    alignment: String,
    tolerance: String,
    /// The compared signals' units, as the outcome writes them.
    units: Vec<String>,
    x_unit: String,
    digits: usize,
    shown: CompareOutcome,
    outcome: ComparisonOutcome,
}

/// Start the comparison the form asks for. The form shows its progress until
/// it answers; once the outcome shows, `Done` closes.
pub(super) fn commit(
    state: &mut ResultsParts<'_>,
    draft: CompareDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    if draft.outcome.is_some() {
        return Ok(None);
    }
    if state.results.compare_run.is_some() {
        return Ok(Some(ResultDocumentDialog::Compare(draft)));
    }
    let numbers = numbers(&draft)?;
    let (job, request) = {
        let reads = state.results_reads();
        let (run, analysis) = reading_analysis(reads)?;
        let baseline = draft
            .baseline
            .and_then(|id| retained_run(reads, id))
            .ok_or_else(|| "The baseline dataset is no longer retained.".to_owned())?;
        let baseline_analysis = matching_comparison_analysis(analysis, baseline)
            .ok_or_else(|| "The baseline holds no matching analysis.".to_owned())?;
        let shared = baselines(reads, run, analysis)
            .iter()
            .find(|candidate| candidate.dataset_id == baseline.dataset_id)
            .map(|baseline| Arc::clone(&baseline.signals))
            .ok_or_else(|| NO_BASELINE.to_owned())?;
        let signals = chosen(&draft, &shared);
        if signals.is_empty() {
            return Err(NO_SIGNAL.to_owned());
        }
        // The worker owns the waveforms it reads, and nothing else of either
        // run: each shares its samples with the run, so none is copied.
        let read = signals
            .iter()
            .map(String::as_str)
            .chain(
                (draft.alignment != ComparisonAlignmentDraft::AbsoluteXAxis)
                    .then_some(draft.signal.as_str()),
            )
            .collect::<HashSet<_>>();
        let side = |run: &SimulationRun, analysis: &AnalysisResult| worker::Side {
            binding: DatasetBinding::new(
                run.dataset_id,
                super::super::retained_dataset_digest(reads, run),
            ),
            analysis: analysis.data.picked_waveforms(|waveform| {
                read.contains(waveform.name.as_str())
                    .then(|| waveform.data.clone())
            }),
        };
        let job = worker::Job {
            candidate: side(run, analysis),
            baseline: side(baseline, baseline_analysis),
            signals,
            alignment: draft.alignment,
            alignment_signal: draft.signal.clone(),
            threshold: numbers.threshold,
            maximum_lag_samples: numbers.maximum_lag,
            absolute_tolerance: numbers.absolute_tolerance,
            relative_tolerance: numbers.relative_tolerance,
        };
        let request = Request {
            candidate: (run.dataset_id, analysis.id),
            baseline: (baseline.dataset_id, baseline_analysis.id),
            document: active_document_id(reads),
            alignment_signal: draft.signal.clone(),
            absolute_tolerance: numbers.absolute_tolerance,
            relative_tolerance: numbers.relative_tolerance,
        };
        (job, request)
    };
    state.results.compare_run = Some(Arc::new(Running {
        worker: worker::Worker::start(job),
        request,
    }));
    Ok(Some(ResultDocumentDialog::Compare(draft)))
}

/// Collect a running comparison's answer into the open form, and say how far
/// one still running has got. A comparison whose form has closed is stopped.
pub(super) fn poll(state: &mut ResultsParts<'_>) -> Option<(usize, usize)> {
    let running = state.results.compare_run.clone()?;
    if !matches!(
        state.result_document_dialog,
        Some(ResultDocumentDialog::Compare(_))
    ) {
        state.results.compare_run = None;
        return None;
    }
    match running.worker.poll() {
        worker::Poll::Running(done, total) => Some((done, total)),
        worker::Poll::Done(answer) => {
            state.results.compare_run = None;
            answered(state, &running.request, answer);
            None
        }
    }
}

/// Stop a running comparison; the form stays open on its draft.
pub(super) fn stop(state: &mut ResultsParts<'_>) {
    if let Some(running) = state.results.compare_run.take() {
        running.worker.stop();
    }
}

/// Whether a comparison is running.
pub(super) fn running(state: ResultsReads<'_>) -> bool {
    state.results.compare_run.is_some()
}

/// The run `@baseline` reads over `candidate`: the baseline of the last
/// comparison recorded with the open result document, or, over a quick
/// view, of the comparison that last answered, while it is retained and is
/// not the candidate itself.
pub(crate) fn pinned_baseline(state: ResultsReads<'_>, candidate: DatasetId) -> Option<DatasetId> {
    let pinned = match active_document_id(state) {
        Some(document_id) => project_document(state, document_id)
            .ok()?
            .comparisons()
            .last()
            .map(|receipt| receipt.baseline.dataset_id),
        None => state
            .results
            .last_comparison
            .as_ref()
            .map(|finished| finished.baseline_id),
    }?;
    (pinned != candidate && retained_run(state, pinned).is_some()).then_some(pinned)
}

/// Wait for a running comparison and collect its answer into the open form.
#[cfg(any(test, feature = "test-support"))]
pub fn settle_comparison(state: &mut ResultsParts<'_>) {
    if let Some(running) = state.results.compare_run.take() {
        let answer = running.worker.wait();
        answered(state, &running.request, answer);
    }
}

fn answered(state: &mut ResultsParts<'_>, request: &Request, answer: worker::Answer) {
    if let Err(error) = answer.and_then(|found| finish(state, request, *found)) {
        state.prompt(error);
    }
}

/// Record a comparison's answer: with the document it was asked over, in the
/// open form, and as the comparison a report writes out.
fn finish(
    state: &mut ResultsParts<'_>,
    request: &Request,
    found: ComparisonOutcome,
) -> Result<(), String> {
    let (finished, edits) = {
        let reads = state.results_reads();
        let resolve = |(dataset_id, analysis_id): (DatasetId, u64)| {
            let run = retained_run(reads, dataset_id)?;
            let analysis = run
                .analyses
                .iter()
                .find(|analysis| analysis.id == analysis_id)?;
            Some((run, analysis))
        };
        let gone = || "The compared results are no longer retained.".to_owned();
        let (run, analysis) = resolve(request.candidate).ok_or_else(gone)?;
        let (baseline, baseline_analysis) = resolve(request.baseline).ok_or_else(gone)?;
        let finished = finished(reads, (run, analysis), baseline, request, found);
        let edits = match request.document {
            Some(document_id) => {
                let document = project_document(reads, document_id)?;
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
                edits.push(DocumentEdit::RecordComparison(
                    finished.outcome.receipt.clone(),
                ));
                Some((document_id, edits))
            }
            None => None,
        };
        (finished, edits)
    };
    if let Some((document_id, edits)) = edits {
        transact(state, document_id, edits)?;
    }
    if let Some(ResultDocumentDialog::Compare(draft)) = state.result_document_dialog.as_mut() {
        draft.outcome = Some(finished.shown.clone());
    }
    state.results.last_comparison = Some(Arc::new(finished));
    Ok(())
}

/// The answer as the form and the report state it: each signal by name, its
/// worst error in the signal's own unit, and its verdict.
fn finished(
    state: ResultsReads<'_>,
    (run, analysis): (&SimulationRun, &AnalysisResult),
    baseline: &SimulationRun,
    request: &Request,
    outcome: ComparisonOutcome,
) -> Finished {
    let digits = usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let analysis_unit = super::super::analysis_default_unit(analysis.analysis_type);
    let units = outcome
        .signals
        .iter()
        .zip(&outcome.units)
        .map(|(name, unit)| {
            super::super::browser_signal_unit(name, unit.as_deref(), analysis_unit).to_owned()
        })
        .collect::<Vec<_>>();
    let signals = outcome
        .receipt
        .signals
        .iter()
        .zip(outcome.signals.iter().zip(&units))
        .map(|(signal, (name, unit))| SignalOutcome {
            name: name.clone(),
            compared_rows: signal.compared_rows,
            failed_rows: signal.failed_rows,
            maximum_error: fmt_si_significant(signal.maximum_absolute_error, unit, digits),
            verdict: Verdict::of(
                signal.failed_rows,
                signal.maximum_absolute_error,
                signal.maximum_relative_error,
                request.absolute_tolerance,
                request.relative_tolerance,
            ),
        })
        .collect();
    let x_unit = outcome.coordinate_unit.clone().unwrap_or_default();
    let x = |value: f64| fmt_si_significant(value, &x_unit, digits);
    let alignment = match &outcome.receipt.policy.execution.alignment {
        ComparisonAlignmentMethod::ExactCoordinateRows
        | ComparisonAlignmentMethod::AbsoluteXAxis => {
            "Aligned by X value: rows where both runs hold the same X.".to_owned()
        }
        ComparisonAlignmentMethod::FirstThresholdCrossing {
            threshold,
            baseline_crossing,
            candidate_crossing,
            ..
        } => format!(
            "Aligned where {} first crosses {}: at {} here and at {} in the baseline.",
            request.alignment_signal,
            fmt_si_significant(*threshold, "", digits),
            x(*candidate_crossing),
            x(*baseline_crossing),
        ),
        ComparisonAlignmentMethod::CrossCorrelation {
            coefficient,
            baseline_shift,
            ..
        } => format!(
            "Aligned by correlating {}: the baseline moved by {} (correlation {:.3}).",
            request.alignment_signal,
            x(*baseline_shift),
            coefficient,
        ),
    };
    let tolerance = format!(
        "Within {} absolute and {} relative to the baseline.",
        fmt_si_significant(request.absolute_tolerance, "", digits),
        fmt_si_significant(request.relative_tolerance, "", digits),
    );
    Finished {
        candidate_id: run.dataset_id,
        baseline_id: baseline.dataset_id,
        analysis: AnalysisPresentationKey::new(run.dataset_id, analysis),
        absolute_tolerance: request.absolute_tolerance,
        relative_tolerance: request.relative_tolerance,
        bands: Mutex::default(),
        place_spans: (0..outcome.signals.len())
            .map(|signal| {
                outcome
                    .places
                    .iter()
                    .filter(|place| place.signal == signal)
                    .map(|place| (place.first, place.last))
                    .collect()
            })
            .collect(),
        candidate: format!("{} · {}", run_label(run), analysis.label),
        baseline: run_label(baseline),
        alignment,
        tolerance,
        units,
        x_unit,
        digits,
        shown: CompareOutcome {
            baseline: run_label(baseline),
            rows_compared: outcome.receipt.rows_compared,
            signals,
        },
        outcome,
    }
}

/// The comparison written out as text: what was compared and how, each
/// signal's verdict, and every place the runs part by more than the
/// tolerance.
fn report(finished: &Finished) -> String {
    use std::fmt::Write as _;

    let mut text = String::new();
    let _ = writeln!(
        text,
        "Comparison of {} with {}",
        finished.candidate, finished.baseline
    );
    let _ = writeln!(text, "{}", finished.alignment);
    let _ = writeln!(text, "{}", finished.tolerance);
    let _ = writeln!(text, "Rows compared: {}", finished.shown.rows_compared);
    let _ = writeln!(text);
    let mut table = vec![[
        "Signal".to_owned(),
        "Rows outside".to_owned(),
        "Max error".to_owned(),
        "Verdict".to_owned(),
    ]];
    table.extend(finished.shown.signals.iter().map(|signal| {
        [
            signal.name.clone(),
            format!("{} of {}", signal.failed_rows, signal.compared_rows),
            signal.maximum_error.clone(),
            signal.verdict.label().to_owned(),
        ]
    }));
    write_table(&mut text, &table);
    let _ = writeln!(text);
    if finished.outcome.places.is_empty() {
        let _ = writeln!(
            text,
            "No place parts from the baseline by more than the tolerance."
        );
        return text;
    }
    let _ = writeln!(text, "Where the runs part by more than the tolerance");
    let x = |value: f64| fmt_si_significant(value, &finished.x_unit, finished.digits);
    let mut places = vec![[
        "Signal".to_owned(),
        "From".to_owned(),
        "To".to_owned(),
        "Worst at".to_owned(),
        "Difference".to_owned(),
    ]];
    places.extend(finished.outcome.places.iter().map(|place| {
        [
            finished.outcome.signals[place.signal].clone(),
            x(place.first),
            x(place.last),
            x(place.worst_x),
            fmt_si_significant(
                place.worst_error,
                &finished.units[place.signal],
                finished.digits,
            ),
        ]
    }));
    write_table(&mut text, &places);
    text
}

/// Rows of cells in columns as wide as their widest cell.
fn write_table<const N: usize>(text: &mut String, rows: &[[String; N]]) {
    let widths: [usize; N] = std::array::from_fn(|column| {
        rows.iter()
            .map(|row| row[column].chars().count())
            .max()
            .unwrap_or(0)
    });
    for row in rows {
        let mut line = String::new();
        for (column, cell) in row.iter().enumerate() {
            line.push_str(cell);
            if column + 1 < N {
                let pad = widths[column] - cell.chars().count() + 2;
                line.extend(std::iter::repeat_n(' ', pad));
            }
        }
        text.push_str(line.trim_end());
        text.push('\n');
    }
}

/// Write the comparison that last answered to a text file the reader names.
pub(crate) fn save_report_with_io(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let Some(finished) = state.results.last_comparison.clone() else {
        return;
    };
    let places = finished.outcome.places.len();
    let detail = format!(
        "{} signals, {places} {} outside tolerance",
        finished.outcome.signals.len(),
        if places == 1 { "place" } else { "places" },
    );
    crate::workbench::menu_bar::waveform_export::publish_result_text(
        state,
        io,
        "Comparison Report",
        "txt",
        "comparison.txt",
        &report(&finished),
        &detail,
    );
}

#[cfg(test)]
mod tests;
