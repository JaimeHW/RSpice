//! Calculator expressions plotted on a pane are kept as text and dataset
//! identities, survive a save and reload, follow a re-pointed run, and leave
//! with their pane.

use super::*;

fn binding() -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([3; 32]))
}

fn dataset(binding: DatasetBinding) -> SourceDataset {
    let column = |key: &str, role| {
        SourceColumn::new(key, key, ValueType::Real, role, None).expect("valid column")
    };
    SourceDataset::new(
        binding,
        vec![
            column("time", ColumnRole::Coordinate),
            column("V(out)", ColumnRole::Signal),
            column("I(vdd)", ColumnRole::Signal),
        ],
        vec![
            SourceRow::new(vec![TypedValue::Real(0.0); 3]),
            SourceRow::new(vec![
                TypedValue::Real(1.0e-6),
                TypedValue::Real(1.0),
                TypedValue::Real(-2.0e-3),
            ]),
        ],
    )
    .expect("valid dataset")
}

fn power(pane: PaneId, source: DatasetBinding) -> ExpressionTrace {
    ExpressionTrace {
        pane_id: pane,
        text: "V(out)*I(vdd)".to_owned(),
        datasets: vec![source],
        unit: None,
        visible: true,
    }
}

fn document() -> (VisualizationDocument, PaneId, DatasetBinding) {
    let source = binding();
    let document = VisualizationDocument::new("Power", vec![dataset(source)]).unwrap();
    let pane = document.panes()[0].id;
    (document, pane, source)
}

#[test]
fn an_expression_survives_a_save_and_reload_as_text_and_dataset_ids() {
    let (mut document, pane, source) = document();
    let mut declared = power(pane, source);
    declared.text = "V(out)/I(vdd)".to_owned();
    declared.unit = Some("Ω".to_owned());
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddExpressionTrace(power(pane, source)),
                DocumentEdit::AddExpressionTrace(declared.clone()),
            ],
        )
        .unwrap();

    let saved = serde_json::to_string(&document).unwrap();
    // Text and identities, never samples.
    let wire: serde_json::Value = serde_json::from_str(&saved).unwrap();
    assert_eq!(
        wire["expression_traces"][1],
        serde_json::json!({
            "pane_id": pane,
            "text": "V(out)/I(vdd)",
            "datasets": [source],
            "unit": "Ω",
            "visible": true,
        })
    );
    let reloaded: VisualizationDocument = serde_json::from_str(&saved).unwrap();
    assert_eq!(reloaded, document);
    assert_eq!(
        reloaded.expression_traces(),
        [power(pane, source), declared]
    );
    assert_eq!(
        reloaded.content_digest().unwrap(),
        document.content_digest().unwrap()
    );
}

#[test]
fn adding_an_expression_again_replaces_it_and_removing_it_takes_it_off() {
    let (mut document, pane, source) = document();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddExpressionTrace(power(pane, source))],
        )
        .unwrap();
    let mut hidden = power(pane, source);
    hidden.visible = false;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddExpressionTrace(hidden.clone())],
        )
        .unwrap();
    assert_eq!(document.expression_traces(), [hidden]);

    let remove = |text: &str| DocumentEdit::RemoveExpressionTrace {
        pane_id: pane,
        text: text.to_owned(),
    };
    let error = document
        .transact(document.revision(), vec![remove("V(in)")])
        .unwrap_err();
    assert!(error.to_string().contains("plots no expression"), "{error}");
    document
        .transact(document.revision(), vec![remove("V(out)*I(vdd)")])
        .unwrap();
    assert!(document.expression_traces().is_empty());
}

#[test]
fn an_expression_reads_attached_datasets_on_an_existing_pane() {
    let (mut document, pane, source) = document();
    let mut detached = power(pane, source);
    detached.datasets = vec![binding()];
    let mut blank = power(pane, source);
    blank.text = "  ".to_owned();
    let mut none = power(pane, source);
    none.datasets.clear();
    let mut unit = power(pane, source);
    unit.unit = Some(" ".to_owned());
    for trace in [detached, blank, none, unit] {
        assert!(
            document
                .transact(
                    document.revision(),
                    vec![DocumentEdit::AddExpressionTrace(trace.clone())]
                )
                .is_err(),
            "{trace:?}"
        );
    }
    assert!(document.expression_traces().is_empty());
}

#[test]
fn an_expression_leaves_with_its_pane() {
    let (mut document, pane, source) = document();
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
                DocumentEdit::AddExpressionTrace(power(pane, source)),
                DocumentEdit::AddExpressionTrace(power(second, source)),
            ],
        )
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Pane(second))],
        )
        .unwrap();
    assert_eq!(document.expression_traces(), [power(pane, source)]);
}

/// A horizontal axis switches scale; a range read on the linear axis that a
/// logarithmic one cannot show is dropped rather than kept.
#[test]
fn an_axis_switches_scale_and_drops_a_range_a_log_axis_cannot_show() {
    let (mut document, pane, _) = document();
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddAxis(NewAxis {
                pane_id: pane,
                label: "Time".to_owned(),
                orientation: AxisOrientation::Horizontal,
                scale: AxisScale::Linear,
                unit: Some("s".to_owned()),
                range: Some(AxisRange::new(0.0, 1.0e-6).unwrap()),
            })],
        )
        .unwrap();
    let axis = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Axis(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetAxisScale {
                axis_id: axis,
                scale: AxisScale::Logarithmic,
            }],
        )
        .unwrap();
    let held = document.axes().iter().find(|held| held.id == axis).unwrap();
    assert_eq!(held.scale, AxisScale::Logarithmic);
    assert_eq!(held.range, None);
    let reloaded: VisualizationDocument =
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
    assert_eq!(reloaded, document);
}
