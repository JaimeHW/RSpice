use super::*;

fn assert_editor_spans_disjoint(left: EditorSpan, right: EditorSpan) {
    assert!(
        left.end() <= right.start + f32::EPSILON,
        "editor spans overlap: {left:?} and {right:?}"
    );
}

#[test]
fn compact_expression_editor_reserves_add_and_stacks_error() {
    let layout = expr_editor_layout(350.0, 28.0, 42.0, Some(420.0));

    assert!(layout.stack_error);
    assert_eq!(layout.add.end(), 350.0);
    assert_eq!(layout.error.width, 0.0);
    assert!(layout.input.width > EXPR_EDITOR_MIN_INLINE_INPUT);
    assert_editor_spans_disjoint(layout.label, layout.input);
    assert_editor_spans_disjoint(layout.input, layout.add);
}

#[test]
fn wide_expression_editor_bounds_inline_error_without_starving_input() {
    let layout = expr_editor_layout(900.0, 28.0, 42.0, Some(640.0));

    assert!(!layout.stack_error);
    assert!(layout.error.width > 0.0);
    assert!(layout.error.width <= 900.0 * 0.28);
    assert!(layout.input.width >= EXPR_EDITOR_MIN_INLINE_INPUT);
    assert_editor_spans_disjoint(layout.label, layout.input);
    assert_editor_spans_disjoint(layout.input, layout.error);
    assert_editor_spans_disjoint(layout.error, layout.add);
    assert_eq!(layout.add.end(), 900.0);
}

#[test]
fn expression_editor_geometry_stays_inside_pathological_widths() {
    for width in [0.0, 20.0, 64.0, 180.0, 560.0] {
        let layout = expr_editor_layout(width, 28.0, 42.0, None);
        for span in [layout.label, layout.input, layout.error, layout.add] {
            assert!(span.start >= 0.0);
            assert!(span.end() <= width + f32::EPSILON);
        }
        assert_editor_spans_disjoint(layout.label, layout.input);
        assert_editor_spans_disjoint(layout.input, layout.add);
    }
}
