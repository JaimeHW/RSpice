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
    let slots = bounded_pane_slots(stage, PageLayout::Grid { columns: 2 }, 4);
    assert_slots_are_finite_and_bounded(stage, &slots, 4);
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
