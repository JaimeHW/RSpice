//! A pane axis' curves plotted against a signal or an expression: the plots
//! and their sweep spans survive a save and reload, are replaced per pane in
//! order, leave with their pane, and need schema 13. A trace may name another
//! signal column as its coordinate.

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

fn plot(pane: PaneId, unit: &str, abscissa: Abscissa) -> XyPlot {
    XyPlot {
        pane_id: pane,
        unit: unit.to_owned(),
        abscissa,
        sweep_range: None,
        follow_sweep_view: false,
        log_abscissa: false,
    }
}

fn plots(pane: PaneId, plots: Vec<XyPlot>) -> DocumentEdit {
    DocumentEdit::SetXyPlots {
        pane_id: pane,
        plots,
    }
}

fn signal(name: &str) -> Abscissa {
    Abscissa::Signal {
        name: name.to_owned(),
    }
}

#[test]
fn a_plot_against_a_signal_and_its_sweep_span_survive_a_save_and_reload() {
    let (mut document, pane, _) = document();
    let expression = Abscissa::Expression {
        text: "V(in)-V(out)".to_owned(),
    };
    let settled = XyPlot {
        sweep_range: Some(AxisRange::new(1.0e-6, 2.0e-6).unwrap()),
        follow_sweep_view: true,
        log_abscissa: true,
        ..plot(pane, "A", expression.clone())
    };
    document
        .transact(
            document.revision(),
            vec![plots(
                pane,
                vec![plot(pane, "V", signal("V(in)")), settled.clone()],
            )],
        )
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(saved["schema_version"], serde_json::json!(13));
    assert_eq!(
        saved["xy_plots"][0]["abscissa"],
        serde_json::json!({"kind": "signal", "name": "V(in)"})
    );
    // The whole record writes no span; a narrowed one writes its ends.
    assert!(saved["xy_plots"][0].get("sweep_range").is_none());
    assert_eq!(
        saved["xy_plots"][1]["sweep_range"],
        serde_json::json!({"minimum": 1.0e-6, "maximum": 2.0e-6})
    );
    // Following the pane's window and a logarithmic abscissa are written
    // only when on.
    for field in ["follow_sweep_view", "log_abscissa"] {
        assert!(saved["xy_plots"][0].get(field).is_none(), "{field}");
        assert_eq!(saved["xy_plots"][1][field], serde_json::json!(true));
    }
    let reloaded: VisualizationDocument = serde_json::from_value(saved).unwrap();
    assert_eq!(reloaded, document);
    assert_eq!(
        reloaded.xy_plots(),
        [plot(pane, "V", signal("V(in)")), settled]
    );
}

/// A pane's plots are replaced as one list, in the reader's order: two
/// plots of one axis against different signals stand side by side, and an
/// empty list takes them all away.
#[test]
fn a_panes_plots_are_replaced_in_order_and_an_empty_list_clears_them() {
    let (mut document, pane, _) = document();
    let first = vec![
        plot(pane, "V", signal("V(out)")),
        plot(pane, "V", signal("V(in)")),
    ];
    document
        .transact(document.revision(), vec![plots(pane, first.clone())])
        .unwrap();
    assert_eq!(document.xy_plots(), first);
    let reordered = vec![first[1].clone(), first[0].clone()];
    document
        .transact(document.revision(), vec![plots(pane, reordered.clone())])
        .unwrap();
    assert_eq!(document.xy_plots(), reordered);
    document
        .transact(document.revision(), vec![plots(pane, Vec::new())])
        .unwrap();
    assert!(document.xy_plots().is_empty());
    // With nothing plotted the document writes what it always wrote.
    let saved = serde_json::to_value(&document).unwrap();
    assert!(saved.get("xy_plots").is_none());
}

#[test]
fn a_plot_leaves_with_its_pane() {
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
                plots(pane, vec![plot(pane, "V", signal("V(in)"))]),
                plots(second, vec![plot(second, "V", signal("V(in)"))]),
            ],
        )
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Pane(second))],
        )
        .unwrap();
    assert_eq!(document.xy_plots(), [plot(pane, "V", signal("V(in)"))]);
}

/// A blank expression, an unknown pane, a plot listed under another pane, a
/// plot listed twice and a span that is no interval are each refused, and
/// leave the document as it was.
#[test]
fn a_malformed_plot_is_refused() {
    let (mut document, pane, _) = document();
    let refused = |document: &mut VisualizationDocument, edit: DocumentEdit| {
        let error = document
            .transact(document.revision(), vec![edit])
            .unwrap_err();
        assert!(document.xy_plots().is_empty());
        error
    };
    let blank = Abscissa::Expression {
        text: "  ".to_owned(),
    };
    let field = |error: VisualizationError| match error {
        VisualizationError::InvalidValue { field, .. } => field,
        other => panic!("{other:?}"),
    };
    assert_eq!(
        field(refused(
            &mut document,
            plots(pane, vec![plot(pane, "V", blank)])
        )),
        "xy-plot.expression"
    );
    let gone = PaneId::allocate(9_999).unwrap();
    let error = refused(
        &mut document,
        plots(gone, vec![plot(gone, "V", signal("V(in)"))]),
    );
    assert!(
        matches!(
            error,
            VisualizationError::EntityNotFound(EntityRef::Pane(_))
        ),
        "{error:?}"
    );
    assert_eq!(
        field(refused(
            &mut document,
            plots(pane, vec![plot(gone, "V", signal("V(in)"))])
        )),
        "xy-plot.pane-id"
    );
    assert_eq!(
        field(refused(
            &mut document,
            plots(
                pane,
                vec![
                    plot(pane, "V", signal("V(in)")),
                    plot(pane, "V", signal("V(in)")),
                ],
            )
        )),
        "xy-plot.abscissa"
    );
    let backwards = XyPlot {
        sweep_range: Some(AxisRange {
            minimum: 2.0e-6,
            maximum: 1.0e-6,
        }),
        ..plot(pane, "V", signal("V(in)"))
    };
    assert_eq!(
        field(refused(&mut document, plots(pane, vec![backwards.clone()]))),
        "xy-plot.sweep-range"
    );
    // A file that holds such a span is refused when read, too.
    let mut wire = serde_json::to_value(&document).unwrap();
    wire["xy_plots"] = serde_json::to_value([backwards]).unwrap();
    assert!(serde_json::from_value::<VisualizationDocument>(wire).is_err());
}

#[test]
fn a_schema_twelve_document_reads_unchanged_and_cannot_carry_a_plot() {
    let (mut document, pane, _) = document();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(12);
    let reloaded: VisualizationDocument = serde_json::from_value(legacy).unwrap();
    assert_eq!(reloaded, document);

    document
        .transact(
            document.revision(),
            vec![plots(pane, vec![plot(pane, "V", signal("V(in)"))])],
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
