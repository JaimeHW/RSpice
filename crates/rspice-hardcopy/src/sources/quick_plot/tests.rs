use super::*;

/// A retained document marker reaches the page as itself.
///
/// The two stores allocate independently and label distinctly, so a `D`
/// marker must arrive with its own tag rather than being restated as a quick
/// one. A spec limit arrives as the full-height line the sheet draws, because
/// it constrains the axis position rather than one curve.
#[test]
fn a_retained_document_marker_and_a_spec_limit_reach_the_page_as_themselves() {
    use rspice_results::result_presentation::MarkerKind;

    let series = vec![QuickResultSeries {
        identity: "trace-identity".to_owned(),
        label: "V(out)".to_owned(),
        points: vec![(0.0, 0.0), (1.0, 4.0), (2.0, 8.0)],
    }];
    let overlay = RetainedQuickViewOverlay::try_new(
        None,
        None,
        None,
        vec![
            RetainedQuickMarker {
                label: "D7 · settling".to_owned(),
                kind: MarkerKind::Peak,
                x: 1.0,
                trace_name: Some("V(out)".to_owned()),
            },
            RetainedQuickMarker {
                label: "M2 · upper limit".to_owned(),
                kind: MarkerKind::Spec,
                x: 2.0,
                trace_name: None,
            },
            RetainedQuickMarker {
                label: "M3 · other pane".to_owned(),
                kind: MarkerKind::Note,
                x: 1.0,
                trace_name: Some("V(elsewhere)".to_owned()),
            },
        ],
        RetainedCursorInterpolation::default(),
        None,
    )
    .unwrap();

    let plot = quick_plot_from_series(ResultViewer::Waves, "Results", 0, series, Some(&overlay))
        .expect("plot resolves");

    assert_eq!(
        plot.markers
            .iter()
            .map(|marker| marker.label.as_str())
            .collect::<Vec<_>>(),
        ["D7 · settling"],
        "a spec limit is a line, and a marker whose trace is not on this page is skipped"
    );
    assert_eq!(plot.markers[0].source_y_bits, Some(4.0f64.to_bits()));
    assert_eq!(
        plot.cursors
            .iter()
            .map(|cursor| cursor.label.as_str())
            .collect::<Vec<_>>(),
        ["M2 · upper limit"]
    );
    let limit = &plot.cursors[0];
    assert_eq!(limit.start.x_um, limit.end.x_um);
    assert_eq!(limit.source_x_bits, 2.0f64.to_bits());

    let reversed = RetainedQuickViewport {
        x: Some((2.0, 1.0)),
        y: None,
    };
    assert!(
        RetainedQuickViewOverlay::try_new(
            None,
            None,
            None,
            Vec::new(),
            RetainedCursorInterpolation::default(),
            Some(reversed),
        )
        .is_err()
    );
    let mut encoded = serde_json::to_value(&overlay).unwrap();
    encoded["viewport"] = serde_json::to_value(reversed).unwrap();
    let decoded = serde_json::from_value(encoded).unwrap();
    assert!(matches!(
        quick_plot_from_series(
            ResultViewer::Waves,
            "Results",
            0,
            vec![QuickResultSeries {
                identity: "trace-identity".to_owned(),
                label: "V(out)".to_owned(),
                points: vec![(0.0, 0.0), (2.0, 8.0)],
            }],
            Some(&decoded),
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));
}
