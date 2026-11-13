//! Add a scalar measurement to the pane the reader is working in.
//!
//! The expression is evaluated on the selected pane's bound analysis and
//! kept with the document as `AddScalarMeasurement`, referencing the pane's
//! traces it reads. Over a quick view the form still evaluates, so the value
//! can be read, but nothing is kept: a quick view has no document to keep it.
//!
//! When the pane shows a family — a sweep, corners, Monte Carlo trials, or
//! points of one analysis solved one by one — the expression is evaluated
//! once per member instead, as ADE Assembler evaluates an output per point,
//! and the document keeps the distribution (`AddFamilyMeasurement`): the
//! statistic the reader picks as its value, with mean, sigma, min, max and a
//! yield against the reader's limits naming the failing members.

use rspice_results::family_measurements::{self, FamilyMeasurement};
use rspice_results::number_format::fmt_si_significant;
use rspice_results::visualization_document::{FamilyMeasurementSummary, FamilyStatistic};
use rspice_results_ui::document_dialogs::measurement::{
    FamilyPreview, MeasurementDraft, MeasurementPreview,
};

use super::*;
use crate::analysis::calculator;
use crate::results::visualization_document::TraceId;
use crate::state::{AnalysisResultFamilyMetadata, RetainedTrace};

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
                family: None,
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
    let family = family_of(run, analysis);
    let format = |value: f64| fmt_si_significant(value, "", digits);
    let (value, family) = match family {
        Some(noun) => {
            let measured = (!expression.is_empty())
                .then(|| measure_family(run, analysis, expression))
                .transpose();
            let summary = measured
                .as_ref()
                .ok()
                .and_then(Option::as_ref)
                .map(|measured| summarize(measured, draft));
            let value = match (&measured, summary) {
                (Ok(None), _) => Ok(None),
                (Err(reason), _) => Err(reason.clone()),
                (Ok(Some(_)), Some(Ok(summary))) => Ok(Some(kept_line(&summary, format))),
                (Ok(Some(_)), Some(Err(reason))) => Err(reason),
                (Ok(Some(_)), None) => Ok(None),
            };
            let preview = FamilyPreview {
                family: noun,
                statistics: measured
                    .clone()
                    .map(|measured| measured.map(|measured| statistics_line(&measured, format))),
                judged: match (&measured, limits(draft)) {
                    (_, Ok((None, None))) => None,
                    (_, Err(reason)) => Some(Err(reason)),
                    (Ok(Some(measured)), Ok((lower, upper))) => measured
                        .yield_against(lower, upper)
                        .map(|judged| Ok(judged_line(measured, &judged))),
                    (_, Ok(_)) => None,
                },
            };
            (value, Some(preview))
        }
        None => (
            if expression.is_empty() {
                Ok(None)
            } else {
                evaluate(analysis, expression).map(|value| Some(format(value)))
            },
            None,
        ),
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
        family,
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
    let (run, analysis) = reading_analysis(state.results_reads())?;
    let family = match family_of(run, analysis) {
        Some(_) => Some(
            measure_family(run, analysis, &expression)
                .and_then(|measured| summarize(&measured, draft))?,
        ),
        None => None,
    };
    let value = match &family {
        Some(summary) => summary
            .statistic_value()
            .ok_or_else(|| "The family has no value for that statistic.".to_owned())?,
        None => evaluate(analysis, &expression)?,
    };
    let (_, pane_id) =
        super::super::persistent_document::reading_position(state.results_reads(), document_id)
            .ok_or_else(|| "The result document has no pane.".to_owned())?;
    let trace_ids = pane_traces(
        project_document(state.results_reads(), document_id)?,
        pane_id,
        &expression,
    )?;
    let edit = match family {
        Some(family) => DocumentEdit::AddFamilyMeasurement {
            pane_id,
            trace_ids,
            expression,
            value,
            family,
        },
        None => DocumentEdit::AddScalarMeasurement {
            pane_id,
            trace_ids,
            expression,
            value,
        },
    };
    transact(state, document_id, vec![edit])?;
    Ok(None)
}

/// What family `analysis` belongs to, named with its member count, or
/// `None` when it belongs to none.
fn family_of(run: &SimulationRun, analysis: &AnalysisResult) -> Option<String> {
    let members = family_members(run, analysis)?.len();
    let noun = match analysis.family_metadata.as_ref() {
        Some(AnalysisResultFamilyMetadata::MonteCarlo { .. }) => "Monte Carlo trials",
        Some(AnalysisResultFamilyMetadata::Parametric { .. }) => "sweep points",
        Some(AnalysisResultFamilyMetadata::Corner { .. }) => "corners",
        _ => "solved points",
    };
    Some(format!("{members} {noun}, each measured on its own"))
}

fn family_members<'a>(
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
) -> Option<Vec<family_measurements::FamilyMember<'a, RetainedTrace>>> {
    family_measurements::family_members(
        analysis,
        run.analyses.iter().map(
            |sibling| -> &rspice_results::analysis_result::AnalysisResult<RetainedTrace> {
                sibling
            },
        ),
    )
}

/// `expression` evaluated once on every member of the family `analysis`
/// belongs to.
fn measure_family(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    expression: &str,
) -> Result<FamilyMeasurement, String> {
    let parsed = calculator::parser::try_parse(expression)
        .map_err(|error| format!("Parse error: {error}"))?;
    let members = family_members(run, analysis)
        .ok_or_else(|| "The pane no longer shows a family.".to_owned())?;
    let measured = family_measurements::evaluate_members(&parsed, &members);
    if measured.values().is_empty() {
        // Every member failed the same way more often than not; say why
        // the first one did.
        let reason = measured
            .members
            .iter()
            .find_map(|member| member.value.as_ref().err())
            .cloned()
            .unwrap_or_else(|| "The family has no members.".to_owned());
        return Err(format!("No member could be measured: {reason}"));
    }
    Ok(measured)
}

/// The lower and upper limit as typed; an empty field is no limit.
fn limits(draft: &MeasurementDraft) -> Result<(Option<f64>, Option<f64>), String> {
    let parse = |label: &str, text: &str| {
        let text = text.trim();
        if text.is_empty() {
            return Ok(None);
        }
        rspice_results::spice_value::parse_spice_value_checked(text)
            .ok()
            .filter(|value| value.is_finite())
            .map(Some)
            .ok_or_else(|| format!("The {label} limit is not a number."))
    };
    let lower = parse("lower", &draft.lower)?;
    let upper = parse("upper", &draft.upper)?;
    if let (Some(lower), Some(upper)) = (lower, upper)
        && lower > upper
    {
        return Err("The lower limit is above the upper limit.".to_owned());
    }
    Ok((lower, upper))
}

/// What the document keeps for `measured` under the draft's statistic and
/// limits.
fn summarize(
    measured: &FamilyMeasurement,
    draft: &MeasurementDraft,
) -> Result<FamilyMeasurementSummary, String> {
    let statistics = measured
        .statistics()
        .ok_or_else(|| "No member could be measured.".to_owned())?;
    let (lower, upper) = limits(draft)?;
    let judged = measured.yield_against(lower, upper);
    if draft.statistic == FamilyStatistic::Yield && judged.is_none() {
        return Err("A yield needs a lower limit, an upper limit or both.".to_owned());
    }
    if draft.statistic == FamilyStatistic::StdDev && statistics.std_dev.is_none() {
        return Err("A standard deviation needs two measured members.".to_owned());
    }
    let count = |count: usize| u32::try_from(count).map_err(|_| "The family is too large.");
    Ok(FamilyMeasurementSummary {
        statistic: draft.statistic,
        members: count(measured.members.len())?,
        measured: count(statistics.count)?,
        mean: statistics.mean,
        std_dev: statistics.std_dev,
        min: statistics.min,
        max: statistics.max,
        lower,
        upper,
        passed: judged
            .as_ref()
            .map(|judged| count(judged.passed))
            .transpose()?,
        failing: judged
            .map(|judged| {
                judged
                    .failing
                    .iter()
                    .map(|position| measured.members[*position].label.clone())
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// The members' distribution in one line.
fn statistics_line(measured: &FamilyMeasurement, format: impl Fn(f64) -> String) -> String {
    let Some(statistics) = measured.statistics() else {
        return "No member could be measured.".to_owned();
    };
    let count = if statistics.unmeasured == 0 {
        format!("n {}", statistics.count)
    } else {
        format!(
            "n {} of {} ({} not measured)",
            statistics.count,
            measured.members.len(),
            statistics.unmeasured
        )
    };
    format!(
        "{count} \u{b7} mean {} \u{b7} \u{3c3} {} \u{b7} min {} \u{b7} max {}",
        format(statistics.mean),
        statistics
            .std_dev
            .map_or_else(|| "\u{2014}".to_owned(), &format),
        format(statistics.min),
        format(statistics.max),
    )
}

/// The yield against the limits, and the members that fail it.
fn judged_line(
    measured: &FamilyMeasurement,
    judged: &rspice_results::family_measurements::FamilyYield,
) -> String {
    let share = judged.percent();
    let share = if share.fract() == 0.0 {
        format!("{share:.0} %")
    } else {
        format!("{share:.1} %")
    };
    let mut line = format!("{} of {} pass ({share})", judged.passed, judged.total);
    if !judged.failing.is_empty() {
        const NAMED: usize = 4;
        let names: Vec<&str> = judged
            .failing
            .iter()
            .take(NAMED)
            .map(|position| measured.members[*position].label.as_str())
            .collect();
        line.push_str(&format!(" \u{b7} fails {}", names.join(", ")));
        if judged.failing.len() > NAMED {
            line.push_str(&format!(" +{}", judged.failing.len() - NAMED));
        }
    }
    line
}

/// What Add keeps, in words.
fn kept_line(summary: &FamilyMeasurementSummary, format: impl Fn(f64) -> String) -> String {
    let value = summary.statistic_value().map_or_else(
        || "\u{2014}".to_owned(),
        |value| match summary.statistic {
            FamilyStatistic::Yield if value.fract() == 0.0 => format!("{value:.0} %"),
            FamilyStatistic::Yield => format!("{value:.1} %"),
            _ => format(value),
        },
    );
    let name = match summary.statistic {
        FamilyStatistic::Mean => "the mean",
        FamilyStatistic::StdDev => "the standard deviation",
        FamilyStatistic::Min => "the minimum",
        FamilyStatistic::Max => "the maximum",
        FamilyStatistic::Yield => "the yield",
    };
    format!("Keeps {name}: {value}")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisType, FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use crate::workbench::state::Workspace;

    /// Five Monte Carlo trials whose `gain_dc` is 1..5 dB, the third with no
    /// value.
    fn host() -> ResultsTestHost {
        let members = [Some(1.0), Some(2.0), None, Some(4.0), Some(5.0)]
            .into_iter()
            .enumerate()
            .map(|(index, gain)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 40 + index as u64,
                    },
                    vec![FamilyMeasurementEvidence {
                        unit: None,
                        name: "gain_dc".to_owned(),
                        value: gain,
                        passed: gain.is_some(),
                        error: gain.is_none().then(|| "no crossing".to_owned()),
                    }],
                )
            })
            .collect();
        let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 40,
                runs_requested: 5,
                runs_completed: 5,
                failures: 0,
                all_converged: true,
                variables: Vec::new(),
                member_measurements: members,
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut host = ResultsTestHost::default();
        host.model.simulation.retained.runs = vec![run].into();
        assert!(host.model.simulation.select_run(0));
        assert!(host.model.simulation.select_analysis(0));
        host.workbench.workspace = Workspace::Results;
        host
    }

    /// Over a Monte Carlo population the form measures every trial on its own
    /// and states the distribution, the yield and who fails it; the trial the
    /// expression could not measure is counted, named and failed.
    #[test]
    fn a_monte_carlo_pane_measures_each_trial_and_judges_the_yield() {
        let host = host();
        let mut draft = MeasurementDraft {
            expression: "gain_dc".to_owned(),
            statistic: FamilyStatistic::Yield,
            lower: "1.5".to_owned(),
            ..MeasurementDraft::default()
        };
        let preview = preview(host.results_reads(), &draft);
        let family = preview.family.expect("a Monte Carlo pane is a family");
        assert_eq!(
            family.family,
            "5 Monte Carlo trials, each measured on its own"
        );
        // {1, 2, 4, 5}: mean 3, sample sigma sqrt(10 / 3) = 1.826.
        let statistics = family.statistics.unwrap().unwrap();
        assert!(
            statistics.starts_with("n 4 of 5 (1 not measured) · mean 3.000000 · σ 1.825742"),
            "{statistics}"
        );
        assert_eq!(
            family.judged.unwrap().unwrap(),
            "3 of 5 pass (60 %) · fails Trial 0 · seed 40, Trial 2 · seed 42"
        );
        assert_eq!(preview.value.unwrap().unwrap(), "Keeps the yield: 60 %");
        // A quick view keeps nothing, family or not.
        assert!(preview.refusal.is_some());

        // A yield with no limit keeps nothing; a mean needs none.
        draft.lower.clear();
        let preview = super::preview(host.results_reads(), &draft);
        assert!(preview.value.is_err());
        assert!(preview.family.unwrap().judged.is_none());
        draft.statistic = FamilyStatistic::Mean;
        let preview = super::preview(host.results_reads(), &draft);
        assert_eq!(preview.value.unwrap().unwrap(), "Keeps the mean: 3.000000");

        draft.upper = "lots".to_owned();
        let judged = super::preview(host.results_reads(), &draft)
            .family
            .unwrap()
            .judged;
        assert_eq!(
            judged,
            Some(Err("The upper limit is not a number.".to_owned()))
        );
    }

    /// The summary the document keeps is the one the form showed.
    #[test]
    fn the_kept_summary_is_the_statistic_the_reader_picked() {
        let host = host();
        let reads = host.results_reads();
        let (run, analysis) = reading_analysis(reads).unwrap();
        let measured = measure_family(run, analysis, "gain_dc").unwrap();
        let draft = MeasurementDraft {
            statistic: FamilyStatistic::StdDev,
            lower: "1.5".to_owned(),
            upper: "4.5".to_owned(),
            ..MeasurementDraft::default()
        };
        let summary = summarize(&measured, &draft).unwrap();
        assert_eq!((summary.members, summary.measured), (5, 4));
        assert_eq!(summary.passed, Some(2));
        assert_eq!(
            summary.failing,
            [
                "Trial 0 · seed 40",
                "Trial 2 · seed 42",
                "Trial 4 · seed 44"
            ]
        );
        let sigma = summary.statistic_value().unwrap();
        assert!((sigma - (10.0_f64 / 3.0).sqrt()).abs() < 1e-12);
    }
}
