use super::*;

fn assert_slots_are_finite_and_bounded(stage: Rect, slots: &[Rect], expected: usize) {
    assert_eq!(slots.len(), expected);
    for slot in slots {
        for value in [
            slot.min.x,
            slot.min.y,
            slot.max.x,
            slot.max.y,
            slot.width(),
            slot.height(),
        ] {
            assert!(value.is_finite(), "pane slot must remain finite: {slot:?}");
        }
        assert!(slot.width() >= 0.0 && slot.height() >= 0.0, "{slot:?}");
        assert!(
            slot.left() >= stage.left() && slot.top() >= stage.top(),
            "{slot:?}"
        );
        assert!(
            slot.right() <= stage.right() && slot.bottom() <= stage.bottom(),
            "{slot:?} exceeds {stage:?}"
        );
    }
}

#[test]
fn multi_pane_slots_never_exceed_narrow_or_short_result_stages() {
    let short = Rect::from_min_size(pos2(17.0, 23.0), vec2(480.0, 19.0));
    let rows = bounded_pane_slots(short, PageLayout::Rows, 7);
    assert_slots_are_finite_and_bounded(short, &rows, 7);

    let narrow = Rect::from_min_size(pos2(3.0, 5.0), vec2(13.0, 360.0));
    let columns = bounded_pane_slots(narrow, PageLayout::Columns, 8);
    assert_slots_are_finite_and_bounded(narrow, &columns, 8);

    let compact = Rect::from_min_size(pos2(11.0, 13.0), vec2(29.0, 17.0));
    let grid = bounded_pane_slots(compact, PageLayout::Grid { columns: 3 }, 11);
    assert_slots_are_finite_and_bounded(compact, &grid, 11);
}

#[test]
fn pane_slot_geometry_sanitizes_nonfinite_available_extents() {
    let size = finite_stage_size(vec2(f32::INFINITY, f32::NAN));
    assert_eq!(size, vec2(0.0, 0.0));
    let stage = Rect::from_min_size(pos2(0.0, 0.0), size);
    let slots = bounded_pane_slots(stage, PageLayout::Grid { columns: 2 }, 4);
    assert_slots_are_finite_and_bounded(stage, &slots, 4);
}
