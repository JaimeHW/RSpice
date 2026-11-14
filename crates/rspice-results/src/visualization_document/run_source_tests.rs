//! A document that references its runs: it holds each run's signals and no
//! samples, provisions panes from them, re-points at a run with the same
//! signals, embeds a run only when archived, and carries a figure's samples
//! only in a report's copy.

use super::*;
use crate::analysis_type::AnalysisType;

fn binding(seed: u8) -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([seed; 32]))
}

fn signals(names: &[&str]) -> Vec<RunSignal> {
    names
        .iter()
        .enumerate()
        .map(|(index, name)| RunSignal::new(u32::try_from(index).unwrap(), *name, false))
        .collect()
}

fn reference(
    binding: DatasetBinding,
    analysis: AnalysisInstanceId,
    names: &[&str],
) -> SourceDataset {
    SourceDataset::run_reference(
        binding,
        "Run 1 (9:00:00 AM)",
        vec![RunSourceAnalysis::new(
            analysis,
            AnalysisType::Transient,
            signals(names),
        )],
    )
    .expect("a valid run reference")
}

fn waveform_pane(binding: DatasetBinding, analysis: AnalysisInstanceId) -> NewPagePane {
    NewPagePane {
        title: "Transient".to_owned(),
        kind: PaneKind::Cartesian,
        viewer_id: "viewer-waveform".to_owned(),
        binding: Some(PaneDataBinding {
            analysis_id: analysis,
            dataset: binding,
        }),
    }
}

fn twenty_names() -> Vec<String> {
    (0..20).map(|index| format!("V(n{index})")).collect()
}

fn referencing_document() -> (VisualizationDocument, DatasetBinding, AnalysisInstanceId) {
    let source = binding(1);
    let analysis = AnalysisInstanceId::new();
    let names = twenty_names();
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    let document = VisualizationDocument::new_with_initial_pane(
        "Twenty signals",
        vec![reference(source, analysis, &names)],
        waveform_pane(source, analysis),
    )
    .expect("a document over a run reference");
    (document, source, analysis)
}

#[test]
fn a_run_reference_provisions_every_signal_and_saves_without_samples() {
    let (document, source, analysis) = referencing_document();
    assert_eq!(document.traces().len(), 20, "one trace per signal");
    let first = &document.traces()[0];
    assert_eq!(first.binding, source);
    assert_eq!(first.source_signal(), Some("V(n0)"));
    assert_eq!(first.source_analysis(), Some(analysis));
    assert!(!document.is_archived());

    let json = serde_json::to_string(&document).unwrap();
    assert!(
        json.len() < 32 * 1024,
        "a referencing document holds no samples: {} bytes",
        json.len()
    );
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(
        value["schema_version"],
        serde_json::json!(VisualizationDocument::SCHEMA_VERSION)
    );
    assert!(value["datasets"][0].get("rows").is_none());
    let reopened: VisualizationDocument = serde_json::from_str(&json).unwrap();
    assert_eq!(reopened, document);
}

#[test]
fn a_run_reference_selects_whole_signals_never_samples() {
    let (mut document, source, _) = referencing_document();
    let pane_id = document.panes()[0].id;
    let x_axis_id = document.traces()[0].x_axis_id;
    let y_axis_id = document.traces()[0].y_axis_id;
    let trace = |predicates: Vec<QueryCoordinate>| {
        DocumentEdit::AddTrace(NewTrace {
            pane_id,
            binding: source,
            signal_key: "y".to_owned(),
            coordinate_key: "x".to_owned(),
            row_predicates: predicates,
            x_axis_id,
            y_axis_id,
            label: "Sample".to_owned(),
        })
    };
    let by_sample = document.transact(
        document.revision(),
        vec![trace(vec![QueryCoordinate {
            column: "x".to_owned(),
            value: TypedValue::Real(0.0),
        }])],
    );
    assert!(
        by_sample
            .unwrap_err()
            .to_string()
            .contains("needs the samples")
    );
    let absent = document.transact(
        document.revision(),
        vec![trace(vec![QueryCoordinate {
            column: TRACE_NAME_COLUMN.to_owned(),
            value: TypedValue::Text("V(absent)".to_owned()),
        }])],
    );
    assert!(absent.unwrap_err().to_string().contains("select no signal"));
}

#[test]
fn a_document_compares_any_number_of_run_references() {
    let (mut document, _, _) = referencing_document();
    for seed in 2..=40 {
        let edit = DocumentEdit::AttachDataset(reference(
            binding(seed),
            AnalysisInstanceId::new(),
            &["V(n0)"],
        ));
        document.transact(document.revision(), vec![edit]).unwrap();
    }
    assert_eq!(document.datasets().len(), 40);
}

#[test]
fn analyses_of_one_run_merge_into_its_reference_and_cannot_be_rewritten() {
    let (mut document, source, analysis) = referencing_document();
    let second = AnalysisInstanceId::new();
    let merge = |analysis, names: &[&str]| {
        DocumentEdit::MergeDatasetProjection(reference(source, analysis, names))
    };
    document
        .transact(document.revision(), vec![merge(second, &["I(R1)"])])
        .unwrap();
    let run = document.datasets()[0].run_source().unwrap();
    assert_eq!(run.analyses().len(), 2);
    assert!(
        document
            .transact(document.revision(), vec![merge(analysis, &["V(other)"])])
            .unwrap_err()
            .to_string()
            .contains("cannot be rewritten")
    );
}

#[test]
fn repointing_moves_every_pane_and_trace_onto_the_same_signals_of_another_run() {
    let (mut document, previous, analysis) = referencing_document();
    let next = binding(9);
    let next_analysis = AnalysisInstanceId::new();
    // The other run lists its signals in another order.
    let mut names = twenty_names();
    names.reverse();
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    let repoint = |names: &[&str]| DocumentEdit::RepointDataset {
        previous,
        next: reference(next, next_analysis, names),
        analyses: vec![AnalysisRepoint {
            previous: analysis,
            next: next_analysis,
        }],
    };

    let missing = document.transact(document.revision(), vec![repoint(&names[1..])]);
    assert!(
        missing.unwrap_err().to_string().contains("V(n19)"),
        "a run without one of the signals is refused by name"
    );
    assert_eq!(document.datasets()[0].binding(), previous, "nothing moved");

    document
        .transact(document.revision(), vec![repoint(&names)])
        .unwrap();
    assert_eq!(document.datasets().len(), 1, "the missing run is let go");
    assert_eq!(document.datasets()[0].binding(), next);
    let pane = document.panes()[0].binding.unwrap();
    assert_eq!((pane.dataset, pane.analysis_id), (next, next_analysis));
    let first = &document.traces()[0];
    assert_eq!(first.binding, next);
    assert_eq!(first.source_signal(), Some("V(n0)"));
    assert_eq!(first.source_analysis(), Some(next_analysis));
    assert!(first.row_predicates.contains(&QueryCoordinate {
        column: "trace-index".to_owned(),
        value: TypedValue::Integer(19),
    }));
}

#[test]
fn an_archive_embeds_the_run_once_and_reads_back() {
    let (mut document, source, _) = referencing_document();
    let archive = || DocumentEdit::ArchiveDataset {
        binding: source,
        archive: RunArchive::new("test-run", "{\"run\":1}").unwrap(),
    };
    document
        .transact(document.revision(), vec![archive()])
        .unwrap();
    assert!(document.is_archived());
    assert!(
        document
            .transact(document.revision(), vec![archive()])
            .unwrap_err()
            .to_string()
            .contains("already holds")
    );
    let reopened: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    let archived = reopened.datasets()[0]
        .run_source()
        .unwrap()
        .archive()
        .unwrap();
    assert_eq!(archived.record(), "{\"run\":1}");
}

struct Samples {
    binding: DatasetBinding,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl SourceSamples for Samples {
    fn series_samples(
        &self,
        binding: DatasetBinding,
        series: SeriesKey,
    ) -> Option<(&[f64], &[f64])> {
        (binding == self.binding && series.component == SignalComponent::Display)
            .then_some((&self.x, &self.y))
    }
}

#[test]
fn a_report_copy_carries_exactly_the_samples_its_traces_draw() {
    let (document, source, _) = referencing_document();
    let samples = Samples {
        binding: source,
        x: vec![0.0, 1.0e-9, 2.0e-9],
        y: vec![0.0, f64::NAN, 1.0],
    };
    let copy = document.with_figure_samples(&samples).unwrap();
    assert_eq!(copy.id(), document.id());
    assert_eq!(copy.revision(), document.revision());
    assert_eq!(
        copy.content_digest().unwrap(),
        document.content_digest().unwrap(),
        "the copy authenticates as the document it copies"
    );
    let run = copy.datasets()[0].run_source().unwrap();
    let series = run.select_one(&copy.traces()[3].row_predicates).unwrap();
    let carried = run.figure_samples(series).unwrap();
    assert_eq!(carried.x(), samples.x.as_slice());
    assert!(carried.y()[1].is_nan(), "an unavailable sample stays one");
    let reopened: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&copy).unwrap()).unwrap();
    assert_eq!(
        reopened.datasets()[0]
            .run_source()
            .unwrap()
            .figure_samples(series)
            .unwrap()
            .x(),
        samples.x.as_slice()
    );
    assert!(
        document.datasets()[0]
            .run_source()
            .unwrap()
            .figure_samples(series)
            .is_none()
    );

    let elsewhere = Samples {
        binding: binding(5),
        ..samples
    };
    assert!(
        document
            .with_figure_samples(&elsewhere)
            .unwrap_err()
            .to_string()
            .contains("not retained")
    );
}
