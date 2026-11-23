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
    // A run reference names the run's analyses, never its signals: the names
    // here say what the run holds, for the reader of the test.
    SourceDataset::run_reference(
        binding,
        "Run",
        analyses
            .iter()
            .map(|(id, kind, _)| RunSourceAnalysis::new(*id, *kind))
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

/// The analysis the data browser shows is the view's, so the plan's next run
/// and the project reopened open on it; showing another forgets the first.
#[test]
fn a_view_keeps_the_one_analysis_the_browser_shows_across_a_reload() {
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    assert!(document.shown_pane().is_none(), "nothing was picked yet");
    let pane_of = |document: &VisualizationDocument, analysis| {
        document
            .pane_of_analysis(analysis)
            .expect("each analysis has a pane")
            .id
    };
    for analysis in [ac, tran, ac] {
        let pane_id = pane_of(&document, analysis);
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::ShowPaneAnalysis { pane_id }],
            )
            .expect("a view shows one of its panes' analyses");
        let back: VisualizationDocument =
            serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
        assert_eq!(
            back.shown_pane().and_then(|pane| pane.binding),
            document
                .pane_of_analysis(analysis)
                .and_then(|pane| pane.binding),
        );
        assert_eq!(back.panes().iter().filter(|pane| pane.shown).count(), 1);
    }
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
                vec![DocumentEdit::SetSignalsPlotted {
                    pane_id,
                    signals: vec![signal.to_owned()],
                    plotted: true,
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
            ("V(mid)".to_owned(), true, false)
        ],
        "a signal the re-run lacks keeps its trace: the run says it has none when the trace is drawn"
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

/// The Bode card's measured response and the pole-zero map's plane are kept
/// as their sheets' settings, one per pane and sheet, across a re-run the
/// view follows and a save; a later setting replaces the earlier, and a
/// measured response with no name is refused.
#[test]
fn the_bode_and_pole_zero_settings_stay_across_a_rerun_and_a_reload() {
    let TwoAnalyses {
        mut document,
        tran,
        ac,
    } = two_analyses();
    let ac_pane = document.pane_of_analysis(ac).unwrap().id;
    let keep = |setting: SheetSettingValue| DocumentEdit::SetSheetSetting {
        pane_id: ac_pane,
        sheet: setting.sheet(),
        setting: Some(setting),
    };
    let measure = |signal: &str| SheetSettingValue::Bode {
        measured_signal: signal.to_owned(),
    };
    let logarithmic = SheetSettingValue::PoleZero { logarithmic: true };
    document
        .transact(
            document.revision(),
            vec![
                keep(measure("V(out)")),
                keep(measure("V(mid)")),
                keep(logarithmic.clone()),
            ],
        )
        .expect("the settings are kept");
    assert!(
        document
            .transact(document.revision(), vec![keep(measure(" "))])
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
    assert_eq!(
        back.sheet_settings(),
        [
            SheetSetting {
                pane_id: ac_pane,
                setting: measure("V(mid)"),
            },
            SheetSetting {
                pane_id: ac_pane,
                setting: logarithmic,
            },
        ]
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
                DocumentEdit::SetSignalsPlotted {
                    pane_id: tran_pane,
                    signals: vec!["V(late)".to_owned()],
                    plotted: true,
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

/// Ticking a group of signals is one edit however many it names, and so is
/// taking them all off again: no count cuts either short.
#[test]
fn one_edit_plots_or_takes_off_every_signal_it_names() {
    let tran = AnalysisInstanceId::new();
    let names: Vec<String> = (0..5_000).map(|index| format!("V(n{index})")).collect();
    let named: Vec<&str> = names.iter().map(String::as_str).collect();
    let mut document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        run(binding(1), &[(tran, AnalysisType::Transient, &named)]),
        ResultDocumentTracking::pinned(),
        vec![pane("Transient", tran, &[])],
    )
    .expect("a working view");
    let pane_id = document.pane_of_analysis(tran).expect("the pane").id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: names.clone(),
                plotted: true,
            }],
        )
        .expect("one edit plots every signal it names");
    assert_eq!(
        document
            .traces()
            .iter()
            .filter(|trace| trace.visible)
            .count(),
        names.len()
    );
    let back: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(back.traces().len(), names.len(), "and reads back whole");
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: names.clone(),
                plotted: false,
            }],
        )
        .expect("one edit takes every signal it names off");
    assert!(document.traces().iter().all(|trace| !trace.visible));
    assert_eq!(
        document.traces().len(),
        names.len(),
        "a signal taken off keeps its trace, hidden"
    );
}

/// Any name a run holds can be plotted, and its trace carries it whole as
/// its label: a signal name and a label share one bound.
#[test]
fn a_signal_of_the_longest_name_a_run_holds_plots_under_that_name() {
    let tran = AnalysisInstanceId::new();
    let longest = format!("V({})", "n".repeat(MAX_VISUALIZATION_LABEL_BYTES - 3));
    assert_eq!(longest.len(), MAX_VISUALIZATION_LABEL_BYTES);
    let mut document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        run(binding(1), &[(tran, AnalysisType::Transient, &[])]),
        ResultDocumentTracking::pinned(),
        vec![pane("Transient", tran, &[])],
    )
    .expect("a working view");
    let pane_id = document.pane_of_analysis(tran).expect("the pane").id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: vec![longest.clone()],
                plotted: true,
            }],
        )
        .expect("the longest name plots");
    assert_eq!(
        plotted(&document, pane_id),
        [(longest.clone(), true, false)],
        "its trace is labelled with the whole name"
    );
    let back: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(back, document, "and reads back with it");

    let longer = format!("{longest}n");
    assert!(
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::SetSignalsPlotted {
                    pane_id,
                    signals: vec![longer],
                    plotted: true,
                }],
            )
            .is_err(),
        "a name no run can hold is refused"
    );
}
