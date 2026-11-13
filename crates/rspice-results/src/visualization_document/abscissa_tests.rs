//! A unit axis of a pane drawn against a signal or an expression: the choice
//! survives a save and reload, is cleared by choosing the sweep again, leaves
//! with its pane, and needs schema 13. A trace may name another signal
//! column as its coordinate.

use super::*;

fn binding() -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([5; 32]))
}

fn dataset(binding: DatasetBinding) -> SourceDataset {
    let column = |key: &str, role| {
        SourceColumn::new(key, key, ValueType::Real, role, None).expect("valid column")
    };
    let row = |t: f64, vin: f64, vout: f64| {
        SourceRow::new(vec![
            TypedValue::Real(t),
            TypedValue::Real(vin),
            TypedValue::Real(vout),
        ])
    };
    SourceDataset::new(
        binding,
        vec![
            column("time", ColumnRole::Coordinate),
            column("V(in)", ColumnRole::Signal),
            column("V(out)", ColumnRole::Signal),
        ],
        vec![
            row(0.0, 0.0, 1.8),
            row(1.0e-6, 0.9, 0.9),
            row(2.0e-6, 1.8, 0.0),
        ],
    )
    .expect("valid dataset")
}

fn document() -> (VisualizationDocument, PaneId, DatasetBinding) {
    let source = binding();
    let document = VisualizationDocument::new("Transfer", vec![dataset(source)]).unwrap();
    let pane = document.panes()[0].id;
    (document, pane, source)
}

fn against(pane: PaneId, unit: &str, abscissa: Option<Abscissa>) -> DocumentEdit {
    DocumentEdit::SetUnitAbscissa {
        pane_id: pane,
        unit: unit.to_owned(),
        abscissa,
    }
}

fn signal(name: &str) -> Abscissa {
    Abscissa::Signal {
        name: name.to_owned(),
    }
}

#[test]
fn a_unit_axis_drawn_against_a_signal_survives_a_save_and_reload() {
    let (mut document, pane, _) = document();
    let expression = Abscissa::Expression {
        text: "V(in)-V(out)".to_owned(),
    };
    document
        .transact(
            document.revision(),
            vec![
                against(pane, "V", Some(signal("V(in)"))),
                against(pane, "A", Some(expression.clone())),
            ],
        )
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(saved["schema_version"], serde_json::json!(13));
    assert_eq!(
        saved["unit_abscissas"][0]["abscissa"],
        serde_json::json!({"kind": "signal", "name": "V(in)"})
    );
    let reloaded: VisualizationDocument = serde_json::from_value(saved).unwrap();
    assert_eq!(reloaded, document);
    let held: Vec<(&str, &Abscissa)> = reloaded
        .unit_abscissas()
        .iter()
        .map(|choice| (choice.unit.as_str(), &choice.abscissa))
        .collect();
    assert_eq!(held, [("V", &signal("V(in)")), ("A", &expression)]);
}

/// Choosing again replaces the axis' abscissa, and choosing none draws it
/// against its analysis' sweep again.
#[test]
fn choosing_again_replaces_the_abscissa_and_choosing_none_clears_it() {
    let (mut document, pane, _) = document();
    document
        .transact(
            document.revision(),
            vec![
                against(pane, "V", Some(signal("V(in)"))),
                against(pane, "V", Some(signal("V(out)"))),
            ],
        )
        .unwrap();
    assert_eq!(document.unit_abscissas().len(), 1);
    assert_eq!(document.unit_abscissas()[0].abscissa, signal("V(out)"));
    document
        .transact(document.revision(), vec![against(pane, "V", None)])
        .unwrap();
    assert!(document.unit_abscissas().is_empty());
    // With nothing chosen the document writes what it always wrote.
    let saved = serde_json::to_value(&document).unwrap();
    assert!(saved.get("unit_abscissas").is_none());
}

#[test]
fn a_unit_abscissa_leaves_with_its_pane() {
    let (mut document, pane, _) = document();
    let page = document.pages()[0].id;
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddPane {
                page_id: page,
                title: "Second".to_owned(),
                kind: PaneKind::Cartesian,
            }],
        )
        .unwrap();
    let second = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Pane(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![
                against(pane, "V", Some(signal("V(in)"))),
                against(second, "V", Some(signal("V(in)"))),
            ],
        )
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Pane(second))],
        )
        .unwrap();
    assert_eq!(document.unit_abscissas().len(), 1);
    assert_eq!(document.unit_abscissas()[0].pane_id, pane);
}

#[test]
fn a_blank_expression_or_an_unknown_pane_is_refused() {
    let (mut document, pane, _) = document();
    let blank = Abscissa::Expression {
        text: "  ".to_owned(),
    };
    let error = document
        .transact(document.revision(), vec![against(pane, "V", Some(blank))])
        .unwrap_err();
    assert!(
        matches!(
            error,
            VisualizationError::InvalidValue {
                field: "unit-abscissa.expression",
                ..
            }
        ),
        "{error:?}"
    );
    let gone = PaneId::allocate(9_999).unwrap();
    let error = document
        .transact(
            document.revision(),
            vec![against(gone, "V", Some(signal("V(in)")))],
        )
        .unwrap_err();
    assert!(
        matches!(
            error,
            VisualizationError::EntityNotFound(EntityRef::Pane(_))
        ),
        "{error:?}"
    );
    assert!(document.unit_abscissas().is_empty());
}

#[test]
fn a_schema_twelve_document_reads_unchanged_and_cannot_carry_an_abscissa() {
    let (mut document, pane, _) = document();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(12);
    let reloaded: VisualizationDocument = serde_json::from_value(legacy).unwrap();
    assert_eq!(reloaded, document);

    document
        .transact(
            document.revision(),
            vec![against(pane, "V", Some(signal("V(in)")))],
        )
        .unwrap();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(12);
    let error = serde_json::from_value::<VisualizationDocument>(legacy)
        .unwrap_err()
        .to_string();
    assert!(error.contains("schema 13"), "{error}");
}

/// A trace may read one signal column against another, as a transfer curve
/// does, and still never against itself.
#[test]
fn a_trace_reads_against_another_signal_column_but_not_against_itself() {
    let (mut document, pane, source) = document();
    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "V(in)".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id: pane,
                    label: "V(out)".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let axes: Vec<AxisId> = receipt
        .created
        .iter()
        .filter_map(|entity| match entity {
            EntityRef::Axis(id) => Some(*id),
            _ => None,
        })
        .collect();
    let trace = |coordinate: &str| NewTrace {
        pane_id: pane,
        binding: source,
        signal_key: "V(out)".to_owned(),
        coordinate_key: coordinate.to_owned(),
        row_predicates: Vec::new(),
        x_axis_id: axes[0],
        y_axis_id: axes[1],
        label: "V(out) vs V(in)".to_owned(),
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(trace("V(in)"))],
        )
        .expect("a signal reads against another signal");
    let error = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(trace("V(out)"))],
        )
        .unwrap_err();
    assert!(
        matches!(
            error,
            VisualizationError::InvalidValue {
                field: "trace.source-columns",
                ..
            }
        ),
        "{error:?}"
    );
    let reloaded: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(reloaded, document);
    assert_eq!(reloaded.traces()[0].coordinate_key, "V(in)");
}
