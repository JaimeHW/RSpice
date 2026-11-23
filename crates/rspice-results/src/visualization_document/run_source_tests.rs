//! A document that references its runs: it names each run and the signals
//! its traces draw, and holds neither the run's samples nor its list of
//! signals; it re-points at another run by signal name, embeds a run only
//! when archived, and carries a figure's samples only in a report's copy.

use super::*;
use crate::analysis_type::AnalysisType;

fn binding(seed: u8) -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([seed; 32]))
}

fn reference(binding: DatasetBinding, analysis: AnalysisInstanceId) -> SourceDataset {
    SourceDataset::run_reference(
        binding,
        "Run 1 (9:00:00 AM)",
        vec![RunSourceAnalysis::new(analysis, AnalysisType::Transient)],
    )
    .expect("a valid run reference")
}

fn waveform_pane(
    binding: DatasetBinding,
    analysis: AnalysisInstanceId,
    plotted: Vec<String>,
) -> NewPagePane {
    NewPagePane {
        title: "Transient".to_owned(),
        kind: PaneKind::Cartesian,
        viewer_id: "viewer-waveform".to_owned(),
        binding: Some(PaneDataBinding {
            analysis_id: analysis,
            dataset: binding,
        }),
        plotted,
    }
}

fn twenty_names() -> Vec<String> {
    (0..20).map(|index| format!("V(n{index})")).collect()
}

fn referencing_document() -> (VisualizationDocument, DatasetBinding, AnalysisInstanceId) {
    let source = binding(1);
    let analysis = AnalysisInstanceId::new();
    let document = VisualizationDocument::new_with_initial_pane(
        "Twenty signals",
        vec![reference(source, analysis)],
        waveform_pane(source, analysis, twenty_names()),
    )
    .expect("a document over a run reference");
    (document, source, analysis)
}

#[test]
fn a_run_reference_plots_the_signals_it_is_given_and_saves_without_its_run() {
    let (document, source, analysis) = referencing_document();
    assert_eq!(document.traces().len(), 20, "one trace per signal named");
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
    assert!(
        !json.contains("\"signals\"") && !json.contains("trace-index"),
        "the document lists none of the run's signals: {json}"
    );
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
    let unplaced = document.transact(
        document.revision(),
        vec![trace(vec![QueryCoordinate {
            column: TRACE_NAME_COLUMN.to_owned(),
            value: TypedValue::Text("V(n0)".to_owned()),
        }])],
    );
    assert!(
        unplaced
            .unwrap_err()
            .to_string()
            .contains("names an analysis"),
        "a trace on a run names the analysis its signal is of"
    );
}

#[test]
fn a_document_compares_any_number_of_run_references() {
    let (mut document, _, _) = referencing_document();
    for seed in 2..=40 {
        let edit = DocumentEdit::AttachDataset(reference(binding(seed), AnalysisInstanceId::new()));
        document.transact(document.revision(), vec![edit]).unwrap();
    }
    assert_eq!(document.datasets().len(), 40);
}

#[test]
fn analyses_of_one_run_merge_into_its_reference_and_cannot_be_rewritten() {
    let (mut document, source, analysis) = referencing_document();
    let second = AnalysisInstanceId::new();
    let merge = |analysis, kind| {
        DocumentEdit::MergeDatasetProjection(
            SourceDataset::run_reference(
                source,
                "Run 1 (9:00:00 AM)",
                vec![RunSourceAnalysis::new(analysis, kind)],
            )
            .unwrap(),
        )
    };
    document
        .transact(
            document.revision(),
            vec![merge(second, AnalysisType::Transient)],
        )
        .unwrap();
    let run = document.datasets()[0].run_source().unwrap();
    assert_eq!(run.analyses().len(), 2);
    assert!(
        document
            .transact(document.revision(), vec![merge(analysis, AnalysisType::Ac)])
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
    // Whether the other run holds the same signals is the host's to check:
    // the document names its signals and reads neither run's list of them.
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RepointDataset {
                previous,
                next: reference(next, next_analysis),
                analyses: vec![AnalysisRepoint {
                    previous: analysis,
                    next: next_analysis,
                }],
            }],
        )
        .unwrap();
    assert_eq!(document.datasets().len(), 1, "the missing run is let go");
    assert_eq!(document.datasets()[0].binding(), next);
    let pane = document.panes()[0].binding.unwrap();
    assert_eq!((pane.dataset, pane.analysis_id), (next, next_analysis));
    let first = &document.traces()[0];
    assert_eq!(first.binding, next);
    assert_eq!(first.source_signal(), Some("V(n0)"));
    assert_eq!(first.source_analysis(), Some(next_analysis));
    assert!(
        first
            .row_predicates
            .iter()
            .all(|predicate| predicate.column != "trace-index"),
        "a trace names its signal, not the place one run lists it at"
    );
}

#[test]
fn an_archive_embeds_the_run_once_names_its_stores_and_reads_back() {
    let (mut document, source, _) = referencing_document();
    let store = "0123456789abcdef".repeat(4);
    let archive = || DocumentEdit::ArchiveDataset {
        binding: source,
        archive: RunArchive::new("test-run", "{\"run\":1}", vec![store.clone()]).unwrap(),
    };
    assert!(
        RunArchive::new("test-run", "{}", vec!["0123".to_owned()])
            .unwrap_err()
            .to_string()
            .contains("64-digit digest")
    );
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
    assert_eq!(archived.segments(), std::slice::from_ref(&store));
    assert_eq!(
        reopened.archived_segments().collect::<Vec<_>>(),
        [store.as_str()]
    );
}

struct Samples {
    binding: DatasetBinding,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl SourceSamples for Samples {
    fn series_figure(
        &self,
        binding: DatasetBinding,
        series: &SeriesKey,
        _range: Option<(f64, f64)>,
        _columns: usize,
    ) -> Result<Option<FigurePoints>, RunNotHeld> {
        if binding != self.binding {
            return Err(RunNotHeld);
        }
        // This run has every signal of the twenty but the last.
        Ok(
            (series.component == SignalComponent::Display && series.name != "V(n19)")
                .then(|| (self.x.clone(), self.y.clone())),
        )
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
    let carried = run.figure_samples(&series).unwrap();
    assert_eq!(carried.x(), samples.x.as_slice());
    assert!(carried.y()[1].is_nan(), "an unavailable sample stays one");
    let reopened: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&copy).unwrap()).unwrap();
    assert_eq!(
        reopened.datasets()[0]
            .run_source()
            .unwrap()
            .figure_samples(&series)
            .unwrap()
            .x(),
        samples.x.as_slice()
    );
    assert!(
        document.datasets()[0]
            .run_source()
            .unwrap()
            .figure_samples(&series)
            .is_none()
    );
    let absent = run.select_one(&copy.traces()[19].row_predicates).unwrap();
    assert!(
        run.figure_samples(&absent).is_none(),
        "a signal the run does not have is carried by no figure"
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
