//! What the compiler makes of a plot's declared cursors and markers.
//!
//! The semantic capture carries the reader's A/B cursors and anchored markers
//! onto the page; this is the other half of that claim — that the compiler
//! turns each declaration into ink at the position it declared. Structure,
//! not pixels: a marker is a closed three-point polyline with its tag beside
//! it, a cursor is a two-point line whose X does not move between endpoints.
//!
//! The plot is stated here rather than resolved from a session because this
//! module sits above `app_state` in the workbench layer order and must not
//! reach back down into it. What a session produces is the sources module's
//! claim, and it is made there.

use super::*;
use crate::workbench::hardcopy_adapters::sources::{
    SemanticPlotCursor, SemanticPlotMarker, SemanticPlotTrace,
};

fn point(x_um: i64, y_um: i64) -> SemanticPoint {
    SemanticPoint::new(x_um, y_um)
}

fn overlay_plot() -> SemanticPlot {
    SemanticPlot {
        viewer: crate::workbench::ResultViewer::Waves,
        page_id: 1,
        pane_id: 0,
        x_scale: crate::results::visualization_document::AxisScale::Linear,
        y_scale: crate::results::visualization_document::AxisScale::Linear,
        axis_ticks: Vec::new(),
        traces: vec![SemanticPlotTrace {
            trace_id: 11,
            label: "V(out)".to_owned(),
            paths: vec![vec![point(12_700, 130_175), point(241_300, 12_700)]],
            source_samples: vec![
                (0.0f64.to_bits(), 0.0f64.to_bits()),
                (2.0f64.to_bits(), 8.0f64.to_bits()),
            ],
        }],
        cursors: vec![
            SemanticPlotCursor {
                cursor_id: 21,
                label: "A".to_owned(),
                source_x_bits: 0.5f64.to_bits(),
                start: point(70_000, 130_175),
                end: point(70_000, 12_700),
            },
            SemanticPlotCursor {
                cursor_id: 22,
                label: "B".to_owned(),
                source_x_bits: 1.5f64.to_bits(),
                start: point(184_000, 130_175),
                end: point(184_000, 12_700),
            },
        ],
        markers: vec![SemanticPlotMarker {
            marker_id: 31,
            label: "M1 · settling".to_owned(),
            trace_id: Some(11),
            source_x_bits: Some(1.0f64.to_bits()),
            source_y_bits: Some(4.0f64.to_bits()),
            position: Some(point(127_000, 71_437)),
        }],
        annotations: Vec::new(),
    }
}

#[test]
fn a_declared_marker_and_cursor_pair_compile_to_ink_at_their_positions() {
    let bounds = SemanticBounds::try_new(point(0, 0), point(254_000, 142_875)).unwrap();
    let extent = bounds.content_extent().unwrap();
    let mapping = PrintMappingTable::default();
    let mut compiler =
        SemanticSceneCompiler::new(bounds, extent, &mapping, SchematicHardcopySetup::default());
    let plot = overlay_plot();
    compiler.plot(&plot).expect("the plot compiles");

    // Two cursors: two-point lines that do not move in X between endpoints,
    // in the order and at the abscissae the plot declared.
    let verticals = compiler
        .primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Polyline {
                points,
                closed: false,
                ..
            } if points.len() == 2 && points[0].x == points[1].x => Some(points[0].x),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(verticals.len(), 2, "both declared cursors are drawn");
    assert!(verticals[0] < verticals[1], "A is drawn left of B");
    assert_eq!(
        verticals[0],
        Length::from_micrometres(u64::try_from(plot.cursors[0].start.x_um).unwrap()),
        "cursor A is drawn at the abscissa it declared"
    );

    // One marker: a closed three-point tag centred on its declared position.
    let triangles = compiler
        .primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Polyline {
                points,
                closed: true,
                ..
            } if points.len() == 3 => Some(points.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(triangles.len(), 1, "the declared marker is drawn");
    let apex = triangles[0][0];
    assert_eq!(
        apex.x,
        Length::from_micrometres(u64::try_from(plot.markers[0].position.unwrap().x_um).unwrap()),
        "the marker's apex sits on the abscissa it declared"
    );

    // A page that declares neither carries neither.
    let mut bare_compiler =
        SemanticSceneCompiler::new(bounds, extent, &mapping, SchematicHardcopySetup::default());
    let mut bare = overlay_plot();
    bare.cursors.clear();
    bare.markers.clear();
    bare_compiler.plot(&bare).expect("the bare plot compiles");
    assert!(
        !bare_compiler
            .primitives
            .iter()
            .any(|primitive| matches!(primitive, ScenePrimitive::Polyline { closed: true, .. })),
        "no marker tag is drawn for a page that declares none"
    );
}
