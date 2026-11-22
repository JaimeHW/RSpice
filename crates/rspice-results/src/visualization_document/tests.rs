//! Tests for document edits, family policy, and the schema a document reads.
//!
//! A rejected edit must roll back everything it touched, a typed family
//! filter must catch type drift before commit, and a document stating any
//! other schema version is refused.

use super::*;

fn binding(seed: u8) -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([seed; 32]))
}

fn dataset(binding: DatasetBinding, offset: f64) -> SourceDataset {
    SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "v(out)",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ],
        vec![
            SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(offset)]),
            SourceRow::new(vec![TypedValue::Real(1.0), TypedValue::Real(1.0 + offset)]),
            SourceRow::new(vec![TypedValue::Real(2.0), TypedValue::Real(2.0 + offset)]),
        ],
    )
    .unwrap()
}

fn document() -> (VisualizationDocument, DatasetBinding) {
    let source = binding(1);
    (
        VisualizationDocument::new("Transient review", vec![dataset(source, 0.0)]).unwrap(),
        source,
    )
}

#[test]
fn unavailable_source_samples_are_typed_versioned_and_never_certified_as_zeroes() {
    let baseline = binding(90);
    let candidate = binding(91);
    let mut source = dataset(baseline, 0.0);
    source.rows[1].values[1] = TypedValue::real_sample(f64::NAN);
    source.validate().unwrap();
    let document =
        VisualizationDocument::new("Gaps", vec![source.clone(), dataset(candidate, 0.0)]).unwrap();
    let encoded = serde_json::to_value(&document).unwrap();
    assert_eq!(
        encoded["datasets"][0]["rows"][1]["values"][1],
        serde_json::json!({"type":"missing","value":"real"})
    );
    let restored: VisualizationDocument = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(
        restored.content_digest().unwrap(),
        document.content_digest().unwrap()
    );
    let request = ComparisonRequest {
        baseline,
        candidate,
        signal_keys: vec!["v(out)".into()],
        policy: ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
            require_identical_units: true,
            execution: ComparisonExecutionContract::default(),
        },
    };
    assert!(
        restored
            .compare(&request)
            .unwrap_err()
            .to_string()
            .contains("unavailable")
    );
    source.rows[1].values[0] = TypedValue::Missing(ValueType::Real);
    assert!(source.validate().is_err());
    assert!(
        TypedValue::Missing(ValueType::Real)
            .validate("cursor.position")
            .is_err()
    );
}

fn long_form_analysis_dataset(
    binding: DatasetBinding,
    analysis_id: AnalysisInstanceId,
    trace_name: &str,
    value: f64,
) -> SourceDataset {
    SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "trace-index",
                "Trace index",
                ValueType::Integer,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "trace-name",
                "Trace",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "component",
                "Component",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "sample",
                "Sample",
                ValueType::Integer,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
            SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
            SourceColumn::new(
                "analysis-id",
                "Analysis identity",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Integer(0),
            TypedValue::Text(trace_name.to_owned()),
            TypedValue::Text("display".to_owned()),
            TypedValue::Integer(0),
            TypedValue::Real(0.0),
            TypedValue::Real(value),
            TypedValue::Text(analysis_id.to_string()),
        ])],
    )
    .unwrap()
}

#[test]
fn same_run_analysis_projections_merge_without_cross_binding_trace_rows() {
    let source = binding(87);
    let first_analysis = AnalysisInstanceId::new();
    let second_analysis = AnalysisInstanceId::new();
    let mut document = VisualizationDocument::new(
        "Multi-analysis review",
        vec![long_form_analysis_dataset(
            source,
            first_analysis,
            "V(first)",
            1.0,
        )],
    )
    .unwrap();
    let page_id = document.pages()[0].id;
    let first_pane = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id: first_pane,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id: first_analysis,
                    dataset: source,
                }),
            }],
        )
        .unwrap();

    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::MergeDatasetProjection(long_form_analysis_dataset(
                    source,
                    second_analysis,
                    "V(second)",
                    2.0,
                )),
                DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Second analysis".to_owned(),
                    kind: PaneKind::Cartesian,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: second_analysis,
                        dataset: source,
                    }),
                    placement: PanePlacement::Below {
                        anchor_pane_id: first_pane,
                    },
                    plotted: Vec::new(),
                }),
            ],
        )
        .unwrap();

    assert_eq!(document.datasets()[0].rows().len(), 2);
    assert_eq!(document.traces().len(), 2);
    for (label, analysis_id) in [("V(first)", first_analysis), ("V(second)", second_analysis)] {
        let trace = document
            .traces()
            .iter()
            .find(|trace| trace.label == label)
            .unwrap();
        assert!(trace.row_predicates.iter().any(|predicate| {
            predicate.column == "analysis-id"
                && predicate.value == TypedValue::Text(analysis_id.to_string())
        }));
    }
}

#[test]
fn result_document_tracking_is_revisioned_and_requires_exact_plan_authority() {
    let (mut document, _) = document();
    assert_eq!(
        document.tracking(),
        ResultDocumentTracking::pinned(),
        "new immutable documents begin pinned until a prepared plan identity is supplied"
    );

    let plan_id = SimulationPlanId::new();
    let analysis_id = AnalysisInstanceId::new();
    let latest =
        ResultDocumentTracking::for_plan(ResultDocumentTrackingMode::Latest, plan_id, analysis_id);
    let previous_revision = document.revision();
    document
        .transact(previous_revision, vec![DocumentEdit::SetTracking(latest)])
        .unwrap();
    assert_eq!(document.tracking(), latest);
    assert_ne!(document.revision(), previous_revision);

    let malformed = ResultDocumentTracking {
        mode: ResultDocumentTrackingMode::Latest,
        simulation_plan_id: Some(plan_id),
        authored_analysis_id: None,
    };
    let revision = document.revision();
    let error = document
        .transact(revision, vec![DocumentEdit::SetTracking(malformed)])
        .expect_err("partial tracking authority must fail closed");
    assert!(error.to_string().contains("present together"));
    assert_eq!(document.revision(), revision);
    assert_eq!(document.tracking(), latest);
}

#[test]
fn latest_retarget_moves_the_document_onto_the_newer_run_and_drops_the_one_it_left() {
    let (mut document, previous) = document();
    let plan_id = SimulationPlanId::new();
    let authored_analysis = AnalysisInstanceId::new();
    let previous_execution = AnalysisInstanceId::new();
    let next_execution = AnalysisInstanceId::new();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                    ResultDocumentTrackingMode::Latest,
                    plan_id,
                    authored_analysis,
                )),
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: previous_execution,
                        dataset: previous,
                    }),
                },
            ],
        )
        .unwrap();

    let next = binding(2);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous,
                next: dataset(next, 10.0),
                analysis_id: next_execution,
            }],
        )
        .unwrap();
    assert_eq!(
        document
            .datasets()
            .iter()
            .map(SourceDataset::binding)
            .collect::<Vec<_>>(),
        [next],
        "nothing reads the run left behind"
    );
    assert_eq!(
        document.panes()[0].binding,
        Some(PaneDataBinding {
            analysis_id: next_execution,
            dataset: next,
        })
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                ResultDocumentTrackingMode::Pinned,
                plan_id,
                authored_analysis,
            ))],
        )
        .unwrap();
    let revision = document.revision();
    let third = binding(3);
    let error = document
        .transact(
            revision,
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: next,
                next: dataset(third, 20.0),
                analysis_id: AnalysisInstanceId::new(),
            }],
        )
        .expect_err("pinned documents cannot advance");
    assert!(error.to_string().contains("latest-bound"));
    assert_eq!(document.revision(), revision);
    assert_eq!(document.datasets().len(), 1);
}

#[test]
fn a_latest_document_follows_three_hundred_reruns() {
    // Each re-run used to leave its predecessor attached, so the document
    // held one more run for every re-run it followed.
    let (mut document, mut previous) = document();
    let plan_id = SimulationPlanId::new();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                    ResultDocumentTrackingMode::Latest,
                    plan_id,
                    AnalysisInstanceId::new(),
                )),
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: AnalysisInstanceId::new(),
                        dataset: previous,
                    }),
                },
            ],
        )
        .unwrap();
    for rerun in 0..300 {
        let next = binding((rerun % 250) as u8 + 2);
        let analysis_id = AnalysisInstanceId::new();
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::RetargetTrackedDataset {
                    previous,
                    next: dataset(next, f64::from(rerun)),
                    analysis_id,
                }],
            )
            .unwrap_or_else(|error| panic!("re-run {rerun} was not followed: {error}"));
        assert_eq!(
            document.panes()[0].binding,
            Some(PaneDataBinding {
                analysis_id,
                dataset: next,
            })
        );
        previous = next;
    }
    assert_eq!(document.datasets().len(), 1);
}

#[test]
fn content_digest_authenticates_the_exact_document_revision() {
    let (mut document, _) = document();
    let first = document.content_digest().unwrap();
    assert_eq!(document.content_digest().unwrap(), first);

    let restored: VisualizationDocument =
        serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(restored.content_digest().unwrap(), first);

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Rename {
                entity: EntityRef::Page(document.pages()[0].id),
                value: "Updated page".to_owned(),
            }],
        )
        .unwrap();
    assert_ne!(document.content_digest().unwrap(), first);
}

/// A latest-bound document holding one trace provisioned from `trace_name`.
fn provisioned_trace_document(
    trace_name: &str,
) -> (VisualizationDocument, DatasetBinding, AnalysisInstanceId) {
    let source = binding(41);
    let analysis_id = AnalysisInstanceId::new();
    let mut document = VisualizationDocument::new(
        "Bus review",
        vec![long_form_analysis_dataset(
            source,
            analysis_id,
            trace_name,
            1.0,
        )],
    )
    .unwrap();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                    ResultDocumentTrackingMode::Latest,
                    SimulationPlanId::new(),
                    AnalysisInstanceId::new(),
                )),
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id,
                        dataset: source,
                    }),
                },
            ],
        )
        .unwrap();
    (document, source, analysis_id)
}

/// Taking an undo step puts back what the step changed as the next revision,
/// over the datasets the document reads now, and gives the step that redoes
/// it. An identity the undone edit created is never handed out again, and a
/// stale revision or another document's step is refused without touching
/// the document.
#[test]
fn an_undo_step_puts_back_what_the_edit_changed_as_a_new_revision() {
    let (mut document, _, _) = provisioned_trace_document("V(out)");
    let pane_id = document.panes()[0].id;
    let trace_id = document.traces()[0].id;
    let before = document.clone();
    let (receipt, undo) = document
        .transact_with_undo(
            document.revision(),
            vec![
                DocumentEdit::SetTraceVisibility {
                    trace_id,
                    visible: false,
                },
                DocumentEdit::AddMarker {
                    pane_id,
                    trace_id,
                    coordinate: TypedValue::Real(0.0),
                    label: "peak".to_owned(),
                },
            ],
        )
        .unwrap();
    let undone_marker = receipt.created[0];
    let edited = document.revision();
    let after = document.clone();

    let (restored, redo) = document.take_step(edited, undo.clone()).unwrap();
    assert!(document.traces()[0].visible, "the hidden trace shows again");
    assert!(document.markers().is_empty(), "the placed marker is gone");
    assert_eq!(restored.previous_revision, edited);
    assert_eq!(restored.committed_revision, document.revision());
    assert_ne!(document.revision(), edited);
    assert_eq!(
        view_of(&document),
        view_of(&before),
        "the view is as it was"
    );

    let (_, undo_again) = document.take_step(document.revision(), redo).unwrap();
    assert_eq!(
        view_of(&document),
        view_of(&after),
        "redo puts the edit back"
    );
    document.take_step(document.revision(), undo_again).unwrap();

    let again = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddMarker {
                pane_id,
                trace_id,
                coordinate: TypedValue::Real(0.0),
                label: "peak".to_owned(),
            }],
        )
        .unwrap();
    assert_ne!(again.created[0], undone_marker, "identities are not reused");

    let unchanged = document.clone();
    assert!(matches!(
        document.take_step(edited, undo),
        Err(VisualizationError::RevisionConflict { .. })
    ));
    let (mut other, _, _) = provisioned_trace_document("V(out)");
    let other_trace = other.traces()[0].id;
    let (_, others) = other
        .transact_with_undo(
            other.revision(),
            vec![DocumentEdit::SetTraceVisibility {
                trace_id: other_trace,
                visible: false,
            }],
        )
        .unwrap();
    assert!(document.take_step(document.revision(), others).is_err());
    assert_eq!(document, unchanged);
}

/// What a view shows apart from its revision and the serials it hands out:
/// what undo and redo must put back exactly.
fn view_of(document: &VisualizationDocument) -> serde_json::Value {
    let mut view = serde_json::to_value(document).unwrap();
    let fields = view.as_object_mut().unwrap();
    fields.remove("revision");
    fields.remove("next_serial");
    view
}

/// A long history of mixed edits on a view of many traces steps back to
/// where it began and forward to where it ended, exactly, each step on the
/// way, and the steps hold only what their edits changed: no more traces in
/// all than the edits touched, not a view each.
#[test]
fn a_long_mixed_history_steps_back_and_forward_exactly() {
    let analysis = AnalysisInstanceId::new();
    let names: Vec<String> = (0..1_000).map(|index| format!("V(n{index})")).collect();
    let source = SourceDataset::run_reference(
        binding(61),
        "Run",
        vec![RunSourceAnalysis::new(
            analysis,
            crate::analysis_type::AnalysisType::Transient,
        )],
    )
    .unwrap();
    let mut document = VisualizationDocument::new_working_view(
        WorkingViewSource::Plan {
            plan_id: SimulationPlanId::new(),
        },
        source,
        ResultDocumentTracking::pinned(),
        vec![WorkingPane {
            title: "Transient".to_owned(),
            kind: PaneKind::Cartesian,
            viewer_id: "viewer-waveform".to_owned(),
            analysis_id: analysis,
            plotted: names.clone(),
        }],
    )
    .unwrap();
    let pane_id = document.panes()[0].id;
    let axis_id = document.axes()[0].id;
    let start = view_of(&document);

    let mut undo = Vec::new();
    let mut views = vec![start.clone()];
    let mut touched = 0;
    for step in 0..120_usize {
        let traces = document.traces();
        let trace = &traces[(step * 37) % traces.len()];
        let edits = match step % 8 {
            0 => vec![DocumentEdit::SetTraceColor {
                trace_id: trace.id,
                color: Some(TraceColor::from_rgb(step as u8, 1, 2)),
            }],
            1 => vec![DocumentEdit::SetTraceVisibility {
                trace_id: trace.id,
                visible: !trace.visible,
            }],
            2 => vec![DocumentEdit::Rename {
                entity: EntityRef::Trace(trace.id),
                value: format!("Signal {step}"),
            }],
            3 => vec![DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position: TypedValue::Real(step as f64),
                label: format!("C{step}"),
            }],
            4 => match document.cursors().first() {
                Some(cursor) => vec![DocumentEdit::Remove(EntityRef::Cursor(cursor.id))],
                None => vec![DocumentEdit::SetTraceStyle {
                    trace_id: trace.id,
                    style: TraceStyle::default(),
                }],
            },
            5 => vec![DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals: names[step..step + 40].to_vec(),
                plotted: false,
            }],
            6 => vec![
                DocumentEdit::SetSignalsPlotted {
                    pane_id,
                    signals: names[step - 1..step + 39].to_vec(),
                    plotted: true,
                },
                DocumentEdit::Rename {
                    entity: EntityRef::Pane(pane_id),
                    value: format!("Pane {step}"),
                },
            ],
            _ => vec![
                DocumentEdit::SetTraceColor {
                    trace_id: traces[0].id,
                    color: None,
                },
                DocumentEdit::SetTraceColor {
                    trace_id: traces[traces.len() - 1].id,
                    color: Some(TraceColor::from_rgb(9, 9, 9)),
                },
            ],
        };
        touched += [1, 1, 1, 0, 1, 40, 40, 2][step % 8];
        let (_, step) = document
            .transact_with_undo(document.revision(), edits)
            .unwrap();
        undo.push(step);
        views.push(view_of(&document));
    }
    let held: usize = undo.iter().map(ViewStep::traces_held).sum();
    assert!(
        held <= touched,
        "the steps hold {held} traces; the edits touched {touched}"
    );

    let mut redo = Vec::new();
    while let Some(step) = undo.pop() {
        let (_, reverse) = document.take_step(document.revision(), step).unwrap();
        redo.push(reverse);
        assert_eq!(view_of(&document), views[undo.len()]);
    }
    assert_eq!(view_of(&document), start, "undo steps back to the start");
    while let Some(step) = redo.pop() {
        let (_, reverse) = document.take_step(document.revision(), step).unwrap();
        undo.push(reverse);
        assert_eq!(view_of(&document), views[undo.len()]);
    }
    assert_eq!(
        &view_of(&document),
        views.last().unwrap(),
        "redo steps forward to the end"
    );
}

/// An earlier view put back after the document followed a newer run is
/// restored on the run now shown: the step addresses the view, not the run
/// it read when the step was taken.
#[test]
fn an_undo_step_puts_back_onto_the_run_the_document_follows_now() {
    let (mut document, source, _) = provisioned_trace_document("V(out)");
    let trace_id = document.traces()[0].id;
    let (_, undo) = document
        .transact_with_undo(
            document.revision(),
            vec![DocumentEdit::SetTraceVisibility {
                trace_id,
                visible: false,
            }],
        )
        .unwrap();
    let next = binding(44);
    let next_analysis = AnalysisInstanceId::new();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: source,
                next: long_form_analysis_dataset(next, next_analysis, "V(out)", 2.0),
                analysis_id: next_analysis,
            }],
        )
        .unwrap();
    assert_eq!(document.datasets().len(), 1, "the earlier run is let go");

    document
        .take_step(document.revision(), undo)
        .expect("the step survives the view following the newer run");
    let trace = &document.traces()[0];
    assert!(trace.visible, "the hidden trace shows again");
    assert_eq!(trace.binding, next, "on the run the document follows now");
    assert!(!trace.missing);
    assert_eq!(
        document.panes()[0].binding,
        Some(PaneDataBinding {
            analysis_id: next_analysis,
            dataset: next,
        })
    );
}

/// A document following the latest run keeps a plotted signal the new run
/// lacks: the trace stays, marked missing, the document still moves onto the
/// new run, and the first later run that has the signal draws it again.
#[test]
fn a_signal_the_new_run_lacks_stays_marked_missing_and_the_document_moves_on() {
    let (mut document, source, analysis_id) = provisioned_trace_document("V(out)");
    let trace_id = document.traces()[0].id;
    let without = binding(42);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: source,
                next: long_form_analysis_dataset(without, analysis_id, "V(in)", 2.0),
                analysis_id,
            }],
        )
        .expect("a missing signal never holds the document back");
    let trace = &document.traces()[0];
    assert_eq!(trace.id, trace_id);
    assert_eq!(trace.binding, without);
    assert!(trace.missing);
    assert_eq!(trace.source_signal(), Some("V(out)"));
    assert_eq!(
        document.panes()[0].binding.map(|pane| pane.dataset),
        Some(without)
    );

    let saved = serde_json::to_string(&document).unwrap();
    let mut document: VisualizationDocument = serde_json::from_str(&saved).unwrap();
    assert!(document.traces()[0].missing, "a saved view keeps the mark");

    let with = binding(43);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: without,
                next: long_form_analysis_dataset(with, analysis_id, "V(out)", 3.0),
                analysis_id,
            }],
        )
        .unwrap();
    let trace = &document.traces()[0];
    assert_eq!(trace.id, trace_id);
    assert_eq!(trace.binding, with);
    assert!(!trace.missing, "a run that has the signal draws it again");
}

/// A colour the reader gives a trace is the view's: it is saved with it,
/// stays when the view follows the next run, and is given back to the
/// signal's own colour by clearing it. A saved colour not written `#rrggbb`
/// is refused.
#[test]
fn a_trace_colour_is_kept_by_the_view_through_a_save_and_a_rerun() {
    let (mut document, source, analysis_id) = provisioned_trace_document("V(out)");
    let trace_id = document.traces()[0].id;
    assert_eq!(document.traces()[0].color, None);
    let orange = TraceColor::parse("#FF8800").expect("a colour");
    assert_eq!(orange.rgb(), [0xff, 0x88, 0x00]);
    assert_eq!(orange.to_string(), "#ff8800");
    assert_eq!(TraceColor::parse("ff8800"), Some(orange));
    for malformed in ["#ff880", "#ff88000", "#gg8800", "", "#"] {
        assert_eq!(TraceColor::parse(malformed), None, "{malformed:?}");
    }
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceColor {
                trace_id,
                color: Some(orange),
            }],
        )
        .unwrap();

    let saved = serde_json::to_string(&document).unwrap();
    assert!(saved.contains("\"color\":\"#ff8800\""), "{saved}");
    let mut document: VisualizationDocument = serde_json::from_str(&saved).unwrap();
    assert_eq!(
        document.traces()[0].color,
        Some(orange),
        "a saved view keeps it"
    );
    let malformed = saved.replace("#ff8800", "orange");
    assert!(serde_json::from_str::<VisualizationDocument>(&malformed).is_err());

    let next = binding(45);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: source,
                next: long_form_analysis_dataset(next, analysis_id, "V(out)", 2.0),
                analysis_id,
            }],
        )
        .unwrap();
    assert_eq!(document.traces()[0].binding, next);
    assert_eq!(
        document.traces()[0].color,
        Some(orange),
        "the next run's trace keeps it"
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceColor {
                trace_id,
                color: None,
            }],
        )
        .unwrap();
    assert_eq!(document.traces()[0].color, None);
    assert!(
        !serde_json::to_string(&document)
            .unwrap()
            .contains("\"color\"")
    );
}

/// A trace's style is kept by its view through a save and the next run, in
/// readable words, and the default style leaves nothing written.
#[test]
fn a_trace_style_is_kept_by_the_view_through_a_save_and_a_rerun() {
    let (mut document, source, analysis_id) = provisioned_trace_document("V(out)");
    let trace_id = document.traces()[0].id;
    let style = TraceStyle {
        width: Some(TraceWidth::Thick),
        dash: Some(TraceDash::DashDot),
        symbol: Some(TraceSymbol::None),
        draw: TraceDraw::Steps,
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceStyle { trace_id, style }],
        )
        .unwrap();
    let saved = serde_json::to_string(&document).unwrap();
    for word in [
        "\"width\":\"thick\"",
        "\"dash\":\"dash-dot\"",
        "\"symbol\":\"none\"",
        "\"draw\":\"steps\"",
    ] {
        assert!(saved.contains(word), "{word} in {saved}");
    }
    let mut document: VisualizationDocument = serde_json::from_str(&saved).unwrap();
    assert_eq!(document.traces()[0].style, style, "a saved view keeps it");
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous: source,
                next: long_form_analysis_dataset(binding(46), analysis_id, "V(out)", 2.0),
                analysis_id,
            }],
        )
        .unwrap();
    assert_eq!(
        document.traces()[0].style,
        style,
        "the next run's trace keeps it"
    );
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceStyle {
                trace_id,
                style: TraceStyle::default(),
            }],
        )
        .unwrap();
    assert!(
        !serde_json::to_string(&document)
            .unwrap()
            .contains("\"style\"")
    );
}

/// Provisioning labels a trace with the vector's own name, and the vector it
/// came from stays recorded beside it. A display surface may re-spell a label
/// it can prove derivation wrote — the two are still equal — and must leave one
/// the reader typed exactly as typed.
#[test]
fn a_renamed_trace_still_states_the_vector_it_was_provisioned_from() {
    let (mut document, _, _) = provisioned_trace_document("V(x1.DATA#3)");
    let trace = &document.traces()[0];
    assert_eq!(trace.label, "V(x1.DATA#3)");
    assert_eq!(trace.source_signal(), Some(trace.label.as_str()));

    let trace_id = trace.id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Rename {
                entity: EntityRef::Trace(trace_id),
                value: "Data bit 3".to_owned(),
            }],
        )
        .unwrap();
    let renamed = &document.traces()[0];
    assert_eq!(renamed.label, "Data bit 3");
    assert_eq!(
        renamed.source_signal(),
        Some("V(x1.DATA#3)"),
        "a rename names the trace, it does not repoint it at another vector"
    );
    assert_ne!(
        renamed.source_signal(),
        Some(renamed.label.as_str()),
        "a label the reader typed is never re-spelled for them"
    );
}

/// Re-spelling a vector bit is a rendering decision and never an edit: a saved
/// document loads and is written back byte for byte, and every persisted field
/// still carries the deck's own spelling.
#[test]
fn showing_a_bit_in_its_declared_notation_leaves_the_saved_document_alone() {
    let (document, _, _) = provisioned_trace_document("V(x1.DATA#3)");
    let saved = serde_json::to_vec(&document).unwrap();

    let loaded: VisualizationDocument = serde_json::from_slice(&saved).unwrap();
    assert_eq!(
        serde_json::to_vec(&loaded).unwrap(),
        saved,
        "loading and saving a document must not rewrite a single byte of it"
    );

    let trace = &loaded.traces()[0];
    assert_eq!(trace.label, "V(x1.DATA#3)");
    assert!(
        trace.row_predicates.iter().any(|predicate| {
            predicate.column == TRACE_NAME_COLUMN
                && predicate.value == TypedValue::Text("V(x1.DATA#3)".to_owned())
        }),
        "the vector the trace was provisioned from stays the deck's own name"
    );
}

/// Retargeting matches the newer snapshot on the vector a trace was
/// provisioned from. Matching on the label instead refused every renamed
/// trace, which is the same confusion of a reader's words with the engine's
/// that quoting a deck bit back at them is.
#[test]
fn a_renamed_trace_retargets_on_its_vector_and_keeps_the_typed_label() {
    let (mut document, previous, _) = provisioned_trace_document("V(x1.DATA#3)");
    let trace_id = document.traces()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Rename {
                entity: EntityRef::Trace(trace_id),
                value: "Data bit 3".to_owned(),
            }],
        )
        .unwrap();

    let next_execution = AnalysisInstanceId::new();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous,
                next: long_form_analysis_dataset(binding(42), next_execution, "V(x1.DATA#3)", 2.0),
                analysis_id: next_execution,
            }],
        )
        .expect("a renamed trace still follows its own vector");

    let trace = &document.traces()[0];
    assert_eq!(trace.label, "Data bit 3");
    assert_eq!(trace.source_signal(), Some("V(x1.DATA#3)"));
}

fn family_dataset(binding: DatasetBinding) -> SourceDataset {
    SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "process",
                "Process",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "temperature",
                "Temperature",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("degC".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "sample",
                "Sample",
                ValueType::Integer,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "v(out)",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ],
        vec![
            SourceRow::new(vec![
                TypedValue::Real(0.0),
                TypedValue::Text("TT".to_owned()),
                TypedValue::Real(27.0),
                TypedValue::Integer(1),
                TypedValue::Real(0.0),
            ]),
            SourceRow::new(vec![
                TypedValue::Real(1.0),
                TypedValue::Text("TT".to_owned()),
                TypedValue::Real(27.0),
                TypedValue::Integer(1),
                TypedValue::Real(1.0),
            ]),
            SourceRow::new(vec![
                TypedValue::Real(0.0),
                TypedValue::Text("SS".to_owned()),
                TypedValue::Real(125.0),
                TypedValue::Integer(2),
                TypedValue::Real(-0.1),
            ]),
            SourceRow::new(vec![
                TypedValue::Real(1.0),
                TypedValue::Text("SS".to_owned()),
                TypedValue::Real(125.0),
                TypedValue::Integer(2),
                TypedValue::Real(0.9),
            ]),
        ],
    )
    .unwrap()
}

fn dimension(key: &str, value_type: ValueType) -> FamilyDimension {
    FamilyDimension::new(key, value_type).unwrap()
}

fn family_policy() -> FamilyPresentationPolicy {
    let process = dimension("process", ValueType::Text);
    let temperature = dimension("temperature", ValueType::Real);
    let sample = dimension("sample", ValueType::Integer);
    FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: dimension("time", ValueType::Real),
            ordering: FamilyXOrdering::Ascending,
        },
        family_dimensions: vec![process.clone(), temperature.clone(), sample.clone()],
        facet_layout: Some(FamilyFacetLayout {
            axis_sharing: FacetAxisSharing::Shared,
            overflow: FacetOverflowPolicy::Paginate,
            maximum_panels: 12,
        }),
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::Mean,
            over_dimensions: vec![sample],
        },
        filter: Some(FamilyFilterExpression {
            source: "process in {TT,SS} and temperature >= 27".to_owned(),
            predicate: FamilyPredicate::All {
                predicates: vec![
                    FamilyPredicate::In {
                        dimension: process.clone(),
                        values: vec![
                            TypedValue::Text("TT".to_owned()),
                            TypedValue::Text("SS".to_owned()),
                        ],
                    },
                    FamilyPredicate::Compare {
                        dimension: temperature.clone(),
                        operator: FamilyComparisonOperator::GreaterThanOrEqual,
                        value: TypedValue::Real(27.0),
                    },
                ],
            },
        }),
        missing_points: MissingPointPolicy::PreserveAsNotRun,
        encodings: vec![
            FamilyEncodingMap::Color {
                dimension: process.clone(),
                palette: AccessibleColorPalette::OkabeItoCategorical,
            },
            FamilyEncodingMap::Label {
                dimension: process.clone(),
                prefix: Some("P=".to_owned()),
            },
            FamilyEncodingMap::Dash {
                dimension: temperature.clone(),
            },
            FamilyEncodingMap::Thickness {
                dimension: temperature,
                minimum_points: 1.0,
                maximum_points: 3.0,
            },
            FamilyEncodingMap::Facet {
                dimension: process,
                direction: FacetDirection::Rows,
            },
        ],
    }
}

#[test]
fn source_dataset_rejects_shape_type_and_duplicate_coordinate_errors() {
    let source = binding(2);
    let columns = vec![
        SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
        SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
    ];
    assert!(matches!(
        SourceDataset::new(
            source,
            columns.clone(),
            vec![SourceRow::new(vec![TypedValue::Real(0.0)])]
        ),
        Err(VisualizationError::RowWidth { .. })
    ));
    assert!(matches!(
        SourceDataset::new(
            source,
            columns.clone(),
            vec![SourceRow::new(vec![
                TypedValue::Integer(0),
                TypedValue::Real(1.0)
            ])]
        ),
        Err(VisualizationError::ColumnTypeMismatch { .. })
    ));
    assert!(matches!(
        SourceDataset::new(
            source,
            columns,
            vec![
                SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(1.0)]),
                SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(2.0)]),
            ]
        ),
        Err(VisualizationError::DuplicateCoordinateRow(1))
    ));
}

#[test]
fn a_text_cell_longer_than_its_bound_is_refused_at_construction() {
    let oversized_text = SourceDataset::new(
        binding(32),
        vec![
            SourceColumn::new(
                "corner",
                "Corner",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new("value", "Value", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Text("x".repeat(MAX_SOURCE_TEXT_BYTES + 1)),
            TypedValue::Real(1.0),
        ])],
    );
    assert!(matches!(
        oversized_text,
        Err(VisualizationError::InvalidValue {
            field: "source-row.value",
            ..
        })
    ));
}

#[test]
fn a_transaction_holds_any_number_of_edits_but_not_none() {
    let (mut document, _) = document();
    let before = document.clone();
    assert!(matches!(
        document.transact(document.revision(), Vec::new()),
        Err(VisualizationError::InvalidValue {
            field: "transaction.edits",
            ..
        })
    ));
    assert_eq!(document, before);
    let page = document.pages()[0].id;
    let edits: Vec<_> = (0..10_000)
        .map(|step| DocumentEdit::Rename {
            entity: EntityRef::Page(page),
            value: format!("Step {step}"),
        })
        .collect();
    let receipt = document.transact(document.revision(), edits).unwrap();
    assert_eq!(receipt.edit_count, 10_000);
    assert_eq!(document.pages()[0].title, "Step 9999");
}

/// A logical group of any breadth reads back: how many children a filter
/// holds is bounded only by the document holding it. A text cell longer
/// than its bound is still refused while it is read.
#[test]
fn a_predicate_of_any_breadth_reads_back_but_an_oversized_text_cell_does_not() {
    let leaf = FamilyPredicate::Constant { value: true };
    let branch = FamilyPredicate::All {
        predicates: vec![leaf; 100],
    };
    let broad = FamilyPredicate::Any {
        predicates: vec![branch; 100],
    };
    let restored: FamilyPredicate =
        serde_json::from_value(serde_json::to_value(&broad).unwrap()).unwrap();
    assert_eq!(restored, broad);

    let source = dataset(binding(33), 0.0);
    let mut source_json = serde_json::to_value(source).unwrap();
    source_json["rows"][0]["values"][0] = serde_json::json!({
        "type": "text",
        "value": "x".repeat(MAX_SOURCE_TEXT_BYTES + 1),
    });
    let text_error = serde_json::from_value::<SourceDataset>(source_json).unwrap_err();
    assert!(text_error.to_string().contains("resource limit"));
}

/// Reading a predicate bounds only how deeply it nests, which keeps a
/// hostile document from overflowing the stack, and a refused read leaves
/// the next read unaffected.
#[test]
fn reading_a_predicate_bounds_how_deeply_it_nests() {
    let leaf = FamilyPredicate::Constant { value: true };
    let nested_not = |levels: usize| {
        (1..levels).fold(leaf.clone(), |predicate, _| FamilyPredicate::Not {
            predicate: Box::new(predicate),
        })
    };
    let at_depth_limit = nested_not(MAX_FAMILY_PREDICATE_DEPTH);
    let restored: FamilyPredicate =
        serde_json::from_value(serde_json::to_value(&at_depth_limit).unwrap()).unwrap();
    assert_eq!(restored, at_depth_limit);

    let over_depth_limit = nested_not(MAX_FAMILY_PREDICATE_DEPTH + 1);
    let depth_error =
        serde_json::from_value::<FamilyPredicate>(serde_json::to_value(over_depth_limit).unwrap())
            .unwrap_err();
    assert!(depth_error.to_string().contains("32-level"));

    let restored_after_errors: FamilyPredicate =
        serde_json::from_value(serde_json::to_value(leaf).unwrap()).unwrap();
    assert_eq!(
        restored_after_errors,
        FamilyPredicate::Constant { value: true }
    );
}

/// A filter picking every run of a ten-thousand-run Monte Carlo by its
/// sample number, typed out in full, is a policy a pane keeps and reads
/// back; a pick naming one run twice is refused.
#[test]
fn a_filter_picking_every_run_of_a_large_monte_carlo_is_kept() {
    const RUNS: i64 = 10_000;
    let source = binding(43);
    let mut document =
        VisualizationDocument::new("Monte Carlo", vec![family_dataset(source)]).unwrap();
    let pane_id = document.panes()[0].id;
    let sample = dimension("sample", ValueType::Integer);
    let picks: Vec<_> = (1..=RUNS).map(TypedValue::Integer).collect();
    let typed = (1..=RUNS)
        .map(|run| run.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let mut policy = family_policy();
    policy.filter = Some(FamilyFilterExpression {
        source: format!("sample in {{{typed}}}"),
        predicate: FamilyPredicate::In {
            dimension: sample.clone(),
            values: picks.clone(),
        },
    });
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: AnalysisInstanceId::new(),
                        dataset: source,
                    }),
                },
                DocumentEdit::SetPaneFamilyPresentation {
                    pane_id,
                    policy: Some(policy.clone()),
                },
            ],
        )
        .expect("every run is picked");
    assert_eq!(document.panes()[0].family_policy.as_ref(), Some(&policy));
    let restored: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(restored, document);

    let mut twice = picks;
    twice.push(TypedValue::Integer(RUNS));
    policy.filter = Some(FamilyFilterExpression {
        source: format!("sample in {{{typed}, {RUNS}}}"),
        predicate: FamilyPredicate::In {
            dimension: sample,
            values: twice,
        },
    });
    assert!(matches!(
        policy.validate(),
        Err(VisualizationError::InvalidValue {
            field: "family.filter.values",
            ..
        })
    ));
}

#[test]
fn retained_source_text_accounting_accepts_the_boundary_and_rejects_one_more_byte() {
    assert_eq!(
        checked_bounded_sum(
            "source-dataset.retained-text-bytes",
            MAX_SOURCE_TEXT_BYTES_PER_DATASET - 1,
            1,
            MAX_SOURCE_TEXT_BYTES_PER_DATASET,
        )
        .unwrap(),
        MAX_SOURCE_TEXT_BYTES_PER_DATASET
    );
    assert!(matches!(
        checked_bounded_sum(
            "source-dataset.retained-text-bytes",
            MAX_SOURCE_TEXT_BYTES_PER_DATASET,
            1,
            MAX_SOURCE_TEXT_BYTES_PER_DATASET,
        ),
        Err(VisualizationError::InvalidValue {
            field: "source-dataset.retained-text-bytes",
            ..
        })
    ));
}

#[test]
fn unknown_extensions_are_ignored_and_another_schema_version_is_refused() {
    let source = dataset(binding(34), 0.0);
    let mut source_json = serde_json::to_value(&source).unwrap();
    source_json["untrusted_extension"] = serde_json::json!([]);
    source_json["rows"][0]["untrusted_row_extension"] = serde_json::json!({});
    let restored_source: SourceDataset = serde_json::from_value(source_json).unwrap();
    assert_eq!(restored_source, source);

    let (document, source_binding) = document();
    let mut extended = serde_json::to_value(&document).unwrap();
    extended["untrusted_document_extension"] = serde_json::json!({ "future": true });
    let restored_extended: VisualizationDocument = serde_json::from_value(extended).unwrap();
    assert_eq!(restored_extended, document);
    assert_eq!(restored_extended.datasets()[0].binding(), source_binding);
    for schema_version in [
        1,
        VisualizationDocument::SCHEMA_VERSION - 1,
        VisualizationDocument::SCHEMA_VERSION + 1,
    ] {
        let mut other = serde_json::to_value(&document).unwrap();
        other["schema_version"] = serde_json::json!(schema_version);
        let error = serde_json::from_value::<VisualizationDocument>(other)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(&format!(
                "unsupported visualization document schema version {schema_version}"
            )),
            "{error}"
        );
    }
    let mut unversioned = serde_json::to_value(&document).unwrap();
    unversioned
        .as_object_mut()
        .expect("document JSON is an object")
        .remove("schema_version");
    assert!(serde_json::from_value::<VisualizationDocument>(unversioned).is_err());
}

#[test]
fn coordinate_identity_uses_exact_typed_values_and_real_bit_patterns() {
    let source = binding(8);
    let columns = vec![
        SourceColumn::new(
            "real",
            "Real coordinate",
            ValueType::Real,
            ColumnRole::Coordinate,
            None,
        )
        .unwrap(),
        SourceColumn::new(
            "integer",
            "Integer coordinate",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )
        .unwrap(),
        SourceColumn::new(
            "boolean",
            "Boolean coordinate",
            ValueType::Boolean,
            ColumnRole::Coordinate,
            None,
        )
        .unwrap(),
        SourceColumn::new(
            "text",
            "Text coordinate",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )
        .unwrap(),
        SourceColumn::new("value", "Value", ValueType::Real, ColumnRole::Signal, None).unwrap(),
    ];
    let row = |real| {
        SourceRow::new(vec![
            TypedValue::Real(real),
            TypedValue::Integer(1),
            TypedValue::Boolean(true),
            TypedValue::Text("corner-tt".to_owned()),
            TypedValue::Real(1.0),
        ])
    };

    let distinct_signed_zeroes =
        SourceDataset::new(source, columns.clone(), vec![row(0.0), row(-0.0)]).unwrap();
    assert_eq!(distinct_signed_zeroes.rows().len(), 2);

    assert_eq!(
        SourceDataset::new(source, columns, vec![row(0.0), row(-0.0), row(0.0)]),
        Err(VisualizationError::DuplicateCoordinateRow(2))
    );
}

#[test]
fn one_hundred_thousand_exact_coordinate_rows_validate_in_one_pass() {
    const ROW_COUNT: usize = 100_000;
    let source = binding(9);
    let columns = vec![
        SourceColumn::new(
            "sample",
            "Sample",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )
        .unwrap(),
        SourceColumn::new(
            "v(out)",
            "V(out)",
            ValueType::Real,
            ColumnRole::Signal,
            Some("V".to_owned()),
        )
        .unwrap(),
    ];
    let rows = (0..ROW_COUNT)
        .map(|index| {
            SourceRow::new(vec![
                TypedValue::Integer(index as i64),
                TypedValue::Real(index as f64 * 1.0e-6),
            ])
        })
        .collect();

    let dataset = SourceDataset::new(source, columns, rows).unwrap();
    assert_eq!(dataset.rows().len(), ROW_COUNT);
}

#[test]
fn transaction_is_atomic_and_preserves_identity_on_rollback() {
    let (mut document, _) = document();
    let before = document.clone();
    let result = document.transact(
        document.revision(),
        vec![
            DocumentEdit::AddPage {
                title: "Review".to_owned(),
            },
            DocumentEdit::AddPane {
                page_id: PageId::allocate(999).unwrap(),
                title: "Invalid".to_owned(),
                kind: PaneKind::Cartesian,
            },
        ],
    );
    assert!(matches!(result, Err(VisualizationError::EntityNotFound(_))));
    assert_eq!(document, before);
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddPage {
                title: "Review".to_owned(),
            }],
        )
        .unwrap();
    assert_eq!(
        receipt.created,
        vec![EntityRef::Page(PageId::allocate(3).unwrap())]
    );
}

#[test]
fn stale_revision_and_source_digest_rebinding_do_not_commit() {
    let (mut document, source) = document();
    let stale = ObjectRevision::new(document.revision().get() + 1).unwrap();
    assert!(matches!(
        document.transact(
            stale,
            vec![DocumentEdit::AddPage {
                title: "X".to_owned()
            }]
        ),
        Err(VisualizationError::RevisionConflict { .. })
    ));
    let before = document.clone();
    let conflicting = DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([7; 32]));
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![DocumentEdit::AttachDataset(dataset(conflicting, 0.0))]
        ),
        Err(VisualizationError::SourceDigestMismatch { .. })
    ));
    assert_eq!(document, before);
}

#[test]
fn full_presentation_graph_validates_and_cascade_creates_tombstones() {
    let (mut document, source) = document();
    let pane = document.panes()[0].id;
    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Time".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("s".to_owned()),
                    range: Some(AxisRange::new(0.0, 2.0).unwrap()),
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Voltage".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let x_axis = match receipt.created[0] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    let y_axis = match receipt.created[1] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    let trace_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id: pane,
                binding: source,
                signal_key: "v(out)".to_owned(),
                coordinate_key: "time".to_owned(),
                row_predicates: Vec::new(),
                x_axis_id: x_axis,
                y_axis_id: y_axis,
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let trace = match trace_receipt.created[0] {
        EntityRef::Trace(id) => id,
        _ => unreachable!(),
    };
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddCursor {
                    pane_id: pane,
                    axis_id: x_axis,
                    position: TypedValue::Real(1.0),
                    label: "C1".to_owned(),
                },
                DocumentEdit::AddMarker {
                    pane_id: pane,
                    trace_id: trace,
                    coordinate: TypedValue::Real(1.0),
                    label: "M1".to_owned(),
                },
                DocumentEdit::AddMeasurement {
                    pane_id: pane,
                    trace_ids: vec![trace],
                    kind: MeasurementKind::Maximum,
                    label: "Peak".to_owned(),
                },
                DocumentEdit::AddAnnotation {
                    pane_id: pane,
                    anchor: AnnotationAnchor::Trace {
                        trace_id: trace,
                        coordinate: TypedValue::Real(1.0),
                    },
                    text: "Nominal peak".to_owned(),
                },
            ],
        )
        .unwrap();
    let removed = document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Pane(pane))],
        )
        .unwrap();
    assert!(removed.tombstoned.contains(&EntityRef::Trace(trace)));
    assert!(removed.tombstoned.contains(&EntityRef::Pane(pane)));
    assert!(document.traces().is_empty());
    assert_eq!(document.tombstones().len(), removed.tombstoned.len());
}

#[test]
fn comparison_policy_produces_auditable_pass_and_fail_receipts() {
    let baseline = binding(3);
    let candidate = binding(4);
    let document = VisualizationDocument::new(
        "Comparison",
        vec![dataset(baseline, 0.0), dataset(candidate, 0.01)],
    )
    .unwrap();
    let request = |absolute| ComparisonRequest {
        baseline,
        candidate,
        signal_keys: vec!["v(out)".to_owned()],
        policy: ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance: NumericTolerance::new(absolute, 0.0).unwrap(),
            require_identical_units: true,
            execution: ComparisonExecutionContract::default(),
        },
    };
    assert_eq!(
        document.compare(&request(0.02)).unwrap().disposition,
        ComparisonDisposition::Passed
    );
    let failed = document.compare(&request(0.001)).unwrap();
    assert_eq!(failed.disposition, ComparisonDisposition::Failed);
    assert_eq!(failed.rows_compared, 3);
    assert_eq!(failed.signals[0].failed_rows, 3);
}

#[test]
fn comparison_rejects_mismatched_bindings_and_inconsistent_receipts() {
    let baseline = binding(31);
    let candidate = binding(32);
    let baseline_data = dataset(baseline, 0.0);
    let candidate_data = dataset(candidate, 0.0);
    let policy = ComparisonPolicy {
        row_alignment: RowAlignmentPolicy::RequireIdentical,
        tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
        require_identical_units: true,
        execution: ComparisonExecutionContract::default(),
    };
    let request = ComparisonRequest {
        baseline: binding(33),
        candidate,
        signal_keys: vec!["v(out)".to_owned()],
        policy: policy.clone(),
    };
    assert!(matches!(
        compare_source_datasets(&baseline_data, &candidate_data, &request),
        Err(VisualizationError::InvalidValue {
            field: "comparison.binding",
            ..
        })
    ));

    let malformed = ComparisonReceipt {
        baseline,
        candidate,
        policy: policy.clone(),
        rows_compared: 3,
        signals: vec![SignalComparison {
            signal_key: "v(out)".to_owned(),
            compared_rows: 2,
            failed_rows: 3,
            maximum_absolute_error: f64::NAN,
            maximum_relative_error: 0.0,
        }],
        disposition: ComparisonDisposition::Failed,
    };
    assert!(malformed.validate_structure().is_err());
}

#[test]
fn comparison_rejects_overlong_signal_keys_before_result_work() {
    let baseline = binding(35);
    let candidate = binding(36);
    let baseline_data = dataset(baseline, 0.0);
    let candidate_data = dataset(candidate, 0.0);
    let policy = ComparisonPolicy {
        row_alignment: RowAlignmentPolicy::RequireIdentical,
        tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
        require_identical_units: true,
        execution: ComparisonExecutionContract::default(),
    };
    let overlong = ComparisonRequest {
        baseline,
        candidate,
        signal_keys: vec!["s".repeat(MAX_VISUALIZATION_KEY_BYTES + 1)],
        policy,
    };
    assert!(matches!(
        compare_source_datasets(&baseline_data, &candidate_data, &overlong),
        Err(VisualizationError::InvalidValue {
            field: "comparison.signal-key",
            ..
        })
    ));
}

#[test]
fn comparison_exact_intersection_never_synthesizes_rows() {
    let baseline = binding(5);
    let candidate = binding(6);
    let candidate_data = SourceDataset::new(
        candidate,
        dataset(candidate, 0.0).columns().to_vec(),
        vec![SourceRow::new(vec![
            TypedValue::Real(1.0),
            TypedValue::Real(1.0),
        ])],
    )
    .unwrap();
    let document =
        VisualizationDocument::new("Intersection", vec![dataset(baseline, 0.0), candidate_data])
            .unwrap();
    let receipt = document
        .compare(&ComparisonRequest {
            baseline,
            candidate,
            signal_keys: vec!["v(out)".to_owned()],
            policy: ComparisonPolicy {
                row_alignment: RowAlignmentPolicy::ExactIntersection,
                tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
                require_identical_units: true,
                execution: ComparisonExecutionContract::default(),
            },
        })
        .unwrap();
    assert_eq!(receipt.rows_compared, 1);
}

#[test]
fn composed_page_and_bound_pane_commit_exact_source_identity() {
    let (mut document, source) = document();
    let analysis_id = AnalysisInstanceId::new();
    let page_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddComposedPage(NewPage {
                title: "Publication".to_owned(),
                layout: PageLayout::Columns,
                template_id: "design-review".to_owned(),
                update_policy: PageUpdatePolicy::FreezeFigureRevision,
            })],
        )
        .unwrap();
    let page_id = match page_receipt.created[0] {
        EntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let pane_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Exact transient".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id,
                    dataset: source,
                }),
                placement: PanePlacement::Primary,
                plotted: Vec::new(),
            })],
        )
        .unwrap();
    let pane_id = match pane_receipt.created[0] {
        EntityRef::Pane(id) => id,
        _ => unreachable!(),
    };
    let page = document
        .pages()
        .iter()
        .find(|page| page.id == page_id)
        .unwrap();
    assert_eq!(page.layout, PageLayout::Columns);
    assert_eq!(page.template_id, "design-review");
    assert_eq!(page.update_policy, PageUpdatePolicy::FreezeFigureRevision);
    let pane = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .unwrap();
    assert_eq!(pane.viewer_id, "viewer-waveform");
    assert_eq!(pane.binding.unwrap().analysis_id, analysis_id);
    assert_eq!(pane.binding.unwrap().dataset, source);
    assert_eq!(pane.placement, PanePlacement::Primary);
    assert_eq!(pane.order, 0);
    let restored: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(restored, document);
}

#[test]
fn report_page_assignment_reorder_and_link_inheritance_are_atomic() {
    let (mut document, source) = document();
    let page_id = document.pages()[0].id;
    let primary = document.panes()[0].id;
    let binding = PaneDataBinding {
        analysis_id: AnalysisInstanceId::new(),
        dataset: source,
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id: primary,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(binding),
            }],
        )
        .unwrap();
    let second_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Second".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(binding),
                placement: PanePlacement::Below {
                    anchor_pane_id: primary,
                },
                plotted: Vec::new(),
            })],
        )
        .unwrap();
    let second = second_receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Pane(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    let horizontal_axis = |document: &VisualizationDocument, pane_id| {
        document
            .axes()
            .iter()
            .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
            .unwrap()
            .id
    };
    let primary_x = horizontal_axis(&document, primary);
    let second_x = horizontal_axis(&document, second);
    let link_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddLinkGroup {
                label: "Shared X".to_owned(),
                kind: LinkKind::HorizontalViewport,
                members: vec![EntityRef::Axis(primary_x), EntityRef::Axis(second_x)],
            }],
        )
        .unwrap();
    let link_id = link_receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::LinkGroup(id) => Some(*id),
            _ => None,
        })
        .unwrap();

    let third_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Third".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(binding),
                placement: PanePlacement::Below {
                    anchor_pane_id: second,
                },
                plotted: Vec::new(),
            })],
        )
        .unwrap();
    let third = third_receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Pane(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    let third_x = horizontal_axis(&document, third);
    assert!(
        document
            .link_groups()
            .iter()
            .find(|group| group.id == link_id)
            .unwrap()
            .members
            .contains(&EntityRef::Axis(third_x))
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::ReorderPagePanes {
                page_id,
                pane_ids: vec![third, primary, second],
            }],
        )
        .unwrap();
    let mut ordered = document
        .panes()
        .iter()
        .filter(|pane| pane.page_id == page_id)
        .collect::<Vec<_>>();
    ordered.sort_by_key(|pane| pane.order);
    assert_eq!(
        ordered.iter().map(|pane| pane.id).collect::<Vec<_>>(),
        vec![third, primary, second]
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AssignPaneToReportPage {
                pane_id: primary,
                page_title: "Publication".to_owned(),
                template_id: "design-review".to_owned(),
                update_policy: PageUpdatePolicy::FreezeFigureRevision,
            }],
        )
        .unwrap();
    let publication = document
        .pages()
        .iter()
        .find(|page| page.title == "Publication")
        .unwrap();
    assert_eq!(publication.template_id, "design-review");
    assert_eq!(
        publication.update_policy,
        PageUpdatePolicy::FreezeFigureRevision
    );
    assert_eq!(
        document
            .panes()
            .iter()
            .find(|pane| pane.id == primary)
            .unwrap()
            .page_id,
        publication.id
    );
}

#[test]
fn invalid_pane_binding_and_single_pane_layout_roll_back_atomically() {
    let (mut document, source) = document();
    let page_id = document.pages()[0].id;
    let primary = document.panes()[0].id;
    let wrong_digest = DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([99; 32]));
    let before = document.clone();
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Mismatched viewer".to_owned(),
                kind: PaneKind::Table,
                viewer_id: "viewer-waveform".to_owned(),
                binding: None,
                placement: PanePlacement::Below {
                    anchor_pane_id: primary,
                },
                plotted: Vec::new(),
            })],
        ),
        Err(VisualizationError::InvalidValue {
            field: "pane.kind",
            ..
        })
    ));
    assert_eq!(document, before);
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Invalid".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id: AnalysisInstanceId::new(),
                    dataset: wrong_digest,
                }),
                placement: PanePlacement::Below {
                    anchor_pane_id: primary,
                },
                plotted: Vec::new(),
            })],
        ),
        Err(VisualizationError::SourceDigestMismatch { .. })
    ));
    assert_eq!(document, before);

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddPane {
                page_id,
                title: "Second".to_owned(),
                kind: PaneKind::Table,
            }],
        )
        .unwrap();
    let before = document.clone();
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![DocumentEdit::SetPageComposition {
                page_id,
                layout: PageLayout::SinglePane,
                template_id: "engineering-dark".to_owned(),
                update_policy: PageUpdatePolicy::RefreshLinkedFigures,
            }],
        ),
        Err(VisualizationError::InvalidValue {
            field: "page.layout",
            ..
        })
    ));
    assert_eq!(document, before);
}

#[test]
fn pane_placement_orders_are_stable_across_insert_move_and_remove() {
    let (mut document, _) = document();
    let page_id = document.pages()[0].id;
    let primary = document.panes()[0].id;
    let first = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Below".to_owned(),
                kind: PaneKind::Table,
                viewer_id: "viewer-table".to_owned(),
                binding: None,
                placement: PanePlacement::Below {
                    anchor_pane_id: primary,
                },
                plotted: Vec::new(),
            })],
        )
        .unwrap();
    let below = match first.created[0] {
        EntityRef::Pane(id) => id,
        _ => unreachable!(),
    };
    let second = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: "Right".to_owned(),
                kind: PaneKind::Histogram,
                viewer_id: "viewer-histogram".to_owned(),
                binding: None,
                placement: PanePlacement::RightOf {
                    anchor_pane_id: primary,
                },
                plotted: Vec::new(),
            })],
        )
        .unwrap();
    let right = match second.created[0] {
        EntityRef::Pane(id) => id,
        _ => unreachable!(),
    };
    assert_eq!(
        document
            .panes()
            .iter()
            .find(|pane| pane.id == right)
            .unwrap()
            .order,
        1
    );
    assert_eq!(
        document
            .panes()
            .iter()
            .find(|pane| pane.id == below)
            .unwrap()
            .order,
        2
    );
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::PlacePane {
                pane_id: below,
                page_id,
                placement: PanePlacement::RightOf {
                    anchor_pane_id: right,
                },
            }],
        )
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Pane(right))],
        )
        .unwrap();
    let mut panes: Vec<_> = document.panes().iter().collect();
    panes.sort_by_key(|pane| pane.order);
    assert_eq!(
        panes.iter().map(|pane| pane.order).collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(panes[0].placement, PanePlacement::Primary);
    assert!(matches!(panes[1].placement, PanePlacement::Below { .. }));
}

#[test]
fn pane_family_policy_roundtrips_typed_dimensions_and_accessible_encodings() {
    let source = binding(41);
    let mut document =
        VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
    let pane_id = document.panes()[0].id;
    let policy = family_policy();
    policy.validate().unwrap();

    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: AnalysisInstanceId::new(),
                        dataset: source,
                    }),
                },
                DocumentEdit::SetPaneFamilyPresentation {
                    pane_id,
                    policy: Some(policy.clone()),
                },
            ],
        )
        .unwrap();

    assert_eq!(receipt.edit_count, 2);
    assert_eq!(document.panes()[0].family_policy.as_ref(), Some(&policy));
    let serialized = serde_json::to_string_pretty(&document).unwrap();
    let restored: VisualizationDocument = serde_json::from_str(&serialized).unwrap();
    assert_eq!(restored, document);
    assert!(serialized.contains("okabe-ito-categorical"));
    assert!(serialized.contains("preserve-as-not-run"));
}

#[test]
fn inaccessible_or_source_incompatible_family_policy_rolls_back_every_edit() {
    let source = binding(42);
    let mut document =
        VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id: AnalysisInstanceId::new(),
                    dataset: source,
                }),
            }],
        )
        .unwrap();

    let mut inaccessible = family_policy();
    inaccessible.encodings.retain(|encoding| {
        !matches!(
            encoding,
            FamilyEncodingMap::Label { .. } | FamilyEncodingMap::Facet { .. }
        )
    });
    inaccessible.facet_layout = None;
    let before = document.clone();
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![
                DocumentEdit::Rename {
                    entity: EntityRef::Pane(pane_id),
                    value: "Must roll back".to_owned(),
                },
                DocumentEdit::SetPaneFamilyPresentation {
                    pane_id,
                    policy: Some(inaccessible),
                },
            ],
        ),
        Err(VisualizationError::InvalidValue {
            field: "family.encodings.accessibility",
            ..
        })
    ));
    assert_eq!(document, before);

    let mut incompatible = family_policy();
    incompatible.x_dimension.dimension = dimension("frequency", ValueType::Real);
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![DocumentEdit::SetPaneFamilyPresentation {
                pane_id,
                policy: Some(incompatible),
            }],
        ),
        Err(VisualizationError::InvalidValue {
            field: "pane.family-policy.dimension",
            ..
        })
    ));
    assert_eq!(document, before);
}

#[test]
fn typed_family_filter_rejects_type_drift_before_commit() {
    let mut policy = family_policy();
    policy.filter = Some(FamilyFilterExpression {
        source: "temperature >= hot".to_owned(),
        predicate: FamilyPredicate::Compare {
            dimension: dimension("temperature", ValueType::Real),
            operator: FamilyComparisonOperator::GreaterThanOrEqual,
            value: TypedValue::Text("hot".to_owned()),
        },
    });
    assert!(matches!(
        policy.validate(),
        Err(VisualizationError::InvalidValue {
            field: "family.filter.value",
            ..
        })
    ));
}

/// One pane with one `V(out)` trace, ready to keep a measurement on.
fn document_with_trace() -> (VisualizationDocument, PaneId, TraceId) {
    let (mut document, source) = document();
    let pane = document.panes()[0].id;
    let axes = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Time".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("s".to_owned()),
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Voltage".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let [EntityRef::Axis(x_axis), EntityRef::Axis(y_axis)] = axes.created[..] else {
        unreachable!()
    };
    let trace = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id: pane,
                binding: source,
                signal_key: "v(out)".to_owned(),
                coordinate_key: "time".to_owned(),
                row_predicates: Vec::new(),
                x_axis_id: x_axis,
                y_axis_id: y_axis,
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let EntityRef::Trace(trace) = trace.created[0] else {
        unreachable!()
    };
    (document, pane, trace)
}

/// A yield judged against [1.5, 4.5].
fn family_measure(statistic: FamilyStatistic) -> FamilyMeasure {
    FamilyMeasure {
        statistic,
        lower: Some(1.5),
        upper: Some(4.5),
    }
}

/// A family measurement keeps how it reports its members, never what they
/// read, and reads back the same.
#[test]
fn a_family_measurement_keeps_its_statistic_and_limits_and_reads_back() {
    let (mut document, pane, _) = document_with_trace();
    let measure = family_measure(FamilyStatistic::Yield);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddFamilyMeasurement {
                pane_id: pane,
                expression: "avg(V(out))".to_owned(),
                window: MeasurementWindow::WholeRecord,
                family: measure,
            }],
        )
        .unwrap();
    assert_eq!(document.measurements()[0].family, Some(measure));

    let encoded = serde_json::to_value(&document).unwrap();
    assert!(encoded["measurements"][0].get("value").is_none());
    let decoded: VisualizationDocument = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded.measurements()[0].family, Some(measure));
}

/// A measurement the reader authors is bound to the result, not to what the
/// pane plots: it names its signals, analog or digital, in its expression,
/// as `.MEASURE` does, and keeps no trace. Removing the pane's trace leaves
/// it standing, and a stored one that names a trace is refused.
#[test]
fn an_authored_measurement_names_its_signals_and_no_trace() {
    let (mut document, pane, trace) = document_with_trace();
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddScalarMeasurement {
                    pane_id: pane,
                    expression: "delay(D(samp), D(eoc), 0.5, \"falling\", 1, 0.5, \"rising\", 1)"
                        .to_owned(),
                    window: MeasurementWindow::WholeRecord,
                },
                DocumentEdit::AddFamilyMeasurement {
                    pane_id: pane,
                    expression: "gain_dc".to_owned(),
                    window: MeasurementWindow::WholeRecord,
                    family: family_measure(FamilyStatistic::Yield),
                },
            ],
        )
        .unwrap();
    let mut naming_a_trace = serde_json::to_value(&document).unwrap();
    naming_a_trace["measurements"][0]["trace_ids"] = serde_json::json!([trace]);
    let refused = serde_json::from_value::<VisualizationDocument>(naming_a_trace).unwrap_err();
    assert!(
        refused.to_string().contains("names its signals"),
        "{refused}"
    );

    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Trace(trace))],
        )
        .unwrap();
    let encoded = serde_json::to_value(&document).unwrap();
    assert!(encoded["measurements"][0].get("trace_ids").is_none());
    let decoded: VisualizationDocument = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded.measurements().len(), 2);
    assert!(
        decoded
            .measurements()
            .iter()
            .all(|measurement| measurement.trace_ids.is_empty())
    );
}

/// A family measurement's limits are finite and in order, and a yield has
/// at least one to be judged against.
#[test]
fn a_family_measurement_states_limits_a_yield_can_be_judged_against() {
    let (mut document, pane, _) = document_with_trace();
    let add = |family: FamilyMeasure| DocumentEdit::AddFamilyMeasurement {
        pane_id: pane,
        expression: "avg(V(out))".to_owned(),
        window: MeasurementWindow::WholeRecord,
        family,
    };
    let revision = document.revision();
    let unlimited = FamilyMeasure {
        lower: None,
        upper: None,
        ..family_measure(FamilyStatistic::Yield)
    };
    assert!(document.transact(revision, vec![add(unlimited)]).is_err());
    let crossed = FamilyMeasure {
        lower: Some(4.5),
        upper: Some(1.5),
        ..family_measure(FamilyStatistic::Mean)
    };
    assert!(document.transact(revision, vec![add(crossed)]).is_err());
    let endless = FamilyMeasure {
        upper: Some(f64::INFINITY),
        ..family_measure(FamilyStatistic::Mean)
    };
    assert!(document.transact(revision, vec![add(endless)]).is_err());
    document
        .transact(
            revision,
            vec![add(FamilyMeasure {
                lower: None,
                upper: None,
                ..family_measure(FamilyStatistic::StdDev)
            })],
        )
        .unwrap();
}

/// The ids of the entities of `kind` a transaction created, in order.
fn created<T>(
    receipt: &VisualizationTransactionReceipt,
    kind: fn(&EntityRef) -> Option<T>,
) -> Vec<T> {
    receipt.created.iter().filter_map(kind).collect()
}

#[test]
fn a_document_holds_three_hundred_panes_and_ten_thousand_cursors() {
    let (mut document, _) = document();
    let receipt = document
        .transact(
            document.revision(),
            (0..300)
                .map(|index| DocumentEdit::AddPage {
                    title: format!("Page {index}"),
                })
                .collect(),
        )
        .unwrap();
    let pages = created(&receipt, |entity| match entity {
        EntityRef::Page(id) => Some(*id),
        _ => None,
    });
    let receipt = document
        .transact(
            document.revision(),
            pages
                .iter()
                .map(|page_id| DocumentEdit::AddPane {
                    page_id: *page_id,
                    title: "Pane".to_owned(),
                    kind: PaneKind::Cartesian,
                })
                .collect(),
        )
        .unwrap();
    let pane = created(&receipt, |entity| match entity {
        EntityRef::Pane(id) => Some(*id),
        _ => None,
    })[0];
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddAxis(NewAxis {
                pane_id: pane,
                label: "Time".to_owned(),
                orientation: AxisOrientation::Horizontal,
                scale: AxisScale::Linear,
                unit: Some("s".to_owned()),
                range: None,
            })],
        )
        .unwrap();
    let axis_id = created(&receipt, |entity| match entity {
        EntityRef::Axis(id) => Some(*id),
        _ => None,
    })[0];
    document
        .transact(
            document.revision(),
            (0..10_000)
                .map(|index| DocumentEdit::AddCursor {
                    pane_id: pane,
                    axis_id,
                    position: TypedValue::Real(f64::from(index)),
                    label: format!("C{index}"),
                })
                .collect(),
        )
        .unwrap();
    assert_eq!(document.panes().len(), 301);
    assert_eq!(document.cursors().len(), 10_000);
    let restored: VisualizationDocument =
        serde_json::from_value(serde_json::to_value(&document).unwrap()).unwrap();
    assert_eq!(restored, document, "it reads back whole");
}

/// An iterator that holds nothing and claims to hold `0` items.
struct Claims(usize);

impl Iterator for Claims {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0, Some(self.0))
    }
}

#[test]
fn a_hostile_view_file_is_refused_quickly_and_reserves_nothing_it_claims() {
    use serde::de::value::{Error, MapDeserializer, SeqDeserializer};

    // A row that claims more values than a table may hold is refused before
    // one value is read or reserved.
    let row = MapDeserializer::<_, Error>::new(std::iter::once((
        "values",
        SeqDeserializer::<_, Error>::new(Claims(usize::MAX)),
    )));
    let error = SourceRow::deserialize(row).unwrap_err();
    assert!(error.to_string().contains("resource limit"), "{error}");
    // A sequence of the document's own claims as much and reserves nothing.
    let traces =
        Vec::<Trace>::deserialize(SeqDeserializer::<_, Error>::new(Claims(usize::MAX))).unwrap();
    assert!(traces.is_empty());

    // A file of a hundred thousand traces and as many markers, the last
    // naming a trace the file lacks, is checked in one pass and refused by
    // the trace it names.
    let (mut document, source) = document();
    let pane = document.panes()[0].id;
    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Time".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("s".to_owned()),
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "Voltage".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let axes = created(&receipt, |entity| match entity {
        EntityRef::Axis(id) => Some(*id),
        _ => None,
    });
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id: pane,
                binding: source,
                signal_key: "v(out)".to_owned(),
                coordinate_key: "time".to_owned(),
                row_predicates: Vec::new(),
                x_axis_id: axes[0],
                y_axis_id: axes[1],
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let trace = created(&receipt, |entity| match entity {
        EntityRef::Trace(id) => Some(*id),
        _ => None,
    })[0];
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddMarker {
                pane_id: pane,
                trace_id: trace,
                coordinate: TypedValue::Real(1.0),
                label: "M".to_owned(),
            }],
        )
        .unwrap();
    const COUNT: u64 = 100_000;
    let mut encoded = serde_json::to_value(&document).unwrap();
    let one_trace = encoded["traces"][0].clone();
    let one_marker = encoded["markers"][0].clone();
    let first = 1_000_000_u64;
    encoded["traces"] = (0..COUNT)
        .map(|index| {
            let mut trace = one_trace.clone();
            trace["id"] = serde_json::json!(first + index);
            trace
        })
        .collect();
    encoded["markers"] = (0..COUNT)
        .map(|index| {
            let mut marker = one_marker.clone();
            marker["id"] = serde_json::json!(first + COUNT + index);
            marker["trace_id"] = serde_json::json!(first + index + u64::from(index + 1 == COUNT));
            marker
        })
        .collect();
    encoded["next_serial"] = serde_json::json!(first + 2 * COUNT);
    let started = std::time::Instant::now();
    let error = serde_json::from_value::<VisualizationDocument>(encoded).unwrap_err();
    assert!(
        error.to_string().contains(&(first + COUNT).to_string()),
        "refused by the trace it names: {error}"
    );
    println!("refused in {:?}", started.elapsed());
}

/// A kept measurement is its definition: its expression and window change
/// in place, it moves in the list, and none of it stores a value.
#[test]
fn a_measurement_changes_its_definition_and_its_place() {
    let (mut document, pane, _) = document_with_trace();
    let add = |expression: &str| DocumentEdit::AddScalarMeasurement {
        pane_id: pane,
        expression: expression.to_owned(),
        window: MeasurementWindow::WholeRecord,
    };
    document
        .transact(
            document.revision(),
            vec![add("max(V(out))"), add("min(V(out))")],
        )
        .unwrap();
    let [first, second] = [0, 1].map(|index| document.measurements()[index].id);
    assert_eq!(document.measurements()[0].label, "max(V(out))");
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetMeasurement {
                    measurement_id: first,
                    expression: "avg(V(out))".to_owned(),
                    window: MeasurementWindow::VisibleRange,
                },
                DocumentEdit::MoveMeasurement {
                    measurement_id: second,
                    index: 0,
                },
            ],
        )
        .unwrap();
    let order: Vec<_> = document
        .measurements_in_order()
        .iter()
        .map(|kept| kept.id)
        .collect();
    assert_eq!(order, [second, first]);
    let changed = document.measurements_in_order()[1];
    assert_eq!(changed.expression.as_deref(), Some("avg(V(out))"));
    assert_eq!(changed.window, MeasurementWindow::VisibleRange);
    let refused = [
        DocumentEdit::MoveMeasurement {
            measurement_id: first,
            index: 2,
        },
        DocumentEdit::SetMeasurement {
            measurement_id: first,
            expression: " ".to_owned(),
            window: MeasurementWindow::WholeRecord,
        },
    ];
    for edit in refused {
        assert!(document.transact(document.revision(), vec![edit]).is_err());
    }
    let decoded: VisualizationDocument =
        serde_json::from_value(serde_json::to_value(&document).unwrap()).unwrap();
    assert_eq!(decoded.measurements(), document.measurements());
}
