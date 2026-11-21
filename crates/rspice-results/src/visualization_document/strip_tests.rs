//! A pane keeps the strips the reader arranged its waveform stack into: by
//! signal name and expression text, through a re-run, a save and a copy; and
//! it refuses a strip that cannot stand.

use super::*;
use crate::analysis_type::AnalysisType;

fn view() -> (VisualizationDocument, PaneId, AnalysisInstanceId) {
    let tran = AnalysisInstanceId::new();
    let source = SourceDataset::run_reference(
        DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([1; 32])),
        "Run",
        vec![RunSourceAnalysis::new(
            tran,
            AnalysisType::Transient,
            ["V(rail)", "V(ripple)", "I(load)"]
                .iter()
                .enumerate()
                .map(|(index, name)| RunSignal::new(u32::try_from(index).unwrap(), *name, false))
                .collect(),
        )],
    )
    .expect("a run reference");
    let document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        source,
        ResultDocumentTracking::pinned(),
        vec![WorkingPane {
            title: "Transient".to_owned(),
            kind: PaneKind::Cartesian,
            viewer_id: "viewer-waveform".to_owned(),
            analysis_id: tran,
            plotted: vec!["V(rail)".to_owned(), "V(ripple)".to_owned()],
        }],
    )
    .expect("a working view");
    let pane_id = document.pane_of_analysis(tran).expect("the pane").id;
    (document, pane_id, tran)
}

fn signal(name: &str) -> StripMember {
    StripMember::Signal {
        name: name.to_owned(),
        form: SignalForm::Value,
    }
}

fn strip(key: &str, axes: Vec<(&str, Vec<StripMember>)>) -> Strip {
    Strip {
        key: key.to_owned(),
        weight: None,
        locked: false,
        axes: axes
            .into_iter()
            .map(|(unit, members)| StripAxis {
                unit: unit.to_owned(),
                members,
            })
            .collect(),
    }
}

fn arranged(document: &VisualizationDocument, pane_id: PaneId) -> Vec<Strip> {
    document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .expect("the pane")
        .strips
        .clone()
}

#[test]
fn a_panes_strips_survive_a_rerun_a_save_and_a_copy() {
    let (mut document, pane_id, tran) = view();
    assert!(
        arranged(&document, pane_id).is_empty(),
        "unarranged at first"
    );
    let strips = vec![
        strip("rail", vec![("V", vec![signal("V(rail)")])]),
        Strip {
            weight: Some(2.0),
            locked: true,
            ..strip(
                "ripple",
                vec![
                    ("V", vec![signal("V(ripple)")]),
                    (
                        "A",
                        vec![
                            signal("I(load)"),
                            StripMember::Expression {
                                text: "I(load)*2".to_owned(),
                            },
                        ],
                    ),
                ],
            )
        },
    ];
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneStrips {
                pane_id,
                strips: strips.clone(),
            }],
        )
        .expect("the pane takes its strips");
    assert_eq!(arranged(&document, pane_id), strips);

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::FollowRun {
                next: SourceDataset::run_reference(
                    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([2; 32])),
                    "Run",
                    vec![RunSourceAnalysis::new(
                        tran,
                        AnalysisType::Transient,
                        vec![
                            RunSignal::new(0, "V(rail)", false),
                            RunSignal::new(1, "V(ripple)", false),
                        ],
                    )],
                )
                .expect("the re-run"),
            }],
        )
        .expect("the view follows the re-run");
    assert_eq!(
        arranged(&document, pane_id),
        strips,
        "the strips follow by name, a signal the re-run lacks included"
    );

    let json = serde_json::to_string(&document).expect("the view serializes");
    let back: VisualizationDocument = serde_json::from_str(&json).expect("and reads back");
    assert_eq!(arranged(&back, pane_id), strips);
    let copy = document.saved_copy("Arranged").expect("a saved copy");
    assert_eq!(arranged(&copy, pane_id), strips);

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneStrips {
                pane_id,
                strips: Vec::new(),
            }],
        )
        .expect("an empty list unarranges the pane");
    assert!(arranged(&document, pane_id).is_empty());
}

#[test]
fn a_strip_that_cannot_stand_is_refused_with_its_reason() {
    let (mut document, pane_id, _) = view();
    let mut refuse = |strips: Vec<Strip>, reason: &str| {
        let revision = document.revision();
        let error = document
            .transact(
                revision,
                vec![DocumentEdit::SetPaneStrips { pane_id, strips }],
            )
            .expect_err("the strips are refused");
        assert!(error.to_string().contains(reason), "{error}");
        assert_eq!(document.revision(), revision, "nothing was written");
    };
    refuse(
        vec![strip(
            "five",
            ["V", "A", "W", "Hz", "s"]
                .iter()
                .map(|unit| (*unit, vec![signal(&format!("X({unit})"))]))
                .collect(),
        )],
        "4 Y axes at most",
    );
    refuse(
        vec![strip(
            "twice",
            vec![
                ("V", vec![signal("V(rail)")]),
                ("V", vec![signal("V(rail)")]),
            ],
        )],
        "twice",
    );
    refuse(
        vec![strip("a key", vec![("V", vec![signal("V(rail)")])])],
        "stable ASCII key",
    );
    refuse(
        vec![
            strip("same", vec![("V", vec![signal("V(rail)")])]),
            strip("same", vec![("V", vec![signal("V(ripple)")])]),
        ],
        "share the key",
    );
    refuse(vec![strip("bare", Vec::new())], "has no Y axis");
    refuse(
        vec![Strip {
            weight: Some(0.0),
            ..strip("flat", vec![("V", vec![signal("V(rail)")])])
        }],
        "above zero",
    );
}
