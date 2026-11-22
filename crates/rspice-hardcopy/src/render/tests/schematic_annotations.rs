//! A printed schematic draws its documentation shapes in the stroke, dash
//! and fill the canvas draws them in.

use super::schematic_labels::{compile, sheet};
use super::*;
use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeGeometry, SymbolDash, SymbolFill, SymbolStroke,
    SymbolStyle,
};

/// The documentation layer's nominal pen, in micrometres.
const DOCUMENTATION_PEN_UM: u64 = 220;

fn styled(id: u64, geometry: DocumentationShapeGeometry, style: SymbolStyle) -> DocumentationShape {
    DocumentationShape::new(id, geometry)
        .unwrap()
        .with_style(style)
}

/// A heavy dashed rectangle shaded inside, and a thin dotted polygon filled
/// solid, print in those strokes, patterns and fills; each fill prints before
/// every outline, so it lies under what the shape encloses.
#[test]
fn a_shapes_stroke_dash_and_fill_print_as_the_canvas_draws_them() {
    let mut printed = sheet(Vec::new());
    printed.documentation_shapes = vec![
        styled(
            1,
            DocumentationShapeGeometry::Rectangle {
                first: rspice_design_model::Point::new(0, 0),
                opposite: rspice_design_model::Point::new(200, 100),
            },
            SymbolStyle {
                stroke: SymbolStroke::Heavy,
                dash: SymbolDash::Dashed,
                fill: SymbolFill::Background,
            },
        ),
        styled(
            2,
            DocumentationShapeGeometry::Polygon {
                points: vec![
                    rspice_design_model::Point::new(300, 0),
                    rspice_design_model::Point::new(400, 0),
                    rspice_design_model::Point::new(350, 80),
                ],
            },
            SymbolStyle {
                stroke: SymbolStroke::Thin,
                dash: SymbolDash::Dotted,
                fill: SymbolFill::Solid,
            },
        ),
        DocumentationShape::new(
            3,
            DocumentationShapeGeometry::Line {
                start: rspice_design_model::Point::new(0, 200),
                end: rspice_design_model::Point::new(200, 200),
            },
        )
        .unwrap(),
    ];
    let compiled = compile(&printed);

    let outline = |wanted: &dyn Fn(&ScenePrimitive) -> bool| {
        compiled
            .primitives
            .iter()
            .rposition(wanted)
            .expect("the outline prints")
    };
    let rectangle = outline(&|primitive| {
        matches!(
            primitive,
            ScenePrimitive::Rect {
                stroke: Some(_),
                ..
            }
        )
    });
    let ScenePrimitive::Rect {
        stroke: Some(stroke),
        fill: None,
        ..
    } = &compiled.primitives[rectangle]
    else {
        panic!("the rectangle prints as an outline");
    };
    assert_eq!(stroke.width.micrometres(), DOCUMENTATION_PEN_UM * 2);
    assert_eq!(stroke.pattern, StrokePattern::Dashed);

    let polygon =
        outline(&|primitive| matches!(primitive, ScenePrimitive::Polyline { closed: true, .. }));
    let ScenePrimitive::Polyline { stroke, .. } = &compiled.primitives[polygon] else {
        unreachable!();
    };
    assert_eq!(stroke.width.micrometres(), DOCUMENTATION_PEN_UM / 2);
    assert_eq!(stroke.pattern, StrokePattern::Dotted);

    let line = outline(&|primitive| matches!(primitive, ScenePrimitive::Line { .. }));
    let ScenePrimitive::Line { stroke, .. } = &compiled.primitives[line] else {
        unreachable!();
    };
    assert_eq!(stroke.width.micrometres(), DOCUMENTATION_PEN_UM);
    assert_eq!(stroke.pattern, StrokePattern::Solid);

    let fills = compiled
        .primitives
        .iter()
        .enumerate()
        .filter_map(|(index, primitive)| match primitive {
            ScenePrimitive::Rect {
                stroke: None,
                fill: Some(fill),
                ..
            }
            | ScenePrimitive::Polyline {
                fill: Some(fill), ..
            } => Some((index, *fill)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(fills.len(), 2, "{fills:?}");
    assert!(matches!(fills[0].1, SceneFill::Tint { .. }), "{fills:?}");
    assert!(matches!(fills[1].1, SceneFill::Solid { .. }), "{fills:?}");
    assert!(
        fills.iter().all(|(index, _)| *index < rectangle.min(line)),
        "a fill prints under the outlines"
    );
}
