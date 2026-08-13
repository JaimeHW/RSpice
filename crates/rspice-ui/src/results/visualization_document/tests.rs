//! Tests for document edits, family policy, and schema migration.
//!
//! A rejected edit must roll back everything it touched, a typed family
//! filter must catch type drift before commit, and every schema migration
//! must be deterministic for the same input document.

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
fn schema_three_migrates_to_pinned_tracking_without_inventing_plan_identity() {
    let (document, _) = document();
    let mut value = serde_json::to_value(document).unwrap();
    value["schema_version"] = serde_json::json!(3);
    value
        .as_object_mut()
        .expect("document JSON is an object")
        .remove("tracking");

    let restored: VisualizationDocument = serde_json::from_value(value).unwrap();
    assert_eq!(restored.tracking(), ResultDocumentTracking::pinned());
    assert_eq!(
        serde_json::to_value(restored).unwrap()["schema_version"],
        serde_json::json!(VisualizationDocument::SCHEMA_VERSION)
    );
}

#[test]
fn schema_five_traces_migrate_with_their_original_unfiltered_row_meaning() {
    let (mut document, source) = document();
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
    let mut legacy = serde_json::to_value(document).unwrap();
    legacy["schema_version"] = serde_json::json!(5);
    for trace in legacy["traces"].as_array_mut().unwrap() {
        trace.as_object_mut().unwrap().remove("row_predicates");
    }

    let migrated: VisualizationDocument = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        migrated.schema_version,
        VisualizationDocument::SCHEMA_VERSION
    );
    assert!(
        migrated
            .traces()
            .iter()
            .all(|trace| trace.row_predicates.is_empty())
    );
}

#[test]
fn latest_retarget_is_atomic_and_keeps_the_prior_immutable_snapshot() {
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
    assert_eq!(document.datasets().len(), 2);
    assert!(
        document
            .datasets()
            .iter()
            .any(|source| source.binding() == previous)
    );
    assert!(
        document
            .datasets()
            .iter()
            .any(|source| source.binding() == next)
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
    assert_eq!(document.datasets().len(), 2);
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
fn source_dataset_limits_are_enforced_at_construction_and_deserialization() {
    let source = binding(31);
    let columns = (0..MAX_SOURCE_COLUMNS)
        .map(|index| {
            SourceColumn::new(
                format!("c{index}"),
                format!("Column {index}"),
                ValueType::Real,
                if index == 0 {
                    ColumnRole::Coordinate
                } else {
                    ColumnRole::Signal
                },
                None,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let at_limit = SourceDataset::new(source, columns, Vec::new()).unwrap();

    let mut encoded = serde_json::to_value(&at_limit).unwrap();
    let encoded_columns = encoded["columns"].as_array_mut().unwrap();
    let extra_column = encoded_columns[0].clone();
    encoded_columns.push(extra_column);
    let error = serde_json::from_value::<SourceDataset>(encoded).unwrap_err();
    assert!(error.to_string().contains("resource limit"));

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
fn visualization_document_deserialization_rejects_oversized_entity_sequences() {
    let (document, _) = document();
    let mut encoded = serde_json::to_value(&document).unwrap();
    let pages = encoded["pages"].as_array_mut().unwrap();
    let page = pages[0].clone();
    while pages.len() <= MAX_VISUALIZATION_PAGES {
        pages.push(page.clone());
    }
    let error = serde_json::from_value::<VisualizationDocument>(encoded).unwrap_err();
    assert!(error.to_string().contains("resource limit"));
}

#[test]
fn atomic_transactions_reject_oversized_edit_batches_before_mutation() {
    let (mut document, _) = document();
    let before = document.clone();
    let page = document.pages()[0].id;
    let edits = (0..=MAX_VISUALIZATION_TRANSACTION_EDITS)
        .map(|_| DocumentEdit::Rename {
            entity: EntityRef::Page(page),
            value: "Bounded".to_owned(),
        })
        .collect();
    assert!(matches!(
        document.transact(document.revision(), edits),
        Err(VisualizationError::InvalidValue {
            field: "transaction.edits",
            ..
        })
    ));
    assert_eq!(document, before);
}

#[test]
fn nested_sequences_and_source_strings_are_bounded_during_deserialization() {
    let predicate = FamilyPredicate::All {
        predicates: vec![FamilyPredicate::Constant { value: true }],
    };
    let mut predicate_json = serde_json::to_value(predicate).unwrap();
    let child = predicate_json["predicates"][0].clone();
    let children = predicate_json["predicates"].as_array_mut().unwrap();
    while children.len() <= MAX_FAMILY_PREDICATE_CHILDREN {
        children.push(child.clone());
    }
    let predicate_error = serde_json::from_value::<FamilyPredicate>(predicate_json).unwrap_err();
    assert!(predicate_error.to_string().contains("resource limit"));

    let measurement = Measurement {
        id: MeasurementId::allocate(1).unwrap(),
        pane_id: PaneId::allocate(2).unwrap(),
        trace_ids: vec![TraceId::allocate(3).unwrap()],
        kind: MeasurementKind::Point,
        label: "Point".to_owned(),
        expression: None,
        value: None,
    };
    let mut measurement_json = serde_json::to_value(measurement).unwrap();
    let trace = measurement_json["trace_ids"][0].clone();
    let traces = measurement_json["trace_ids"].as_array_mut().unwrap();
    while traces.len() <= MAX_ENTITY_REFERENCES {
        traces.push(trace.clone());
    }
    let measurement_error = serde_json::from_value::<Measurement>(measurement_json).unwrap_err();
    assert!(measurement_error.to_string().contains("resource limit"));

    let source = dataset(binding(33), 0.0);
    let mut source_json = serde_json::to_value(source).unwrap();
    source_json["rows"][0]["values"][0] = serde_json::json!({
        "type": "text",
        "value": "x".repeat(MAX_SOURCE_TEXT_BYTES + 1),
    });
    let text_error = serde_json::from_value::<SourceDataset>(source_json).unwrap_err();
    assert!(text_error.to_string().contains("resource limit"));
}

#[test]
fn predicate_deserialization_enforces_shared_depth_and_total_node_budgets() {
    let leaf = FamilyPredicate::Constant { value: true };
    let branch = FamilyPredicate::All {
        predicates: vec![leaf.clone(); MAX_FAMILY_PREDICATE_CHILDREN],
    };
    let broad = FamilyPredicate::All {
        predicates: vec![branch; 16],
    };
    let broad_error =
        serde_json::from_value::<FamilyPredicate>(serde_json::to_value(broad).unwrap())
            .unwrap_err();
    assert!(broad_error.to_string().contains("1024-node"));

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

#[test]
fn document_aggregate_nested_resource_budgets_accept_boundaries_and_reject_overflow() {
    for (field, maximum) in [
        (
            "visualization-document.comparison-signals",
            MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL,
        ),
        (
            "visualization-document.measurement-trace-references",
            MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL,
        ),
        (
            "visualization-document.link-member-references",
            MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL,
        ),
    ] {
        assert_eq!(
            checked_bounded_sum(field, maximum - 1, 1, maximum).unwrap(),
            maximum
        );
        assert!(matches!(
            checked_bounded_sum(field, maximum, 1, maximum),
            Err(VisualizationError::InvalidValue { .. })
        ));
    }

    let measurement = Measurement {
        id: MeasurementId::allocate(1).unwrap(),
        pane_id: PaneId::allocate(2).unwrap(),
        trace_ids: vec![TraceId::allocate(3).unwrap(); MAX_ENTITY_REFERENCES],
        kind: MeasurementKind::Point,
        label: "Aggregate boundary".to_owned(),
        expression: None,
        value: None,
    };
    let mut encoded = serde_json::to_value(document().0).unwrap();
    encoded["measurements"] = serde_json::to_value(vec![measurement; 5]).unwrap();
    let error = serde_json::from_value::<VisualizationDocument>(encoded).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("aggregate nested resource limit")
    );
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
fn resource_hardening_preserves_extensions_and_bounded_schema_migrations() {
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
    for schema_version in [1, 2] {
        let mut legacy = serde_json::to_value(&document).unwrap();
        legacy["schema_version"] = serde_json::json!(schema_version);
        let restored: VisualizationDocument = serde_json::from_value(legacy).unwrap();
        assert_eq!(
            restored.schema_version,
            VisualizationDocument::SCHEMA_VERSION
        );
        assert_eq!(restored.datasets()[0].binding(), source_binding);
        assert_eq!(restored.datasets()[0].rows(), document.datasets()[0].rows());
    }
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
fn exact_query_returns_typed_values_and_never_interpolates() {
    let (document, source) = document();
    let exact = document
        .query_exact_row(&ExactRowQuery {
            binding: source,
            coordinates: vec![QueryCoordinate {
                column: "time".to_owned(),
                value: TypedValue::Real(1.0),
            }],
            projections: vec!["v(out)".to_owned()],
        })
        .unwrap();
    assert_eq!(exact.row_index, 1);
    assert!(exact.values[0].1.exact_eq(&TypedValue::Real(1.0)));
    assert_eq!(
        document.query_exact_row(&ExactRowQuery {
            binding: source,
            coordinates: vec![QueryCoordinate {
                column: "time".to_owned(),
                value: TypedValue::Real(1.5),
            }],
            projections: vec!["v(out)".to_owned()],
        }),
        Err(VisualizationError::InterpolationRequired)
    );
}

#[test]
fn digest_mismatch_and_incomplete_queries_fail_explicitly() {
    let (document, source) = document();
    let wrong = DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([9; 32]));
    assert!(matches!(
        document.query_exact_row(&ExactRowQuery {
            binding: wrong,
            coordinates: vec![],
            projections: vec!["v(out)".to_owned()],
        }),
        Err(VisualizationError::SourceDigestMismatch { .. })
    ));
    assert_eq!(
        document.query_exact_row(&ExactRowQuery {
            binding: source,
            coordinates: vec![],
            projections: vec!["v(out)".to_owned()],
        }),
        Err(VisualizationError::IncompleteCoordinateQuery)
    );
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
fn comparison_rejects_oversized_or_overlong_signal_keys_before_result_work() {
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
        policy: policy.clone(),
    };
    assert!(matches!(
        compare_source_datasets(&baseline_data, &candidate_data, &overlong),
        Err(VisualizationError::InvalidValue {
            field: "comparison.signal-key",
            ..
        })
    ));

    let oversized = ComparisonRequest {
        baseline,
        candidate,
        signal_keys: vec!["v(out)".to_owned(); MAX_COMPARISON_SIGNALS + 1],
        policy,
    };
    assert!(matches!(
        compare_source_datasets(&baseline_data, &candidate_data, &oversized),
        Err(VisualizationError::InvalidValue {
            field: "comparison.signal-keys",
            ..
        })
    ));
    let wire_error =
        serde_json::from_value::<ComparisonRequest>(serde_json::to_value(oversized).unwrap())
            .unwrap_err();
    assert!(wire_error.to_string().contains("resource limit"));
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
fn progressive_operation_enforces_progress_cancel_and_recovery_transitions() {
    let (mut document, _) = document();
    let (_, mut operation) = document
        .start_operation(document.revision(), ProgressiveOperationKind::Export, 10)
        .unwrap();
    operation
        .advance(4, ContentDigest::from_bytes([2; 32]), None)
        .unwrap();
    assert!(matches!(
        operation.advance(3, ContentDigest::from_bytes([3; 32]), None),
        Err(VisualizationError::InvalidProgress { .. })
    ));
    operation.request_cancel().unwrap();
    operation.confirm_cancelled().unwrap();
    operation.recover().unwrap();
    assert_eq!(operation.recovery_count(), 1);
    operation
        .advance(
            10,
            ContentDigest::from_bytes([4; 32]),
            Some(ContentDigest::from_bytes([5; 32])),
        )
        .unwrap();
    assert!(matches!(
        operation.state(),
        ProgressiveOperationState::Completed { .. }
    ));
}

#[test]
fn invalid_operation_updates_leave_operation_unchanged() {
    let (mut document, _) = document();
    let (_, mut operation) = document
        .start_operation(document.revision(), ProgressiveOperationKind::Transform, 2)
        .unwrap();
    let before = operation.clone();
    assert_eq!(
        operation.advance(
            1,
            ContentDigest::from_bytes([3; 32]),
            Some(ContentDigest::from_bytes([4; 32]))
        ),
        Err(VisualizationError::UnexpectedOutputDigest)
    );
    assert_eq!(operation, before);
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

#[test]
fn schema_v2_documents_migrate_to_an_unconfigured_family_policy_deterministically() {
    let source = binding(43);
    let mut document =
        VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
    let pane_id = document.panes()[0].id;
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
                    policy: Some(family_policy()),
                },
            ],
        )
        .unwrap();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(2);

    let migrated: VisualizationDocument = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        migrated.schema_version,
        VisualizationDocument::SCHEMA_VERSION
    );
    assert!(
        migrated
            .panes()
            .iter()
            .all(|pane| pane.family_policy.is_none())
    );
}

#[test]
fn schema_v1_documents_migrate_page_and_pane_composition_deterministically() {
    let (mut document, _) = document();
    let page_id = document.pages()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddPane {
                page_id,
                title: "Legacy table".to_owned(),
                kind: PaneKind::Table,
            }],
        )
        .unwrap();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(1);
    for page in legacy["pages"].as_array_mut().unwrap() {
        let page = page.as_object_mut().unwrap();
        page.remove("layout");
        page.remove("template_id");
        page.remove("update_policy");
    }
    for pane in legacy["panes"].as_array_mut().unwrap() {
        let pane = pane.as_object_mut().unwrap();
        pane.remove("viewer_id");
        pane.remove("binding");
        pane.remove("placement");
        pane.remove("order");
    }
    let migrated: VisualizationDocument = serde_json::from_value(legacy).unwrap();
    assert_eq!(
        migrated.schema_version,
        VisualizationDocument::SCHEMA_VERSION
    );
    assert_eq!(migrated.pages()[0].layout, PageLayout::Rows);
    assert_eq!(migrated.pages()[0].template_id, "engineering-dark");
    assert_eq!(migrated.panes()[0].viewer_id, "viewer-waveform");
    assert_eq!(migrated.panes()[1].viewer_id, "viewer-table");
    assert_eq!(migrated.panes()[0].order, 0);
    assert_eq!(migrated.panes()[1].order, 1);
    assert_eq!(
        migrated.panes()[1].placement,
        PanePlacement::Below {
            anchor_pane_id: migrated.panes()[0].id,
        }
    );
}
