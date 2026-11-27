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
fn pane_slot_geometry_sanitizes_nonfinite_available_extents() {
    let size = finite_stage_size(vec2(f32::INFINITY, f32::NAN));
    assert_eq!(size, vec2(0.0, 0.0));
    let stage = Rect::from_min_size(pos2(0.0, 0.0), size);
    let slots = bounded_pane_slots(stage, PageLayout::Grid { columns: 2 }, 4, 1.0);
    assert_slots_are_finite_and_bounded(stage, &slots, 4);
}

/// Every pane of a page is the same size to the physical pixel, two
/// neighbours stand one hairline apart, across and down, and the panes run
/// to the stage's edges but for the odd pixels rounding leaves there,
/// whatever the stage's size.
#[test]
fn panes_abut_equal_to_the_pixel_with_one_hairline_between() {
    for (ppp, width, height) in [
        (1.0, 1203.0, 707.0),
        (1.25, 1517.6, 811.2),
        (2.0, 2049.5, 1201.0),
    ] {
        let stage = Rect::from_min_size(pos2(37.0, 61.0), vec2(width, height));
        for (layout, count) in [
            (PageLayout::SinglePane, 1),
            (PageLayout::Columns, 3),
            (PageLayout::Rows, 2),
            (PageLayout::Grid { columns: 2 }, 4),
            (PageLayout::Grid { columns: 3 }, 9),
        ] {
            let slots = bounded_pane_slots(stage, layout, count, ppp);
            assert_slots_are_finite_and_bounded(stage, &slots, count);
            let pixels = |value: f32| (value * ppp).round();
            let first = slots[0];
            for slot in &slots {
                assert_eq!(
                    pixels(slot.width()),
                    pixels(first.width()),
                    "{layout:?} {slot:?}"
                );
                assert_eq!(
                    pixels(slot.height()),
                    pixels(first.height()),
                    "{layout:?} {slot:?}"
                );
            }
            let hairline = pixels(PANE_DIVIDER);
            let (columns, rows) = grid_of(layout, count);
            for (index, slot) in slots.iter().enumerate() {
                if index % columns + 1 < columns {
                    let next = slots[index + 1];
                    assert_eq!(
                        pixels(next.left()) - pixels(slot.right()),
                        hairline,
                        "{layout:?} at {ppp}: {slot:?} beside {next:?}"
                    );
                }
                if let Some(below) = slots.get(index + columns) {
                    assert_eq!(
                        pixels(below.top()) - pixels(slot.bottom()),
                        hairline,
                        "{layout:?} at {ppp}: {slot:?} over {below:?}"
                    );
                }
            }
            let left = slots
                .iter()
                .map(|slot| slot.left())
                .fold(f32::MAX, f32::min);
            let top = slots.iter().map(|slot| slot.top()).fold(f32::MAX, f32::min);
            let right = slots
                .iter()
                .map(|slot| slot.right())
                .fold(f32::MIN, f32::max);
            let bottom = slots
                .iter()
                .map(|slot| slot.bottom())
                .fold(f32::MIN, f32::max);
            // What rounding leaves over is fewer pixels than there are panes
            // along the axis, shared between its two ends.
            let across =
                pixels(left) - pixels(stage.left()) + pixels(stage.right()) - pixels(right);
            let down = pixels(top) - pixels(stage.top()) + pixels(stage.bottom()) - pixels(bottom);
            assert!(
                (0.0..columns as f32).contains(&across),
                "{layout:?} at {ppp}: {across} px left over across"
            );
            assert!(
                (0.0..rows as f32).contains(&down),
                "{layout:?} at {ppp}: {down} px left over down"
            );
        }
    }
}

/// A pane is named by where it sits on its page, once the page has more
/// than one.
#[test]
fn a_pane_is_named_by_its_place_on_the_page() {
    let places = |layout, count| {
        (0..count)
            .map(|index| pane_place(layout, index, count))
            .collect::<Vec<_>>()
    };
    assert_eq!(places(PageLayout::SinglePane, 1), [None]);
    assert_eq!(
        places(PageLayout::Rows, 2),
        [Some("Top".to_owned()), Some("Bottom".to_owned())]
    );
    assert_eq!(
        places(PageLayout::Grid { columns: 2 }, 4),
        ["Top left", "Top right", "Bottom left", "Bottom right"]
            .map(|place| Some(place.to_owned()))
    );
    assert_eq!(
        places(PageLayout::Columns, 3),
        ["Left", "Middle", "Right"].map(|place| Some(place.to_owned()))
    );
    assert_eq!(pane_place(PageLayout::Rows, 2, 5), Some("Row 3".to_owned()));
}

/// A kept measurement reads in engineering notation in its unit at the
/// document's digits, never as a fixed six decimals that print a microvolt
/// as 0.000001, and one that read nothing says why.
#[test]
fn a_measurement_reads_in_engineering_notation_at_the_document_digits() {
    assert_eq!(
        measurement_text("rms(V(out))", Ok((12.345_678, "V", None)), 7),
        "rms(V(out)) = 12.34568 V"
    );
    assert_eq!(
        measurement_text("max(V(out))", Ok((1.234_567_8e-6, "V", None)), 4),
        "max(V(out)) = 1.235 \u{b5}V"
    );
    assert_eq!(
        measurement_text("max(V(gone))", Err("This run has no such analysis."), 4),
        "max(V(gone)): This run has no such analysis."
    );
}

/// A family measurement leads with the statistic it keeps and follows with
/// the whole distribution, the yield against its limits and who failed.
#[test]
fn a_family_measurement_reads_its_distribution_and_yield() {
    use rspice_results::visualization_document::{FamilyMeasurementSummary, FamilyStatistic};
    let mut family = FamilyMeasurementSummary {
        statistic: FamilyStatistic::Yield,
        members: 6,
        measured: 5,
        mean: 3.0,
        std_dev: Some(2.5_f64.sqrt()),
        min: 1.0,
        max: 5.0,
        lower: Some(1.5),
        upper: Some(4.5),
        passed: Some(3),
        failing: vec![
            "Trial 0".to_owned(),
            "Trial 4".to_owned(),
            "Trial 5".to_owned(),
        ],
    };
    assert_eq!(
        measurement_text(
            "avg(V(out)) yield",
            Ok((50.0, "%", Some(FamilyStatistic::Yield))),
            4
        ),
        "avg(V(out)) yield = 50 %"
    );
    assert_eq!(
        family_text(&family, 3),
        "n 5 of 6 \u{b7} mean 3 \u{b7} \u{3c3} 1.58 \u{b7} min 1 \u{b7} max 5 \u{b7} \
         yield 3/6 [1.5, 4.5] \u{b7} fails Trial 0, Trial 4 +1"
    );
    family.lower = None;
    family.failing = (0..6).map(|index| format!("Trial {index}")).collect();
    family.passed = Some(0);
    assert!(
        family_text(&family, 3)
            .ends_with("yield 0/6 \u{2264} 4.5 \u{b7} fails Trial 0, Trial 1 +4")
    );
    // A kept mean states the yield's share itself.
    family.statistic = FamilyStatistic::Mean;
    assert!(
        family_text(&family, 3)
            .ends_with("yield 0/6 (0 %) \u{2264} 4.5 \u{b7} fails Trial 0, Trial 1 +4")
    );
}
