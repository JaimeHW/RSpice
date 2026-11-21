//! A working view plots only what the reader plots, one pane per analysis,
//! and follows a newer run of its plan analysis by analysis and signal by
//! name, never moving two panes onto one analysis.

use super::*;
use crate::analysis_type::AnalysisType;

fn binding(seed: u8) -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([seed; 32]))
}

fn run(
    binding: DatasetBinding,
    analyses: &[(AnalysisInstanceId, AnalysisType, &[&str])],
) -> SourceDataset {
    SourceDataset::run_reference(
        binding,
        "Run",
        analyses
            .iter()
            .map(|(id, kind, names)| {
                RunSourceAnalysis::new(
                    *id,
                    *kind,
                    names
                        .iter()
                        .enumerate()
                        .map(|(index, name)| {
                            RunSignal::new(u32::try_from(index).unwrap(), *name, false)
                        })
                        .collect(),
                )
            })
            .collect(),
    )
    .expect("a valid run reference")
}

fn pane(title: &str, analysis_id: AnalysisInstanceId, plotted: &[&str]) -> WorkingPane {
    WorkingPane {
        title: title.to_owned(),
        kind: PaneKind::Cartesian,
        viewer_id: "viewer-waveform".to_owned(),
        analysis_id,
        plotted: plotted.iter().map(|name| (*name).to_owned()).collect(),
    }
}

fn plotted(document: &VisualizationDocument, pane_id: PaneId) -> Vec<(String, bool, bool)> {
    document
        .traces()
        .iter()
        .filter(|trace| trace.pane_id == pane_id)
        .map(|trace| (trace.label.clone(), trace.visible, trace.missing))
        .collect()
}

struct TwoAnalyses {
    document: VisualizationDocument,
    tran: AnalysisInstanceId,
    ac: AnalysisInstanceId,
}

fn two_analyses() -> TwoAnalyses {
    let (tran, ac) = (AnalysisInstanceId::new(), AnalysisInstanceId::new());
    let source = run(
        binding(1),
        &[
            (tran, AnalysisType::Transient, &["V(in)", "V(out)", "I(R1)"]),
            (ac, AnalysisType::Ac, &["V(out)", "V(mid)"]),
        ],
    );
    let document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        source,
        ResultDocumentTracking::pinned(),
        vec![
            pane("Transient", tran, &["V(out)"]),
            pane("AC", ac, &["V(out)", "V(mid)"]),
        ],
    )
    .expect("a working view");
    TwoAnalyses { document, tran, ac }
}

#[test]
fn a_working_view_has_a_pane_per_analysis_plotting_only_what_it_was_given() {
    let TwoAnalyses { document, tran, ac } = two_analyses();
    assert!(document.working_view().is_some());
    assert_eq!(document.panes().len(), 2, "no placeholder pane stays");
    let tran_pane = document
        .pane_of_analysis(tran)
        .expect("the transient pane")
        .id;
    let ac_pane = document.pane_of_analysis(ac).expect("the AC pane").id;
    assert_eq!(
        plotted(&document, tran_pane),
        [("V(out)".to_owned(), true, false)]
    );
    assert_eq!(plotted(&document, ac_pane).len(), 2);

    let json = serde_json::to_string(&document).expect("a working view serializes");
    let back: VisualizationDocument = serde_json::from_str(&json).expect("and reads back");
    assert_eq!(back.working_view(), document.working_view());
}

#[test]
fn plotting_a_signal_adds_its_trace_once_and_closing_a_strip_keeps_it() {
    let TwoAnalyses {
        mut document, tran, ..
    } = two_analyses();
    let pane_id = document
        .pane_of_analysis(tran)
        .expect("the transient pane")
        .id;
    let plot = |document: &mut VisualizationDocument, signal: &str| {
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::PlotSignal {
                    pane_id,
                    signal: signal.to_owned(),
                }],
            )
            .expect("the pane plots its analysis' signal");
    };
    plot(&mut document, "I(R1)");
    let trace = document
        .traces()
        .iter()
        .find(|trace| trace.pane_id == pane_id && trace.label == "I(R1)")
        .map(|trace| trace.id)
        .expect("the plotted trace");
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceVisibility {
                trace_id: trace,
                visible: false,
            }],
        )
        .unwrap();
    plot(&mut document, "I(R1)");
    assert_eq!(
        plotted(&document, pane_id),
        [
            ("V(out)".to_owned(), true, false),
            ("I(R1)".to_owned(), true, false)
        ],
        "plotting a hidden signal shows its trace instead of adding another"
    );
    assert!(
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::PlotSignal {
                    pane_id,
                    signal: "V(nowhere)".to_owned(),
                }],
            )
            .is_err()
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneHidden {
                pane_id,
                hidden: true,
            }],
        )
        .unwrap();
    let back: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert!(
        back.panes()
            .iter()
            .find(|pane| pane.id == pane_id)
            .unwrap()
            .hidden
    );
    assert_eq!(
        plotted(&back, pane_id).len(),
        2,
        "a closed strip keeps its traces"
    );
}

#[test]
fn following_a_run_moves_each_pane_onto_its_own_analysis() {
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    let next = binding(2);
    // The re-run lost V(mid) and has a new analysis the view does not draw.
    let noise = AnalysisInstanceId::new();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::FollowRun {
                next: run(
                    next,
                    &[
                        (tran, AnalysisType::Transient, &["V(in)", "V(out)", "I(R1)"]),
                        (ac, AnalysisType::Ac, &["V(out)"]),
                        (noise, AnalysisType::Noise, &["onoise"]),
                    ],
                ),
            }],
        )
        .expect("the view follows the re-run");
    let tran_pane = document.pane_of_analysis(tran).expect("the transient pane");
    let ac_pane = document.pane_of_analysis(ac).expect("the AC pane");
    assert_eq!(tran_pane.binding.unwrap().dataset, next);
    assert_eq!(ac_pane.binding.unwrap().dataset, next);
    assert_ne!(
        tran_pane.id, ac_pane.id,
        "two panes never collapse onto one analysis"
    );
    assert_eq!(
        plotted(&document, ac_pane.id),
        [
            ("V(out)".to_owned(), true, false),
            ("V(mid)".to_owned(), true, true)
        ],
        "a signal the re-run lacks keeps its trace, marked missing"
    );
    assert_eq!(document.datasets().len(), 1, "the earlier run is let go");
}

/// A readout pinned on a pane's sheet is a place on its X axis: it stays
/// where it was when the view follows a re-run and across a save, one pin
/// per pane and sheet, and a pin at no finite X is refused.
#[test]
fn a_pinned_readout_stays_at_its_x_across_a_rerun_and_a_reload() {
    use crate::result_presentation::ResultViewer;
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    let tran_pane = document.pane_of_analysis(tran).unwrap().id;
    let pin = |sheet, x| DocumentEdit::SetReadoutPin {
        pane_id: tran_pane,
        sheet,
        x,
    };
    document
        .transact(
            document.revision(),
            vec![
                pin(ResultViewer::Fft, Some(1.0e3)),
                pin(ResultViewer::Fft, Some(2.5e3)),
                pin(ResultViewer::HarmonicBalance, Some(5.0e3)),
            ],
        )
        .expect("the readouts pin");
    assert!(
        document
            .transact(
                document.revision(),
                vec![pin(ResultViewer::Fft, Some(f64::NAN))]
            )
            .is_err()
    );
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::FollowRun {
                next: run(
                    binding(2),
                    &[
                        (tran, AnalysisType::Transient, &["V(in)", "V(out)", "I(R1)"]),
                        (ac, AnalysisType::Ac, &["V(out)", "V(mid)"]),
                    ],
                ),
            }],
        )
        .expect("the view follows the re-run");
    let json = serde_json::to_string(&document).unwrap();
    let back: VisualizationDocument = serde_json::from_str(&json).unwrap();
    let pins = |document: &VisualizationDocument| {
        document
            .readout_pins()
            .iter()
            .map(|pin| (pin.pane_id, pin.sheet, pin.x))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        pins(&back),
        [
            (tran_pane, ResultViewer::Fft, 2.5e3),
            (tran_pane, ResultViewer::HarmonicBalance, 5.0e3)
        ]
    );
    document
        .transact(document.revision(), vec![pin(ResultViewer::Fft, None)])
        .unwrap();
    assert_eq!(
        pins(&document),
        [(tran_pane, ResultViewer::HarmonicBalance, 5.0e3)]
    );
}

/// The run a view reads, grown since (it was shown while still running and
/// has more signals now, and a digest of its finished samples): the view
/// reads it again in place, its traces kept, and plots the new signals.
#[test]
fn a_run_read_while_running_is_read_again_once_it_grows() {
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    let first = document.datasets()[0].binding();
    let tran_pane = document.pane_of_analysis(tran).unwrap().id;
    let hidden = document
        .traces()
        .iter()
        .find(|trace| trace.pane_id == tran_pane)
        .unwrap()
        .id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceVisibility {
                trace_id: hidden,
                visible: false,
            }],
        )
        .unwrap();
    let grown = DatasetBinding::new(first.dataset_id, ContentDigest::from_bytes([9; 32]));
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::FollowRun {
                    next: run(
                        grown,
                        &[
                            (
                                tran,
                                AnalysisType::Transient,
                                &["V(in)", "V(out)", "I(R1)", "V(late)"],
                            ),
                            (ac, AnalysisType::Ac, &["V(out)", "V(mid)"]),
                        ],
                    ),
                },
                DocumentEdit::PlotSignal {
                    pane_id: tran_pane,
                    signal: "V(late)".to_owned(),
                },
            ],
        )
        .expect("the view reads the grown run and plots its new signal");
    assert_eq!(document.datasets().len(), 1);
    assert_eq!(document.datasets()[0].binding(), grown);
    assert!(
        document
            .panes()
            .iter()
            .all(|pane| pane.binding.is_none_or(|binding| binding.dataset == grown))
    );
    assert!(document.traces().iter().all(|trace| trace.binding == grown));
    assert_eq!(
        plotted(&document, tran_pane),
        [
            ("V(out)".to_owned(), false, false),
            ("V(late)".to_owned(), true, false)
        ],
        "the reader's hide stays, and the new signal is plotted"
    );
}

#[test]
fn a_pane_whose_analysis_the_new_run_lacks_stays_on_its_run() {
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    let first = document
        .pane_of_analysis(ac)
        .unwrap()
        .binding
        .unwrap()
        .dataset;
    let next = binding(3);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::FollowRun {
                next: run(next, &[(tran, AnalysisType::Transient, &["V(out)"])]),
            }],
        )
        .expect("the view follows what the re-run solves");
    assert_eq!(
        document
            .pane_of_analysis(tran)
            .unwrap()
            .binding
            .unwrap()
            .dataset,
        next
    );
    assert_eq!(
        document
            .pane_of_analysis(ac)
            .unwrap()
            .binding
            .unwrap()
            .dataset,
        first
    );
    assert_eq!(
        document.datasets().len(),
        2,
        "the run the AC pane reads stays"
    );
}

/// Saving a working view as a document copies the whole view: a hidden
/// trace stays hidden, the log axis and the marker stay, and so does the
/// choice to follow the plan's newest run. The copy is a saved document of
/// its own, which no run's view is.
#[test]
fn a_working_view_saved_as_a_document_is_copied_whole() {
    let TwoAnalyses {
        mut document, tran, ..
    } = two_analyses();
    let pane_id = document.pane_of_analysis(tran).unwrap().id;
    let plan_id = SimulationPlanId::new();
    let trace = document
        .traces()
        .iter()
        .find(|trace| trace.pane_id == pane_id)
        .unwrap()
        .clone();
    let axis = document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::VerticalLeft)
        .unwrap()
        .id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetTraceVisibility {
                    trace_id: trace.id,
                    visible: false,
                },
                DocumentEdit::SetAxisScale {
                    axis_id: axis,
                    scale: AxisScale::Logarithmic,
                },
                DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                    ResultDocumentTrackingMode::Latest,
                    plan_id,
                    tran,
                )),
            ],
        )
        .expect("the reader's changes apply");

    let saved = document.saved_copy("Review").expect("the view is saved");

    assert_eq!(saved.working_view(), None);
    assert_ne!(saved.id(), document.id());
    assert_eq!(saved.title(), "Review");
    assert_eq!(saved.revision(), ObjectRevision::INITIAL);
    assert_eq!(saved.tracking(), document.tracking());
    assert_eq!(saved.tracking().mode, ResultDocumentTrackingMode::Latest);
    assert_eq!(saved.panes(), document.panes());
    assert_eq!(saved.traces(), document.traces());
    assert!(
        !saved
            .traces()
            .iter()
            .any(|kept| kept.id == trace.id && kept.visible)
    );
    assert_eq!(saved.axes(), document.axes());
    assert_eq!(saved.markers(), document.markers());
    assert!(
        saved.saved_copy("Again").is_err(),
        "only a working view is saved as a document"
    );
}
